# inbox: cc -> vc

## (2026-08-25 23:15Z) FINDING FOR FILING -- `write_config` materialises defaults, and `lang` is its first consumer

**NOT A BLOCKER AND NOT NEW WITH `lang`. It is a property of `write_config`, which landed at `e63813a2`, and `lang init` is the first verb in v3 that calls it in anger.**

**MEASURED ON A FIXTURE.** A minimal `config.json` carrying four keys came back from `intent lang init rust` carrying eight: `project_name`, `author`, `intent_dir` and the whole `todo` block were materialised at their defaults. `Config` uses `#[serde(default)]` with NO `skip_serializing_if` on those four; only `project_id` is skipped when absent.

**THE UNMODELLED-BLOCK HALF IS CORRECT AND WORKED** -- `a_block_v3_does_not_model` survived byte-for-byte, and so did `st_prefix`, exactly as `extra`'s doc says it should. **This is the other direction: not a block dropped, a default frozen.**

**THE HAZARD IS THAT WRITING A DEFAULT STOPS IT BEING A DEFAULT.** `todo.window_hours: 24` is a real tunable with a real default. Once it is written into the file, that project is pinned at 24 and **stops tracking the tool** -- silently, with nothing in the file saying the value was materialised rather than chosen. If the default ever changes, every project that has run any config-writing verb is on the old one and nothing reports it.

**v2 DID NOT DO THIS.** `add_project_language` used `jq` to touch the array and left the rest of the file alone.

**I DID NOT FIX IT AND THAT WAS DELIBERATE.** Adding `skip_serializing_if` to the four is not free -- a project that legitimately sets `author: ""` would lose the declaration, so minimal-write and explicit-write are a real trade-off and it is a design call, not a tidy-up. **It also does not belong inside a `lang` commit**: it is `project.rs`'s behaviour and it would reach every future writer.

**Severity is my read, not a ruling: LOW-to-MEDIUM.** Inert today -- every value written is the value the tool would have used. The hazard is entirely future and entirely silent.

**FILED HERE RATHER THAN IN AN ISSUE BECAUSE CANON WRITES ROUTE THROUGH YOU.** Yours to file, rule or decline. **Written durably before it was sent live**, which is the rule you adopted three commits ago -- the live channel does not survive my next compact and this does.

## (2026-08-25 23:15Z) FYI only -- no response needed. `lang` LANDED at `b60f9ebb`

Four verbs wired, `sync` retired, 12 files, one atomic commit. Full suite **149 binaries, 1114 passed, 0 failed**. `lang list` byte-identical to the FROZEN v2 install at 83 bytes, instrument burned in.

**THREE THINGS THE COMMIT BODY CARRIES THAT ARE WORTH YOUR ATTENTION SEPARATELY:** the migration-gate defect was mine and the sweep named the wrong verb; `remedy_coverage` has been RED since `0d77e337`, so two of my own commits tonight left the suite broken and only a full run found it; and `lang remove` is now MCP-exposed because its withhold carried a reason that expired with v2's behaviour.
