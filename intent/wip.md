---
verblock: "2026-09-11:v1.42: vc - 3.0.1 SHIPPED (tag v3.0.1 at a8942aead). The 3.0.1 work list is finished; what is still open and what was surfaced to hv at the cut. The list as it stood at the cut is verbatim in git at 89531a8f6."
intent_version: 3.0.1
---

# Work In Progress -- after 3.0.1

## THE RULE (hv, 2026-09-11 09:14Z, verbatim)

> _THERE IS NO NEW WORK TO BE DONE. We are working on these items and these items ONLY._

**It stands until hv sets new work.** The 3.0.1 list it governed is finished. Nothing below is claimed or in flight: each item waits on a ruling from hv or on a machine this estate does not have.

**The live state is the register, not this file.** `intent issues list` is what is open; `intent ac gate <ST>` is where a thread stands.

## Open

- **`0177`** (medium): `ext` ships the creating half without the undoing half. Post-cut, no owner (hv, 2026-09-11). `config`, `ext` and `learn` ship declared-and-unbuilt (hv, 2026-08-31).
- **ST0056 stays open on AC-00.5 and AC-11.1**, and WP-11 with it. Both ask for a `brew install` on a machine that has never seen this repository. rhadamanth has, so its cold tap install (`int macos smoke --reinstall`, green at formula `fc32170`) does not satisfy them. Needs a clean Mac: install from the tap, run `intent` and the `intentd` lifecycle, then satisfy both by evidence and close WP-11 and ST0056.
- **Out of 3.0.1 by ruling:** ST0057, ST0060 (vault), ST0069 (post-cut), ST0070 (LLM config).

## Surfaced to hv at the cut, not worked

Each is recorded in the named commit's message. None is on a list until hv rules.

1. **A version bump turns every realised view into doctor view-skew** (`36839061a`). The banner carries the tool version; the finding blames a hand edit; for an unlisted thread its only remedy, `st hydrate`, also pins it. 24 banner-only views on 9 unlisted threads here, and a user upgrading from 3.0.0 will likely see the same. The release preflight's doctor refuses until this is resolved.
2. **A stranger's `brew tap` may refuse our tap as untrusted** (`549031396`, dc). Seen on Homebrew main with an empty trust store; not measured on stable. If stable does it too, the install docs need `brew trust --tap matthewsinclair/intent` first.
3. **Linux and Intel macOS binaries are not built.** hv, 2026-09-11: _"I only care about macOS and Linux."_ The formula is macOS arm64 by `depends_on`, and taps cleanly everywhere. A Linux build, package and publish is size L.
4. **CI did not run for 3.0.1** (no credit; hv waived it). The rust.yml clippy-to-zero and the tests.yml bats-on-v3 changes are unexercised by CI. The local evidence is fmt, clippy -D warnings, Rust 2353/0 and shell 716/0 at `1409aff70`, plus the release preflight's suites at the tag.
5. **Stale plugin references** (`77b724c55`, dc): MODULES.md's plugin table lists six scripts that are gone, and DECISION_TREE.md:29 and writing-extensions.md:151 cite `claude_plugin_helpers.sh` as live.
6. **ST0073 AC-05.1 names a WP-05 the thread lacks.** `doctor --scope all` shows it; the thread is closed.
7. **`set <ac> kind non-test` leaves state `computed`**, illegal for non-test, and only withdraw-then-reinstate moves it (`7c40da0ab`).
