---
st_id: ST0081
title: Intent on Windows through WSL: Linux binaries on every release
status: WIP
created: 2026-10-12
completed:
---

# ST0081: Intent on Windows through WSL: Linux binaries on every release

## Objective

Developers who work on Windows, starting with another estate's, can use Intent by running its Linux build inside WSL 2. Every release tag on GitHub carries Linux binaries built by GitHub Actions beside the macOS assets, a documented install takes a WSL distro to a working Intent, and the docs name WSL as Intent's Windows route. There is no native Windows port.

Concretely:

1. A release workflow builds `intent` and `intentd` for x86_64 Linux, and for aarch64 Linux if that is cheap, and packages them with the support payload the binaries read at run time, so a download installs rather than being a bare binary.
2. The binaries run on Linux distributions older than the build host: they are linked statically or built against an old glibc floor, because the distributions the developers run are not known, and the workflow proves it by running them in a container at that floor.
3. A `workflow_dispatch` run builds the same assets without a tag, so a tag is never the workflow's first run.
4. The Linux assets reach each tag's GitHub release with one writer per release object, so nothing in CI races the local release flow's `gh release create`.
5. The docs carry Intent's own Linux install recipe and a WSL page: where to unpack a release so the install root resolves, `intent bootstrap`, keeping repos in the WSL filesystem rather than under `/mnt/c`, and running Claude Code inside WSL.
6. A developer installs Intent from a tagged release on a real Windows machine and the result reaches hv.

## Context

Asked by hv on 2026-10-12: Intent should be usable on Windows by another estate's developers, built with GitHub Actions on GitHub. That estate's dc node relayed the ask, and hv asked for this steel thread in vc's session the same day, with no work started until hv says so. hv's rulings that day, through that node: WSL with Linux binaries only; no native Windows port while the Linux binaries run under WSL, so the native route and its compile-check job are off. Devbin already takes WSL as its prerequisite on Windows.

What the other estate said on 2026-10-12: its developers use Windows laptops, and their architecture is not recorded (x86_64 likely, aarch64 unknown). It chose WSL on 2026-09-29 and its research points to WSL 2. Which distributions and releases its developers run is not known, so neither is the glibc floor, and an old floor or a static build is the safe default. Its repositories live inside WSL's filesystem. Whether its developers run Claude Code, and on which side, is not known. Today they get devbin vendored in each repository and nothing installs Intent for them; Intent's own Linux install recipe is needed whether or not devbin later installs Intent, which is the Devbin project's call. Its maintainers use threads, work packages, issues, the whiteboard, the hooks and skills, and the pre-commit gate; which workflows must work for its developers is hv's to rule. The first release it would take is 3.2.4 if this thread lands in it, and no date is set. Whether its machines reach github.com is not known, and who runs the first install on a real machine is hv's to arrange through that estate.

As built, measured 2026-10-12 on main a581c15c7 and on GitHub's v3.2.3:

- `rust.yml` builds and tests on Linux for every push to main that touches `native/rust`, `schema` or `surface`, and for pull requests, and adds macOS on tags and on `workflow_dispatch`. It uploads nothing.
- v3.2.3's release assets are macOS arm64 only (`intent-aarch64-apple-darwin`, `intentd-aarch64-apple-darwin`, `Intent.app.zip`, `intent-support.tar.gz`), all made outside CI by devbin's release flow.
- The binaries read their templates from an install root found from the executable's own location, so a bare binary is not an install. Whether `intent-support.tar.gz` serves Linux unchanged is WP-01's first measurement.
- The Linux build is already exercised by the Linux leg on every qualifying push. The risk this thread carries is packaging, release mechanics and where repos live, not compilation.
- `bin/devbin release check` warns on any workflow that has a `tags:` trigger and calls `gh release`, an upload included (`check_release_writers` in `bin/.devbin/lib/cmd/release`: one writer per release object). devbin is vendored here, so a change to its release steps is the Devbin project's to make.
- Claude Code's hooks call `intent claude hook`, which runs bash scripts from the install root, so Claude Code has to run inside WSL, where Intent is installed.
- Known WSL behaviour, not measured here: under `/mnt/c`, a Linux file watcher does not see edits made from the Windows side and SQLite locking is unreliable, so repos belong in the WSL filesystem, where the other estate already keeps them.

Out of scope: a native Windows build, and Linux package managers (apt, Homebrew on Linux).

Separate from this thread, and outside Intent's tree: the other estate mirrors GitHub's tags (v3.2.3 and later) and main to its own server from a bare clone of GitHub outside every Intent checkout, and each sync starts on vc's word that a release is tagged. hv ruled on 2026-10-12, through that estate's dc node, that its server also hosts each release's binaries beside the tag, starting with the first release that ships Linux binaries (3.2.4 if this thread lands in it), while v3.2.3 stays tag-only. Each sync copies the GitHub release's notes and files as they are, so the asset names and any checksum file this thread puts on GitHub's release are what that estate gets.

In lock-step with devbin's ST0008: hv started both threads together on 2026-10-12, and the two vcs agreed three interfaces, recorded on devbin's side in ST0008's design v0.3. (1) The install step has one home, this thread's install page (WP-03): devbin's README points at it and never restates it, and the page's path and the asset names WP-01 produces go to devbin-vc as soon as they exist. (2) The critic gate's contract is `intent`, `shellcheck` and `git` on PATH, because Intent's runner calls shellcheck for the tool-armed shell rules and reports a rule whose tool is absent as unasked rather than failed, so the WSL page names shellcheck beside Intent. (3) A project receives LF rules from both tools, devbin's vendored copy and the chain blocks `intent claude upgrade --apply` writes into tracked `.githooks/` hooks, and one owner writes them (devbin's D46). The joint proof runs before either tag: devbin's WSL job installs this thread's dispatch-run artifact and runs `check all` on a fixture that carries `intent/`; then Intent tags, the job re-runs against the release asset, and devbin tags after.

hv's rulings in vc's session, 2026-10-12: start both threads now; ST0008 waits for this thread's Linux binaries rather than descoping its AC-6.5; this thread rides Intent 3.2.4, built by cc after the 3.2.4 CHANGELOG section, and WP-01's dispatch run goes first because it produces the install step devbin needs; and `intent daemon start --at-login` refuses on Linux before it writes anything (WP-05). Still open for hv: how the assets reach the release (devbin's D55: devbin's release core attaches CI-built assets, which both vcs lean to, or an upload-only workflow here), where the joint job lives, a checksum file on each release, and whether a hosted runner's WSL 1 counts as evidence. Both vcs propose that it counts for the CLI and the critic gate, and not for intentd's file watching or WSL 2's kernel, so WP-04's run is the WSL 2 evidence.

Local Linux runs before CI: hv's Ubuntu 24.04 arm64 VM on rhadamanth (Parallels, guest host `kovacs`), reachable from rhadamanth with `prlctl exec` and `ssh kovacs`. It proves aarch64 Linux natively and x86_64 only through Rosetta or CI, and it does not prove WSL. A build inside it loads rhadamanth like any other heavy run, so it goes through the census and START/END.

## Work Packages

| WP    | Title                                                                    | Size | Status      |
| ----- | ------------------------------------------------------------------------ | ---- | ----------- |
| WP-01 | Linux release binaries built by GitHub Actions                           | S    | Not Started |
| WP-02 | Linux assets on each tag's GitHub release, one writer per release object | S    | Not Started |
| WP-03 | Intent's Linux install recipe and the WSL page                           | S    | Not Started |
| WP-04 | First install on a real Windows machine                                  | XS   | Not Started |
| WP-05 | intent daemon start --at-login refuses on Linux before writing anything  | XS   | Not Started |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
