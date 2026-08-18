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

### Evidence, both arms, driven in a sacrificial copy

```
4/4  critic runs (rc=0); `st list` and `wp list` STILL REFUSE at rc=2
     -- so the version guard is not weakened

ARM A  clone as-is                       8 failures, 20 passes
ARM B  critic added to GLOBAL_COMMANDS   0 failures, 28 passes
```

All 8 bats failures are the version guard, at `critic_report_format.bats` 219-225 and `intent_critic.bats` 644, failing verbatim with `error: this project declares Intent v3.0.0-dev, and this is Intent v2.19.0`. **They can ONLY appear in a hoisted tree**, which is why the "100% green" suite run earlier that day is not in conflict -- it was pre-hoist.

**So one word does three things: un-darkens the gate, turns the suite green, and leaves the guard intact.**

### The `4/4` is NOT 4/4 of what matters -- test 2 is vacuous BY CONSTRUCTION

**ic raised the principle: prove whatever lands by making a critic FAIL on purpose, because a gate that goes green after the fix is the same evidence it was giving while dark. Driven against the rig, it is worse than a caution -- it is already true of one of the four.**

The rig's test 2 asserts `rc=0 or 1`, labelled _"the critic actually ran"_. **`rc=0` is exactly what the dark gate returns**, so the pass condition does not separate "ran and found nothing" from "did not run". It was already caught returning a vacuous `rc=0` once, with nothing staged.

**And the two halves interact, which is the part neither of us saw separately: test 2 runs `critic shell`, and Half B says shell has 0 of 6 rules armed. So `critic shell` returns `rc=0` whether it ran perfectly or not at all.** Test 2 picked the one language where its own assertion is unfalsifiable. Half B is not merely a ceiling on the fix -- **it is what makes the headline evidence for Half A untrustworthy.**

Tests 1, 3 and 4 are sound and unaffected: the control reproduces `rc=2`, and `st list` / `wp list` still refuse with the guard intact.

**Re-drive requirement, and this is the one that must not be dropped: test 2 uses `elixir` -- the only armed pack -- against a STAGED deliberate violation, and must assert `rc=1` with a named finding.** A zero is not a result until the check has produced a non-zero. **The end-to-end proof is the hook BLOCKING a commit it should block**, not the critic exiting 0.

### Provenance -- RE-DRIVE THESE, DO NOT CITE THEM

**The figures above were driven on 2026-08-18 and the commit they were driven at was NOT recorded.** That is a defect in the record rather than a hedge: a record names the commit it covers. `native/rust` moved substantially the same day and rung 11 landed after the run, so the tree under the arms is not the tree a pickup will have. **vc raised this before the numbers were carried, so it is a known limit and not a discovered one.**

**Whoever picks this up re-drives both arms at their own HEAD and replaces the numbers, naming the commit.**

One thing to know before re-driving: **the 4/4 rig does not clone.** `critic_global_rig.sh` copies `bin/intent` to a scratchpad pair (`intent.orig` / `intent.mut`), applies the one-line `sed` to the mutant, and points both at the REAL tree via `INTENT_HOME` -- so it reads live project state and live rules, and the only difference between control and subject is the line under test. It aborts if the mutant is byte-identical to the control, because a change proved against a copy of itself is proved against nothing.

## HALF B -- repaired, it can still only report on Elixir

```
shell   80 real project files -> rc=0,  0 findings    0 of 6 rules carry a greppable proxy
rust    80 real project files -> rc=0,  0 findings    0 of 7 rules carry a greppable proxy
elixir  41 real project files -> rc=1, 20 findings   19 of 19 rules carry a proxy
```

**Intent is 114 `.rs` + 57 `.sh` + 71 `bin/` scripts + 108 `.bats`.** The 41 Elixir files are almost entirely template payload under `lib/templates/ext-seeds/worker-bee/`. **A fully repaired gate would enforce Elixir rules on a project that is essentially not Elixir.**

**Elixir at 19/19 is the positive control**, so the instrument is proven in both directions and the shell/rust zeros are a real absence rather than a broken measurement.

### The cause is ours and it is documented

**ST0039 stripped the non-mechanical `Greppable proxy` regexes out of the rule library** -- correctly, because a regex over source measures TEXT rather than BEHAVIOUR, and the headless runner now refuses a non-simple proxy outright. **Only the Elixir pack was ever re-armed.**

### What Half B actually requires

**Re-arm the rust and shell packs with proxies that are mechanically honest.** Six shell rules and seven rust rules currently carry none. **This is real work and nobody has scoped it** -- it is not a one-liner and should not be quoted as one.

**The constraint from ST0039 stands: a proxy must be simple enough for the headless runner to honour, and a rule whose detection is genuinely non-mechanical should carry NO proxy rather than a misleading one.** The correct outcome for some rules may be "critic cannot check this", stated, rather than a regex that passes for the wrong reason.

## Sequencing -- the shim CONFLICTS with Half A

dc's shim resolves a project, reads its declared version, and execs the matching binary. **Installed, this project's `intent` routes to v3 -- and v3 answers `critic` with `known command that is not implemented yet`, exit 2.**

**So installing the shim DEFEATS Half A**, because the fix lives in v2's dispatcher and the shim routes away from it. Both were measured separately, so the conflict is composition rather than inference.

**RULING (dc, carried by vc, approved by hv): take the one-word fix, HOLD the shim.**

## Adjacent, same lane, NOT approved and NOT scoped

**`bin/.devbin/cmd/build.d/release:373-383` never runs `cargo test` at all.** The release pre-flight is `bin/intent doctor` (v2's doctor) then `tests/run_tests.sh` (bats). **On a 3.0.0 whose product IS the Rust binaries, the tag path never runs the Rust suite** -- which is why 2026-08-18's `dispatch_ssot` defect could never have been caught by cutting a release.

dc's proposal, **not yet put to hv**: put `cargo test` in the pre-flight ahead of the dirty-tree check at `release:702`, which already re-reads `git status --porcelain` and **fails CLOSED**. No new mechanism; the gate that would catch it already exists and simply never sees the suite.

**Boundary worth stating with it: `release:702` reads git, so it structurally cannot see a writer that only writes GITIGNORED paths** -- the runtime store being the live example. That is the limit of what that gate can be asked to prove, not an argument against it.

## Ownership

- **Half A** -- `bin/intent:55`. dc measured it and has not touched `bin/**`; that is theirs.
- **Half B** -- the rule library. Needs scoping before it needs an owner.
- **The shim** -- dc's, held.
- **The release pre-flight** -- dc's, and still needs hv.
