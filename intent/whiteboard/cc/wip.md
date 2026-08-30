---
node: cc
name: Control Claude
role: control
session_id: ae8c8153-6f3f-438f-b96b-04bd381ad4ed
heartbeat_at: 2026-08-30 19:47Z
status: paused
focus: "FOLDED 2026-08-30, pre-fold at .history/20260830/wip-fold-1947Z.md. AC-08.8 closed by vc at 99d5d8d1; AC-08.9 built and green at 41f07d45 -- intentd serves the turtle on 51737 and one port carries both protocols. HOLD: the arm-6b guard fix is worktree-only and waits on ic committing tui-design.md. Next: AT-08.4/AT-08.7 authoring, then backup.enabled."
claims: [ST0056/06, ST0056/08, ST0056/10, ST0056/13, ST0057/00]
---

# Control Claude (cc)

## DOING

**`AC-08.9` IS BUILT AND GREEN AT `41f07d45`.** `http://127.0.0.1:51737/` answers a shell page carrying the mark; `POST /op` answers the SAME `dispatch` the socket answers. Six tests through a real `intentd`.

- **BOTH OF MY TRANSPORT READINGS WERE WRONG AND THE MEASUREMENT THAT KILLED THEM IS `candidates_under`:** unix is ALWAYS first and TCP is APPENDED, so that port is **the socket's UNDERSTUDY**, reached exactly when the socket is what is broken. Giving it to HTTP alone removes a fallback whose whole job is the failure nothing else routes around; a second published port reintroduces _which port_ one level up.
- **BYTE 0 IS EXACT, NOT CLEVER.** Every framed request begins `{` (the probe included); every HTTP request begins with a method letter; `fetch` cannot help sending a method first. `peek` leaves the byte in the kernel, which is why the frame branch is **untouched** rather than taught to carry a pushback buffer.
- **SORTED IN A PER-CONNECTION TASK, NEVER IN THE ACCEPT ARM.** Deciding means reading a byte, and a client that connects and says nothing would hold the whole accept loop -- a daemon-wide stall reachable by `nc` and a newline nobody types.
- **THE SHELL IS UNGATED AND `/op` IS NOT: LOOPBACK IS NOT A PERMISSION BOUNDARY.** Any page the browser is showing can `fetch` it. The token is 0600, written BEFORE the port is published, one per run, compare-and-delete on drop.
- **51737 IS A PREFERENCE, NEVER A PROMISE**, so D6 is intact: ask for it, fall back to a kernel port, publish what was bound. Parallel tests already exercise the fallback.
- **HALF THIS CRITERION WAS BUILT BY ic BEFORE I NEEDED IT** -- `form::triples` and `nav::View`, landed today for a reason about a third row. Recorded rather than quietly consumed.

## TODO

- **HOLD -- `shared_artefact_build_guard.sh` IS WORKTREE-ONLY AND CANNOT BE COMMITTED YET.** The fix is right and all 15 arms pass, so **everyone is unblocked by the worktree copy**. It is a recorded ST0056 attachment, so canon must carry its new bytes in the same commit -- and the canon extract currently also carries **ic's uncommitted `tui-design.md`**. Waits on ic landing that file. **Do not commit canon before then.**
- **`AT-08.4` and `AT-08.7` are `to-write` and their cited paths do not exist.** My tests are real and green at `crates/intent-cli/tests/daemon_lifecycle.rs`; the rows cite `crates/intentd/tests/`. **The features are built, the AT rows are not authored, and the citation is wrong about the directory** -- because a package's integration tests can only build ITS OWN binaries, and `daemon start`/`stop`/`--at-login` are the `intent` binary. Same reason `daemon_and_local_agree.rs` lives there and says so.
- **`backup.enabled` IS RATIFIED AND HAS ZERO READERS** -- vc ruled it LIVE and it is mine to implement. It gates the daemon sweep and NOTHING else: not `intent backup`, not doctor staleness, not `BackupFailing`. The ratified shape is three switches and one does not exist.
- **`AC-08.9` follow-on is ic's, not mine.** The UI goes on `/op`; building it here would take WP-17's criterion.
- **WP-13 (search, XL)** stays claimed and unbuilt; hv sequenced it post-tag.

## Watch-outs

**AN UNTRACKED FILE CAN CHANGE WHAT A SHARED GUARD SAYS ABOUT EVERY NODE, WITH NO SIGNAL TO ITS AUTHOR.** My untracked `web.rs` embedded the logo from `docs/design/`; arm 6b refused, and **the refusal landed on vc**, because the guard runs at commit and my last commit predated the file.

**AND WIDENING THE DECLARED HOMES DID NOT FIX IT, BECAUSE THE RULE HAD A THIRD HOME.** The list, the list's shell twin, and a hardcoded `case` inside the arm that recognised exactly one directory. **Only the third decided the verdict**, so widening the first two looked correct, read correctly, and did nothing. A guard whose own message says _the declared scope does not cover them_ could not answer that question.

**THE TWO SHARED-INDEX MITIGATIONS ARE INDIVIDUALLY CORRECT AND MUTUALLY EXCLUSIVE.** `git commit --only` gives PATH granularity and forces you to take the whole file; `git apply --cached` gives HUNK granularity and forces you to commit the whole INDEX. **In a file two people are editing while a third has something staged, no move is safe on both axes, and neither party can see the other's constraint from their own side.** That is the tooling running out, not anyone's discipline. It cost three nodes' states lining up to land one hunk. **The one countermeasure with a clean record all day is a message sent first.**

**THE GATE'S FORMATTER REFUSALS RE-OPEN THE WINDOW THEY ARE PROTECTING.** `prettier`, then `rustfmt`, each a separate refusal -- so _stage, refuse, format, re-stage, commit_ is four index operations where the mitigation assumes one.

**`cargo test -p intent-cli` DOES NOT REBUILD `intentd`.** A control that cannot fail certifies a test that cannot fail. Always `cargo build -p intentd` first.

**A UNIX SOCKET PATH HAS A LENGTH LIMIT AND THE SCRATCHPAD EXCEEDS IT.** A fixture `HOME` under the session scratchpad fails `SUN_LEN` at 143 bytes; `RealDaemon` uses `short_dir` for exactly this.

**A WELL-TESTED FUNCTION ON AN UNTESTED PATH.** `backup_retention.rs` drove the buckets hard and passed its own `Retention` in by hand every time -- structurally blind to whether the value ever came from config. **Good coverage of the destination is what made the missing road invisible.**

**`#[serde(flatten)]` GIVES A CATCH-ALL ONLY THE KEYS NO NAMED FIELD CLAIMED.** Typing `backup` for `schedule`'s sake silently emptied `extra` for the retention reader in the same file.

## Decisions

- (2026-08-30) **One published port, both protocols, disambiguated at byte 0** -- vc's ruling on my two readings, and the understudy measurement is why both were wrong.
- (2026-08-30) **The HTTP body is `wire::frame`'s bytes** -- no second serialiser, and `serde_json` stays out of `intentd`'s manifest.
- (2026-08-30) **`/op` binds per REQUEST where the socket binds per CONNECTION**, and that is faithful: the rule exists so a connection cannot WANDER, and an HTTP request has no history to depend on.
- (2026-08-30) **`Op::Shutdown` is refused over HTTP** -- a page must not be able to end the daemon on a token it read.
- (2026-08-30) **`backup::cycle` is the one composer of a snapshot and its prune**; `take`/`prune` stay public for unit coverage, because a TEST calling both is not a second implementation.
- (2026-08-30) **The retention keys are ic's, typed as `backup.retain.{daily,weekly,monthly}`**, monthly default corrected 6 -> 12.
- (2026-08-30) **`daemon start` on a running daemon exits 0** -- the fact lives in `basis` with its measurement, NOT in `populations.self_loop`.
