---
verblock: "18 Aug 2026:v0.1: vc - the pre-commit critic gate: approved, primed, unbuilt"
---

# The pre-commit critic gate -- APPROVED BY hv 2026-08-18, PRIMED, NOT YET BUILT

**Status: hv approved BOTH halves and then had to reboot. Nothing is built. This document exists so the work can be picked up cold, by anyone, without reconstructing the argument.**

Measured by dc across 2026-08-18; carried and cross-checked by vc. Every figure below is a measurement, and where it is not, it says so.

## The one-sentence problem

**Since the hoist at `0ec2ac79`, every commit any node has made passed a pre-commit gate that enforced NOTHING across all five declared languages, while reporting success.**

## HALF A -- the gate fails open. One word.

### Mechanism, and it is three correct behaviours composing into a wrong outcome

- The hook calls `intent critic <lang>`.
- `intent` on PATH is **v2**.
- **v2 correctly refuses a v3-declared project at exit 2** (`bin/intent:277`).
- **`intent critic` emits `2` for its own invocation errors** (`bin/intent_critic:89,95`) -- and the version refusal exits `2` as well, so the hook cannot tell the two apart.
- **The hook's `*)` branch treats every rc other than 0 or 1 as fail-open** (`lib/templates/hooks/pre-commit.sh:288-292`, contract stated at `:261-264`). It never sets `AGGREGATE`, so the commit proceeds.

Nothing here is a bug on its own. The version guard is right, exit 2 is a legitimate refusal code, and the fail-open on 2 was deliberate. **The defect is the composition, and it only exists because the project hoisted.**

**And it is not silent, which is the part worth keeping.** The `*)` branch prints `intent critic (<lang>) invocation error (exit 2); fail-open.` **once per declared language, on every commit** -- dc's own `d84ac27f` printed it five times, on the commit that documented the finding. **The gate has been announcing its own failure since the hoist**, so "nobody noticed" is a fact about attention, not about instrumentation. A pickup hunting a silent bug will not find one.

### The fix

**Add `critic` to `GLOBAL_COMMANDS` at `bin/intent:55`**, so it dispatches BEFORE the version guard at `bin/intent:277`.

This is the structural classification the guard's own comment already asks for: **`critic` reads the rule library and scans files, and never writes to the project.** It is not a hand-written exemption.

### BUILT AND RE-DRIVEN AT `4ef953db` (dc, 2026-08-18) -- THIS SUPERSEDES THE BLOCK BELOW

**HALF A IS APPLIED.** `bin/intent:55` now reads `GLOBAL_COMMANDS="critic help doctor bootstrap init version info fileindex upgrade plugin ext lang"`. One line, one word.

**THE GATE IS NO LONGER DARK, measured on the real tree after applying: languages returning rc=2 went 5 -> 0.** elixir, author, content, rust and shell all dispatch and return rc=0, so the five `invocation error (exit 2); fail-open.` lines are gone from every commit. **The version guard is intact**: `intent st list` and `intent wp list` still refuse at rc=2 with the v3.0.0-dev message.

**THE RIG IS COMMITTED THIS TIME**, at `intent/st/ST0056/parity/tools/critic_global_rig.sh`. Run it with `bash intent/st/ST0056/parity/tools/critic_global_rig.sh`.

```
                                                          FIXED   CANARY (control as subject)
1  control: critic refused by version guard (rc=2)          PASS   PASS   <- does not depend on the fix
2  subject: STAGED violation -> rc=1, IN-EX-TEST-002        PASS   FAIL   <- rc=2, refused
3  subject: CLEAN staged set -> rc=0                        PASS   FAIL   <- rc=2, refused
4  subject: `st list` STILL refused at rc=2                 PASS   PASS   <- guard intact either way
5  subject: `wp list` STILL refused at rc=2                 PASS   PASS   <- guard intact either way
6  END-TO-END: the hook BLOCKS the commit                   PASS   FAIL   <- THE COMMIT SUCCEEDED
                                                            6/6    3/6

bats  ARM A  clone at 4ef953db, as-is         20 passes,  8 failures
      ARM B  clone at 4ef953db, + the word    28 passes,  0 failures
```

**TEST 2 IS THE CORRECTED ONE AND TEST 3 IS ITS MANDATORY PARTNER.** Test 2 runs `elixir` -- the only armed pack -- against a STAGED deliberate violation and asserts `rc=1` with the rule named. Test 3 runs the SAME command over a clean staged set and asserts `rc=0`, **because rc=1 proves nothing about the violation until the instrument has also produced a zero.**

**TEST 6 IS THE PROOF, AND THE CANARY IS THE FINDING DEMONSTRATED RATHER THAN ARGUED.** `RIG_CANARY=1` drives the UNFIXED control through every subject case. Test 6 then fails **by the commit SUCCEEDING**: a commit titled _"rig: this commit must be REFUSED"_, carrying a staged critical violation, is created, with `intent critic (elixir) invocation error (exit 2); fail-open.` printed immediately above it. **A rig that passed in both modes would be measuring something other than the fix.**

**The fixture is NOT this repo.** Test 2 must stage a deliberate violation, and this repo's git index is shared with concurrent peer sessions. The rig builds a purpose-made project declaring `3.0.0-dev` with `languages: [elixir]`, in its own throwaway git repo, and points control and subject at the REAL tree via `INTENT_HOME` -- so it reads live rules while touching no shared index.

### CORRECTION to the block below: "all 8 fail verbatim with the guard message" is right about the CAUSE and wrong about the EVIDENCE

**All 8 ARM A failures are caused by the missing line -- proved by ARM B being 28/0, since the arms differ by exactly that line.** But **only 2 of the 8 print the guard message.** The other 6 fail on a bare `[ "$status" -eq 1 ]` or `-eq 0`, **which prints nothing at all**. A reader diagnosing from ARM A output alone sees two explained failures and six mute ones.

**Same class as `297/2` really being `299/0`: a classifier keyed on a message is blind to the population that never prints one.** Second time in this estate, both times mine, both times with `[ ]` as the silent operator.

### Evidence as first recorded -- SUPERSEDED, kept for the record

**READ THE RETRACTION BELOW BEFORE CITING THIS BLOCK. The `4/4` is 3/4: test 2 is vacuous by construction and cannot fail whatever the fix does.** The block is left standing rather than corrected in place because the retraction is the more useful record -- **but it is annotated HERE, at the point of citation, because a correction that lives only in the section arguing the point is invisible to a reader who consults the evidence and stops.** That failure has now happened three times in this estate in one day, twice in documents and once in a code comment.

```
3/4  (NOT 4/4 -- test 2 is vacuous, see below)
     critic runs (rc=0)                          <- TEST 2, VACUOUS: rc=0 is what the DARK gate returns,
                                                    and it runs `critic shell`, which has 0 of 6 rules armed
     `st list` and `wp list` STILL REFUSE at rc=2
     -- so the version guard is not weakened      <- tests 1, 3, 4: these stand

ARM A  clone as-is                       8 failures, 20 passes
ARM B  critic added to GLOBAL_COMMANDS   0 failures, 28 passes
```

All 8 bats failures are the version guard, at `critic_report_format.bats` 219-225 and `intent_critic.bats` 644, failing verbatim with `error: this project declares Intent v3.0.0-dev, and this is Intent v2.19.0`. **They can ONLY appear in a hoisted tree**, which is why the "100% green" suite run earlier that day is not in conflict -- it was pre-hoist.

**So one word does three things: un-darkens the gate, turns the suite green, and leaves the guard intact** -- **on the evidence of tests 1, 3 and 4 plus the bats arms. Test 2 proves none of it.**

**AND THE TWO HALVES ARE NOT INDEPENDENT, WHICH IS THE STRUCTURAL FINDING RATHER THAN THE ARITHMETIC ONE (dc, and neither of us could see it from one side).** Test 2 exercises `critic shell`. **Half B says shell has 0 of 6 rules armed. So the test that was supposed to prove the critic RUNS was pointed at the one pack that can produce nothing** -- and a pass was guaranteed before the fix, after the fix, and under any fix. **Half B is not merely a CEILING on Half A. It is what makes Half A's headline number untrustworthy**, and the only pack that can currently discriminate anything is `elixir`.

### The `4/4` is NOT 4/4 of what matters -- test 2 is vacuous BY CONSTRUCTION

**ic raised the principle: prove whatever lands by making a critic FAIL on purpose, because a gate that goes green after the fix is the same evidence it was giving while dark. Driven against the rig, it is worse than a caution -- it is already true of one of the four.**

The rig's test 2 asserts `rc=0 or 1`, labelled _"the critic actually ran"_. **`rc=0` is exactly what the dark gate returns**, so the pass condition does not separate "ran and found nothing" from "did not run". It was already caught returning a vacuous `rc=0` once, with nothing staged.

**And the two halves interact, which is the part neither of us saw separately: test 2 runs `critic shell`, and Half B says shell has 0 of 6 rules armed. So `critic shell` returns `rc=0` whether it ran perfectly or not at all.** Test 2 picked the one language where its own assertion is unfalsifiable. Half B is not merely a ceiling on the fix -- **it is what makes the headline evidence for Half A untrustworthy.**

Tests 1, 3 and 4 are sound and unaffected: the control reproduces `rc=2`, and `st list` / `wp list` still refuse with the guard intact.

**Re-drive requirement, and this is the one that must not be dropped: test 2 uses `elixir` -- the only armed pack -- against a STAGED deliberate violation, and must assert `rc=1` with a named finding.** A zero is not a result until the check has produced a non-zero. **The end-to-end proof is the hook BLOCKING a commit it should block**, not the critic exiting 0.

### Provenance -- RE-DRIVE THESE, DO NOT CITE THEM

**The figures above were driven on 2026-08-18 and the commit they were driven at was NOT recorded.** That is a defect in the record rather than a hedge: a record names the commit it covers. `native/rust` moved substantially the same day and rung 11 landed after the run, so the tree under the arms is not the tree a pickup will have. **vc raised this before the numbers were carried, so it is a known limit and not a discovered one.**

**Whoever picks this up re-drives both arms at their own HEAD and replaces the numbers, naming the commit.**

**AND THE RIG THIS PARAGRAPH DESCRIBED DID NOT EXIST (dc, 2026-08-18).** `critic_global_rig.sh` was written in a session scratchpad and evaporated with the session. `git log --all --diff-filter=A` finds it added **at no point in history**, and it was on no disk. **So this document -- written explicitly for cold pickup -- described the internals of an instrument nobody could run, in the very paragraph telling a re-driver how to use it.** The 39 other instruments under `parity/tools/` are all committed; this one broke the convention and nothing noticed, because a description reads exactly like a description whether or not its subject exists.

**Rebuilt and COMMITTED at `intent/st/ST0056/parity/tools/critic_global_rig.sh`**, with the properties the old paragraph claimed and one it lacked: it still does not clone the tool (control and subject are two copies of `bin/intent` pointed at the REAL tree via `INTENT_HOME`, so the only difference is the line under test), it still **aborts if the mutant is byte-identical to the control** -- a change proved against a copy of itself is proved against nothing -- and it now carries `RIG_CANARY=1`, which asks the rig to produce the failures it claims to detect.

## HALF B -- SCOPED (dc, 2026-08-18, driven at HEAD `ce532a97`, worktree dirty: 25 files)

**Scoped on hv's approval-in-principle. The headline is that "re-arm the rust and shell packs" is NOT achievable, and the honest outcome is mostly declarations rather than regexes.** Two of the figures this section previously carried were wrong; both are corrected below with the probe that settled them.

### The runner's contract is the binding constraint, and it is narrower than "a regex"

`critic_proxy_is_simple` (`intent/plugins/claude/lib/critic_runner.sh:92-120`) accepts exactly one shape:

```
grep [-r|-n|-E|--include=GLOB ...] '<pattern>' [<path>...]
```

One `grep`. No pipes, no chains. Flag clusters drawn from `{r,n,E}` only -- **`-L`, `-v`, `-l`, `-c`, `-o`, `-w`, `-x`, `-A`, `-B` are all rejected.** Single-quoted pattern; path args free of shell metacharacters.

**Consequence, and it decides four of the thirteen before any judgement is applied: the proxy can only express a POSITIVE match. A rule whose violation is an ABSENCE is inexpressible** -- there is no `grep -L` and no `grep -v`. **And a rule needing to aggregate across files (the same function name defined twice) is inexpressible for the same reason: one grep cannot count.**

### Per-rule verdicts -- 13 rules, and at most 4 can carry a proxy

| rule                                         | severity       | verdict                   | why                                                                                                                                                                                                                                                                                     |
| -------------------------------------------- | -------------- | ------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `IN-SH-CODE-002` no-parse-ls                 | warning        | **ARM**                   | `in $(ls` and `ls \|` are literal token sequences. Narrow, positive, honest. The only clean one.                                                                                                                                                                                        |
| `IN-SH-CODE-005` no-silent-exit-codes        | critical       | **ARM as NOMINATOR only** | `\|\| true`, `\|\| :`, `2>/dev/null` are literal -- but the rule's own condition is _"without an adjacent comment explaining why"_, and one grep cannot evaluate the adjacent line. **It fires on every documented, correct use.**                                                      |
| `IN-RS-CODE-001` result-over-panic           | critical       | **ARM at a STATED COST**  | `.unwrap()` / `.expect(` are literal. **But the rule's own suggested grep uses `rg -v '#[cfg(test)]'`, and `-v` is refused.** Rust colocates unit tests in the same file -- `IN-RS-TEST-001` mandates it -- so `--include=` cannot separate them either. Fires on every colocated test. |
| `IN-RS-CODE-004` thiserror/anyhow            | warning        | **ARM PARTIALLY**         | `Result<..., Box<dyn Error>>` / `Result<..., String>` are greppable and `--include=lib.rs` scopes the library half. The binary half (elaborate enums that only reach `main`) is not expressible.                                                                                        |
| `IN-SH-CODE-003` set-euo-pipefail            | warning        | **INEXPRESSIBLE**         | the violation is the ABSENCE of a `set -` line. Needs `grep -L`.                                                                                                                                                                                                                        |
| `IN-SH-CODE-004` setopt-err-exit             | warning        | **INEXPRESSIBLE**         | absence, as above.                                                                                                                                                                                                                                                                      |
| `IN-SH-CODE-006` module-highlander           | warning        | **INEXPRESSIBLE**         | needs the same function name COUNTED across files. One grep cannot aggregate.                                                                                                                                                                                                           |
| `IN-RS-TEST-001` cfg-test-colocated          | warning        | **INEXPRESSIBLE**         | _"source files with no colocated `#[cfg(test)]`"_ -- absence.                                                                                                                                                                                                                           |
| `IN-SH-CODE-001` quote-expansions            | critical       | **DECLARE none**          | shellcheck SC2086/2046/2206/2068 is the parser. A regex cannot tell `[[ ]]` (no word-split) from `[ ]`, nor an assignment from an argument.                                                                                                                                             |
| `IN-RS-CODE-002` ownership-before-clone      | warning        | **DECLARE none**          | clippy `needless_pass_by_value`, `redundant_clone`, `clone_on_copy`. _"cloned value is only read"_ is dataflow.                                                                                                                                                                         |
| `IN-RS-CODE-005` lifetime-elision-first      | style          | **DECLARE none**          | clippy `needless_lifetimes`, `extra_unused_lifetimes`. Requires applying the elision rules, ie a parser.                                                                                                                                                                                |
| `IN-RS-CODE-003` traits-over-enums           | recommendation | **DECLARE none**          | **the rule text already says so**: _"Clippy does not lint this directly; structural review is the detection mechanism."_                                                                                                                                                                |
| `IN-RS-TEST-002` assert-matches-for-variants | warning        | **DECLARE none**          | **the rule text already says so**: _"Clippy has no direct lint; structural review and test-brittleness are the signals."_                                                                                                                                                               |

**Tally: 1 clean arm, 3 arms carrying a stated cost, 4 inexpressible under the contract, 5 declare-none.** **So the work is roughly one regex, three regexes with recorded limitations, and nine written declarations** -- and the nine are the part that changes the gate's behaviour least and its honesty most.

### CORRECTION 1 -- "19 of 19 rules carry a proxy" was FALSE, and the true figure is a better positive control

Measured over `intent/plugins/claude/rules/`:

```
elixir  19 rules   9 ARMED   10 DECLARED-none   0 silent
shell    6 rules   0 ARMED    0 DECLARED-none   6 SILENT
rust     7 rules   0 ARMED    0 DECLARED-none   7 SILENT
```

**Only 9 elixir rules carry a proxy. What is 19 of 19 is that every elixir rule has MADE the decision and RECORDED it** -- the ten unarmed ones carry an explicit _"No greppable proxy is authoritative for this rule"_ plus a pointer to the `critic-elixir` subagent. **That is the property worth copying, and it is a stronger control than the one this document claimed.**

**Shell and rust are not unarmed. They are SILENT** -- no proxy and no declaration -- **and `critic_runner.sh:18` skips a rule without a proxy silently.** So the gate cannot distinguish _"checked and found clean"_ from _"never asked the question"_, which is this estate's standing defect class arriving in the rule library.

### CORRECTION 2 -- "ST0039 stripped them; only Elixir was re-armed" was FALSE

**The shell and rust packs NEVER carried a proxy at any point in history.** `git log --all -S'Greppable proxy'` over each tree:

```
rules/shell    0 commits        <- never present, never removed
rules/rust     0 commits        <- never present, never removed
rules/elixir   6 commits        <- POSITIVE CONTROL: the probe works
```

ST0039 (`2bb1ab2c`) stripped elixir proxies, which is what the ST says it did. **The shell and rust packs shipped untriaged from ST0034 and have never been examined. "Re-arm" is the wrong verb; nothing is being restored.**

### THE DESIGN QUESTION THIS SURFACES, WHICH IS hv'S AND NOT MINE

**7 of the 13 rules name a real parser in their own Detection text** -- shellcheck (`SC2086`, `SC2046`, `SC2206`, `SC2068`, `SC2012`, `SC2015`, `SC2164`, `SC2148`, `SC2154`) or clippy (`unwrap_used`, `expect_used`, `panic`, `needless_pass_by_value`, `redundant_clone`, `clone_on_copy`, `needless_lifetimes`, `extra_unused_lifetimes`). **The runner accepts only `grep`, so for the two languages that HAVE a mature parser, the gate is architecturally barred from using it.**

**This is my own standing rule pointed at the gate: a proxy is not the parser -- nominate with a proxy, adjudicate with the real tool.** The gate currently has no adjudication step at all. **Whether the runner should be able to shell out to `shellcheck` / `cargo clippy` is a design decision, and it is stated here rather than decided.** It would change what a proxy is FOR, so it should be settled before nine declarations are written on the assumption that grep is the only instrument.

### BOUNDARY -- the `.bats` estate is invisible to the gate regardless of Half B

`bin/intent_critic:198-210` filters `--staged` files by extension: `shell` accepts `*.sh`, `*.bash`, `*.zsh`, or an extension-less file whose shebang matches `(bash|zsh|sh)`. **`.bats` files carry `#!/usr/bin/env bats` and match neither branch.** Driven, with a positive control:

```
./bin/.devbin/lib/tmpl/test.bats   REJECTED
bin/intent_critic                  ACCEPTED   <- positive control, extension-less bin/ script
```

**108 `.bats` files -- the largest single population of shell-adjacent code in the estate -- can never reach the shell critic through the gate**, so arming the shell pack does not touch them. Not part of Half B; recorded so nobody measures Half B's effect against a denominator that includes them.

### What this project actually is

**Intent is 114 `.rs` + 57 `.sh` + 71 `bin/` scripts + 108 `.bats`.** The 41 Elixir files are almost entirely template payload under `lib/templates/ext-seeds/worker-bee/`. **A fully repaired gate would enforce Elixir rules on a project that is essentially not Elixir** -- which is why Half B matters here and why its ceiling matters more.

### The constraint that still stands

**ST0039: a proxy must be simple enough for the headless runner to honour, and a rule whose detection is genuinely non-mechanical carries NO proxy rather than a misleading one.** Nine of thirteen land there. **The correct outcome for most of this pack is "critic cannot check this", stated** -- which is a smaller change than re-arming and a larger improvement than a regex that passes for the wrong reason.

## Sequencing -- the shim CONFLICTS with Half A

dc's shim resolves a project, reads its declared version, and execs the matching binary. **Installed, this project's `intent` routes to v3 -- and v3 answers `critic` with `known command that is not implemented yet`, exit 2.**

**So installing the shim DEFEATS Half A**, because the fix lives in v2's dispatcher and the shim routes away from it. Both were measured separately, so the conflict is composition rather than inference.

**RULING (dc, carried by vc, approved by hv): take the one-word fix, HOLD the shim.**

## Adjacent, same lane, NOT approved and NOT scoped

**`bin/.devbin/cmd/build.d/release:373-383` never runs `cargo test` at all.** The release pre-flight is `bin/intent doctor` (v2's doctor) then `tests/run_tests.sh` (bats). **On a 3.0.0 whose product IS the Rust binaries, the tag path never runs the Rust suite** -- which is why 2026-08-18's `dispatch_ssot` defect could never have been caught by cutting a release.

dc's proposal, **not yet put to hv**: put `cargo test` in the pre-flight ahead of the dirty-tree check at `release:702`, which already re-reads `git status --porcelain` and **fails CLOSED**. No new mechanism; the gate that would catch it already exists and simply never sees the suite.

### CORRECTION (dc, driven against the file at working-tree `ce532a97`) -- READ THIS BEFORE QUOTING THE PARAGRAPH ABOVE

**Both line numbers are right and they name DIFFERENT sites; neither is stale.** `release:373-383` is inside `preflight()` (opens `:307`) and is the site of the ABSENCE -- `bin/intent doctor` at `:373`, `tests/run_tests.sh` at `:380`, no `cargo test`. Exhaustively: the only `cargo` in the 801-line file is `cargo metadata` for the lock refresh at `:558-573`, so **the Rust suite is never invoked on the tag path**. `release:702` is a separate site in the TAG path, outside `preflight()`.

**But one clause above is WRONG, and it is dc's own.** **`:702` catches a DIRTY TREE. It does not catch a RED SUITE and never could.** The mechanism that would actually catch a failing `cargo test` is `preflight()`'s own abort pattern -- `:381` aborts when `tests/run_tests.sh` fails -- which already fails closed. **So the corrected proposal is: insert beside `:380`, inheriting `:381`'s abort.** _"No new mechanism"_ survives; _"the gate that would catch it"_ pointing at `:702` does not. `:702` belongs here only for the boundary stated below -- it reads git, so it cannot see a gitignored writer. **The error was conflating the fail-closed exemplar with the gate that does the catching.**

**AND THE PROPOSAL IS INCOMPLETE, WHICH IS WHY IT MUST NOT GO TO hv AS A ONE-LINER.** **`--skip-tests` bypasses the entire block being extended**: `:366-369` returns from `preflight()` before doctor and before the suite, so a `cargo test` added at `:380` inherits that bypass exactly. **And `:706` ACTIVELY RECOMMENDS THE FLAG** -- _"refusing to tag a dirty tree -- commit or revert the above, then re-run with `--skip-tests`"_. **The tool's own documented recovery from the dirty-tree gate is to re-run with the test gates off, and that re-run is the one that tags.** The realistic failure is a releaser following the instruction, not a releaser being lazy. More than one shape of fix exists (a separate `--skip-rust-tests`; or `:706` recommending something narrower), so it is hv's call and it is not scoped here.

**Boundary worth stating with it: `release:702` reads git, so it structurally cannot see a writer that only writes GITIGNORED paths** -- the runtime store being the live example. That is the limit of what that gate can be asked to prove, not an argument against it.

## Ownership

- **Half A** -- `bin/intent:55`. dc measured it and has not touched `bin/**`; that is theirs.
- **Half B** -- the rule library. Needs scoping before it needs an owner.
- **The shim** -- dc's, held.
- **The release pre-flight** -- dc's, and still needs hv.
