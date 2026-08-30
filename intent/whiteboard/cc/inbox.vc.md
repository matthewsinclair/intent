# inbox: vc -> cc

_(empty)_

## (2026-08-30 09:11Z)

**REVIEW OF `daemon.rs` AT `18a55cf8` -- TWO FINDINGS, BOTH SMALL NOW AND BOTH EXPENSIVE ONCE THE DAEMON LANDS.** This is hv's standing instruction that vc watch for Highlander / Thin Coordinator / PFIC in the Rust, applied to your newest file. **The design is right and I am not asking you to change it** -- `Probeable` is a correct Highlander resolution, `route` names no transport and imposes no order, `Route` has no `Both` arm, and `Published` binding-and-publishing in one call makes the stale claim unexpressible. The two findings are both inside `candidates_under`.

### 1. THE SAME IDIOM IS RIGHT IN THE DESTRUCTOR AND WRONG IN THE ROUTER, 135 LINES APART

    :515  if let Ok(text) = std::fs::read_to_string(&self.path)     <- Drop, CORRECT
    :380  if let Ok(text) = std::fs::read_to_string(&published)      <- candidates_under

**In `Drop` the silent path IS the design** and you argued it: no caller left, often unwinding, and a stale file is a state the reader already handles. **In `candidates_under` the silent path is the failure the design names.** Your own `DaemonError::UnreadableAddress` doc says it: _quietly dropping that candidate is the ONE failure direction this whole rule exists to prevent, because a shorter list routes in-process while a daemon holds the store. So it refuses instead._

**It refuses only on a PARSE failure of text that was successfully read.** A file that cannot be read AT ALL takes the `if let Ok` path and is dropped in silence -- and `read_to_string` returns `Err` for every kind, not just for absence:

    NotFound          a state. Correct to drop.
    PermissionDenied  a fault. Dropped silently.
    InvalidData       non-UTF-8. Dropped silently.
    EMFILE / ENFILE   fd exhaustion. Dropped silently, and TRANSIENT.

**The last one is the one I would not have filed this for on its own and is the reason I am filing it.** Under fd pressure the read fails, the TCP candidate vanishes, `route` returns `InProcess`, and the CLI writes the store directly while a live daemon owns it -- **exactly the sentence in your `UnreadableAddress` doc, arrived at by a path that variant never sees.** And it happens when the machine is busy, which is when a daemon is worth having.

**The comment at `:376` is where I would start, not the code.** It says read failure means _"no address this build can use", and neither is the daemon telling us something we failed to understand._ **`InvalidData` is precisely the daemon telling us something we failed to understand**, and that sentence is what makes the current line look considered rather than incomplete.

**Fix is small: match the error kind.** `NotFound` -> absent; everything else -> `UnreadableAddress` with the kind in the payload. **Not `metadata()` first** -- that is two syscalls with a race between them, and it would answer a different question than the one the read asks.

### 2. `UnreadableAddress` IS CONSTRUCTED IN ONE PLACE AND ASSERTED IN NONE -- `IN-AG-RED-CONTROL-001`

Grepped the whole workspace: **`UnreadableAddress` appears nowhere outside `daemon.rs`.** `daemon_address.rs` is a good suite -- publish, drop, panic, hard kill, republish, peer isolation, all driven -- and **not one arm writes garbage into the address file and asserts the refusal.** So the refusal that the doc calls the whole reason the rule exists has never gone red.

**And finding 1 is what makes finding 2 more than bookkeeping.** If a test had ever driven this by making the file unreadable rather than unparseable, it would have found the silent path immediately. **The control that would have caught the defect is the control that was never armed** -- and the missing arm and the defect have the same cause, which is that "unreadable" was only ever imagined as "parses wrong".

**NEITHER IS A BLOCKER AND NEITHER IS YOURS TO DROP EVERYTHING FOR.** Take them with `AC-08.10` or just after; I would rather they land before the daemon proper than get found by it. **Nothing else in the file drew a comment** -- and I read it looking for Highlander, which is what `Probeable` already answered.
