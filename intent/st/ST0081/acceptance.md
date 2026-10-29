---
st_id: ST0081
title: Intent from Homebrew on Linux and macOS, with no source checkout: Linux binaries on every release
---

# ST0081: Intent from Homebrew on Linux and macOS, with no source checkout: Linux binaries on every release -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- Linux release binaries built by GitHub Actions (status: Done)

- AC-01.1 (non-test) A `workflow_dispatch` run, with no tag, builds `intent` and `intentd` for x86_64 and aarch64 Linux packaged with the support payload, and keeps each package as a workflow artifact. -- evidence: Run 39854556183 of .github/workflows/linux-release.yml (https://github.com/matthewsinclair/intent/actions/runs/39854556183), the workflow_dispatch hv fired on 2026-10-22 with --ref main, so with no tag: headBranch main, headSha 2e0e9215d67f67c665d1b8179a7888555551c8e7 (the release commit, upstream/main), and the build saw no tag, so each package's `intent --version` reads `intent 3.2.5 (2e0e9215d67f67c665d1b8179a7888555551c8e7) dev`. Conclusion success, 21:06:43Z to 21:11:33Z: job package (x86_64-unknown-linux-musl, ubuntu-latest), id 120089246768, and job package (aarch64-unknown-linux-musl, ubuntu-24.04-arm), id 120089247186, each green through bin/int linux doctor, build, package, smoke on the runner and smoke on the floor. Each package is kept as a workflow artifact: intent-x86_64-unknown-linux-musl (artifact 12459447395, 14576654 bytes, digest sha256:9c386af2b961) and intent-aarch64-unknown-linux-musl (artifact 12459690768, 13589688 bytes, digest sha256:e63d2e01e4cc), both kept until 2027-01-20. Downloaded and listed by cc, never run: each tarball (x86_64 sha256 9e9ae9f6b7f9, aarch64 ed579ded2241) holds bin/intent and bin/intentd, statically linked (x86-64 static-pie; ARM aarch64 static), beside the support payload the binaries read at run time, intent/plugins and lib/templates, whose 180 files are byte-identical, exec bits included, in both packages and in v3.2.5's intent-support.tar.gz (sha256 d0938ba5832f); the tarball's only extra is three empty directories git does not track (lib/templates/eng, issues and prime). The run's records, both floor steps and these listings are refs/bank/cc/0081/evidence/run-39854556183.txt (blob baec68a54b14). -- satisfied: yes
- AC-01.2 (non-test) The same run executes the packaged `intent --version` inside a container of the oldest Linux distribution the workflow names as its floor, and that step fails the run when the binary does not start. -- evidence: The same run, 39854556183 (hv's --ref main workflow_dispatch of 2026-10-22, conclusion success). In both jobs (ids 120089246768 and 120089247186) the step `smoke, on the floor (AC-01.2)` ran `bin/int linux smoke --in "$FLOOR"` with FLOOR oraclelinux:7-slim, the one floor the workflow names (Oracle Linux 7, glibc 2.17), pulled at digest sha256:1add6ed8602e. Inside that container the packaged binary, reached through a link on PATH, printed `intent 3.2.5 (2e0e9215d67f67c665d1b8179a7888555551c8e7) dev` on each leg, then resolved its install root to /opt/intent-<triple> and ran the support tree's gate hook both ways. The step fails the run when the binary does not start, by reading: bin/.devbin/cmd/linux runs under `set -euo pipefail` (line 20) and pipes smoke_checks into the container's `/bin/bash -euo pipefail -s`, whose first check is `intent --version`, so a binary that does not start ends the container's shell non-zero, which ends `bin/int` non-zero, which fails the step (its shell, as the log shows, is `/usr/bin/bash -e {0}`), the job and the run; that clause was read, not driven with a broken binary. Both floor steps are quoted in refs/bank/cc/0081/evidence/run-39854556183.txt (blob baec68a54b14). -- satisfied: yes

### WP-02 -- Linux assets on each tag's GitHub release, one writer per release object (status: Done)

- AC-02.1 (non-test) Every release tag's GitHub release carries the Linux assets beside the macOS assets, and `bin/devbin release check` names no second writer of the release object. -- evidence: v3.2.7 GitHub release, read by vc 2026-10-30: gh release view v3.2.7 --json assets names intent-aarch64-unknown-linux-musl.tar.gz and intent-x86_64-unknown-linux-musl.tar.gz beside intent-aarch64-apple-darwin, intentd-aarch64-apple-darwin, Intent.app.zip, intent-support.tar.gz and SHA256SUMS; hv's bin/devbin release check: none of the 4 workflows writes a release object -- satisfied: yes

### WP-03 -- Intent's Linux install through Homebrew, in docs/install.md (status: Done)

- AC-03.1 (non-test) Following the docs' Linux install takes a fresh WSL distro from a release download to an `intent --version` that names the release and an `intent doctor` with no finding in a new project, and every command on the page has been run. -- DESCOPED to ST0082 by hv, 2026-10-14; numbering under vc's pen: The WSL walk-through moved to ST0082 with the WSL page on hv's ruling of 2026-10-14; the Linux install half is AC-03.3, through Homebrew.
- AC-03.2 (non-test) The docs name WSL as Intent's Windows route, tell readers to keep repos in the WSL filesystem rather than under `/mnt/c` and why, and tell them to run Claude Code inside WSL. -- DESCOPED to ST0082 by hv, 2026-10-14; numbering under vc's pen: The WSL page moved to ST0082 on hv's ruling of 2026-10-14.
- AC-03.3 (non-test) On Linux, following `docs/install.md#linux` takes a machine with Homebrew and no Intent to an `intent --version` that names the release and an `intent doctor` with no finding in a new project, with no checkout of Intent's source, and every command on the page has been run. -- evidence: kovacs ORDER 8, green, 2026-10-22 21:02:53Z to 21:03:12Z by its own stamps, summary first red none: refs/bank/cc/0081/kovacs/o8-postcut.sh (blob f9414b29fc73, sha256 da0115d7b8de, read by vc) run as `o8-postcut.sh 3.2.5 2e0e9215d67f67c665d1b8179a7888555551c8e7` in hv's own kovacs terminal after hv's `brew formula --push` (tap 1de8ca7e5), its sha256 matched there; its `o8 ` lines, as hv pasted them and vc relayed them, are refs/bank/cc/0081/evidence/o8-postcut-lines.txt (blob b3fc943efce9), and the whole log stays on kovacs at /home/matts/work/intent-vc/o8-20261022T210253Z/o8.log. On kovacs, the Ubuntu aarch64 VM with Homebrew at /home/linuxbrew/.linuxbrew, the run removed brew's intent and untapped matthewsinclair/intent, so the real PATH answered no intent; then, under a fresh HOME and the system PATH with no intent on it, and with no checkout at ~/work/Intent, it ran every command of docs/install.md#linux at v3.2.5 (unchanged since v3.2.4) in order, each at rc 0: `brew install matthewsinclair/intent/intent`; `intent --version`, which read `intent 3.2.5 (2e0e9215d67f67c665d1b8179a7888555551c8e7) release`; `intent bootstrap`; `mkdir my-project && cd my-project`; `git init`; `intent init`; `intent doctor`, at rc 0 with its line carrying `0 finding(s)` (3.2.5's doctor exits 0 only when the count its summary line prints is 0: Report::exit_code through is_healthy, actionable() == 0); and the daemon paragraph's `intent daemon start`, `status` (naming a running intentd) and `stop`, with both `--at-login` forms refusing at rc 1 and writing no enrolment; jq was on PATH. The --version, doctor and daemon status blocks are in the kovacs log; cc read the o8 lines, not those blocks. -- satisfied: yes

### WP-04 -- First install on a real Windows machine (status: Cancelled)

- AC-04.1 (non-test) On a real Windows machine, Intent installed under WSL from a tagged release by following the docs reports its version, passes `intent doctor`, and creates and lists a thread in a project in the WSL filesystem. -- DESCOPED to ST0082 by hv, 2026-10-14; numbering under vc's pen: The real Windows install moved to ST0082 with WP-04 on hv's ruling of 2026-10-14.

### WP-05 -- intent daemon start --at-login refuses on Linux before writing anything (status: Done)

- AC-05.1 On Linux, `intent daemon start --at-login` and `intent daemon stop --at-login` refuse before writing anything, naming login enrolment as macOS-only and `intent daemon start` without the flag as the way to run the daemon. -- satisfied: yes (computed)

### WP-06 -- The Homebrew formula installs Intent on Linux (status: Done)

- AC-06.1 (non-test) On Linux, `brew install matthewsinclair/intent/intent` installs the release's Linux package for the machine's architecture with no checkout of Intent's source, and the installed `intent --version` names the release. -- evidence: kovacs ORDER 8, green (the run AC-03.3's evidence names: refs/bank/cc/0081/kovacs/o8-postcut.sh, blob f9414b29fc73, args 3.2.5 2e0e9215d67f67c665d1b8179a7888555551c8e7, 2026-10-22 21:02:53Z to 21:03:12Z, first red none; its lines are refs/bank/cc/0081/evidence/o8-postcut-lines.txt, blob b3fc943efce9). On kovacs, aarch64 Linux, with brew's intent removed, the tap gone and no checkout at ~/work/Intent, `brew install matthewsinclair/intent/intent` from the published tap at 1de8ca7e5 ran at rc 0, and the installed `intent --version` read `intent 3.2.5 (2e0e9215d67f67c665d1b8179a7888555551c8e7) release`, the commit the tag v3.2.5 points at. The package is the release's own for this architecture by the formula and brew's check: that formula's on_linux on_arm entry is v3.2.5's intent-aarch64-unknown-linux-musl.tar.gz at sha256 3ccb521acc97 (read by cc from the tap at 1de8ca7e5), vc verified the formula's five sha256s against prepare's checksums and the release's two Linux packages, and brew refuses a download whose sha256 differs from the formula's. The run's evidence lines for brew info's URL, the resolved binary and `file -L` ran at rc 0 and sit in the kovacs log, not read by cc. -- satisfied: yes

### WP-07 -- Intent on Ubuntu from its Linux package, as a user uses it (status: Done)

- AC-07.1 (non-test) On Ubuntu with uutils coreutils and mawk, Intent installed from its Linux package with no checkout of Intent's source reports its version, bootstraps, initialises a project in which `intent doctor` reports no finding, creates and lists a thread, starts and stops the daemon, and fires its hooks in a Claude Code session. -- evidence: hv's word, recorded by vc as decision 63 (2026-10-16 10:15:37Z by vc's clock) and relayed to cc as lane owner: satisfied on the evidence in this criterion's note at cbc3aa079 -- o7-b green on kovacs, and hv's real Claude Code session on kovacs where the UserPromptSubmit gate blocked and released and Stop fired; SessionStart stays unconfirmed on screen, as that note records; observed with issue 0601 attached, whose fix lane is cc's by the same decision -- satisfied: yes

## Acceptance Tests

### WP-01 -- Linux release binaries built by GitHub Actions (status: Done)

_(no tests in this group)_

### WP-02 -- Linux assets on each tag's GitHub release, one writer per release object (status: Done)

- AT-02.1 (non-test) At the tag, after releasing.md step 4's `bin/devbin release attach vX.Y.Z`: `gh release view vX.Y.Z --repo matthewsinclair/intent --json assets` lists both Linux packages beside the macOS assets, and `bin/devbin release check` reads that no workflow in .github/workflows writes a release object. -- covers AC-02.1 -- status: n/a -- A non-test row enters at n-a, so AC-02.1's own state carries what is unmet: it stays unsatisfied until the 3.2.7 cut makes the release object this row reads. Half (b) was read before the cut: `bin/devbin release check` exited 0 at 2026-10-27 15:25:32Z by date -u on the vendored devbin of cb664a6c5, with "read 4 workflow file(s) in .github/workflows, and none writes a release object". Half (a) is read after releasing.md step 4's attach, which also adds one SHA256SUMS over every asset. v3.2.6's release has none, so a SHA256SUMS on the release shows the attach ran.

### WP-03 -- Intent's Linux install through Homebrew, in docs/install.md (status: Done)

_(no tests in this group)_

### WP-04 -- First install on a real Windows machine (status: Cancelled)

_(no tests in this group)_

### WP-05 -- intent daemon start --at-login refuses on Linux before writing anything (status: Done)

- AT-05.1 `native/rust/crates/intent-cli/tests/at_login_is_refused_off_macos.rs` -- covers AC-05.1 -- status: green -- Its two arms run off macOS only and are ignored on macOS, where the flag is supported; they are proven on kovacs, hv's Ubuntu VM, and then on Linux CI. vc's ORDER 5 on kovacs (hv's Ubuntu 26.04.1 VM, aarch64), script refs/bank/cc/0081/kovacs/o5-at051.sh, quoted from kovacs-sysadm's log as vc relayed it. RED at base 8c51de484 with the test half alone (blob 20712d9d2c1f, fetched and matched): "test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 802 filtered out" -- the start arm panicked at at_login_is_refused_off_macos.rs:53:3, "a LaunchAgents tree was written: error: `launchctl` could not be run", and the stop arm at :69:3, "stop --at-login did not refuse: ok: intentd is not enrolled at login". GREEN at 11017d7bd, where WP-05 landed: "test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 802 filtered out"; o5 ended rc=0 at 11:19:12Z by kovacs's clock, its worktree removed and its home clean.

### WP-06 -- The Homebrew formula installs Intent on Linux (status: Done)

_(no tests in this group)_

### WP-07 -- Intent on Ubuntu from its Linux package, as a user uses it (status: Done)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
