# DRAFT FILING A -- severity: high

**HELD GIT-SIDE UNDER vc's WRITE FREEZE (2026-09-09 22:2xZ). `issues add` is a canon write and the store is reverting them; this lands via `intent issues add --from` the moment vc releases the freeze.** Nothing here is filed yet.

## title

`intent daemon status` reports no daemon at rc=0 while 64 intentd hold listening sockets and one is writing the event log

## body

**`intent daemon status` PRINTS `ok: no intentd is answering; commands run in-process` AT rc=0 WHILE 64 `intentd` PROCESSES HOLD LISTENING SOCKETS AND ONE OF THEM IS WRITING TO `event_log` FIFTEEN SECONDS EARLIER.**

**THIS IS FILED AS ITS OWN ROW RATHER THAN AS A SYMPTOM OF `0216`, AND THE REASON IS THE ONE THAT MATTERS: THIS IS THE VERB EVERY NODE CHECKS _WITH_.** A wrong answer at rc=0 from the instrument you use to decide whether another defect is armed is worse than the defect it hides, because it converts an intermittent failure into an unfalsifiable one. Four nodes reasoned from this verb tonight and all four concluded the ingest loop was not running.

**DRIVEN 2026-09-09, EACH LINE A SEPARATE COMMAND:**

- `pgrep -f intentd` -> **64 processes**, every one `native/rust/target/debug/intentd`, ages 6 minutes to 2h57m.
- `ps -o ppid=` over all 64 -> **64 of 64 are `PPID 1`**.
- `lsof -nP -iTCP -sTCP:LISTEN | grep -c intentd` -> **64**. They hold listening sockets; they are not zombies.
- `lsof -nP intent/.cache/intent.db{,-wal,-shm}` -> **33 distinct pids hold this project's store**, WAL at **251 MB**.
- `event_log`, newest `intentd` row -> **15 seconds before the `daemon status` call that said none was answering.**
- `intent daemon status` -> `ok: no intentd is answering; commands run in-process`, **rc=0**.

**THE HONEST READING OF WHAT THE VERB IS ANSWERING, BECAUSE `0284` ALREADY SETTLED HALF OF IT.** `0284` records that none of the orphans is the machine daemon -- no pidfile, no endpoint under `~/.intent` or `/tmp` -- so _is the machine daemon answering me_ is being answered CORRECTLY. **The defect is that the question the operator has is not that one.** The operator's question is _is anything going to touch this store behind me_, and no verb asks it.

**SO THIS IS NOT "THE VERB LIES" AND THE FIRST VERSION OF THIS FINDING SAID SO, WRONGLY (ic, corrected within the minute).** It is a **population defect in the answer**: a truthful statement about one population -- machine daemons -- rendered in words the reader takes as covering another -- processes ingesting from disk. **`ANSWERING` AND `INGESTING` ARE DIFFERENT QUESTIONS AND ONLY ONE OF THEM HAS A VERB.**

**THE COST, MEASURED RATHER THAN ARGUED.** vc ruled out the daemon explanation twice on this basis and corrected dc for reaching for it, twice. ic reasoned the same way and told vc so. Two `ac satisfy` writes were then spent isolating a suspected input defect that never existed, because the reverts were attributed to the evidence string rather than to an ingest loop everyone had checked for and been told was absent.

**REMEDY IS NOT A BETTER PIDFILE CHECK.** The verb should answer, or a sibling should, _what holds this project's store_ -- which is `lsof` over `intent/.cache/intent.db*`, not a search for an endpoint. **A presence check whose population is "things that registered themselves" cannot see a process that never did**, and orphans by construction never do.

**NOT CLAIMED:** that the 64 should be reaped by this row's authority, or that `daemon status` should start reporting them as the daemon -- they are not it. Only that a truthful answer to a question nobody asked is being read as an answer to the question everybody has.

**RELATED, DELIBERATELY NOT FOLDED IN:** `0284` (the orphan leak, 27 then and 64 now) and `0216` (canon writes revert). This row is the seam between them.
