---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-30 18:37Z
status: paused
focus: "FOLDED 2026-08-30, pre-fold at .history/20260830/wip-fold-1836Z.md. WP-08 is 11 of 12: AC-08.5 closed, AC-08.4 and AC-08.7 built and green, --at-login wired on ic rows. AC-08.7 REASONING LIVES BELOW BECAUSE ITS COMMIT MESSAGE WENT INTO vc a1cf22c0 -- second time today a message went to somebody else. Next: AC-08.8 (backup through the SAME service call) then AC-08.9 (the web face)."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0056/13, ST0057/00]
---

# Control Claude (cc)

## DOING

**WP-08 IS 11 OF 12. `AC-08.4` AND `AC-08.7` ARE BUILT AND GREEN; `AC-08.5` CLOSED AND vc DROVE IT.** Landed today: `fa84769f` (init mints), `11323207` (`AC-08.5`), `1b9a84c4` (LaunchAgent), `08bf9936` (`Op::Shutdown`), `0d89ab4d` (pid in the lock), `a402625f` (start/stop + the sweep hazard), and `AC-08.7` inside `a1cf22c0`.

**`AC-08.7`'s RECORD LIVES HERE BECAUSE ITS COMMIT MESSAGE WENT INTO vc's FOLD.** Second time today; the bytes are correct and verified at HEAD, only the message was lost.

- **A generated local artefact carries a version marker; on boot, missing or stale regenerates.** WP-08's own such artefact is the LaunchAgent plist, which carried no marker at all.
- **THE STAMP IS THE TRIGGER AND THE CONTENT IS WHAT HEALS.** A version bump alone rewrites a byte-identical file, which is a no-op. What the marker buys is that when content DOES change -- a new log path, a key we start emitting, a binary that moved -- the operator's plist comes up to date with no step anyone can skip. That is the whole of _heals without a migration_.
- **AN ABSENT MARKER IS STALE, AND THAT IS THE REAL POPULATION**: every plist written before the stamp existed looks exactly like that.
- **BUT NOT ENROLLED IS NOT STALE, AND THAT ARM BOUNDS THE FEATURE.** Treating absence as staleness would enrol an operator who never asked, silently, as a side effect of starting a daemon for one session. **Absent and out-of-date are different states.**
- **THE DAEMON REGENERATES AND DOES NOT RELOAD ITS OWN JOB.** Unloading our own label stops the process doing the healing, and `KeepAlive` is false, so nothing would bring it back. Reported, never fatal: refusing to boot over a plist turns cosmetic staleness into an outage.
- **THE `launchctl` HALF IS DELIBERATELY UNDRIVEN AND THAT IS A STATED GAP.** `launchctl` is user-scoped, not `HOME`-scoped, so a test calling `load` would enrol THIS machine under the real label. `write_plist` is split from `load` precisely so everything except those two calls is driven.

## TODO

- **`AC-08.8`** -- the scheduled backup through the SAME service call the CLI uses. The check is that the two paths are one function, never two that agree today.
- **`AC-08.9`** -- the web face. Must CALL the shared derivation beside `form.rs`, never re-walk the declaration. Sibling-only resolution per vc, not D19's refuted `PATH-then-sibling`.
- **WP-13 (search, XL)** stays claimed and unbuilt; hv sequenced it post-tag.

## Watch-outs

**`git add` PUBLISHES TO A SHARED INDEX AND NOTHING IN THE PROTOCOL SAYS SO.** Three instances today: ic's `--amend` took seven of my files, dc nearly took vc's in `render.rs`, and vc's fold took four of mine. **`git commit --only` protects the COMMITTER from taking someone else's bytes and does nothing to protect a STAGER from having theirs taken.** That asymmetry is the actual defect and none of our three rules names it. Mitigation: stage and commit in ONE command -- the pre-commit gate's own 15 arms are what widen the window.

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`, AND IT MADE ONE OF MY CONTROLS VACUOUS.** I removed the healing call, ran the test, saw green, and nearly concluded the arm was worthless -- the daemon binary on disk still had the healing compiled in. **A control that cannot fail certifies a test that cannot fail.** Fourth instance of this class today and the only one inside a control. Always `cargo build -p intentd` first.

**`launchctl` IS USER-SCOPED, NOT `HOME`-SCOPED.** Isolating `HOME` isolates the plist's PATH and not the job: `launchctl load <plist>` registers `com.matthewsinclair.intentd` for the real user whatever `HOME` says. ic found the same shape from the other side -- **isolating the PROJECT does not isolate the daemon**, because the socket is a user path.

**`via_library` IN `dual_path_conformance` DOES NOT SET `HOME`.** `via_binary` does. Any row reaching `userstate::home()` in-process reaches the developer's real state. Closed for `daemon start`/`stop` by `Hazard::ActsOnTheRealHome`; **the asymmetry is still there for the next row that touches ambient state**.

**A HALF-TRUE STATEMENT GETS BELIEVED WHERE A WHOLLY WRONG ONE GETS CAUGHT.** The `daemon start` row asserted a refusal that no longer happens beside an eviction claim that still holds -- so a reader who checks the interesting half finds it sound and has no reason to doubt the other. vc's general fix: where a structured field exists for a claim, prose stating the same claim is a second home and goes.

## Decisions

- (2026-08-30) **`init` mints a `project_id`** -- one home, `project::mint_project_id`, reached by both `init` and `stamp_version`. `Config::identity()` treats `""` as absent.
- (2026-08-30) **The sync carve-out asks about THIS project**, not this machine; `watched` is on the wire, filled from the watch's presence.
- (2026-08-30) **`Op::Shutdown` is primary for `stop`, signal is the fallback** -- and the reply is flushed before the loop is told, or a clean stop is indistinguishable from a crash.
- (2026-08-30) **The pid lives in the lock, truncate-then-write** (vc's ordering requirement). Empty AND partial reads are refused: `12` out of `12345` parses and names a stranger.
- (2026-08-30) **`daemon start` on a running daemon exits 0** -- `populations.self_loop`, 30 members, vc's citation rather than my systemctl reasoning.
