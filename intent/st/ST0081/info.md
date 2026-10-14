---
st_id: ST0081
title: Intent from Homebrew on Linux and macOS, with no source checkout: Linux binaries on every release
status: WIP
created: 2026-10-12
completed:
---

# ST0081: Intent from Homebrew on Linux and macOS, with no source checkout: Linux binaries on every release

## Objective

A developer installs Intent with Homebrew on Linux or macOS and has a working Intent with no checkout of Intent's source tree: the CLI, the daemon, the hooks and skills, and the pre-commit gate. Every release tag on GitHub carries Linux packages built by GitHub Actions beside the macOS assets, and the tap's formula installs the one that fits the machine. Windows is reached through WSL, which is ST0082's, built on this thread; there is no native Windows port.

Concretely:

1. A release workflow builds `intent` and `intentd` for x86_64 and aarch64 Linux, and packages them with the support payload the binaries read at run time, so a download installs rather than being a bare binary.
2. The binaries run on Linux distributions older than the build host: they are linked statically against musl, and the workflow proves it by running them in a container at an old floor.
3. A `workflow_dispatch` run builds the same packages without a tag, so a tag is never the workflow's first run.
4. The Linux packages reach each tag's GitHub release with one writer per release object, so nothing in CI races the local release flow's `gh release create`.
5. The formula in `matthewsinclair/homebrew-intent` carries Linux entries for both architectures beside the macOS one, written by the release tooling, so `brew install` works on Linux as it does on macOS.
6. Intent installed from its Linux package works on Ubuntu as a user uses it, proven on hv's Ubuntu VM, whose coreutils are uutils and whose awk is mawk.
7. The docs carry Intent's Linux install through Homebrew.
8. `intent daemon start --at-login` and `intent daemon stop --at-login` refuse on Linux before they write anything.

## Context

Restructured on hv's word of 2026-10-14, in vc's session: "a new ST for both Intent and Devbin for both to run from a brew install on a Linux AND macOS install without the need for a local repo checkout of the source tree. THEN: we make ST0081 depend on that new ST." This thread opened on 2026-10-12 as Intent on Windows through WSL and already held that new thread's foundation: the Linux packages (WP-01), their place on each release (WP-02) and the `--at-login` refusal (WP-05), with work landed and tracked files naming ST0081 for it. So under vc's pen the numbers were swapped rather than the work moved: this thread became the Homebrew thread, and its WSL work became ST0082, which depends on it. The WSL page, half of the former WP-03, and the real Windows install, WP-04, went to ST0082 with their criteria AC-03.1, AC-03.2 and AC-04.1, which are descoped there, and so did the context that is only about WSL.

macOS stands as it is. hv, 2026-10-14: "No, not this mac. The macOS pipeline is fine. I have already tested it on a mac back home (gyges) and it all worked ok. We can loop back and retest later, but that route is ok and we don't need to worry about it for now." So hv's decision 17 holds: rhadamanth keeps the dev tree as the delivery, with brew installed, unlinked and pinned, and the aim of no source checkout is for every other machine.

In lock-step with Devbin, whose half is devbin#0195: a tap, `matthewsinclair/homebrew-devbin`, whose formula runs devbin's own installer from the release tag, and a release step that bumps it. Devbin is to carry its own thread for it (hv, 2026-10-14), with its WSL thread ST0008 depending on that thread as ST0082 depends on this one. devbin's formula needs only a pushed tag; Intent's needs the Linux packages on a release, so it waits on WP-02 and D55. devbin's joint check installs this thread's Linux package, so the package names go to devbin-vc as soon as they exist.

As built, measured 2026-10-12 on main a581c15c7 and on GitHub's v3.2.3:

- `rust.yml` builds and tests on Linux for every push to main that touches `native/rust`, `schema` or `surface`, and for pull requests, and adds macOS on tags and on `workflow_dispatch`. It uploads nothing.
- v3.2.3's release assets are macOS arm64 only (`intent-aarch64-apple-darwin`, `intentd-aarch64-apple-darwin`, `Intent.app.zip`, `intent-support.tar.gz`), all made outside CI by devbin's release flow.
- The binaries read their templates from an install root found from the executable's own location, so a bare binary is not an install. Whether `intent-support.tar.gz` serves Linux unchanged is WP-01's first measurement.
- The Linux build is already exercised by the Linux leg on every qualifying push. The risk this thread carries is packaging, release mechanics and where repos live, not compilation.
- `bin/devbin release check` warns on any workflow that has a `tags:` trigger and calls `gh release`, an upload included (`check_release_writers` in `bin/.devbin/lib/cmd/release`: one writer per release object). devbin is vendored here, so a change to its release steps is the Devbin project's to make.

Measured 2026-10-14 on main 72732afb4:

- `.github/workflows/linux-release.yml` (WP-01, landed at 86e4b0f7d) is dispatch-only and has not run, because nothing has been pushed since. It builds both musl triples, x86_64 on `ubuntu-latest` and aarch64 on `ubuntu-24.04-arm`, through `bin/int linux` doctor, build, package and smoke, and smokes each package in `oraclelinux:7-slim` as the floor.
- The tap's formula for 3.2.3 declares `depends_on :macos` and arm64, by hv's ruling of 2026-08-15, so `brew install` refuses on Linux. This thread widens that ruling to Linux, on hv's word above.

Out of scope: a native Windows build, and apt.

Separate from this thread, and outside Intent's tree: the other estate mirrors GitHub's tags (v3.2.3 and later) and main to its own server from a bare clone of GitHub outside every Intent checkout, and each sync starts on vc's word that a release is tagged. hv ruled on 2026-10-12, through that estate's dc node, that its server also hosts each release's binaries beside the tag, starting with the first release that ships Linux binaries (3.2.4 if this thread lands in it), while v3.2.3 stays tag-only. Each sync copies the GitHub release's notes and files as they are, so the asset names and any checksum file this thread puts on GitHub's release are what that estate gets.

hv's rulings in vc's session, 2026-10-12, that stay with this thread: it rides Intent 3.2.4, built by cc after the 3.2.4 CHANGELOG section; WP-01's dispatch run goes first, because it produces the packages devbin's joint check installs; and `intent daemon start --at-login` refuses on Linux before it writes anything (WP-05). Still open for hv: how the packages reach the release (devbin's D55: devbin's release core attaches CI-built assets, which is devbin's proposal and the lean of both vcs, or an upload-only workflow here), and a checksum file on each release.

Linux proving, hv's rulings of 2026-10-12 given to devbin-vc and relayed: the tests run on hv's Ubuntu VM before anything is pushed, nothing goes to GitHub until it works there, and there is one release, 3.2.4, cut only once the work runs on main's HEAD, with no run of releases in quick succession. So WP-01's steps live in one script, `bin/int linux`, which the VM runs by hand and the dispatch workflow calls, and the dispatch run follows the VM's proof and hv's push. The VM is hv's arm64 Ubuntu on rhadamanth (Parallels, guest host `kovacs`, upgrading from 24.04 to 26.04.1 that day), reachable from rhadamanth with `prlctl exec` and `ssh kovacs`. It proves aarch64 Linux natively and x86_64 only through Rosetta or CI, and it does not prove WSL. The tree reaches it as a git bundle of main, because nothing is pushed. A build inside it loads rhadamanth like any other heavy run, so it goes through the census and START/END.

kovacs as measured 2026-10-14 by kovacs-sysadm, read-only: Ubuntu 26.04.1 LTS on aarch64; its coreutils are uutils 0.10.0 and its awk is mawk, while sed, grep and find are GNU; 2 CPUs, 3.3 GB of memory and a 3.8 GB swap file; `/tmp` is a 1.7 GB tmpfs, so a build there points TMPDIR at disk; no Rust, musl-gcc, shellcheck, xmllint or Homebrew yet. kovacs-sysadm, a Claude Code session on the VM, runs the VM side on instructions from intent-vc and devbin-vc (hv, 2026-10-14). The tree now reaches it over ssh from rhadamanth, read-only and pinned to the commit each instruction names, with a bundle through the VM's shared folder as the fallback. Whether CI's Linux runner has uutils or mawk is not measured, so kovacs is where they are proven.

## Work Packages

| WP    | Title                                                                    | Size | Status      |
| ----- | ------------------------------------------------------------------------ | ---- | ----------- |
| WP-01 | Linux release binaries built by GitHub Actions                           | S    | WIP         |
| WP-02 | Linux assets on each tag's GitHub release, one writer per release object | S    | Not Started |
| WP-03 | Intent's Linux install through Homebrew, in docs/install.md              | S    | Not Started |
| WP-04 | First install on a real Windows machine                                  | XS   | Cancelled   |
| WP-05 | intent daemon start --at-login refuses on Linux before writing anything  | XS   | WIP         |
| WP-06 | The Homebrew formula installs Intent on Linux                            | M    | WIP         |
| WP-07 | Intent on Ubuntu from its Linux package, as a user uses it               | S    | Not Started |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
