## Taking the desk

Take control: `intent estate control take`. It succeeds when no controller was ever recorded, when the last one released, or when its session is gone. A take under a name other than the recorded controller's needs `--hv "<hv's words>"`, quoting hv's yes, which is recorded on the take. A session bound to a whiteboard node takes it only as its board's mediator, or on hv's words. While another controller is live the take is refused and names it: stop there and tell hv.

Arm the watch: `intent estate slots watch`. It prints one line per state change under the requests, the step first (`asked:`, `granted:`, `denied:`, `started:`, `ended:` or `revoked:`) and then the request's full id, and it opens with the asks already waiting.

## Judging an ask

- Every prompt in the wearer's session is told the asks waiting for it: the prompt hook runs `intent estate slots pending`.
- Read the host before a grant: `intent estate status` shows the load, the census, the controller, the queue oldest first, and every lane with its type, capacity, holders and the queue for its type.
- Grant into every lane the run will hold: `intent estate slots grant <id> --lane <lane>,<lane>,...`, or `--lane none` for a negligible run. The tool refuses a grant over any named lane's capacity, under a hold, before a `--not-before` request has ended, or over a lane's load admission.
- Refuse an ask with `intent estate slots deny <id> "<why>"`. End a dead or quiet holder's request with `intent estate slots revoke <id> "<why>"`: the slot is freed and the reason recorded, and no process is killed.
- An ask's tags order nothing: weigh them, eg `security`, on a tie. An ask's `--hold` condition is the wearer's to judge before granting it.
- Open, change and close lanes: `intent estate lane open <lane> --type <type> --capacity <n> [--load-under <n>]` and `intent estate lane close <lane>`.
- Hold the whole desk with `intent estate control hold "<condition>"`, or one lane with `intent estate lane hold <lane> "<condition>"`, and release either with its `unhold`. A desk hold means no grants until hv says continue.
- What the tool cannot decide, ask hv rather than guess.

## Wearing it beside a mediator's own work

- An ask that is waiting is answered before the wearer's own next step.
- The wearer's own project books its runs through the same queue and is granted by the same judgement, on the record like any other.
- The desk is released at session end: `intent estate control release`. The record stays, marked released, so the estate reads NO CONTROLLER rather than naming a controller that has gone.
- It passes to another mediator on hv's word, quoted on the take with `--hv`.
