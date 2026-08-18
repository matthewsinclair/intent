---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-18 09:18Z
status: paused
focus: "four bin/** items with hv; the critic gate is dark and the release never runs the Rust suite"
claims: [ST0056/11]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.** hv ruled it four times and it was reinterpreted after three of them, twice by me inside ten minutes.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** Asking SQLite and writing the answer is still writing a time you obtained.
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE. Read-then-write leaves a gap two writers interleave in.
- **hv's structural close: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times. **Direction is not symmetric -- IN is forbidden, OUT is fine.**
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES, WHICH IS WHY THIS VERSION HOLDS.** A time-typed input parameter is a defect by inspection. Asking where a caller got a value is a discussion; asking whether a signature accepts one is a grep.
- **The defect is always one step earlier.** Reaching for a clock means you are about to write a time into something that is not a durable record.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it's just a label". The third fooled cc, vc and me independently.
- **SCOPE, hv: devbin is NOT Intent** -- external, vendored, no db. My whole D42 directive resolved to a no-op, and reporting that beat inventing work to look responsive.
- **A board stamp is a label, not data.** The ordering that exists and cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. Three points that change what I DO: **the db is the durable SSOT, files are re-creatable**; **the typed API is the only door in**; **migrations are normal.**

## DOING

Nothing in flight. Everything measured today is either with hv below or archived.

## TODO -- all four are `bin/**`, so all four are hv's

1. **`sed -i '' 's/^GLOBAL_COMMANDS="help doctor/GLOBAL_COMMANDS="critic help doctor/' bin/intent`** (line 55). Three proofs: the gate's critic actually runs; the 8 red bats tests go green (0 fail / 28 pass, vs 8 / 20); `st list` and `wp list` still refuse at rc=2 so the guard is intact. **`critic` is read-only -- it never writes the project -- which is the structural classification `bin/intent:248` demands rather than a hand-written exemption.**
2. **The shim: HOLD.** It does not fix the gate (both binaries exit 2 on `critic`); it **defeats** item 1 by routing away from v2's dispatcher; and it pointed at a schema-stale release binary. Built and driven three arms; install was refused by the harness and a peer cannot convert hv's "go for it" into a grant.
3. **`cargo test` into the release pre-flight, ahead of `release:702`.**
4. **`doctor` prints `intent v2.19.0` while auditing a 3.0.0-dev project** -- the version-appropriate doctor is the one that never runs.

**Ceiling on item 1, and it must travel with it: a repaired gate reports ELIXIR ONLY.** 0 of 6 shell rules and 0 of 7 rust rules carry a greppable proxy; elixir is 19 of 19 (positive control: 20 findings on 41 files). `critic_runner.sh:18` skips a proxy-less rule **silently**. Intent is 114 `.rs` + 57 `.sh` + 71 `bin/` scripts. **Necessary, not sufficient.** Arming shell/rust is a program: shellcheck 0.11.0 and clippy 0.1.97 are installed and the rules name their own SC codes, but `critic_proxy_is_simple()` accepts only a bare `grep`, so delegation needs a runner change. **293 shellcheck findings across 85 shell files (9 error) are what the dark gate has never mentioned** -- including `bin/intent_st:196`/`:211`, `for dir in $(find ...)` in the **steel-thread ID-allocation path**, violating Intent's own `no-parse-ls` rule.

## Watch-outs

- **THIS REPO IS A v3 PROJECT** (`3.0.0-dev`; store `intent/.cache/intent.db`, gitignored). **CORRECTION, twice now wrong on this board: v2 does NOT refuse "every verb" -- `GLOBAL_COMMANDS` (help doctor bootstrap init version info fileindex upgrade plugin ext lang) run fine and `doctor` returns rc=0.** v2 refuses PROJECT verbs at exit 2. **And the shim is NOT landing, so "until the shim lands" is not a plan** -- invoke v3 by explicit path, and use `target/debug/`, not `release/`, which runs schema generations behind.
- **PUSH `local` ONLY.** `upstream` closed (hv, CI/CD budget).
- **NEVER `git pull --rebase` here; a peer `.git/index.lock` means WAIT. ALWAYS `git commit --only <paths>`** -- a bare commit sweeps a peer's staged index, and `--amend` ignores `--only`.
- **NEVER mutate `bin/**` or `tests/**` in place while anything runs them**; both PATH symlinks point into `bin/intent`. **Check for a live suite first.**
- **COMMIT BEFORE ANY `intent at` STATUS CHANGE** -- `intent at red|green|na` DESTROYS the row note (issue 0033).
- **`cargo test` IS A CONCURRENT WRITER ON THE ESTATE** (vc, reproduced in a clean clone on a green 598-pass run; `dispatch_ssot.rs` fixed at `1ff7f2c1`, two root-reaching readers remain). **Any measurement against the live estate has an uncontrolled writer under it unless nothing is running.** `git status --porcelain` cannot see it -- `.gitignore:127` ignores the store, **so the release verifies the generated views and never the SSOT.**
- **A PEER CANNOT WAIVE AN hv RULE**, and being right on the substance is not the same as being entitled to say so. Same shape: a peer cannot authorise what a harness refused.
- **MY SHELL IS zsh AND IT DOES NOT WORD-SPLIT AN UNQUOTED `$var`.** Twelve paths passed as one argument came back plausibly green. Use a bash array.
- **NEVER `$?` AFTER A PIPE.** Three of us hit it today; ic reported rc=0 for a tool that exited 2.
- **`git log` prints LOCAL time**; reading one and appending `Z` is wrong by exactly the offset and looks perfect.
- **macOS: signing MUTATES the binary, notarisation does NOT -- checksum AFTER signing.** Only a quarantined copy under `spctl` means anything.
- **`target/release/` IS SHARED MUTABLE STATE.** Private `CARGO_TARGET_DIR`; never sign there.
- **A control refuses; documentation reminds; only one is load-bearing.** My own watch-outs are not controls.

## Decisions

- **THE DAY RULE, and every finding and every error of mine reduces to it: A GREEN PRODUCED BY SOMETHING OTHER THAN THE THING UNDER TEST.** **None was caught by care. Each was caught by MOVING something** -- the target dir, the subject, the clock, the desk.
- **PROVE THE DISCRIMINATOR DISCRIMINATES BEFORE TRUSTING IT.** Six of my own instruments failed this way in two days, each looking exactly like the finding it hunted: `bash -n` on a `.bats` file; `bats --count` on a copy that cannot resolve its helper; `info` printing one banner from both binaries; a sweep reporting `examined: 0` on an estate of 51; `grep -c ... || echo 0` printing two values; and **`across N file(s)` in critic output, which echoes argc -- a nonexistent path reports "1 file(s)" and clean.**
- **A ZERO IS NOT A RESULT UNTIL THE CHECK HAS PRODUCED A NON-ZERO.** I swept 51 shellcheck directives, nominated 16, and confirmed all 16 sound -- **but only reported it after deleting one directive and watching the finding reappear.** Same reason the bats estate-digest verdict is trustworthy: one byte moved it and `git checkout --` returned it.
- **A PROXY IS NOT THE PARSER.** My structural sweep nominated 16 and shellcheck cleared every one. **Nominate with a proxy; adjudicate with the real tool.**
- **THE REHEARSAL POPULATION CANNOT HOLD WHAT THE REAL POPULATION HAS**, and "clone and test" reads as the CAREFUL option, which is why it wins. vc: gitignored. cc: uncommitted. **Mine: the INTERVAL** -- a peer changed state between my two readings and I told them their live finding was a phantom. **The other two are answered by looking at the real subject; mine by re-reading at the moment of the claim.**
- **A CLASSIFIER KEYED ON A MESSAGE IS BLIND TO THE POPULATION THAT NEVER PRINTS ONE.** 297/2 was really 299/0; `[[ ]]` prints nothing on failure. **Both classifications agreed on "2 remain".**
- **A LEGEND MUST BE DERIVED FROM THE DATA IT LABELS.**
- **BYPASS IS A RESPONSE TO NOISE, NOT TO STRICTNESS** (mine, vc adopted).
- **VERIFY THE PREMISE OF A QUEUED ACTION AT THE MOMENT YOU ACT ON IT.** I wrote that rule and then handed vc a task built on a stale one. **Today it paid: hv asked for shim install commands and the premise had gone false since "go for it".**
- **CHECK THE OTHER HALF BEFORE FILING A DESIGN SPLIT AS A DEFECT.** "The release never runs what it ships" was about to go to hv; `int macos publish` verifies the binary on a quarantined copy. The accurate finding was narrower and better.
- **A POPULATION CAN BE A FACT ABOUT THE CORPUS, NOT THE WRITER.** Three nodes measured "two files changed" against three different suspected causes. Exactly two `info.md` carry a deprecation blockquote.
- **A RECORD NAMES THE COMMIT IT COVERS, NEVER "HEAD".**
- **MIS-CREDITING TOWARD YOURSELF GETS CAUGHT; MIS-CREDITING AWAY FROM YOURSELF DOES NOT.**
- **THE PUSH RESULT CARRIES NO INFORMATION ABOUT THE REMOTE IN EITHER DIRECTION.** Only `git ls-remote` plus `merge-base --is-ancestor` is evidence.
