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

### The identity has two postures, and the plain one is the default

- **Plain identity, mode 600.** Equivalent to an SSH key with no passphrase. Fully non-interactive on every platform. This is the default, and it is what keeps R3 and R10 true without qualification.
- **Passphrase-protected identity** (`age -p`, scrypt recipient -- stock age, not a new lane). Opt-in hardening. The passphrase is what the keychain caches.

Making the hardened posture opt-in rather than mandatory is what lets hv's keychain direction ship without making a keyring daemon a precondition for reading a secret.

### The passphrase ladder

Read in order, first hit wins, every rung optional:

| rung | source                                               | for                             |
| ---- | ---------------------------------------------------- | ------------------------------- |
| 1    | `INTENT_VAULT_PASSPHRASE` / `INTENT_VAULT_IDENTITY`  | CI, containers (**R7**)         |
| 2    | OS keychain -- macOS `security`, Linux `secret-tool` | interactive workstations        |
| 3    | explicit `intent vault unlock`                       | the one place a prompt is legal |

**A read verb never reaches rung 3.** With a passphrase-protected identity and nothing on rungs 1 or 2, `vault get` returns the typed error `Locked` with a remedy line -- it does not prompt, and it does not block (**R10**).

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

Fence regex, anchored and lowercase so it cannot false-positive on ordinary config or prose (**R8**):

```
  vault:[a-z0-9][a-z0-9_-]*(/[a-z0-9][a-z0-9_-]*){1,2}
```

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
| `intent vault doctor`                          | mode posture, identity reachable, backend present     |

**There is deliberately no `dump`, `export`, `cat`, or `show --all`, and there never will be.** R6 states the reason better than a rationale could: _if dumping the store is possible it will eventually be done by something careful._

`run` is the one verb beyond Lamplight's list, and it earns its place by making the safe path the convenient one. `intent vault run -- ll cli --profile prod` replaces `LL_API_KEY=$(intent vault get ...) ll cli`, which lands material in shell history and in `ps` output.

## Failure model

```rust
  enum VaultError {
    NoIdentity,                    // exit 5 -- run `intent vault init`
    Locked,                        // exit 4 -- run `intent vault unlock`
    NoSuchProject { project },     // exit 3
    NoSuchEntry { reference },     // exit 3
    MalformedRef { text },         // exit 7
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

**This is the one item in this design that contradicts a requirement as written, and it is flagged for the requirements provider to rule on rather than assumed.**

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

| req     | requirement                              | answered by                                                                  | AC       |
| ------- | ---------------------------------------- | ---------------------------------------------------------------------------- | -------- |
| **R1**  | reference, never material                | the ref format; resolver turns `vault:...` into material at point of need    | AC-00.1  |
| **R2**  | one behaviour on macOS and Linux         | store + identity layers are platform-free; only the passphrase cache adapts  | AC-00.2  |
| **R3**  | no daemon, headless-safe                 | in-process ladder is the default path; daemon is an accelerator              | AC-00.3  |
| **R4**  | available before a project's toolchain   | per-machine identity; project is in the ref, so no cwd dependency            | AC-00.4  |
| **R5**  | pluggable backend, `age` default         | age format; **crate not binary -- deviation flagged above**                  | AC-00.5  |
| **R6**  | one secret easy, whole store hard        | verb roster carries no dump/export/cat/show-all; `run` is the ergonomic path | AC-00.6  |
| **R7**  | CI must never require the vault          | env is rung 1 of the ladder and needs no identity file                       | AC-00.7  |
| **R8**  | mechanically detectable                  | anchored fence regex; `list` and `audit` read no material                    | AC-00.8  |
| **R9**  | multiple profiles per project            | profile is an axis of the ref                                                | AC-00.9  |
| **R10** | non-interactive by default               | read verbs never reach the prompt rung; `Locked` instead of blocking         | AC-00.10 |
| **R11** | typed failure, never an empty string     | `VaultError` variants with distinct exit codes                               | AC-00.11 |
| **R12** | assert the file-mode posture             | **strengthened S1**: refuse on identity, warn on store                       | AC-00.12 |
| **S2**  | (added) material must not reach a print  | `SecretString`, no `Debug` over material, zeroize on drop                    | AC-00.13 |
| **--**  | (added) daemon and in-process must agree | dual-path conformance, WP-08 seam                                            | AC-00.14 |
| **--**  | (added) the consumer actually migrates   | Lamplight's three profiles imported and `ll cli` reads through the vault     | AC-00.15 |
| **--**  | (added) lamplight-vc has VERIFIED it     | advisory read against R1-R12: satisfied where, and unsatisfied where         | AC-00.16 |
| **--**  | (added) hv has RATIFIED it               | the deviation and both strengthenings -- a DIFFERENT act from verification   | AC-00.17 |

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

- **Verification and ratification are two acts and this design needs both.** Lamplight's `vc` is **advisory** -- it verifies against R1-R12 and reports; **hv adjudicates**, and hv is the same human on both boards. An earlier draft carried one criterion reading _the requirements provider has ratified this design_, which named an authority the node it pointed at cannot exercise: **unsatisfiable by construction, and well-formed enough that nothing would have said so.** Split into AC-00.16 (verification) and AC-00.17 (ratification).
- The Linux keychain adapter needs a decision on what "present" means: probing `secret-tool` on PATH is not the same as a keyring daemon actually answering, and **a probe that tests the wrong one reports a cache that then fails at use**.
