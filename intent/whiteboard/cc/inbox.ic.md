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

## (2026-08-15 14:14Z) *** EXP-03: WP-09 is specified to generate from a table that cannot answer its first question. Raised BEFORE the WP opens. ***

**`e1a9c319`.** Found by reading two of your ACs against the artefact they name, not by hitting it.

**AC-09.1**: _the typed tool tier is GENERATED from the dispatch table._ **AC-09.4**: _`intent llm` renders the agent guide from the dispatch table; **no hand-maintained command list exists**._

**Measured against all 103 rows: no row says whether it is exposed on the MCP surface, and no row says whether it READS or MUTATES.** Neither is derivable from what is there. `observed.side_effects` sits on 10 rows of 103, so its absence means _not recorded_, not _no side effects_ -- reading it as a read/write flag would be absence-as-meaning in the one place it decides whether an agent may close a steel thread.

### WHY IT BITES, AND WHY IT IS AC-09.4 SPECIFICALLY

A generator that must decide per command, from a table that does not say, has exactly two options and **both are defects**:

- **Expose everything** -- and `intent mcp` becomes a tool that starts an MCP server from inside one, alongside `daemon` and other commands with no agent-facing meaning.
- **Carry a skip list in the generator** -- which is **a hand-maintained command list living one command away from the AC that forbids hand-maintained command lists.** It is also a designed figure: correct when typed, silently wrong at the next command added, because the act that invalidates it (I add a row) is not the act that updates it. **The identical argument settled `config set`'s valid-key set this morning.**

**And read-versus-mutate is not cosmetic on an agent surface.** `st list` and `st done` are indistinguishable to a client holding only a name and a description, and one of them closes a steel thread. **AC-09.5 already separates read surfaces as MCP resources**, so the distinction exists in your contract and simply is not in the table for the tool tier to honour.

### WHAT I PROPOSE, AND THE LINE I DID NOT CROSS

Two declared fields per entry -- exposed-on-MCP, and read-or-mutate -- **declared per row rather than derived from the verb.** Deriving from a name is the sniffing we keep refusing, and this surface carries the standing proof: `st sync` and `sync` are different commands sharing a spelling, and `ac gate` reads while `wp done` consults the same gate and writes. Then **a refusal that every entry declares both**, so a new row cannot default silently into the tool surface.

**I did not land it.** Adding the fields is authoring and is mine; **classifying 103 rows is a judgement with a safety edge** -- which commands an agent may invoke unattended -- and that is yours and cc's. Say the word and I will author the fields and the refusal, and take a first pass at the classification for you to correct rather than originate.

**Raised now so WP-09 opens with a specification rather than a rediscovery** -- the same row-before-surface order that worked for `ac unsatisfy` and the seven lifecycle verbs.

### TWO CORRECTIONS TO THE EXPOSURE REGISTER WHILE I WAS IN IT

**EXP-01 predicted a defect and then it happened -- to me, that afternoon.** It said the view's zero-emphasis state was LUCK and that one canon note with single-asterisk emphasis would break the skew check on a file nobody touched. I wrote four. **An exposure register that correctly predicts a defect and does not prevent it has done half a job, and the cheaper half.** The formatter fixed-point refusal now closes the mechanism half without waiting on AC-07.6.

**EXP-02 needs your eye.** Its drift check exists and runs clean across 26 families -- but the two artefacts are **not equally checkable**, because the inventory's input is gone. **The drift check compares the authored table against an artefact nobody can currently re-derive.** That is the same finding as my 13:59Z entry, recorded where a reader of the table will meet it.

-- ic
