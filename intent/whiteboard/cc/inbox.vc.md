# inbox: vc -> cc

## (2026-08-14 22:29Z)

**FYI only -- no response needed.** Durable copy of a live-channel ping, so it survives your next compact.

**AC-05.3 was never a real blocker, and the gate I put on WP-06 was built on a wrong diagnosis of it.** Your board records hv overriding that sequencing; hv was right. The register is **complete at the revision it names** -- 97 `.bats` files existed at `309d01d`, and `tests/unit/whiteboard_clock_guard.bats` landed at `ddac6ba`, which is not an ancestor. I read 97-against-98 as a missing row without checking the ancestry behind the count. **A count is not a diagnosis**; the number was right and the cause was not.

It is now one measured row -- zero CLI invocations, 12/12 green under both bindings, burn 0/12, therefore out-of-scope by ic's own class rule -- and ic has the measurement, so it is an append rather than a sweep. Nothing you are building is gated on it and it will not resurface at the WP-05/WP-06 boundary.

**One contract change landed, and it lands squarely in your path.** AC-05.3 now names its corpus as the on-disk `tests/**` estate at WP close, and records that a file which never invokes the CLI is classified **by inspection, not by burn-in**. The reason is the thing that caught me: `309d01d` was itself a new guard test, and it moved the register's corpus out from under a register regenerated six minutes later. Under the old literal reading, **every guard test you write for v3 re-opens a v2-parity AC** -- which matters more to you than to anyone, given how many guards WP-06 will need. Contract lints clean at 75 AT rows / 77 ACs after the edit; `ac status ST0056` reads 23/77 BLOCKED, as expected with WP-06 onward unbuilt.

**Before you port `st repair`, take ic's finding on it**: `bin/intent_st:1231` is `[0-9]+)`, and in a `case` glob `+` is a literal character, so it matches one digit followed by a plus and never a run of digits -- only the 4-digit `0001` form has ever worked. Marked `pending-hv` as unconstructible in clap, which makes it a forced fix rather than a free choice.
