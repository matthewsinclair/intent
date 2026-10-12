---
verblock: "2026-10-12:v3.13: vc - globalfold at EOD; 3.2.4 open and ST0081 in progress, nothing pushed; the TODO is what waits on hv"
intent_version: 3.2.3
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

- **3.2.4, ONE RELEASE, CUT ONLY ONCE THE WORK RUNS ON MAIN'S HEAD (hv, 2026-10-12).** Its `## [3.2.4] - in progress` CHANGELOG section is open, and main carries unpushed commits for it. Nothing is pushed until the Linux proof on hv's VM passes. `bin/devbin build all` runs once, right before hv's push, booked by vc in the box queue. The known-defects re-drive runs at the cut (dc, hold 31).
- **ST0081, INTENT ON WINDOWS THROUGH WSL (cc holds it), in lock-step with devbin's ST0008 (devbin-vc).** WP-01's `bin/int linux` and its dispatch-only workflow, and WP-05's `--at-login` refusal off macOS, are on main and owe their Linux proof on hv's Ubuntu VM, `kovacs`: `bin/int linux doctor`, `build --jobs 1`, `package` and `smoke` from a git bundle of main, then AT-05.1 red at the base 8c51de484 with its test file alone and green at HEAD. WP-03's two pages (`docs/install.md#linux` and `docs/windows-wsl.md`) are written from that run. WP-02 waits on hv's D55, and WP-04 is a real-machine install hv arranges through the requesting estate. The agreed interfaces and hv's rulings are in the thread's context.

## TODO

- **THE VM, hv's hand**: once kovacs's 26.04.1 upgrade is done: rustup stable with `rustup target add aarch64-unknown-linux-musl`; `sudo apt-get install musl-tools git file jq shellcheck`; 4 CPUs and 8 GB, or swap; Rosetta, if the x86_64 package is to be tried before any push. Its runs book through the box queue, because it shares rhadamanth's CPU.
- **3.2.4's SCOPE, AWAITING hv**: 0591, 0592, 0593 and 0594 (vc: in, dc's lane, held by dc hold 31); 0590 (vc: waits, as new surface with a design question); dc's M2 removable-v2-source fix, dc hold 30 (vc: in if dc sizes it at S or less).
- **AWAITING hv**: `intent claude skills sync` and `subagents sync`, which carry 0589's catalogue and critic-prose fix to every session on the machine (vc: now); who fixes llm-tropes, the catalogue's vendor source, and Utilz's `indicators.txt`, which still hold the flattened quote classes (vc: dc for llm-tropes, Utilz's vc for Utilz); the stabilisation line after 3.2.4 (vc: yes); a SHA256SUMS file on each release, in ST0081 WP-02 (vc: yes); naming `local` and `upstream` as the release remotes in `bin/.devbin/config.yaml` (vc: yes).
- **AWAITING hv, from devbin-vc**: D55 (devbin's release core attaches CI-built assets, or an upload-only workflow in Intent; both vcs lean to the first); where the joint WSL job lives; whether a hosted runner's WSL 1 counts as evidence; and one re-vendor of devbin into Intent once ST0008 is complete, which under D55's first option must land before Intent's 3.2.4 cut, after hv republishes devbin's machine install.
- **THE PUSH, hv's hand**: after the VM proof and `build all`, measured at the moment of pushing. GitHub Actions billing does not gate Intent's own CI, because the repository is public and its CI ran green after the 2026-09-26 refusal; private estates still need it fixed.
- **AFTER THE LINE, vc's hand, no push**: hv decision 28's ten Conflab and Baize rulings, which repair records. None is done.
- **AWAITING hv**: a `/refresh` in the explorer (vc: after the line; ic's, size S).
- **WITH hv**: the pushes of the other estates still unpushed, measured at the moment of pushing.
- **GTOOLS**: the WP-04 slot vc's 2026-09-26 resume named is unscheduled; gtools-vc owns it.
- **COURSES**: courses-vc re-drives its five Intent findings against 3.2.3 and sends vc the ones that still reproduce, to file; its trope catalogue finding is covered by 0589.
- **MACHINE, hv's call**: the resident `SetStoreUpdateService` processes, and the Spotlight-family services that pushed today's load high (`ps -axo comm= | grep -c '[S]etStoreUpdateService'` measures the first).
- **ST0060** and **ST0077** stay in Triage as the next line's queue (hv, 2026-09-23).
