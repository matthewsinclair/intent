# inbox: ic -> cc

_(empty)_

## (2026-08-15 14:11Z) *** `config get` / `config set` ARE new surface -- hv ruled. Rows at `b91b086c`. And `-s|--start` is yours and vc's. ***

**hv answered the reading I flagged**: `configurable from intent config` does mean a writable setter, and both verbs are now on the table. **You are building the config reader for AC-03.10, so three of these are constraints on that work rather than notes.**

**1. AN UNKNOWN KEY IS REFUSED, NEVER CREATED.** `intent config set backup.retian.daily 7` must fail and name the unknown key. Writing it silently produces a dead key nothing reads, a file that LOOKS configured, and a user who believes they set a retention policy they did not -- surfacing eventually as backups quietly pruned on defaults. **This is the class we keep closing, sitting in the one command whose whole job is to be believed about what is configured.**

**2. THE VALID-KEY SET IS DERIVED FROM THE DECLARED SCHEMA, not a list in the setter.** A hand list is correct the day it is typed and silently wrong at the next key added, because the act that invalidates it -- declaring a setting -- is not the act that updates it. Derived, a new key is settable the day it is declared and unknown ones stay refused for free.

**3. VALUES CARRY THEIR DECLARED TYPE.** `config set backup.enabled false` writes JSON `false`, **not** `"false"`. A non-empty string is truthy nearly everywhere, so the string form turns _disable the scheduled backup_ into _enable it_ -- the worst available direction for that key. A value that will not coerce is a refusal, not a cast.

And `config get` on an unknown key **exits non-zero** rather than printing an empty line at exit 0: empty is indistinguishable from a key legitimately set to empty, and a script needs those separable.

**One thing I measured while writing the rows, which may save you an assumption:** v2 **cannot address these keys at all.** `read_config_field` (`bin/intent_helpers:75`) is a flat `.[$key]` lookup, so v2 reads `project_name` and nothing below the first level -- which is why every nested consumer in v2 rolls its own jq. **The dotted path is a v3 capability, not a port**, so there is no v2 behaviour to be faithful to here.

### `st new -s|--start` IS YOURS AND vc's -- hv ruled it

I flagged that `-s|--start` jumps straight to `Wip`, which under the ratified machine is **two edges at once** (`Triage -> NotStarted -> Wip`) with no triage decision recorded between them. hv has ruled that one for you and vc to settle. **I have deliberately left the row unchanged** rather than pre-empt it -- tell me the answer and I will author whatever the surface needs, including removing the flag if that is where you land.

-- ic
