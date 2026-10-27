---
st_id: ST0082
title: Intent on Windows through WSL 2: Intent's Linux install inside WSL, no native port
status: Triage
created: 2026-10-14
completed:
---

# ST0082: Intent on Windows through WSL 2: Intent's Linux install inside WSL, no native port

## Objective

Developers who work on Windows, starting with another estate's, can use Intent by installing its Linux build inside WSL 2 through Homebrew on Linux, which ST0081 delivers. The docs name WSL as Intent's Windows route, and a developer installs Intent on a real Windows machine. There is no native Windows port.

Concretely:

1. The docs carry a WSL page: WSL is Intent's Windows route; Intent is installed inside WSL by the Linux install in `docs/install.md`, which the page links to and never restates; repos are kept in the WSL filesystem rather than under `/mnt/c`, and why; Claude Code runs inside WSL; and shellcheck, git and jq are named beside Intent.
2. A developer installs Intent from a tagged release on a real Windows machine and the result reaches hv.

## Context

Opened on hv's word of 2026-10-14 from ST0081's WSL work. hv asked for a thread in each project for running Intent and devbin from a Homebrew install on Linux and macOS with no source checkout, with the WSL work depending on it. ST0081 already held the Linux packages and their place on each release, so under vc's pen it became that thread, and its WSL work came here: the WSL page, from ST0081's WP-03, and the real Windows install, ST0081's WP-04. Their criteria AC-03.1, AC-03.2 and AC-04.1 are descoped from ST0081 and carried here as AC-01.1, AC-01.2 and AC-02.1; AC-01.1 drops its release-download route, because the Linux install is now Homebrew's. ST0081 keeps everything written about the Linux packages, and the paragraphs below are carried from its context of 2026-10-12, with thread numbers updated where they named the work that moved.

Asked by hv on 2026-10-12: Intent should be usable on Windows by another estate's developers, built with GitHub Actions on GitHub. That estate's dc node relayed the ask, and hv asked for ST0081 in vc's session the same day, with no work started until hv says so. hv's rulings that day, through that node: WSL with Linux binaries only; no native Windows port while the Linux binaries run under WSL, so the native route and its compile-check job are off. Devbin already takes WSL as its prerequisite on Windows.

What the other estate said on 2026-10-12: its developers use Windows laptops, and their architecture is not recorded (x86_64 likely, aarch64 unknown). It chose WSL on 2026-09-29 and its research points to WSL 2. Which distributions and releases its developers run is not known, so neither is the glibc floor, and an old floor or a static build is the safe default. Its repositories live inside WSL's filesystem. Whether its developers run Claude Code, and on which side, is not known. Today they get devbin vendored in each repository and nothing installs Intent for them; Intent's own Linux install recipe is needed whether or not devbin later installs Intent, which is the Devbin project's call. Its maintainers use threads, work packages, issues, the whiteboard, the hooks and skills, and the pre-commit gate; which workflows must work for its developers is hv's to rule. The first release it would take is 3.2.4 if this thread lands in it, and no date is set. Whether its machines reach github.com is not known, and who runs the first install on a real machine is hv's to arrange through that estate.

Measured 2026-10-12, carried from ST0081:

- Claude Code's hooks call `intent claude hook`, which runs bash scripts from the install root, so Claude Code has to run inside WSL, where Intent is installed.
- Known WSL behaviour, not measured here: under `/mnt/c`, a Linux file watcher does not see edits made from the Windows side and SQLite locking is unreliable, so repos belong in the WSL filesystem, where the other estate already keeps them.

In lock-step with devbin's ST0008: hv started both threads together on 2026-10-12, and the two vcs agreed three interfaces, recorded on devbin's side in ST0008's design v0.3. (1) The install step has one home, ST0081's install page (its WP-03, `docs/install.md#linux`): devbin's README points at it and never restates it, and the page's path and the package names ST0081's WP-01 produces go to devbin-vc as soon as they exist. (2) The critic gate's contract is `intent`, `shellcheck` and `git` on PATH, because Intent's runner calls shellcheck for the tool-armed shell rules and reports a rule whose tool is absent as unasked rather than failed, so the WSL page names shellcheck beside Intent. (3) A project receives LF rules from both tools, devbin's vendored copy and the chain blocks `intent claude upgrade --apply` writes into tracked `.githooks/` hooks, and one owner writes them (devbin's D46). The joint proof runs before either tag: devbin's WSL job installs ST0081's dispatch-run artifact and runs `check all` on a fixture that carries `intent/`; then Intent tags, the job re-runs against the release asset, and devbin tags after.

hv's rulings in vc's session, 2026-10-12, that came here: start both threads now, and devbin's ST0008 waits for ST0081's Linux binaries rather than descoping its AC-6.5. Still open for hv: where the joint job lives, and whether a hosted runner's WSL 1 counts as evidence. Both vcs propose that it counts for the CLI and the critic gate, and not for intentd's file watching or WSL 2's kernel, so WP-02's run is the WSL 2 evidence.

hv's Ubuntu VM proves Linux and does not prove WSL (ST0081's context), so this thread's WSL evidence is WP-02's real machine and the joint job.

Out of scope: a native Windows build.

DEFERRED PAST 3.2.7 (hv, 2026-10-27, by selection in vc's TUI; vc decision 99). This thread is new surface and no one is working on it, so it is deferred on the record that decision 76 asks for. It stays in Triage, because `intent st hold` is declared only from Not Started and WIP, and it resumes after the 3.2.7 cut on hv's word.

## Work Packages

| WP    | Title                                   | Size | Status      |
| ----- | --------------------------------------- | ---- | ----------- |
| WP-01 | The WSL page                            | XS   | Not Started |
| WP-02 | First install on a real Windows machine | XS   | Not Started |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

## Related Steel Threads

- ST0081: ST0082 depends on ST0081: Intent is installed inside WSL by the Linux install ST0081 delivers, Homebrew on Linux (hv, 2026-10-14)

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
