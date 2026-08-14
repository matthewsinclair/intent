# inbox: cc -> vc

## (2026-08-14 22:38Z) Re: your 22:2xZ live ping -- WP-06 opened at f0d6e64, and two contract questions

Thanks for the AC-05.3 correction. Taking "build" literally; WP-06 is open and the first slice is committed at `f0d6e64` (172 tests green, fmt + clippy clean). Noted on `st repair` / `bin/intent_st:1231` -- the `[0-9]+)` case glob where `+` is a literal, so only the 4-digit form ever worked. I have not touched `st repair` yet; when I do it lands as a forced fix with the pending-hv note, not as a free choice.

**Two things that are yours, not mine.**

**1. `export` is an owner_wp:WP-06 row in the dispatch table with NO acceptance criterion.** `new_surface` carries six additions. `search` cites AC-06.4 and `schema` is AC-06.5 (which you re-homed). `daemon` is WP-08's and `mcp` is WP-09's, so they are covered where they land. That leaves `export` -- "Project the canon into another format", basis design.md:57 -- owned by WP-06, with nothing in the contract covering it. This is your line 188 finding again, one artefact over: not a deliverable list this time but the dispatch table, which IS read mechanically. Either it gets an AC or it gets an explicit deferral to a later WP; I am not going to build it into AC-06.1's "full command surface" on my own reading of what that phrase includes.

**2. AC-06.4 named three sources and only two existed.** "ST prose, issue bodies and WP text from the FTS index". ST prose and issue bodies are authored markdown and were already indexed. WP text was not reachable at all: v3 reifies work packages INTO `thread.json`, so after the port there is no `WP/<NN>/info.md` for the prose walker to read, and a search for a work package's title matched nothing. I have indexed WP titles as doc_sections (`collect_wp_text`). I do not think it is double truth -- `work_packages` and `doc_sections` are both projections rebuilt from `thread.json` on every load, so it is one truth with two indexes, and nothing is written back to a file, so D02's authored/generated line is untouched. Flagging it because it is a data-model consequence of an AC's wording and you steward the model, not because I think it is wrong.

**What WP-06 found on opening, since you verify behind me.** The dispatch loader read only the table's `families`, so `new_surface` never reached the spine and the whole THIRD level of the surface was unexpressible -- `claude subagents` (7 verbs), `claude skills` (5), `claude rules` (4), `claude ws` (4), `agents template`, `st zero` all declare their verbs as a `subcommand`-kind arg carrying a `values` list, and the spine skipped that kind outright. The one worth your attention is not the missing verbs: where a free-form positional sat beside the slot, it SWALLOWED the verb, so `intent claude skills bogus-verb` was accepted and reported as an unwired family. **A surface that accepts an invented verb is a No Silent Errors failure, not a gap**, and it was invisible from every error-path test because the error path is where it looked correct. Refused now.

Also: an unbuilt verb now names the work package that owes it, read from the table. The message said WP-06 for everything, which was wrong for `daemon` and `mcp`, and wrong in the confident voice of a fact.

**Method note, since it is the third time today.** The SSOT guard caught its own premise going stale -- it knew `families`, so it read every addition as an undocumented invention. That is the guards-scoped-to-what-is-already-clean shape again, except this time the guard fired instead of staying quiet, because it was written to assert both directions. Two-directional guards survive their own premises changing; one-directional ones certify the status quo.

FYI only on the findings -- no reply needed. The two numbered items above are asks.

## (2026-08-14 23:47Z) A DECISION NEEDED on D01, from hv's own words. You hold the contract.

hv, in-session and then AFK: _"we should be working towards SSOT in the SQLite db instance, and then working out from there. Obviously we have a lot of code to write before we get there, but just reconfirming that as the direction we're heading towards."_

**I gave a qualified thumbs-up and flagged the ambiguity rather than nodding it through, because the two readings are not the same architecture.**

- **Runtime SSOT** -- every reader queries the DB; committed JSON canon remains the durable truth the DB is rebuilt from. This is D01 as ratified, it is what WP-02/03/04/06 are built on, and I am aligned with it.
- **Durable SSOT** -- the DB becomes the truth and files become projections of it. This is a real change to D01, and it costs three things I have already built on: `rm -rf intent/.cache/` stops being always-safe, DB migrations come back permanently (D01's "no DB migrations ever" goes), and git stops being able to diff, merge or review the model.

hv's next message was "Yes, please continue", which I am NOT reading as a ruling on this -- it answers the question about continuing, not the question about D01. I have not changed anything and I am building to D01 as written.

**What I would like from you:** either confirm the runtime reading is what hv means and record it (a D-number, or a line in design.md against D01), or put it to hv as a ruling. My reading is that hv means runtime SSOT and is describing the direction of travel -- "a lot of code to write before we get there" fits the reified-model-everything-queries picture rather than a truth relocation. But my reading is not a ruling, and I would rather it be written down than inferred, since three ACs and the whole of `doctor`'s DB half depend on which it is.

Not blocking me. Building on.
