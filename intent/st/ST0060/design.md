---
verblock: "25 Aug 2026:v0.1: vc - the intent vault design, specced against Lamplight's twelve measured requirements"
---

# `intent vault` -- design

**Status: SPECCED, NOT STARTED. Post-3.0.0 by hv's ruling.** Requirements are supplied by Lamplight (ST0358) as a requirements provider; Intent owns the capability and builds it once for the estate. This document is the design those requirements are ratified against -- the traceability table at the end is the ratification instrument, and a requirement with no row there is a requirement this design does not yet answer.

## The problem, in one line

**A config file that holds credentials turns ordinary, correct-looking operations into credential disclosure** -- reading a port, diffing a profile, pasting a config into a bug report. Care at the call site cannot fix it, because the care must be perfect every time and the file gives no signal that any is needed. Provenance is measured, not hypothetical: on 2026-08-25 a session ran `cat ~/.lamplight/config.toml` to find a port and printed two live API keys into a transcript. **One field was wanted; the whole file was printed.** hv ruled the exposure low-risk in context and the habit the defect.

## Two collisions, and why they dissolve the same way

hv's direction was: put the vault in `intentd`, keep a master password in an OS-specific store. Two of Lamplight's measured requirements contradict that as stated.

| hv's direction              | the requirement it meets                                        | why it collides                                                                                              |
| --------------------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| put it in `intentd`         | **R3 -- no daemon, headless-safe** (SSH, container, no desktop) | a vault that needs a daemon is unreachable exactly where servers live                                        |
| master password in keychain | **R2 -- one behaviour on macOS and Linux**, one code path       | there is no universal Linux equivalent; `secret-tool` needs a D-Bus keyring daemon, absent on servers and CI |

**Both dissolve by separating the KEY from the STORE.** The store is the thing that must be identical and daemon-free everywhere; the key's home is the thing that is allowed to be platform-specific, because it is a cache and every rung of its ladder is optional. Once that split is made, hv's direction is not weakened -- it becomes the hardened posture rather than the only posture, and it survives the machines that have no keychain.

## Architecture

### Three layers, and only the middle one is platform-conditional

```
  material            per-project age ciphertext        identical on every platform, no daemon
     ^
  identity            age X25519 keypair, on disk       identical on every platform
     ^
  passphrase          ladder: env -> keychain -> prompt  the ONLY platform-conditional code
```

**The keychain holds the passphrase that decrypts the identity. It never holds a secret.** That is the whole reconciliation: the OS-specific part is confined to "where do I find a passphrase", which has a null implementation, and a machine with no keychain falls through to the next rung with the behaviour of `vault get` unchanged.

### Store layout

```
  ~/.intent/vault/
    identity.age        the age identity, mode 600 REQUIRED
    <project>.age       one encrypted store per project, encrypted to the identity's public key
```

**Per-machine identity, per-project store** (B1). The identity is per-machine because R4 requires the vault to be reachable before a project's toolchain exists -- bootstrapping a new project needs credentials before there is a project to hold them. The stores are per-project for three reasons that a single namespaced file cannot give: blast radius is bounded to one project, rotation is per-project, and `list` is scoped by construction rather than by a filter someone can omit.

### Entry names live INSIDE the ciphertext, and that decides two other things

**A store is one opaque blob: names, profiles and material are all inside the encryption.** There is no cleartext name header. Raised by Lamplight's verification as an R8/S1 interaction neither side had resolved, and it has to be decided here because AC-00.8 and AC-00.12 both depend on the answer.

**The alternative was a cleartext header so `list` could enumerate cheaply and while locked. It is rejected for one reason: the inventory is itself sensitive.** Knowing that `lamplight/prod/api-key` exists is reconnaissance, and a 644 store carrying a name header would disclose it -- which would make S1's `warn` on the store an understatement rather than a calibration. With names inside, a 644 store genuinely discloses nothing, and `warn` is the right severity for what is left: untidiness.

**What it costs, stated plainly.** `list` must decrypt, so it needs a reachable identity, so **`list` fails with `Locked` in the hardened posture on a host with no keyring.** That is correct rather than regrettable -- you cannot enumerate what you cannot open -- and it is invisible in the default posture, where there is no locked state at all. `audit` is unaffected: it reads config files for material-shaped values and never touches the store.

### The identity has two postures, and the plain one is the default

- **Plain identity, mode 600.** Equivalent to an SSH key with no passphrase. Fully non-interactive on every platform. This is the default, and it is what keeps R3 and R10 true without qualification.
- **Passphrase-protected identity** (`age -p`, scrypt recipient -- stock age, not a new lane). Opt-in hardening. The passphrase is what the keychain caches.

Making the hardened posture opt-in rather than mandatory is what lets hv's keychain direction ship without making a keyring daemon a precondition for reading a secret.

### Resolution order: the material override comes FIRST, and it is not part of the ladder

**A reference resolves in two steps, and the first one never touches the vault at all.**

**Step 0 -- the per-reference MATERIAL override.** A reference resolves from a deterministic environment variable when one is set, and returns immediately. `vault:lamplight/prod/api-key` reads `INTENT_VAULT_LAMPLIGHT__PROD__API_KEY` -- the derivation is under **The reference format**, and it has to be injective for reasons that cost a finding. **No identity, no passphrase, no ladder, no store touched.**

**This is what answers R7, and an earlier draft got it wrong in a way worth recording** -- it answered R7 with an environment rung carrying `INTENT_VAULT_IDENTITY` / `INTENT_VAULT_PASSPHRASE`, which is a way for CI to UNLOCK the vault. R7's own words are _a vault that CI must unlock is a vault whose key is in CI_. The draft put the key in CI and called that the exemption. Found by Lamplight's verification, 2026-08-25.

The collision was mechanical rather than philosophical: once a committed config carries a `vault:` reference, everything that reads that config must resolve it, CI included -- so CI must hold something. **The whole question is whether that something is a KEY or the MATERIAL.** With step 0 it is the material, supplied by the provider's secret store exactly as CI does today, and CI never holds a vault key. The reference stays in config, one code path reads it, and the vault is genuinely optional in CI rather than nominally so.

Step 0 is the only path that returns material without an identity, which is what makes R7 mechanically checkable -- AC-00.7 asserts `vault get` succeeds with **no identity reachable by any rung**, not merely with no identity file.

**Step 0 is ALSO the only thing in this design that can return a well-formed wrong answer, so it is observable by construction.** A stale `INTENT_VAULT_LAMPLIGHT__PROD__API_KEY` left exported in a shell silently overrides the store for every later read, and `vault get` returns material either way with nothing distinguishing them: a developer sets a prod override for one command, forgets to unset, and every read after that returns prod material while the store holds staging. **Nothing errors, nothing warns, and the wrong value is well formed.** So:

- **Material to stdout, PROVENANCE TO STDERR** -- one line naming which source resolved. Pipelines are unaffected; a human or an agent reading a terminal cannot fail to see that an override is in play.
- **`vault doctor` enumerates every active step-0 override in the environment, BY NAME ONLY, never value** -- R8's discipline applied to the resolver rather than to config.

**And a derived variable that is SET BUT EMPTY is a typed error, not an answer.** Returning empty material is precisely the falsy-for-absent R11 forbids, and it is the worst outcome available here because the caller cannot distinguish it from a legitimately empty secret. Falling through to the store is also wrong: a set-but-empty override is an operator error, not an absence. `VaultError::EmptyOverride` names the variable.

**Step 1 -- the passphrase ladder**, reached only when step 0 does not hit AND the identity is passphrase-protected. Read in order, first hit wins, every rung optional:

| rung | source                                               | for                             |
| ---- | ---------------------------------------------------- | ------------------------------- |
| 1    | `INTENT_VAULT_PASSPHRASE` / `INTENT_VAULT_IDENTITY`  | operator convenience, NOT CI    |
| 2    | OS keychain -- macOS `security`, Linux `secret-tool` | interactive workstations        |
| 3    | explicit `intent vault unlock`                       | the one place a prompt is legal |

**A read verb never reaches rung 3.** With a passphrase-protected identity and nothing on rungs 1 or 2, `vault get` returns the typed error `Locked` with a remedy line -- it does not prompt, and it does not block (**R10**).

**Rung 1 keeps its place and loses its justification.** It is a convenience for an operator who has chosen the hardened posture, and it is no longer what answers R7. **Anything that reaches the ladder is unlocking the vault**, and CI must never be doing that.

### Probing rung 2: PROBE BY DOING, NEVER BY PRESENCE

`secret-tool` on PATH answers "is the binary installed", which is not the question asked. **The adapter attempts a real lookup of a canary entry and treats ANY failure as absent** -- missing binary, no D-Bus, daemon not answering, keyring locked. A presence check reports a cache that then fails at use.

Measured, supplied by Lamplight: every Linux host in that estate is headless with no D-Bus session and no keyring daemon -- `ubuntu-latest` on GitHub Actions, and Fly.io containers from a repo-root Dockerfile. **There is no measured Linux host on which rung 2 is available**, which is evidence for the hardened posture being opt-in rather than default.

### R2's bounded exception, stated rather than implied

With a **plain** identity, R2 holds absolutely: one code path, and the platform-conditional code is never even reached.

With a **passphrase-protected** identity it does not. Same store, same posture: macOS finds the passphrase at rung 2 and a headless Linux box does not, so `vault get` succeeds on one and returns `Locked` on the other. **That is the right failure -- typed, never a wrong answer -- but it is platform-divergent behaviour and R2 as written does not permit it.** Recorded here as a bounded exception with its boundary named, because AC-00.2 asserting one code path would otherwise imply this does not exist. The boundary is exactly: the hardened posture, on a host with no working keyring, with no step-0 override set.

### `intentd` is an accelerator, not a home

age's scrypt passphrase KDF is deliberately slow. hv's instinct is chasing something real: agents shelling `vault get` repeatedly would pay that cost every time. But **WP-08 is Not Started, XL, a 49-line stub**, so a vault that depends on it cannot ship until an XL work package does, and R3 and R4 both break in the meantime.

So the vault follows the routing rule **WP-08 has already ratified for everything else**: _socket present -> route to daemon; absent -> in-process (never two sync engines)._

- **socket present** -- the CLI sends the REF to the daemon and receives material back. **The identity never leaves the daemon**, and the daemon holds it unlocked under a bounded TTL, dropped on `vault lock`.
- **socket absent** -- the in-process ladder resolves it.

This is not a special case invented for the vault; it is one more capability under a rule the thread already carries. The vault ships standalone on the CLI spine, and the daemon path lands in WP-08 adding rows to the dual-path conformance suite WP-08 already lists as a deliverable.

## The reference format

```
  vault:<project>/<profile>/<name>          full form
  vault:<project>/<name>                    profile defaults to `default`
```

**The project is IN the ref, so resolution never depends on the working directory** -- which is what makes R4 mechanically true rather than aspirational. The profile axis is R9: Lamplight carries three today (`default`, `prod`, `worldwright1`) and the profile, not just the project, selects the entry.

Fence regex, anchored and lowercase so it cannot false-positive on ordinary config or prose (**R8**). **Segments carry no `_`, and that restriction is load-bearing rather than tidy** -- see the derivation below:

```
  vault:<seg>(/<seg>){1,2}      where  <seg> = [a-z0-9]([a-z0-9-]*[a-z0-9])?
```

### The step-0 variable name is DERIVED, and the derivation must be injective

```
  segment separator  /  ->  __
  inside a segment   -  ->  _
  vault:lamplight/prod/api-key  ->  INTENT_VAULT_LAMPLIGHT__PROD__API_KEY
```

**A two-segment reference normalises to three before derivation**, so `vault:a/b` and `vault:a/default/b` are one reference with one variable rather than two names for one entry.

**An earlier draft permitted `_` inside segments and mapped `-`, `_` and `/` all onto `_`. Three characters collapsing to one is not injective, so collisions existed BY CONSTRUCTION** rather than as an edge case -- `vault:my-app/key`, `vault:my_app/key` and `vault:my/app/key` all derived `INTENT_VAULT_MY_APP_KEY`. Found by Lamplight's verification of the R7 fix, 2026-08-25, and confirmed mechanically before acting on it.

**The draft's answer was a collision check in `vault set`, and that check was structurally blind.** Stores are per-project, so those three references live in three different stores; a check scoped to one store has no jurisdiction over the collision, which happens exactly at the project boundary. **That is AC-00.7's own failure shape one layer down -- a guard whose scope excludes the case it exists to catch, reading green because it never had jurisdiction.** And the check could not have been repaired: a correct cross-store scan must open every store, which needs the identity, which fails under `Locked` and breaks R10.

**So the check is deleted rather than fixed. The grammar carries it instead.** With `_` forbidden inside segments, a single `_` in the derived name can only have come from a `-` between two alphanumerics, and `__` can only have come from a separator. The map is injective, cross-store collisions cannot exist, and nothing needs to check at runtime. **A grammar restriction that makes a check unnecessary beats a check, because a grammar cannot be out of scope.**

**One departure from the direction as given, and it is where the defect came from.** Lamplight proposed a **declared** environment override; this design uses a **derived** one. Derivation needs no declaration site, which matters because R4 has the resolver running where no project config exists -- but derivation is what created a name-collision surface that a declared mapping does not have. The grammar closes it. Recorded because one changed word produced the whole finding.

## The verb surface

The read interface is the requirement that comes straight from the incident, so it drives the roster rather than being fitted to it.

| verb                                           | what it does                                          |
| ---------------------------------------------- | ----------------------------------------------------- |
| `intent vault init`                            | create the identity, choose the posture, set the mode |
| `intent vault set <ref>`                       | write one entry; value from stdin or prompt           |
| `intent vault get <ref>`                       | read ONE secret to stdout                             |
| `intent vault rm <ref>`                        | remove one entry                                      |
| `intent vault list [<project>]`                | **names and refs ONLY, never material**               |
| `intent vault run [--ref K=<ref>]... -- <cmd>` | inject refs into a child's environment                |
| `intent vault unlock [--ttl <dur>]`            | cache the passphrase; the one legal prompt            |
| `intent vault lock`                            | drop the cache                                        |
| `intent vault import <file> --project <p>`     | migrate a plaintext store and rewrite it to refs      |
| `intent vault audit [<path>]`                  | find material-shaped values that should be refs       |
| `intent vault doctor`                          | mode posture, identity, backend, ACTIVE OVERRIDES     |

**There is deliberately no `dump`, `export`, `cat`, or `show --all`, and there never will be.** R6 states the reason better than a rationale could: _if dumping the store is possible it will eventually be done by something careful._

`run` is the one verb beyond Lamplight's list, and it earns its place by making the safe path the convenient one. `intent vault run -- ll cli --profile prod` replaces `LL_API_KEY=$(intent vault get ...) ll cli`, which lands material in shell history and in `ps` output.

**And `run` is not leak-free, which the next reader must not have to discover.** It puts material in the child's environment, so on Linux `/proc/<pid>/environ` is readable by the same user, and **every descendant of that child inherits it**. It is still clearly better than `$(vault get)`, which lands material in shell history and in `ps` for every user on the box -- but "better than the alternative" is not "safe", and a verb documented only by its advantage gets used as though it had none.

## Failure model

```rust
  enum VaultError {
    NoIdentity,                    // exit 5 -- run `intent vault init`
    Locked,                        // exit 4 -- run `intent vault unlock`
    NoSuchProject { project },     // exit 3
    NoSuchEntry { reference },     // exit 3
    MalformedRef { text },         // exit 7
    EmptyOverride { variable },     // exit 7 -- a step-0 variable set but empty
    BadMode { path, mode },        // exit 6
    StoreUnreadable { path },      // exit 1
    BackendUnavailable,            // exit 1
  }
```

**Never `Option<String>`, never an empty string for absent** (**R11**). A falsy value for "missing" is indistinguishable downstream from "present and empty", and distinct exit codes are what let a calling script discriminate without parsing stderr.

## Two strengthenings of Lamplight's requirements

Both come from the same observation: **R11 is about the return type, but the incident was about a print.**

**S1 -- R12 names the wrong file.** R12 asks the tool to refuse or warn when a store it owns is group- or world-readable, motivated by `~/.lamplight/config.toml` at mode 644. But in this design the store is ciphertext, so 644 on it is untidy rather than disclosing. **The identity file at 644 is the actual disclosure, and R12 does not mention it.** So the posture splits: **refuse on the identity** (B4), warn on the store, warn on the directory.

**S2 -- material must not reach a rendering.** The way material reached a transcript was printing, and `#[derive(Debug)]` over a struct holding a raw `String` is that same accident one layer down. So the secret type is `secrecy::SecretString`, carries no `Debug` over raw material, and zeroizes on drop. R11 is satisfied by the return type; S2 is what stops the value escaping through a channel R11 never considered.

## One deviation from a requirement's letter

**R5 says `age` as the default backend and reasons from "a single static binary, present in Homebrew and every distro".** This design uses the **`age` Rust crate** rather than shelling out to the `age` binary, and keeps format compatibility so `age -d` remains an escape hatch on any store.

The deviation is from R5's letter and serves R5's stated reason. **R5's reasoning is about availability, and the crate makes the vault MORE available, not less**: R4 requires the vault to be reachable before a project's toolchain is, and shelling to `age` would make that depend on `age` being installed -- a second bootstrap problem in the one capability whose job is to solve the first. With the crate, `intent` on PATH is sufficient by construction. Format compatibility is the thing R5 actually protects, and it is preserved.

**ACCEPTED by Lamplight's verification, 2026-08-25**, on the reasoning above plus one it added: the crate is the reference Rust implementation by an age spec author rather than a third-party reimplementation, so the audit argument runs the same way.

**The residual, named rather than left implicit: patch cadence.** With a shelled binary, `brew upgrade age` patches a vault vulnerability. With the crate it needs an Intent release and a re-vendor. **This belongs where the release process can see it, not in a design nobody reads at release time.**

### And the other half of R5, which the traceability table was silent on

R5 is "**pluggable backend**, `age` as the default", and the table answered the `age` half and said nothing about pluggability -- **which is how a reader concludes a requirement was considered when it was not.**

**Stated: R1 subsumes it, and that is the answer rather than a deferral.** Because config holds references and never material, the store backend can be replaced without touching a single config file -- which is exactly what a pluggable backend is for, and R5's own text says so. There is deliberately **no** store-backend abstraction in this design: the keychain adapter is for the PASSPHRASE, not for the STORE, and inventing a trait with one implementation would be a Highlander violation dressed as foresight. **The pluggability is in the reference format, not in a plugin seam.**

## Migration -- Lamplight is the measured case

```
  intent vault import ~/.lamplight/config.toml --project lamplight --format toml
```

1. Parse; identify credential-shaped fields, or take them explicitly with `--field`.
2. Write one entry per profile (**R9** -- Lamplight's three).
3. **Rewrite the source file in place**, replacing material with refs.
4. **Never delete the original.** Report the mode of the source, and warn when it is group- or world-readable.
5. Report plainly if any pre-image backup still holds material.

`intent vault audit` is the standing fence afterwards: it reads config files and reports material-shaped values that should be refs, **without reading any material itself** (**R8**), so an audit can prove "no material in config" without becoming a disclosure.

## Requirements traceability

**This table is the ratification instrument.** Every row names where the requirement is answered and which AC makes it checkable.

| req     | requirement                              | answered by                                                                   | AC       |
| ------- | ---------------------------------------- | ----------------------------------------------------------------------------- | -------- |
| **R1**  | reference, never material                | the ref format; resolver turns `vault:...` into material at point of need     | AC-00.1  |
| **R2**  | one behaviour on macOS and Linux         | platform-free store + identity; **bounded exception in the hardened posture** | AC-00.2  |
| **R3**  | no daemon, headless-safe                 | in-process ladder is the default path; daemon is an accelerator               | AC-00.3  |
| **R4**  | available before a project's toolchain   | per-machine identity; project is in the ref, so no cwd dependency             | AC-00.4  |
| **R5**  | pluggable backend, `age` default         | age format; crate not binary (accepted); **pluggability is R1, not a seam**   | AC-00.5  |
| **R6**  | one secret easy, whole store hard        | verb roster carries no dump/export/cat/show-all; `run` is the ergonomic path  | AC-00.6  |
| **R7**  | CI must never require the vault          | **step 0: per-ref MATERIAL override, no identity, vault never touched**       | AC-00.7  |
| **R8**  | mechanically detectable                  | anchored fence regex; names inside ciphertext, so `list` needs an identity    | AC-00.8  |
| **R9**  | multiple profiles per project            | profile is an axis of the ref                                                 | AC-00.9  |
| **R10** | non-interactive by default               | read verbs never reach the prompt rung; `Locked` instead of blocking          | AC-00.10 |
| **R11** | typed failure, never an empty string     | `VaultError` variants with distinct exit codes                                | AC-00.11 |
| **R12** | assert the file-mode posture             | **S1 accepted**: refuse on identity, warn on store (which discloses nothing)  | AC-00.12 |
| **S2**  | (added) material must not reach a print  | `SecretString`, no `Debug` over material, zeroize on drop                     | AC-00.13 |
| **--**  | (added) daemon and in-process must agree | dual-path conformance, WP-08 seam                                             | AC-00.14 |
| **--**  | (added) the consumer actually migrates   | Lamplight's three profiles imported and `ll cli` reads through the vault      | AC-00.15 |
| **--**  | (added) lamplight-vc has VERIFIED it     | advisory read against R1-R12: satisfied where, and unsatisfied where          | AC-00.16 |
| **--**  | (added) hv has RATIFIED it               | the deviation and both strengthenings -- a DIFFERENT act from verification    | AC-00.17 |

## Proposed build breakdown

**WPs are not minted.** This thread is in Triage and unscheduled; a WP breakdown on an unscheduled thread is a plan pretending to be a schedule. These are the stages a scheduler would cut into WPs.

| stage | scope | content                                                                      |
| ----- | ----- | ---------------------------------------------------------------------------- |
| 1     | M     | store + identity + ref parsing + `init` / `set` / `get` / `rm` / `list`      |
| 2     | S     | failure model, exit codes, mode posture, `doctor`                            |
| 3     | S     | passphrase ladder + keychain adapters + `unlock` / `lock`                    |
| 4     | S     | `run` and `audit`                                                            |
| 5     | M     | `import` + Lamplight migration                                               |
| 6     | S     | `intentd` path + dual-path conformance (**folds into WP-08, not before it**) |

## Explicit non-goals

Carried from Lamplight's non-requirements, all standing:

- **Not a hosted secret manager.** Local vault; `op` / `bw` / HashiCorp are out of scope by hv's ruling.
- **Not rotation.** hv's timing, tied to taking down the stealth front door.
- **Not a repo-secret scanner.** The repository measured zero real credentials; that is the fence's standing job.
- **Not `sops`-style encrypt-in-place.** It preserves config-file shape and is strong, but its TOML support is the weak one and Lamplight's connection profile is TOML.

## Open

**Verified by Lamplight twice on 2026-08-25.** First against 52adfb7e: 10 of 12, R5 satisfied with a named divergence, **R7 NOT SATISFIED** -- and the failing requirement was none of the three departures flagged for them, which is the argument for the provider reading it rather than the specifier. Then against 6dcbd9f5, scoped to R7: **step 0 CLOSES R7**, and it moved two things that are new surface rather than R7 returning. Both are fixed above.

- **ONE DECISION IS hv's AND IT REOPENS A RULED QUESTION.** Lamplight asks that `vault get` **REFUSE to write material to a TTY** unless explicitly forced -- piped or redirected is fine, interactive terminal refuses with a remedy line pointing at `run`. Their argument is the incident: `cat config.toml` printed two keys, and `vault get` prints one into the same transcript, so R6 currently makes the whole store hard and leaves the one-secret print exactly as available as it was. **hv ruled B3 on 2026-08-25 -- no TTY warn -- and this is a materially different proposal (refuse, not warn) rather than a re-litigation of that ruling, which is why it is put up rather than either adopted or dropped.** Not adopted pending hv.
- **AC-00.16 stays open until the two step-0 items are dispositioned.** Lamplight's own disposition: R7 closed, two items new, _that is a verification and not a pass_. **A verification is of a revision, not of a thread**, and this design has changed after both of theirs. Leaving it open with the verdict recorded is the honest state for a post-3.0.0 thread with nothing built.
- **Whether the injective derivation should keep `-` at all.** This design forbids `_` in segments and maps `-` to `_`, separator to `__`, so `api-key` survives as a name and the variable reads `..._API_KEY`. The stricter alternative -- segments `[a-z0-9]` only, separator to a single `_` -- gives shorter, more typo-resistant variable names and forbids `api-key`, which is what Lamplight's config uses today. **Both are injective; the choice is whose ugliness it is**, and the provenance line makes a mistyped variable visible either way rather than silent.
