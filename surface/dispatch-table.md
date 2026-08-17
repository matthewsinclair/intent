# Command dispatch table -- Intent v3 (ST0056, AC-05.1)

> GENERATED VIEW -- the canon is `dispatch-table.json` beside this file. Regenerate with `parity/tools/gen_dispatch_table.sh`; do not hand-edit rows. Measured at `9ec1656` on 2026-08-14 by ic.

**Status:** All 27 v2 families authored + 8 new-surface entries. Targets marked pending-hv await the usage-convention scope ruling.

- THE command-surface source of truth for Intent v3 (AC-05.1). The clap surface, the help text, the MCP tool list and the `intent llm` agent guide all render FROM this file; nothing renders from `bin/**` and nothing describes the surface a second time.
- This is the AUTHORED artefact. `dispatch-table.md` beside it is a GENERATED view -- run `parity/tools/gen_dispatch_table.sh`, never hand-edit the view.
- `observed` records what v2 measurably does, at the revision stamped above. `target` records what v3 will do. They are separate fields because they genuinely differ, and a table that conflated them would launder a v2 defect into a v3 requirement -- which is the failure mode an output-equality parity suite cannot catch (parity.md, 'Parity properties beyond output equality').
- `target.state: pending-hv` is an honest blank, not an omission. hv ratified the `corrected` class at the 2026-08-14 bounce; WHICH v2 behaviours join it is a scope call still open. A guess recorded here would be indistinguishable from a ruling by the time WP-05 read it.
- Every claim carries an `evidence_class`, because 'verified' is not one thing. `measured` = a probe was run against v2 at the stamped revision. `documented-default` = a framework's published default, correct today and CHANGEABLE by a major bump or a single builder setting. `read` = taken from source without executing it. The distinction was forced by vc catching this file claiming clap's exit code was measured -- it cannot have been, since no clap dependency exists in the workspace yet. A documented default that goes unpinned is a finding with a silent expiry date.
- `disposition` uses one vocabulary shared with the keep/retire/deviate register: `keep · retire · deviate · pending` (vc ruling, 2026-08-14). `pending` is written explicitly and never expressed by omitting the field -- absence-as-meaning is un-greppable and reads as an oversight. The payoff is that AC-05.3 (every unit classified, no unclassified rows) becomes mechanical: no row carries `pending` at close.
- Entry-level `defects` reference an invariant by ID and add only the entry-specific locus (`where`). The rule text lives in exactly one place, the invariant. An entry that paraphrased it would be the divergent copy, in the artefact built to stop them.
- There is a FIFTH parity class, `undefined` (vc ruling, 2026-08-14, on `intent config` as its first member). `corrected` needs a v2 antecedent to correct; silence is not an antecedent. Where v2 exhibited NO behaviour at all, v3 is DESIGNING rather than porting or correcting, and that is a different decision needing a different reviewer. Folding it into `corrected` would hide a design decision inside a bug-fix class.
- Each family carries `bats_coverage`: how many test FILES exercise it through the dispatcher (`files_real`), how many name it but never reach the CLI (`files_vacuous`), and how many individual tests actually burn. Produced by `parity/tools/coverage_map.sh`, which joins these families against `burn-baseline.tsv`. The join is the point -- a naive grep reports `treeindex` as well covered when all 53 of its tests exec `bin/intent_treeindex` directly and the dispatcher never sees them. **A family with no burning coverage is a parity hole: v3 can change it freely and the conformance suite stays green.**
- `known_exposures` records defects this artefact does NOT currently have but is not protected against. A file that is clean by luck and a file that is clean by construction look identical in a diff, and only one of them stays clean.
- Every entry declares `exposed_on_mcp` and `read_or_mutate` (AC-09.1, from EXP-03). The MCP tool list renders from this file like everything else, and the alternative was deriving it from the verb -- which dies on one pair alone: `ac gate` READS while `wp done` consults the same gate and WRITES, and the two do not share a spelling. Declare, do not derive. Both fields are REQUIRED on every row and the generator refuses a row missing either, because a defaulted field is how a new command joins the agent surface without anyone deciding it should.
- `read_or_mutate` is a claim about the WHOLE entry, not about its default invocation. `read` means no invocation of the entry, under any flag, changes durable state -- not the store, not the working tree, not a config file. Everything else is `mutate`. Defined that way because the other reading ("what it does when you just run it") makes four rows lie: `at lint` is a report until `--fix`, `doctor` is a diagnosis until `--fix`, `llm usage_rules` prints until `--symlink`, and `todo list` prints unless `todo.md` is absent, in which case it generates it. That last one is the worst shape available -- it reads on every run after the first, so the mutation is invisible in testing and appears on a fresh clone. A field that describes the default is one an agent can be wrong about while reading it correctly.
- The two fields lean in OPPOSITE directions when the answer is uncertain, and both leans take the cheap error over the symmetric one. `exposed_on_mcp` leans FALSE: a command wrongly omitted is an inconvenience a human fixes in one line, while one wrongly included lets an agent run `daemon`. `read_or_mutate` leans MUTATE: a read mislabelled as a mutation costs a confirmation prompt, while a mutation mislabelled as a read lets an agent close a steel thread believing it is querying.
- `mcp_review` is present only on rows wanting a second opinion, and it exists because of how review actually fails: correcting a proposed classification is ANCHORED by the proposal (vc, 2026-08-15). Nobody re-derives a table this size row by row -- they review, and review is biased toward accepting. So the rows are MARKED rather than the confidence scored. `uncertain` names WHICH field is soft, because the two lean opposite ways and an unqualified "uncertain" is unactionable; `why_uncertain` gives the reason to argue with. `counterintuitive` flags a value that disagrees with the obvious reading of the verb -- precisely where a reviewer skimming nods it through, and where classifying from the name would have gone wrong. `grounded_in` cites the source actually read; its ABSENCE means the row was classified from the verb and the help text alone, which is a weaker claim and is meant to look like one.
- Every FLAG declares a `disposition` (AC-06.8, EXP-05), and it answers a DIFFERENT question from the entry-level field of the same name. At entry level `disposition` is a PARITY classification: what becomes of this v2 command. At flag level it is a SHIPPING classification: does this flag reach clap, and is the renderer obliged to read it. The words are shared on purpose, and they do not mean the same thing -- a new-surface flag is `keep`, because it ships, even though its command's entry disposition is `new-surface`. Four values: `keep` ships and must be read; `retire` is recorded from v2 and never reaches clap; `pending` does not ship, because an undecided flag on the surface IS the defect AC-06.8 names; `intrinsic` ships and clap supplies it, so no renderer arm is expected. `deviate` has no flag-level meaning and is deliberately absent -- a flag either reaches the surface or it does not, and a behaviour change is a property of the command. `intrinsic` correspondingly has no entry-level meaning.
- `intrinsic` is a PROPOSAL awaiting vc, and it is proposed because the code already needed it and expressed it the wrong way. `spine.rs:145-151` skips `--help` / `-h` / `help` by MATCHING ON THE SPELLING, with the comment that they are clap's own. That behaviour is correct and its reason is undeclared -- which is precisely the inference-from-name that EXP-05 exists to replace with a declaration. Under a three-value vocabulary these ten flags have no honest answer: `keep` says the renderer must read them and it must not, `retire` says they never reach clap and they do. Ten rows change if vc rules otherwise, which is cheap; a spelling-keyed skip list in the spine is not.
- The three EXP-05 refusals lean the same way `exposed_on_mcp` does, and the direction is chosen rather than inherited. `flags` must be PRESENT as an array, because `flagsig` renders an absent key and `[]` identically as `--`, so 'this command has no flags' and 'nobody has said' were the same glyph on four rows. Every flag must declare a disposition IN VOCABULARY, so a flag cannot join the v3 surface by being typed into this file. And a `retire` command cannot carry a shipping flag -- the one rule here that is derivable rather than authored, which is exactly why it is checked: an inherited value is the kind that gets hand-edited out of agreement with its source, and nothing else would notice. `pending` deliberately does NOT refuse; `doctor` reports the pending count instead, because a guard that must be bypassed is a guard nobody keeps.
- `v2` IS A PATH-TYPED FIELD CARRYING ONE UNDECLARED SENTINEL: the literal string `new-surface` on the 18 rows with no v2 antecedent, where every other row holds a source path (`bin/intent_config`). A consumer reading `v2` for the antecedent gets something SHAPED like a path that is not one, and nothing declares the exception. Named here by vc (2026-08-15) as the same family as `target.state` having been a bare `String` -- much lower stakes, because the sentinel is at least consistent across all 18 and agrees with `target.state` on every one of them. `target_states` closed the vocabulary for `state`; this one is recorded rather than mechanised, because a second closed vocabulary for a field with one exception would cost more than the exception does.
- `target_states` is THE vocabulary for `target.state`, and it is closed in both directions by `gen_dispatch_table.sh`: a row carrying an undeclared state is refused, and a declared state no row uses is refused. It exists because vc demonstrated the hole rather than arguing it -- `banana` as a state on `st start`, with a ratification naming parity.md, passed every check in the repo. The vocabulary had lived in three places that had never been compared, which is how `dispatch.rs`'s doc comment came to list five values while this file used six. `is_parity_class` is DECLARED per state and never derived from 'asserts a deviation': `retire` asserts the largest deviation there is and belongs in no class, because all six of its rows carry their own ratification.

## Provenance

- **Source reads and live probes at:** `9ec1656`
- **Runtime probe matrix at:** `69d42a7`
- **Why two revisions:** A single stamp would have been wrong for half this file. The 108-probe runtime matrix (bare / --help / unknown flag / outside a project, four per command) was captured at 69d42a7. Four commits have touched bin/ since: 205c368 (project-root resolution), e685e90 (at lint / ac gate WP scope), 072d277, and 3563ff4 (devbin -- out of parity scope). Per-arm source reads and every live probe in this file were taken at the revision above.
- **Re-validated after those bin/ changes:**
  - INV-03, the outside-a-project gate, re-probed at the head revision after 205c368 changed project-root resolution -- st / wp / todo / llm all still answer `error: not in an Intent project directory` at exit 1. That was the matrix column most exposed to the change, so it is the one re-run rather than assumed.
  - `ac gate` and `at lint` scope-honouring is recorded from a read of the CURRENT bin/intent_acceptance, post-e685e90, not from the older matrix.
- **Known limit:** The matrix's bare / --help / unknown-flag columns for commands untouched by those four commits are carried forward unre-run. Stated rather than papered over: a full re-probe is one `gen_inventory.sh` run and is the right move before WP-05 leans on this file, not before vc reviews its shape.

## Surface-wide invariants

Rules that hold across the whole command surface. They are stated once here rather than repeated on every entry, and WP-05 must honour them at the framework layer -- several are things clap does differently by default, so inheriting the default silently breaks parity.

| id     | invariant                                                        | v3 target   |
| ------ | ---------------------------------------------------------------- | ----------- |
| INV-01 | Voice: lowercase `ok:` / `error:` prefixes, no banners           | as-observed |
| INV-02 | Usage errors exit 1, NOT clap's default 2                        | as-observed |
| INV-03 | The project-context gate                                         | corrected   |
| INV-04 | Exit codes observed in the shipped surface are 0, 1 and 2 only   | as-observed |
| INV-05 | `error ...; usage` -- the second call is unreachable, everywhere | pending-hv  |
| INV-06 | About a fifth of v2 failure paths write to the wrong stream      | corrected   |
| INV-07 | `--help` reports failure on 10 of 27 commands                    | corrected   |
| INV-08 | Three commands accept an unknown flag silently at exit 0         | corrected   |

### INV-01 -- Voice: lowercase `ok:` / `error:` prefixes, no banners

Every failure writes `error: <message>` to STDERR. Every success line that announces an outcome writes `ok: <message>` to STDOUT. No banners, no unicode decoration.

- **v2:** bin/intent_helpers:7-11 (`error()` -- the single authority; 25 sites were moved to the lowercase voice in v2.19.0)
- **Target:** `as-observed` -- ratified: D17 -- binary voice and exit codes carry over from v2

### INV-02 -- Usage errors exit 1, NOT clap's default 2

A missing required argument, an unknown option, or an unknown subcommand exits 1.

- **v2:** bin/intent_helpers:7-11 -- `error()` is `echo >&2; exit 1`, and it is the only failure exit in the shipped surface bar `intent critic`
- **Evidence class:** `measured (v2 half) + documented-default (clap half)` -- The v2 half is measured: probes on `st show`, `st bogusverb` and `wp list` all exit 1, and `error()` is read directly. The clap half is NOT measured and could not be -- `native/rust/crates/intent-cli/Cargo.toml` carries no clap dependency, so nothing in this workspace exits 2 yet. clap's documented default is 2; that is a framework default, which a major bump or a single `Command::` setting can change.
  - Pinned by: WP-05 must land a test asserting exit 1 on a missing required argument AND on an unknown flag, WRITTEN BEFORE the clap spine exists. Then a changed default reds one named invariant instead of a hundred BATS tests failing for a reason nobody traces back here. (vc, 2026-08-14, on catching this row overclaiming its evidence.)
- **Target:** `as-observed` -- ratified: D17
- **Implementation constraint:** clap exits 2 for both `ErrorKind::MissingRequiredArgument` and `ErrorKind::UnknownArgument` by default. D17 rules the v2 code carries over, so WP-05 MUST override clap's exit code rather than inherit it. This is surface-wide -- it affects nearly every command -- and it is recorded here precisely so it is a build-time constraint rather than something discovered in test triage. Exception: `intent critic` genuinely uses 2 (see INV-04).

### INV-03 -- The project-context gate

Commands that need a project refuse outside one with exactly `error: not in an Intent project directory`, exit 1.

- **v2:** bin/intent:206; measured uniform across 14 commands
- **Target:** `corrected` -- ratified: vc-ruled 2026-08-15 on ic's finding. hv ratification outstanding, NOT blocking, one word to reverse. The basis is RATIFIED rather than new scope: the `error:` + `remedy:` voice that INV-01 and AC-06.11 both push toward is what v3's message already speaks and v2's does not. -- behaviour: v2 emits `error: not in an Intent project directory` then `  '<cmd>' requires an Intent project`. v3 emits `error: no Intent project found at or above <dir> (looked for intent/.config/config.json)` then a `remedy:` line naming `intent init` or changing directory. MEASURED on both binaries, not read.
- **found by:** ic, 2026-08-15, and HOW is the useful part: it surfaced while BUILDING the probe for this very invariant -- one of the two `surface_check.sh` declares unchecked. **The skip was hiding a finding.** Its reason (`needs a probe run outside a project`) was true and was never a claim that nothing was wrong, but it reads like one, and it is the sentence anyone re-reading the file relies on. Revisit skips; do not trust their reasons.
- **why not revert:** The v3 message is BETTER by this estate's own standards -- it names the directory searched, says what it was looking for, and carries a remedy. **Reverting to v2's wording to satisfy a parity row would make the surface worse in order to make a table right.** vc verified the grep rather than taking it on report: nothing in `parity.md`, `acceptance.md` or `design.md` names either string or ratifies the change, so unlike INV-06/07/08 -- which moved to `corrected` with a chain -- this one had simply diverged.
- **ratified in:** vc, 2026-08-15, ruling applied at commit 8d6604c4
- **Exceptions:**
  - `claude` never reaches the gate -- plugin commands exec before the project check (bin/intent:188-191)
  - `agents` runs outside a project for the same plugin-bypass reason, despite not being in GLOBAL_COMMANDS
  - `upgrade` supplies its own message

### INV-04 -- Exit codes observed in the shipped surface are 0, 1 and 2 only

0 success; 1 every failure; 2 only from `intent critic` (findings-present) and from `intent claude hook`, which propagates the hook's own code by design.

- **v2:** bin/intent_critic:89,95; measured across 108 probes
- **Target:** `as-observed`

### INV-05 -- `error ...; usage` -- the second call is unreachable, everywhere

v2 sources read as though a missing-argument error prints the usage block. It never does.

- **v2:** `error()` ends in `exit 1`, so every `error "..."; usage` pair has a dead `usage`. Seven sites in bin/intent_st alone (311, 450, 540, 603, 1048, 1105, 1620).
- **Evidence:** `intent st` bare measured at exit 1, stdout 0B, stderr 40B -- no usage block reached stdout.
- **Target:** `pending-hv`
- **Open question for hv:** v3 renders help from this table, so printing usage on a usage error is nearly free. Do we (a) reproduce v2 -- terse `error:` line only, or (b) `corrected` -- error line plus the command's usage? (b) is what the dead code shows v2 INTENDED. Either way the exit code stays 1 per INV-02.

### INV-06 -- About a fifth of v2 failure paths write to the wrong stream

Across 108 probes, failing invocations split 45 stderr-only, 12 stdout-only, 2 both.

- **v2:** measured; larger than the three known `Error:`-on-stdout sites in the plugin bins already queued for hv
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class, which names this census verbatim: "the stderr/stdout misroute census (45 stderr-only / 12 stdout-only / 2 both on failing invocations -- larger than the three sites in cc's hv queue)". Same numbers, same parenthetical; parity/README.md:39 carries the identical figures. FOUND by ic and VERIFIED by vc against both files rather than taken on report; applied by ic 2026-08-15. Not a new ruling -- hv answered the question WHOLE, and it was already written down. -- behaviour: Errors go to stderr, ALWAYS -- the whole census is corrected rather than enumerated site by site. An error on stdout interleaves with captured command output, which is how a voice becomes data.
- **ratified elsewhere:** VERBATIM MATCH IN parity.md, NOT YET APPLIED HERE (ic, 2026-08-15). parity.md:13 lists the hv-ratified `Corrected` deviation class members and one of them reads: "the stderr/stdout misroute census (45 stderr-only / 12 stdout-only / 2 both on failing invocations -- larger than the three sites in cc's hv queue)". That is this invariant's rule and its v2 note, same numbers and same parenthetical. So the question above -- ratify the whole census or enumerate site by site -- was answered WHOLE by hv at the bounce on 2026-08-14. Recorded rather than applied: this is the SECOND instance of the same drift found in one afternoon (INV-07 was the first), and having found that I over-applied a rule twice earlier today, the pattern I trust is to state the match and let vc apply it. The state stays `pending-hv` until they do.
- **ratified in:** parity.md

### INV-07 -- `--help` reports failure on 10 of 27 commands

Two shapes: usage to STDOUT with exit 1 (`init`, `st`, `wp`, `todo`, `treeindex`, `fileindex`), and an error to STDERR (`ac`, `at`, `help`, `claude`). `intent at --help` is parsed as an unknown verb; `intent help --help` fails outright.

- **v2:** measured
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: `--help` always succeeds: exit 0, usage to stdout. v2 exits 1 on st, wp, ac, at, todo and fileindex (measured). v3 as built already does this -- `spine.rs:193` maps clap's DisplayHelp and DisplayVersion to EXIT_OK. The two v2 BATS assertions on `init --help` (tests/unit/intent_init_lang_flag.bats:23, tests/unit/global_commands.bats:74) are CORRECT as they stand and must NOT be flipped: they assert the incumbent, the estate runs against v2 by default and against v3 only under an INTENT_BIN override, and the conformance harness is built to expect red for ratified reasons. Flipping them would turn v2's own suite red and destroy the evidence of a deviation the contract already classifies.
- **build measured:** MEASURED AT HEAD (ic, 2026-08-15), and it settles the cost side of this question. v2: `--help` exits 1 on st, wp, ac, at, todo and fileindex -- run, not read. v3 AS BUILT: exits 0 on all six plus the bare `intent --help`, because `spine.rs:193` maps clap's `DisplayHelp | DisplayVersion` to `EXIT_OK`, with `spine.rs:27` noting the divergence in as many words. So the candidate `corrected` in the question above is not a proposal any more, it is a description of the binary, and the only open act is recording it. THE COST THIS QUESTION FLAGS IS REAL AND IT IS TWO ASSERTIONS: `tests/unit/intent_init_lang_flag.bats:23` and `tests/unit/global_commands.bats:74`, both `assert_failure`, both on `init --help`, both carrying a comment calling exit-1 the usage convention. Nothing else in the BATS estate pairs `--help` with a status. And the argument from inside this file is stronger than the count: INV-04 declares 0 success and 1 every failure, so a `--help` that succeeds and exits 1 contradicts an invariant this same surface asserts -- reproducing it faithfully means shipping a v3 that breaks its own stated contract on 10 of 27 commands.
- **ratified in:** parity.md

### INV-08 -- Three commands accept an unknown flag silently at exit 0

`intent info --zzz`, `intent config --zzz` (zero output) and `intent version --zzz` all succeed.

- **v2:** measured
- **Evidence class:** `measured (v2 half) + documented-default (clap half)` -- Same split as INV-02: the three v2 commands were probed; clap's rejection of unrecognised arguments is its documented default, not something measured in this workspace.
  - Pinned by: The same WP-05 unknown-flag test that pins INV-02 pins this.
- **Target:** `corrected` -- ratified: hv, 2026-08-14 bounce -- the `corrected` class; this member is forced rather than chosen -- behaviour: Unknown arguments are refused, exit 1 per INV-02.
- **Note:** This one cannot be reproduced by accident: clap rejects unrecognised arguments by default, so v3 diverges here on day one whether or not anyone decides to. Recorded as ratified because the ALTERNATIVE (faithfully reproducing silent acceptance) would take deliberate work to build.
- **ratified in:** parity.md

## Family: `st`

Manage steel threads for the project

- **v2 source:** `bin/intent_st`
- **v2 help file:** none
- **Owning work package:** WP-04
- **BATS coverage:** 267 burning test(s) across 22 file(s) -- **covered**

- `intent help st` falls through to the 'no help available' path (bin/intent_help:37) -- there is no `lib/help/st.help.md`. The usage() block at bin/intent_st:13-88 is the only authored help, and it is unreachable from `intent help`.
- The one-line help strings below are lifted verbatim from that usage() block where it has one, so v3's generated help stays recognisable to existing users. Where v2 has no line (`zero`), the help is newly authored and marked as such.

| command                             | args        | flags                                       | help                                                                                                                               | disposition |
| ----------------------------------- | ----------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| `st`                                | <command>   | help/--help/-h                              | Manage steel threads for the project                                                                                               | keep        |
| `st new`                            | <title>     | -s/--start                                  | Create a new steel thread                                                                                                          | keep        |
| `st start`                          | <id>        | --                                          | Mark a steel thread as in progress                                                                                                 | keep        |
| `st done`                           | <id>        | --                                          | Mark a steel thread as complete                                                                                                    | keep        |
| `st cancel`                         | <id>        | --reason <text>                             | Mark a steel thread as cancelled, with a reason                                                                                    | keep        |
| `st triage`                         | <id>        | --                                          | Move a triaged thread out of Triage into NotStarted                                                                                | new-surface |
| `st hold`                           | <id>        | --reason <text>                             | Put a thread on hold, with a reason                                                                                                | new-surface |
| `st resume`                         | <id>        | --                                          | Take a thread off hold and back into Wip                                                                                           | new-surface |
| `st reopen`                         | <id>        | --reason <text>                             | Reopen a completed thread back into Wip, with a reason                                                                             | new-surface |
| `st reinstate`                      | <id>        | --reason <text>                             | Reinstate a cancelled thread into NotStarted, with a reason                                                                        | new-surface |
| `st list`                           | --          | --status <status>, --width <n>, --markdown  | List steel threads (default: in progress only)                                                                                     | keep        |
| `st show`                           | <id> [file] | --                                          | Show details of a specific steel thread                                                                                            | keep        |
| `st edit`                           | <id> [file] | --                                          | Print the absolute path to a steel thread file                                                                                     | keep        |
| `st sync`                           | --          | --write, --width <n>                        | Synchronize steel_threads.md with individual ST files                                                                              | keep        |
| `st repair`                         | [id]        | --write                                     | Repair malformed steel thread metadata                                                                                             | keep        |
| `st organize` (alias `st organise`) | --          | --write                                     | Organize ST files in directories by status                                                                                         | retire      |
| `st bootstrap`                      | --          | --audit-only, --dry-run, --deliverable <id> | Retrofit ST0000 deliverables into a brownfield project -- audit what is present, missing or partial, then install the missing ones | keep        |

### `st`

Manage steel threads for the project

- **v2:** bin/intent_st:1614-1616, 1618-1621
- **Arguments:**
  - `command` (subcommand, arity `1`)
- **Flags:**
  - `help`, `--help`, `-h` (bool) -- Print the usage block
    - `help` is a bare word arm, not a flag; grouped here because the three spellings share one arm (bin/intent_st:1614)
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `1` -- bare -- `error: Steel thread command is required`
  - `1` -- --help / -h / help -- usage block printed, exit 1
  - `1` -- unknown verb -- `error: Unknown command: <verb>`
- **stdout:** usage block (2330B) on the --help path only
- **stderr:** `error: ...` on the bare and unknown-verb paths (40B / 41B)
- **Defects observed in v2:**
  - INV-05 at bare invocation (bin/intent_st:1620)
  - INV-07 at `st --help` / `-h` / `help`
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: Defers to INV-07, now `corrected`: `--help` exits 0 to stdout. The v3 binary already does this.
- **build measured:** BUILD HAS ALREADY DECIDED (ic measured, 2026-08-15). See INV-07: the v3 binary exits 0 on `--help` today, deliberately (`spine.rs:193` maps clap's DisplayHelp to EXIT_OK, and `spine.rs:27` names the divergence). This row asks whether to reproduce or correct, and the code has answered `corrected` without the contract recording it. Measurement, not a ruling.
- **ratified in:** parity.md
- **MCP:** not exposed -- read-only
- **kind:** family

### `st new`

Create a new steel thread

- **v2:** bin/intent_st:296-445
- **Arguments:**
  - `title` (string, arity `1`)
    - v2 collects ALL non-flag args into ARGS and uses only ARGS[0] (bin/intent_st:305, 314) -- surplus positionals are silently discarded. v3 should refuse them (INV-02); flagged, not assumed.
- **Flags:**
  - `-s`, `--start` (bool) -- Mark the new thread in progress immediately
    - **disposition:** keep
- **Exit codes:**
  - `0` -- created
  - `1` -- no title -- `error: Steel thread title is required`
  - `1` -- unknown option -- `error: Unknown option: <opt>`
  - `1` -- templates missing -- names INTENT_HOME and states that nothing was created (bin/intent_st:375)
- **stdout:** the created id and path
- **stderr:** `error: ...`
- **Side effects:**
  - Writes `intent/st/<ID>/` from `lib/templates/prj/st/ST####`
  - Triggers the index resync (`intent st sync --write`, bin/intent_st:287)
- **Defects observed in v2:**
  - surplus positional args discarded silently (bin/intent_st:305, 314)
- **Target:** `corrected` -- ratified: hv, 2026-08-15 -- Machine 1 (ThreadStatus) enters at `Triage`, not `NotStarted`. cc landed it at `2aec5f6`.
- **entry state changed:** THE ENTRY STATE MOVED AND THAT IS USER-VISIBLE, so this row stops being `as-observed`. v2 `st new` creates a thread at `Not Started`; v3 creates it at `Triage`, and `st triage` is the verb that advances it. A user who runs `st new` then `st start` now meets a refusal that names `not-started` -- correct, and correct FOREVER, but it is a behaviour change and recording it as ported would have hidden one.
- **not a rename:** `Triage` REUSES THE LETTERS OF v2's `TBC` AND NOT ITS MEANING, which is why a parity row matching on the string would be comparing two different things. v2's `TBC` means To Be Commenced -- `bin/intent_helpers:544` maps `tbc` and `to be commenced` to the same stored value `Not Started`, and the tool's own usage at `bin/intent_st:46` says so in words. So every v2 `TBC` migrates to `NotStarted` and `Triage` begins with ZERO legacy members. The display string is `Triage`; `TBC` must not be reused as its abbreviation (see `st list`'s `tbc_trap`).
- **start flag ruled:** RULED by vc, 2026-08-15: **the flag STAYS and performs BOTH declared transitions**, `Triage -> NotStarted -> Wip`. I had flagged it as two edges at once; the measurement reframes it. `-s|--start` is **v2 PARITY, not new surface** (`bin/intent_st:302,381,425`, and in v2's own help as `new [-s|--start] <title>`), and **nothing about the flag changed -- the machine grew a state underneath it.** In v2 `st new` landed at not-started so `-s` was ONE transition; in v3 it enters at `Triage` so the same flag spans two. The triage decision is not skipped: **a user typing `--start` has decided the thread is real work, which IS the triage decision, made explicitly by the same act**, and refusing would ask them to state a conclusion they have already stated.
- **composition constraint:** **`st new -s` must COMPOSE `st triage` and `st start`, never construct the thread directly in `Wip`.** Constructing the end state is the obvious implementation and yields two defects at once: a history with no triage event, and an effective `Triage -> Wip` edge **that is not in the ratified machine** -- which either forces AC-04.6 to accept an undeclared edge or drives construction around `transitions.rs`, contradicting D32. The general rule, now in `data-model.md`: **a convenience flag is sugar over declared transitions and never a new edge.** A bundle that cannot be expressed as a sequence of declared transitions is proposing a machine change and goes to hv as one.
- **ratified in:** hv 2026-08-15
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st start`

Mark a steel thread as in progress

- **v2:** bin/intent_st:599-716
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- started
  - `0` -- ALREADY in progress -- prints `skipped: <ID> already in progress` and exits 0 (bin/intent_st:634-637, 678-681)
  - `1` -- no id / thread not found
- **stdout:** `skipped: ...` on the idempotent path
- **stderr:** `error: ...`
- **Observed notes:** `skipped:` is a THIRD voice prefix alongside `ok:` and `error:`; INV-01 does not currently name it. Carried into v3 or folded into `ok:` is a decision, flagged.
- **Target:** `corrected` -- ratified: vc 2026-08-17, after hv ratified Machine 1; this row needed no separate hv call and the reasoning is vc's: Machine 1 already declares `NotStarted -> Wip` and hv ratified the machine, so `st start` is `corrected` by APPLICATION of an existing ratification rather than a new one. It was `pending-hv`, which was honest and is now answered. THE DEVIATION IS THE FROM-STATE. Machine 1 admits `st.start` from `not-started` ONLY, so v3 REFUSES starting a `hold`, `completed`, `cancelled` or already-`wip` thread -- `st resume` and `st reopen` are the edges for the first two -- where v2 had no lifecycle state guard anywhere (vc's issue 0046). **AND IT REFUSES FROM `triage`, WHICH IS THE ENTRY STATE, so a freshly created thread CANNOT BE STARTED DIRECTLY: `st new` lands on `triage` and `st triage` is the edge to `not-started`.** Verified independently at HEAD by both vc and ic -- `initial: &["triage"]` and `Edge::direct("st.start", &["not-started"], "wip")`, both committed. **This is the sharpest user-visible consequence of the four and the one most likely to be reported as a bug on somebody's first v3 command**, which is why it is on the row rather than left to the machine. It is a deliberate consequence of the ratified machine, not an oversight. **THE SELF-LOOP IS SHIPPED (`61069b16`, cc) AND IT DOES NOT RESOLVE THE `triage` PROBLEM ABOVE -- the two are easy to confuse and this row is where they meet.** `st start` on an already-`wip` thread now returns exit 0 as a NO-OP, no envelope and no write, via the `from == status` test placed FIRST in `set_thread_status` (`facade.rs:1155-1157`) -- ahead of `check_transition`, ahead of `check_gate`, ahead of `check_reason`. **What decides a self-loop is whether the current state equals the verb's TARGET, not whether the verb is declared from that state.** So `wip` leaves the refused set. **`triage` DOES NOT, for exactly that reason: `st.start` targets `wip`, so asking it of a `triage` thread is a real transition and an undeclared one.** A freshly created thread still cannot be started directly, that remains the sharpest user-visible consequence on this row, and the self-loop landing is the most likely thing to be mistaken for having fixed it. `hold`, `completed` and `cancelled` also stay refused. Read at HEAD by ic after cc announced the commit; the paragraph this replaces said the ruling was unshipped, which was true when it was written and is why it was written that way.
- **Open question for hv:** Does `skipped:` survive as a first-class prefix, or become `ok: <ID> already in progress`? Scripts matching on it exist in the BATS estate.
- **ratified in:** vc ruling 2026-08-17, recorded on issue 0046
- **no op:** `ok: <ID> already WIP`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st done`

Mark a steel thread as complete

- **v2:** bin/intent_st:446-535
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- closed
  - `1` -- no id / thread not found
  - `1` -- acceptance contract BLOCKED -- `error: cannot close <ID>: acceptance contract is BLOCKED (run 'intent ac status <ID>')` (bin/intent_st:470-471)
- **stdout:** closure confirmation
- **stderr:** `error: ...`
- **Side effects:**
  - Consults the close-gate by shelling to `bin/intent_acceptance ac gate <ID>` (ST0044/ST0048 fail-by-default)
  - Relocates the thread directory and resyncs the index
- **Target:** `corrected` -- ratified: vc 2026-08-17 09:39Z under the standing grant; precedent `st cancel`, same mechanism and same author. THE DEVIATION IS THE FROM-STATE AND ONLY THE FROM-STATE. Machine 1 admits `st.done` from `wip` ONLY, so v3 REFUSES closing a `triage`, `not-started`, `hold`, `completed` or `cancelled` thread, where v2 had NO lifecycle state guard anywhere and moved it (vc's issue 0046, twelve undeclared movements). AC-04.6 REQUIRES that deviation, so the previous `as-observed` was asserting parity across one that is mandated. **The `GatePass` guard is PARITY, not part of the correction**: v2 already exited 1 on a BLOCKED acceptance contract (`bin/intent_st:470-471`, in this row's own measured exits), so v3 reproduces the gate and restricts the from-state, and only the second is a deviation. **THE SELF-LOOP IS SHIPPED (`61069b16`, cc) AND THIS ROW NOW DESCRIBES IT.** `st done` on an already-`completed` thread returns exit 0 as a NO-OP -- no envelope, no write -- via the `from == status` test placed FIRST in `set_thread_status` (`facade.rs:1155-1157`), ahead of `check_transition`, `check_gate` and `check_reason`. **What decides a self-loop is whether the current state equals the verb's TARGET, not whether the verb is declared from that state**, so `completed` leaves the refused set while `triage`, `not-started`, `hold` and `cancelled` stay in it. Read at HEAD by ic after cc announced the commit; the paragraph this replaces said the ruling was unshipped, which was true when written. **AND THE PARITY CLAIM TRAVELLING WITH THE RULING IS FALSE OF THIS ROW, WHICH IS ONLY VISIBLE FROM HERE.** `facade.rs:383` and `render.rs:1132` both say the ruling brings v3 back to v2's measured `already CLOSED`. **v2 has exactly one `already` arm and it is `intent issues close` (`bin/intent_issues:282`, `ok: issue $id already $status`)** -- `data-model.md:373` attributes it correctly and the two code comments dropped the subject in transit. **v2's `st done` on a completed thread has no state check at all** (`bin/intent_st:446-535`, read): it resolves the thread wherever it sits, RE-RUNS THE GATE, re-`sed`s the status, re-stamps `completed:` from a fresh `date -u`, finds `CURRENT_DIR == NEW_DIR` so moves nothing, and exits 0. **So v2 matches v3's exit code by accident and differs on the two things that matter: v2 re-runs the gate, so it exits 1 once the contract has gone BLOCKED since the close, and v2 silently advances the completion date on every re-close.** v3 does neither. That is a sharper correction than the ruling's own prose claims, and the right one. **WHAT THIS ROW STILL CANNOT STATE IS WHAT THE NO-OP PRINTS.** At HEAD `st done` discards the facade `Outcome` and prints `ok: <ID> done` either way, while `todo done` -- which delegates to this very call -- prints `ok: <spec> was already done`. Nineteen render arms against two. Filed as issue 0050; the table has no notation for a per-row no-op message, so that half of the gap is mine and not the renderer's. Measured by cc against the v3 facade and by vc against v2 at 18 cells, fresh project per cell; edges re-read at HEAD by ic.
- **Note:** The gate becomes an in-process facade call in WP-04 (AC-04.3), not a subprocess. Behaviour and message are parity-bound; the mechanism is not.
- **ratified in:** vc ruling 2026-08-17, recorded on issue 0046
- **no op:** `ok: <ID> already Completed`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st cancel`

Mark a steel thread as cancelled, with a reason

- **v2:** bin/intent_st:536-598
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Flags:**
  - `--reason` `<text>` (string) -- Why it is being cancelled -- required by the machine's guard
    - **disposition:** keep
- **Exit codes:**
  - `0` -- cancelled
  - `1` -- no id / thread not found
- **stdout:** cancellation confirmation
- **stderr:** `error: ...`
- **Observed notes:** No close-gate consultation -- cancel is not a close. Deliberate (feedback: use the existing Cancelled status, never invent a new one).
- **Target:** `corrected` -- ratified: hv, 2026-08-15 -- Machine 1 guards every edge into `Cancelled` with `reason recorded`; cc wired the facade at `2aec5f6` and left the flag for this table to declare.
- **conflict resolved:** RESOLVED 2026-08-15 -- the guard wins and this row is CORRECTED. I raised it as a conflict the machine and this row could not both survive: `data-model.md` guards every edge into `Cancelled` with `reason recorded`, and v2 `st cancel` took no `--reason` and recorded none (measured, its flags array was empty). cc has wired the CLI to read the flag OPTIONALLY at `2aec5f6`, so the facade refuses with `ReasonRequired` naming what is missing until the row declares it -- and declaring it here is what makes the flag start working. **The refusal is the reason this was safe to leave open**: an unimplemented guard that FAILS LOUD costs a clear error message, where one that silently accepted a cancellation with no reason would have put unexplained Cancelled threads in the record permanently. cc deliberately did not add the flag themselves; the table is mine.
- **why not as observed:** This row is no longer faithful to v2 and should not pretend to be: v2 cancels with no reason at all. The change is a `corrected` one -- v2's behaviour is the defect (a state entered with no record of why) rather than a contract to preserve.
- **ratified in:** hv 2026-08-15
- **no op:** `ok: <ID> already Cancelled`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **Self-loops at 0 without `--reason`**, same ordering as `st hold`.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st triage`

Move a triaged thread out of Triage into NotStarted

- **v2:** new-surface
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 1 (ThreadStatus) in data-model.md: the `Triage -> NotStarted` edge. `st new` now enters at `Triage`, so without this verb every new thread is stranded in its entry state and the machine's entry point is a trap.
- **Note:** `Triage` is a NEW state, not a rename of a state that had members. v2's `TBC` token means To Be Commenced and maps to `NotStarted` (bin/intent_helpers:544 maps `tbc` and `to be commenced` to the same value), so `Triage` begins with ZERO legacy members and this verb has no v2 caller to be compatible with.
- **no op:** `ok: <ID> already Not Started`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **`st.triage` is declared only FROM `triage`, so this row's self-loop is the only way to reach the verb at all** -- no other state has an edge into it, which the refusal from `wip` says in as many words. Sharing the `Not Started` display with `st reinstate` is a display fact, not a state fact: the two are distinct machine states that render alike.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st hold`

Put a thread on hold, with a reason

- **v2:** new-surface
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Flags:**
  - `--reason` `<text>` (string) -- Why it is being held -- required by the machine's guard
    - **disposition:** keep
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 1: the `NotStarted -> Hold` and `Wip -> Hold` edges, guard `reason recorded`.
- **Note:** `Hold` ALREADY EXISTS as a state and is reachable only by HAND-EDITING a file (cc's archaeology, confirmed): the v2 status filter recognises `hold|on hold -> HOLD` and no verb sets it. So this is not a new state, it is the missing door to a state v2 already renders, which is why the `--status` normaliser could always name a status the tool could not produce.
- **no op:** `ok: <ID> already On Hold`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **It self-loops at 0 WITHOUT the `--reason` the verb otherwise requires**, which is the ruled ordering visible from the outside: the self-loop test runs ahead of `check_reason`, so asking for a state the thread already holds is not a decision and needs no justification. `st cancel` measures the same way.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st resume`

Take a thread off hold and back into Wip

- **v2:** new-surface
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 1: the `Hold -> Wip` edge, no guard.
- **Note:** Returns to `Wip`, NOT to whichever state the thread was held from. The machine declares one exit edge and this verb implements exactly that one; restoring a remembered prior state would be an undeclared edge and would need the machine changed first.
- **no op:** `ok: <ID> already WIP`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st reopen`

Reopen a completed thread back into Wip, with a reason

- **v2:** new-surface
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Flags:**
  - `--reason` `<text>` (string) -- Why it is being reopened -- required by the machine's guard
    - **disposition:** keep
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 1: the `Completed -> Wip` edge, guard `reason recorded`. D32 forbids terminal states, and `Completed` was one.
- **Note:** `st done` RELOCATES the thread directory (measured on the `st done` row above), so this verb has a file-system half that `wp reopen` does not: reopening has to move the directory back. Flagged for cc because the state change is the easy half and the relocation is where a half-applied reopen would leave a thread findable under neither status.
- **no op:** `ok: <ID> already WIP`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st reinstate`

Reinstate a cancelled thread into NotStarted, with a reason

- **v2:** new-surface
- **Arguments:**
  - `id` (st-id, arity `1`)
- **Flags:**
  - `--reason` `<text>` (string) -- Why it is being reinstated -- required by the machine's guard
    - **disposition:** keep
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 1: the `Cancelled -> NotStarted` edge, guard `reason recorded`.
- **Note:** Lands in `NotStarted`, not in whatever the thread was before it was cancelled, and not in `Triage` -- a reinstated thread has already been triaged once and sending it back to the entry state would ask that decision to be made twice. The verb is spelled `reinstate` to match `ac reinstate`, which already carries this exact meaning at criterion level (undo a withdrawal); one word, one meaning, across both machines.
- **no op:** `ok: <ID> already Not Started`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). Shares the `Not Started` display with `st triage` and is a different machine state underneath.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `st list`

List steel threads (default: in progress only)

- **v2:** bin/intent_st:717-1043
- **Flags:**
  - `--status` `<status>` (string) -- `all`, or a comma-separated list rendered in the order given
    - Accepts: wip|in progress -> WIP; tbc|not started -> TBC; completed|done -> COMPLETED; cancelled|canceled -> CANCELLED; hold|on hold -> HOLD (case-insensitive, normalised to caps)
    - **disposition:** keep
  - `--width` `<n>` (integer) -- Render at a fixed column width; 0 or absent means terminal width
    - **disposition:** keep
  - `--markdown` (bool) -- Emit canonical GFM instead of terminal rendering
    - UNDOCUMENTED in v2's usage() block; consumed by `st sync --write`. Real surface, so it is in the table.
    - **disposition:** keep
- **Exit codes:**
  - `0` -- listed (including an empty list)
  - `1` -- unknown option -- `error: Unknown option: <opt>`
  - `1` -- `error: Steel threads directory not found`
- **stdout:** the table
- **stderr:** `error: ...`
- **Defects observed in v2:**
  - `--status` and `--width` consume the next positional blindly (shift; VALUE="$1"). A trailing `--status` with no value silently yields an empty filter rather than a usage error.
- **Target:** `as-observed`
- **Note:** clap makes the missing-value case an error for free -- that is a `corrected` consequence rather than a choice, same shape as INV-08. Byte-exact column padding is parity-bound: `tests/unit/output_width.bats:44-140` pins it.
- **status vocabulary:** THE RATIFIED SIX-STATE MACHINE REACHES THE SURFACE HERE, and this flag is the only place a user types a status name. v3 must accept the six states of Machine 1: `triage`, `not started`, `wip`, `hold`, `completed`, `cancelled`. `triage` is the addition -- v2 accepts five and has no spelling for it.
- **tbc trap:** MEASURED, and it strengthens the ratified migration rule with a second independent witness. In v2 `TBC` IS NOT A STATE AT ALL -- it is a DISPLAY ABBREVIATION of `Not Started`, narrow enough for the table column. Three sites, all read: `canonical_status()` at bin/intent_helpers:544 maps `tbc` AND `to be commenced` to the stored value `Not Started`; bin/intent_st:120 abbreviates `Not Started` to `TBC` for rendering; and the tool's OWN usage text at bin/intent_st:46 spells it out in words -- `tbc, not started -> TBC   To be commenced`. So the ratified rule (v2 `TBC` maps to `NotStarted`, never to `Triage`) is not merely defensible, it is what the tool has always documented about itself. THE SURFACE TRAP THAT FOLLOWS IS MINE: v3 must NOT abbreviate `Triage` as `TBC`, and must NOT accept `--status tbc` as `Triage`. Either would give a familiar token a second meaning in the render column and the filter -- the two places a v2 user reads fastest and checks least. `tbc` should keep resolving to `NotStarted`, exactly as it always has.
- **render order:** bin/intent_st:941 pins the display order as a five-element list -- `WIP TBC HOLD COMPLETED CANCELLED`. Six states means this list grows, and `Triage` belongs BEFORE the `Not Started` slot because it precedes it in the machine. Named here because it is a surface fact hiding in an array literal, and a new state that renders in the wrong place looks like a sorting bug rather than a missing decision.
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_st:717-1043 -- no write primitive in the arm

### `st show`

Show details of a specific steel thread

- **v2:** bin/intent_st:1044-1100
- **Arguments:**
  - `id` (st-id, arity `1`)
  - `file` (enum, arity `0..1`), default `info` -- one of: `info`, `design`, `impl`, `tasks`, `acceptance`, `all`
- **Exit codes:**
  - `0` -- printed
  - `1` -- no id / thread not found
  - `1` -- `error: Unknown file type: <t>`
  - `1` -- `error: File not found: <t>.md for steel thread <ID>`
- **stdout:** file contents; `all` concatenates with `-- <file>` separators
- **stderr:** `error: ...`
- **Target:** `as-observed`
- **Note:** `info.md` and `acceptance.md` become GENERATED VIEWS in v3 (D02/D04). `show` reads the view, so its output is unchanged in kind -- but the view's bytes are v3's to define, and every BATS test asserting v2's exact info.md bytes retires under the ratified file-layout class. **CORRECTED 2026-08-17: THE BINARY READS NO VIEW, AND THIS NOTE SAYING IT DOES IS A FOURTH INSTANCE OF THE NOTE-TRUE-WHILE-THE-BINARY-WAS-NOT PAIR (issue 0055, found by cc).** `st show` never reads its `file` positional at all: it reads `id` and prints a four-line synthesised summary, so `intent st show ST0001 design` and `intent st show ST0001 nonsense` produce identical output at exit 0, where v2 cats `design.md`. **It is a `keep` row reporting success for a request it discarded** -- worse in kind than 0052, whose unreachable field meant nothing happened, because this one says something did. Three of this row's four declared exit codes are unreachable. The fix is a WP-05 surface call and is deliberately NOT taken here: both readings are in 0055.
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_st:1044-1100 -- no write primitive in the arm

### `st edit`

Print the absolute path to a steel thread file

- **v2:** bin/intent_st:1101-1144
- **Arguments:**
  - `id` (st-id, arity `1`)
  - `file` (enum, arity `0..1`), default `info` -- one of: `info`, `design`, `impl`, `tasks`, `acceptance`
    - NO `all` -- unlike `show`. Asymmetry is deliberate and correct: there is no single path for `all`.
- **Exit codes:**
  - `0` -- path printed
  - `1` -- no id / thread dir not found / unknown file type
- **stdout:** the absolute path, one line, nothing else
- **stderr:** `error: ...`
- **Observed notes:** Pure emit-path: it never launches an editor and never creates the file. The name is a historical misnomer the docs already work around (`$EDITOR "$(intent st edit ST0001 acceptance)"`). The thread DIRECTORY must exist; the file need not.
- **Target:** `as-observed`
- **Note:** Under authored-once, `edit` on a GENERATED view (info/acceptance) hands the user a path to a file their edits will lose at the next regeneration. v3 emitting the path unchanged is defensible only if the skew check (AC-03.4) catches the edit. Flagged for the WP-05 surface cut, not decided here.
- **MCP:** exposed as an agent tool -- read-only
- **Wants review -- the classification disagrees with the verb name:** `edit` is the most obviously-mutating verb name in the table and the command writes nothing -- it is a path resolver, and the entry beside this one already said so in `observed.notes` ("never launches an editor and never creates the file", called a historical misnomer). I still had to read bin/intent_st:1125-1141 to stop classifying it as a mutation, which is the argument for declaring the field: the correct fact was ALREADY WRITTEN DOWN one bullet away and the verb name still won. It also inverts the exposure reading -- an $EDITOR launch could not be an MCP tool at all, since it would block on stdio, while a path resolver is one of the safest things here.
- **MCP classification grounded in:** bin/intent_st:1125-1141 -- 'Pure emit-path ... No touch, no editor'; it prints the absolute path and returns

### `st sync`

Synchronize steel_threads.md with individual ST files

- **v2:** bin/intent_st:1145-1211
- **Flags:**
  - `--write` (bool) -- Write the index; without it the command is a dry run
    - **disposition:** keep
  - `--width` `<n>` (integer) -- Render at a fixed column width
    - **disposition:** keep
- **Exit codes:**
  - `0` -- synced or dry-run reported
  - `1` -- unknown option / `error: Steel threads directory not found` / `error: Steel threads index file not found`
- **stdout:** the diff or the confirmation
- **stderr:** `error: ...`
- **Observed notes:** Called internally by every mutating verb via `"${BASH_SOURCE[0]}" sync --write > /dev/null` (bin/intent_st:287). stdout is suppressed there but stderr FLOWS deliberately -- `2>&1 >/dev/null` once hid every sync failure (issue 0019).
- **Target:** `as-observed`
- **Note:** In v3 `steel_threads.md` is a generated view and the resync is part of the transactional write path (AC-04.1), not a subprocess. The COMMAND survives for explicit regeneration; the implicit call has no v3 analogue because it cannot fall out of date.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `st repair`

Repair malformed steel thread metadata

- **v2:** bin/intent_st:1212-1433
- **Arguments:**
  - `id` (st-id, arity `0..1`)
    - Parsed inside the FLAG loop, not positionally -- accepted spellings are `ST0001` and `0001` ONLY
- **Flags:**
  - `--write` (bool) -- Apply the repairs; without it the command is a dry run
    - **disposition:** keep
- **Exit codes:**
  - `0` -- repaired or dry-run reported
  - `1` -- `error: Unknown option or invalid steel thread ID: <arg>`
  - `1` -- `error: Steel thread not found: <ID>`
- **stdout:** per-thread findings
- **stderr:** `error: ...`
- **Defects observed in v2:**
  - DEAD ARM at bin/intent_st:1231: `[0-9]+)` is a `case` GLOB, where `+` is a literal character -- it matches a single digit followed by a `+`, never a run of digits. So `intent st repair 5` and `intent st repair 12345` both fall through to the error arm. Verified by executing the case statement in isolation, not by reading it. The intended bare-number form is unreachable; only the 4-digit `0001` arm works.
- **Target:** `pending-hv`
- **Open question for hv:** Does v3's id parser accept a bare number of any length (what :1231 evidently INTENDED), or only the two spellings that measurably work? Reproducing the dead arm faithfully is not an option -- it is unconstructible in clap. Same class as INV-08: a defect whose fix is forced.
- **MCP:** not exposed -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Bulk metadata rewrite across threads. Leaning closed because the blast radius is every thread at once and a wrong repair is not obvious in the output.
- **recoverability:** one-way

### `st organize`

Organize ST files in directories by status

- **v2:** bin/intent_st:1434-1609; the `organise` -> `organize` normalisation is at bin/intent_st:289-292
- **Flags:**
  - `--write` (bool) -- Move the files; without it the command is a dry run
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
- **Exit codes:**
  - `0` -- swept
  - `1` -- `error: <n> steel thread(s) could not be moved (see above); the rest of the sweep completed`
- **stdout:** `Already organized: <ID> in intent/st/<STATUS>` per thread (73B for one thread in a fresh project)
- **stderr:** `error: ...`; a move collision also prints mv's raw stderr
- **Observed notes:** `organise` is an alias ONLY here, one level down. `intent organise` at top level is `error: Unknown command 'organise'` -- measured both.
- **Target:** `retire` -- ratified: hv, 2026-08-14 -- organize (both faces) is planned vestigial by construction; a strictly structured model cannot hold data in the wrong spot, so the disorder it repairs cannot arise. Confirmed finally at the surface cut (WP-05/06).
- **Note:** Retiring this face also dissolves the pre-existing Highlander violation: `bin/intent_organize` and `bin/intent_st organize` are two implementations of one concern, both registered in MODULES.md, and they print different things against the same input (117B vs 73B, no shared output).
- **MCP:** not exposed -- **mutates**

### `st bootstrap`

Retrofit ST0000 deliverables into a brownfield project -- audit what is present, missing or partial, then install the missing ones

- **v2:** bin/intent_st_zero, reached in v2 by TWO spellings: `intent st_zero` (top level, auto-dispatched) and `intent st zero` (bin/intent_st:1610-1612 execs the binary). Only the second was ever documented by the command itself.
- **Flags:**
  - `--audit-only` (bool) -- Show gap analysis only, no changes
    - **disposition:** keep
  - `--dry-run` (bool) -- Show what would change, no writes
    - **disposition:** keep
  - `--deliverable` `<id>` (string) -- Target a single deliverable (D2-D11)
    - **disposition:** keep
- **Exit codes:**
  - `0` -- bare -- prints `Usage: intent st zero install` and exits 0
- **stdout:** the usage line
- **stderr:** --
- **Defects observed in v2:**
  - UNDOCUMENTED: absent from bin/intent_st's usage() block entirely. It was missing from parity.md's command-level table until the deep pass measured it.
  - INV-07 at inverted -- bare invocation prints only usage and exits 0, where every other family exits 1
- **Target:** `corrected` -- ratified: hv, 2026-08-15 -- `st_zero` is wrong and the root spelling dies. `zero` was never a verb: it is the NAME of the thing (Steel Thread Zero / ST0000), which is why `intent st zero install` parses noun-then-verb and why the spelling reads as "initialise something to zero" -- not what the command does. It audits which ST0000 deliverables are present, missing or partial in a brownfield project and installs the missing ones. `bootstrap` names that operation and promotes the real verb to the right position. hv considered `initzero` and preferred `bootstrap`.
- **Note:** `install` COLLAPSED into the bare form, deliberately, as part of the same correction. It was the only value of the subcommand and the real verb hiding a level down; keeping it gives `intent st bootstrap install`, two stacked verbs, which rebuilds the exact defect this ruling removes. The audit path is already a flag (`--audit-only`), so nothing is lost. Landed rather than asked because delivering the ruled verb on top of the unruled noise word would deliver the problem in a new costume; one sentence reverses it.
- **consequence:** `intent bootstrap` (top level) already exists and means "first-time setup: create global Intent configuration". This is NOT a collision, it is the same verb meaning the same thing at two levels -- bootstrap the machine, bootstrap the steel-thread structure in a project. Checked before landing; it strengthens the choice rather than qualifying it.
- **face:** surviving
- **never built:** false
- **ratified in:** hv 2026-08-15
- **MCP:** not exposed -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - One-time project scaffolding. An agent that runs it in an already-bootstrapped project is doing something nobody asked for.
- **Cross-reference:** THE surviving face. The top-level `st_zero` family is the deleted root spelling; see its entry for the divergence cost.
- **recoverability:** one-way

## Family: `wp`

Manage work packages within steel threads

- **v2 source:** `bin/intent_wp`
- **v2 help file:** none
- **Owning work package:** WP-04
- **BATS coverage:** 79 burning test(s) across 8 file(s) -- **covered**

- Specifier syntax is shared across every verb and parsed by `parse_wp_specifier` (bin/intent_helpers, ST0050): `STID` accepts `ST0011` or the bare number `11`; `STID/NN` accepts `ST0011/01` or `11/01`. Unlike `st repair`, the bare-number form here actually works -- the resolver is a function, not a `case` glob (contrast the dead arm at bin/intent_st:1231).
- No help file; `intent help wp` falls through to the no-help path. The usage() block is the only authored help and is unreachable from `intent help`.

| command      | args               | flags           | help                                                    | disposition |
| ------------ | ------------------ | --------------- | ------------------------------------------------------- | ----------- |
| `wp`         | <command>          | help/--help/-h  | Manage work packages within steel threads               | keep        |
| `wp new`     | <stid> <title>     | --              | Create a new work package                               | keep        |
| `wp start`   | <specifier>        | --              | Mark a work package as WIP                              | keep        |
| `wp done`    | <specifier>        | --              | Mark a work package as Done                             | keep        |
| `wp reopen`  | <specifier>        | --reason <text> | Reopen a done work package back into Wip, with a reason | new-surface |
| `wp unstart` | <specifier>        | --              | Return a started work package to NotStarted             | new-surface |
| `wp rescope` | <specifier> <size> | --              | Change a work package's T-shirt size                    | new-surface |
| `wp list`    | <stid>             | --              | List work packages for a steel thread                   | keep        |
| `wp show`    | <specifier>        | --              | Show work package info.md                               | keep        |

### `wp`

Manage work packages within steel threads

- **v2:** bin/intent_wp:287 (`*)` arm) + the arity check above it
- **Arguments:**
  - `command` (subcommand, arity `1`)
- **Flags:**
  - `help`, `--help`, `-h` (bool) -- Print the usage block
    - bare-word arm plus the two flag spellings
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `1` -- bare -- `error: Work package command is required`
  - `1` -- `--help` prints usage to STDOUT and exits 1
  - `1` -- unknown verb
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** usage block on the --help path
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - INV-05 at bare invocation
  - INV-07 at `wp --help`
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: Defers to INV-07, now `corrected`: `--help` exits 0 to stdout. The v3 binary already does this.
- **build measured:** BUILD HAS ALREADY DECIDED (ic measured, 2026-08-15). See INV-07: the v3 binary exits 0 on `--help` today, deliberately (`spine.rs:193` maps clap's DisplayHelp to EXIT_OK, and `spine.rs:27` names the divergence). This row asks whether to reproduce or correct, and the code has answered `corrected` without the contract recording it. Measurement, not a ruling.
- **ratified in:** parity.md
- **MCP:** not exposed -- read-only

### `wp new`

Create a new work package

- **v2:** bin/intent_wp:83
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `title` (string, arity `1`)
- **Exit codes:**
  - `0` -- created
  - `1` -- missing STID or title
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the created WP id and path
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Writes `intent/st/<ID>/WP/<NN>/info.md` from template
  - Sets the work package's scope to `S` -- hardcoded, not enterable. v2's template seeds `scope: Small`, so the VALUE is parity; see `target.scope_ruling` for why it is a starting value rather than a verdict.
- **Target:** `as-observed`
- **scope ruling:** **hv RULED 2026-08-17: NO FLAG. `wp rescope` IS THE ENTRANCE AS WELL AS THE EXIT** -- issue 0052, closed at commit `8e5ef648`. Permanently, not provisionally. **The hardcoded `S` was never the defect and must not be 'fixed' to `absent`.** v2's template seeds `scope: Small`, so any other default is a parity break hiding in a value rather than in an output. What was indefensible was that nobody could override it: **a default nobody can override is a VERDICT; a default with an exit is a STARTING VALUE.** Same `S`, different meaning, no change to the code -- the exit is what changed its status (vc's framing, adopted). **And the self-loop distinction now has the user it was missing.** `wp_rescope` treating a same-size rescope as a movement only where a `scope_legacy` carry is present (`facade.rs:1443-1456`) is, with `rescope` as the sole entrance, the mechanism by which a migrated carry gets ADJUDICATED BY A HUMAN rather than having a size coerced onto it. That capability is what the unwired verb was denying. **Recorded here because it was ABSENT, not stale.** vc verified while closing 0052 that neither this row nor `wp rescope` carried an `open_question`, which is true and is a different question from the one that matters: a scan for a claim that has GONE WRONG is structurally blind to a claim that was never made. `wp rescope`'s openness was in prose inside its `note` and this row said nothing at all -- so the field-scoped scan found nothing on both, for two opposite reasons. That is the INV-06/INV-07 direction again (the decision was made and the register never moved), and it is why `corrected_check.sh` runs both directions. **No `ratified_in` on this row, deliberately: it is `as-observed` and ratifies no deviation.** `ratified_in` names where a row's DEVIATION was ratified, and putting a workflow ruling in it would dilute the one field two sibling checks read for a single meaning. The provenance (authority, date, commit) is inline above, which is where a reader of this ruling looks.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `wp start`

Mark a work package as WIP

- **v2:** bin/intent_wp:190
- **Arguments:**
  - `specifier` (st-id/NN, arity `1`)
- **Exit codes:**
  - `0` -- started
  - `1` -- missing or unresolvable specifier
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `corrected` -- ratified: vc 2026-08-17 09:39Z under the standing grant; one ruling covering `st start`, `st done`, `wp start` and `wp done`, all deviating in the SAME single respect -- the from-state. Machine 2 admits `wp.start` from `not-started` ONLY, a DIRECT edge with no guard, so v3 REFUSES restarting a `wip` package and refuses restarting a `done` one; v2 accepted both, having no lifecycle state guard anywhere (vc's issue 0046). This is the cleanest of the four because there is no guard on the edge to confuse with the restriction. **THE SELF-LOOP IS SHIPPED (`61069b16`, cc), AND IT MAKES THIS ROW PARITY ON THE SELF-LOOP AND A CORRECTION ONLY ON `done`.** `wp start` on an already-`wip` package returns exit 0 as a NO-OP via the `from == status` test placed FIRST in `set_wp_status` (`facade.rs:1512-1514`), ahead of the transition check. **What decides a self-loop is whether the current state equals the verb's TARGET, not whether the verb is declared from that state**, so `wip` leaves the refused set. v2 reaches the same exit code by a different route, with no guard on either side: its `wp start` has no state check either (`bin/intent_wp:190-215`, read), so it re-`sed`s `status: WIP` over `WIP` and prints `started:` at 0. **The deviation that survives is a `done` package, which v3 refuses and v2 SILENTLY REOPENS by `sed` -- issue 0046, and `wp reopen` is the edge that does it with a recorded reason.** Read at HEAD by ic after cc announced the commit; the paragraph this replaces said the ruling was unshipped, which was true when written. AC-04.6 REQUIRES the deviation.
- **ratified in:** vc ruling 2026-08-17, recorded on issue 0046
- **no op:** `ok: <ID>/NN already WIP`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `wp done`

Mark a work package as Done

- **v2:** bin/intent_wp:135
- **Arguments:**
  - `specifier` (st-id/NN, arity `1`)
- **Exit codes:**
  - `0` -- closed
  - `1` -- missing or unresolvable specifier
  - `1` -- acceptance contract BLOCKED for the WP group (close-gate, ST0044/ST0048)
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Consults the close-gate; warns on an unedited `## Objective` placeholder (`warn_unedited_objective`, issue 0010)
- **Target:** `corrected` -- ratified: vc 2026-08-17 09:39Z under the standing grant; one ruling covering `st start`, `st done`, `wp start` and `wp done`. THE DEVIATION IS THE FROM-STATE AND ONLY THE FROM-STATE. Machine 2 admits `wp.done` from `wip` ONLY, so v3 REFUSES closing a `not-started` or already-`done` package where v2 had no lifecycle state guard and moved it. **The `GatePass` guard is PARITY and this row said so before ic touched it** -- the machine note below records that Machine 2's BLOCKED-gate refusal was ALREADY the measured v2 behaviour, and the measured exits carry `1 <- acceptance contract BLOCKED for the WP group`. That note caught ic over-claiming the gate as part of the correction, so it has earned its keep twice. **THE SELF-LOOP IS SHIPPED (`61069b16`, cc) AND THE CONSEQUENCE THIS ROW PREDICTED IS NOW THE MEASURED BEHAVIOUR: `wp done` on an already-`done` package returns 0 EVEN WHERE THE GATE WOULD BLOCK**, because the short-circuit (`facade.rs:1512-1514`) precedes the gate. That is the POINT of the ruling and not a side effect -- a criterion added after a close must not block a unit that is legitimately finished, which happened in this very thread with AC-04.6. **AND IT HOLDS BY CONSTRUCTION RATHER THAN BY ORDERING, WHICH IS THE HALF THAT WAS NOT IN THE RULING.** `check_gate` now enforces `GatePass` from the `transitions.rs` declaration (`facade.rs:1213-1232`) instead of by hand in `st_done` and `wp_done`, so the gate sits after the `from == status` test structurally rather than because someone remembered the order. **The hand-written enforcement was a live Highlander defect in the file that implements AC-04.6**: deleting `GatePass` from the table changed nothing, which is exactly the declaration-versus-implementation split AC-04.6 exists to find. cc found it by getting this ordering right. **v2 IS THE OPPOSITE, AND THAT IS THE DEVIATION.** v2's `wp done` has no state check (`bin/intent_wp:135-185`, read), so a re-close RE-RUNS the gate and exits 1 when the contract has since gone BLOCKED, then re-`sed`s `status: Done` and reprints the full `done:` block. **Do not read the ruling's `already CLOSED` citation as v2 parity here** -- that string is `intent issues close`'s alone (`bin/intent_issues:282`); the `st done` row unpicks the claim once, and it is not repeated here. What the no-op PRINTS is issue 0050: this arm discards the `Outcome` and says `ok: <spec> done` regardless. Read beside `wp reopen`, which is `new-surface`: v2 shipped nothing that undid `wp done`.
- **Note:** The gate becomes an in-process facade call at WP-04 (AC-04.3). Behaviour and message are parity-bound; the mechanism is not.
- **machine note:** hv, 2026-08-15 -- Machine 2 ratifies `wp done` REFUSED on a BLOCKED gate, and the measured v2 behaviour above ALREADY does that (exit 1 when the WP group's contract is BLOCKED). So the ratification adds no surface change here; the change is `wp reopen` below. Recorded because `as-observed` staying correct after a ratification is a fact worth stating -- the alternative is a later reader assuming this row was never re-checked.
- **ratified in:** vc ruling 2026-08-17, recorded on issue 0046
- **no op:** `ok: ST0001/01 already Done`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **Measured against a PASSING contract only.** `facade.rs:1233-1234` states that the gate now sits after the self-loop test by construction, so an already-Done package whose contract has since gone BLOCKED should self-loop rather than refuse -- that is the code's claim, cited and NOT measured here, because reaching the state needs a contract that passes and then stops passing.
- **MCP:** exposed as an agent tool -- **mutates**
- **MCP note:** Pairs with `ac gate` below and is the reason the field is DECLARED, not derived: `wp done` consults the same gate `ac gate` runs, and then WRITES. The two do not share a spelling, so no naming rule separates them.
- **recoverability:** reversible

### `wp reopen`

Reopen a done work package back into Wip, with a reason

- **v2:** new-surface
- **Arguments:**
  - `specifier` (st-id/NN, arity `1`)
- **Flags:**
  - `--reason` `<text>` (string) -- Why it is being reopened -- required by the machine's guard
    - **disposition:** keep
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 2 (WpStatus): the `Done -> Wip` edge, guard `reason recorded`.
- **Note:** THE URGENT ONE, and it is not urgent in the abstract: its absence is CURRENTLY CORRUPTING this thread's own tracking data. Three of five WPs disagree with their own gate (WP-02 Done/BLOCKED, WP-04 Done/BLOCKED, WP-05 Wip/PASS), because adding an AC reopens a WP in the contract while nothing moves the status back. Until this verb exists the ONLY repair is hand-editing the file the CLI exists to own -- which is the same trap `ac satisfy` had before `ac unsatisfy`, in the same tool, found the same way. Second instance of one class; the guard against a third is Machine 2 itself, which now declares the edge whether or not anyone has built it.
- **no op:** `ok: <ID>/NN already WIP`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `wp unstart`

Return a started work package to NotStarted

- **v2:** new-surface
- **Arguments:**
  - `specifier` (st-id/NN, arity `1`)
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 2: the `Wip -> NotStarted` edge, no guard.
- **Note:** No `--reason`: the machine declares no guard on this edge, and adding one would be a stricter surface than the ratified machine rather than a safer one. Unstarting is the cheap correction of a mis-click; reopening a closed WP is a claim about finished work, which is why only the second one has to be justified.
- **no op:** `ok: <ID>/NN already Not Started`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `wp rescope`

Change a work package's T-shirt size

- **v2:** new-surface
- **Arguments:**
  - `specifier` (st-id/NN, arity `1`)
  - `size` (enum, arity `1`) -- one of: `XS`, `S`, `M`, `L`, `XL`, `XXL`
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- Machine 2's second field (`WorkPackage.scope`): six `wp.rescope` edges, each with an EMPTY from-set, so the verb leaves any state including `absent`. Ratified as a machine before any surface existed for it.
- **Note:** **THE VERB WAS BUILT AND NEVER WIRED, AND THE FIELD IS UNREACHABLE IN BOTH DIRECTIONS -- issue 0052.** `Facade::wp_rescope` (`facade.rs:1428`) is `pub` and implemented; its only callers in the workspace are two tests. `intent wp --help` listed seven subcommands and `rescope` was not among them, and `intent wp new --help` lists NO options, so the scope could not be supplied at creation either. Every work package v3 creates sits at `absent` permanently, and a migrated `scope_legacy` -- carried deliberately so a human can adjudicate it later rather than having a size coerced onto it -- can never be adjudicated. **This row exists so the verb can be wired: the spine builds the clap surface FROM this table, so there is nothing for `m.subcommand()` to match until the row is here** (cc, 2026-08-17). Shape is cc's; the `no_op` is ic's. **hv RULED 2026-08-17: NO FLAG. `wp rescope` IS THE ENTRANCE AS WELL AS THE EXIT** -- issue 0052, closed at commit `8e5ef648`. Permanently, not provisionally. **The hardcoded `S` was never the defect and must not be 'fixed' to `absent`.** v2's template seeds `scope: Small`, so any other default is a parity break hiding in a value rather than in an output. What was indefensible was that nobody could override it: **a default nobody can override is a VERDICT; a default with an exit is a STARTING VALUE.** Same `S`, different meaning, no change to the code -- the exit is what changed its status (vc's framing, adopted). **And the self-loop distinction now has the user it was missing.** `wp_rescope` treating a same-size rescope as a movement only where a `scope_legacy` carry is present (`facade.rs:1443-1456`) is, with `rescope` as the sole entrance, the mechanism by which a migrated carry gets ADJUDICATED BY A HUMAN rather than having a size coerced onto it. That capability is what the unwired verb was denying.
- **no op:** **WIRED at `4a0c905c`, and MEASURED -- the declaration this row made before the arm existed is confirmed exactly.** It is the only no-op in the population not decidable from the two states alone, and the discrimination holds in the binary. On a settled package, `intent wp rescope ST0001/01 M` against a package already at `M` prints `ok: ST0001/01 already M` at exit 0 -- the self-loop. On a package carrying `scope_legacy: {raw: "Smallish"}`, `intent wp rescope ST0001/02 S` prints `ok: ST0001/02 rescoped to S` at exit 0 -- **a MOVEMENT, with the same to** -- and only the NEXT call prints `ok: ST0001/02 already S`. **Same specifier, same size, two different answers, and both are right -- but NOT for the reason first written here (cc, 2026-08-17, measured).** The original rationale said a carry sits beside a recorded size and the same from and same to is therefore a movement. **In a valid model that state cannot exist**: settled is `(size, no carry)`, carried is `(none, carry)`, absent is neither, and in the carried case `from` renders as `"<raw> (legacy)"` or the empty string -- **never equal to a size, so the same-state case never arises.** The two inputs are jointly reachable only in a fourth combination that `doctor` reports as residue. **The clause is right as the REPAIR path, which is a better reason than the one it had**: `wp rescope L` on a package that records `L` AND carries `Medium-Large` clears the carry and reports a movement, and **without it no verb in the tool could clear that residue.** cc built the contradictory state by hand and drove it, rather than reasoning about it (`facade.rs:1443-1456`). A row recording only `ok: <ST/NN> already <size>` would have been wrong about half the cases and would have looked complete. **And the size argument refuses with the six named** -- `error: `Huge` is not a T-shirt size`, remedy `one of: XS, S, M, L, XL, XXL`, exit 1 -- which is what `arg_values_note` asked for, since `values` on a non-subcommand slot constrains nothing at the spine and the enum has to be parsed in the arm.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `wp list`

List work packages for a steel thread

- **v2:** bin/intent_wp:218
- **Arguments:**
  - `stid` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- listed, including the empty case -- prints `no work packages for <ID>` and exits 0
  - `1` -- no STID -- `error: Usage: intent wp list <STID>`
  - `1` -- steel thread not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** table via the shared `render_table`, the same renderer `st list` uses
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Renders through `render_table` in bin/intent_helpers deliberately, so `wp list` and `st list` column layout cannot drift apart.
- **Defects observed in v2:**
  - The arity message is `error: Usage: intent wp list <STID>` -- `error()` used to print a usage line, so the `error:` prefix and the `Usage:` voice collide in one string. Voice nit, not a wrong answer. NOTE: an earlier report that this path answers `error: Unknown command 'wp list'` was a zsh harness artefact (unquoted parameters are not word-split, so the dispatcher received one argument literally named `wp list`); it does NOT reproduce with separated arguments and is not a v2 defect.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `wp show`

Show work package info.md

- **v2:** bin/intent_wp:263
- **Arguments:**
  - `specifier` (st-id/NN, arity `1`)
- **Exit codes:**
  - `0` -- printed
  - `1` -- missing or unresolvable specifier
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the info.md contents
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **Note:** WP info.md becomes a generated view in v3 (D02/D04); the command reads the view, so its output is unchanged in kind. **THAT SENTENCE WAS TRUE OF THE NOTE AND FALSE OF THE BINARY, and it is the reason to keep the correction visible here rather than to quietly amend it.** v2 implements `wp show` by catting `info.md` (`bin/intent_wp:263`), and `views.rs` writes that file's status line with `display()`, so v2 printed `status: WIP`. v3 printed `status: wip` -- `enum_str`, not `display()` -- until cc corrected it at `d0f345b5`. **A test was pinning the divergence as expected output**: `cli_end_to_end.rs` asserted `contains("status: wip")` while measuring something else entirely (that a reopen had moved the package), and captured the wrong spelling on the way. **So the estate simultaneously held a CORRECT statement of the requirement, in this note, and an INCORRECT assertion of it, in a test -- about one command, at one time -- and the assertion is the one with teeth, because it runs on every commit while a note is read once at authoring.** Nothing anywhere compares a row's prose to the test that covers it (vc, 2026-08-17). Found because cc's witness reads state back from the tool rather than from a literal, so `st show` said `WIP` and `wp show` said `wip` in the same run. The row was right and the code was not; the row is now right and so is the code.
- **MCP:** exposed as an agent tool -- read-only

## Family: `ac`

Acceptance criteria: the ratified completeness boundary of a unit

- **v2 source:** `bin/intent_acceptance`
- **v2 help file:** none
- **Owning work package:** WP-04
- **BATS coverage:** 57 burning test(s) across 4 file(s) -- **covered**

- `ac` and `at` are two nouns over ONE binary (`bin/intent_acceptance`), dispatched on `$1` as the noun and `$2` as the verb. They share a usage block, so `intent ac --help` and `intent at --help` are the same text -- and both FAIL, because `--help` is parsed as the verb (INV-07).
- An AC has four states, not two (issue 0013): in-scope, satisfied, descoped-to-a-named-thread, withdrawn-with-reason. Descoped and withdrawn are non-blocking and reported separately rather than folded into the satisfied count. This is already reified in the v3 model as `AcScope` (native/rust/crates/intentsvcs/src/model.rs), so the CLI surface here maps onto it directly.
- Satisfaction for test-backed ACs is COMPUTED from covering green ATs and never stored; only non-test ACs carry `satisfied` inline with their evidence. v3 must preserve that asymmetry -- storing it would be double truth (data-model.md).

| command        | args          | flags                                    | help                                                                            | disposition |
| -------------- | ------------- | ---------------------------------------- | ------------------------------------------------------------------------------- | ----------- |
| `ac`           | <command>     | --                                       | Acceptance criteria commands                                                    | keep        |
| `ac list`      | <stid>        | --                                       | List ACs + covering AT + satisfied state                                        | keep        |
| `ac status`    | <stid>        | --                                       | Report N/M satisfied + verdict (PASS/BLOCKED)                                   | keep        |
| `ac satisfy`   | <stid> <acid> | --evidence <ref>                         | Satisfy a non-test AC by named evidence                                         | keep        |
| `ac unsatisfy` | <stid> <acid> | --                                       | Reopen a satisfied non-test AC -- clears satisfaction AND its evidence together | new-surface |
| `ac gate`      | <stid>        | --                                       | Close-gate: exit non-zero + BLOCKED if unsatisfied                              | keep        |
| `ac descope`   | <stid> <acid> | --to <stid>, --by <who>, --reason <text> | Record that an AC moved to another thread (non-blocking)                        | keep        |
| `ac rescope`   | <stid> <acid> | --                                       | Undo a descope: back in scope, unsatisfied                                      | keep        |
| `ac withdraw`  | <stid> <acid> | --reason <text>, --by <who>              | Withdraw an AC outright, with its reason on the record (non-blocking)           | keep        |
| `ac reinstate` | <stid> <acid> | --                                       | Undo a withdrawal: back in scope, unsatisfied                                   | keep        |

### `ac`

Acceptance criteria commands

- **v2:** bin/intent_acceptance:1345-1352 + :22 (shared usage)
- **Arguments:**
  - `command` (subcommand, arity `1`)
- **Exit codes:**
  - `1` -- bare -- prints the 1341B shared usage block to STDOUT and exits 1
  - `1` -- `--help` parsed as an unknown verb
  - `1` -- unknown verb -- names the valid set
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the shared ac/at usage block (1341B), on STDOUT
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - INV-06 at usage to STDOUT on a failing invocation
  - INV-07 at `ac --help` parsed as an unknown verb
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: Defers to INV-07, now `corrected`: `--help` exits 0 to stdout. The v3 binary already does this.
- **build measured:** BUILD HAS ALREADY DECIDED (ic measured, 2026-08-15). See INV-07: the v3 binary exits 0 on `--help` today, deliberately (`spine.rs:193` maps clap's DisplayHelp to EXIT_OK, and `spine.rs:27` names the divergence). This row asks whether to reproduce or correct, and the code has answered `corrected` without the contract recording it. Measurement, not a ruling.
- **ratified in:** parity.md
- **MCP:** not exposed -- read-only

### `ac list`

List ACs + covering AT + satisfied state

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id[/NN], arity `1`)
    - the optional `/NN` narrows to one WP group
- **Exit codes:**
  - `0` -- listed
  - `1` -- thread not found / contract missing
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** one row per AC
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `ac status`

Report N/M satisfied + verdict (PASS/BLOCKED)

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
- **Exit codes:**
  - `0` -- reported
  - `1` -- thread not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the count and verdict
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Issue 0004 item 4 queried this verb's exit code; the premise did not reproduce, and it is parked awaiting a close ruling rather than work.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `ac satisfy`

Satisfy a non-test AC by named evidence

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `acid` (ac-id, arity `1`)
- **Flags:**
  - `--evidence` `<ref>` (string) -- The named evidence reference
    - **required:** true
    - **disposition:** keep
- **Exit codes:**
  - `0` -- satisfied
  - `1` -- missing --evidence
  - `1` -- AC not found, or is test-backed -- satisfaction there is computed, never written
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AC> satisfied by evidence`
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **no op:** `ok: <AC> already satisfied`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **voice ruling:** **BUG, not a deviation: restore ` by evidence`.** v2 prints `ok: <AC> satisfied by evidence` and v3 prints `ok: <AC> satisfied`. The phrase names WHY the criterion is satisfied, and under v3's model that distinction became STRUCTURAL rather than decorative -- `AcState::Satisfied` carries evidence that must not be empty, and a test-backed criterion is `Computed` with nothing stored. **So the words are more load-bearing in v3 than they were in v2**, and dropping them is the one change that cannot be argued as tidying.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `ac unsatisfy`

Reopen a satisfied non-test AC -- clears satisfaction AND its evidence together

- **v2:** new-surface
- **Arguments:**
  - `stid` (positional, arity `1`)
  - `acid` (positional, arity `1`)
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: cc, 2026-08-15, service half at `acf8491` -- the inverse D32 requires. `ac satisfy` was a one-way door: a verifier whose evidence later proved incomplete had to HAND-EDIT acceptance.md, the one file the CLI exists to own. Recorded here BEFORE the surface is wired, per AC-06.3, because the spine builds from this table and the command cannot exist until the row does.
- **Note:** Clears satisfaction and evidence TOGETHER, deliberately. Evidence outliving the claim it supported is the defect this fixes, not a convenience it preserves -- a cleared AC keeping its old evidence reads as satisfied-with-provenance to every later reader.
- **consequence:** Refuses a test-backed AC (satisfaction there is COMPUTED from covering green ATs and never stored -- unsetting it would be writing to a derived field) and refuses an AC that is not satisfied (nothing to undo; silent success on a no-op is INV-01 territory).
- **placement:** FIRST sub-verb addition in this canon: every `new_surface[]` entry is a top-level command (search, sync, schema, export, ingest, backup, daemon, mcp). This one is recorded as a FAMILY ENTRY instead, because the spine places verbs under their family from `families[].entries[]` and a bare `ac unsatisfy` in the top-level array would have no parent. Flagged rather than assumed -- cc owns the spine and should confirm it builds from here; vc owns whether the contract wants one home or two.
- **no op:** `ok: <AC> already unsatisfied`, exit 0 -- **SHIPPED at `d0f345b5`, and this row's EXIT LIST changed with it**: the already-unsatisfied case was exit 1 until that commit. Measured 2026-08-17 by driving the verb twice through the real binary (ic). It carried a hand-written `if !matches!(state, Satisfied {..})` ahead of delegating, so the shared setter's self-loop test was unreachable for this verb -- 0051's mechanism, and the refusal is now mapped from the declared machine rather than hand-run ahead of it (`facade.rs:1658-1690`). **Its two neighbours twenty lines down were not fixed: see issue 0053.**
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `ac gate`

Close-gate: exit non-zero + BLOCKED if unsatisfied

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id[/NN], arity `1`)
- **Exit codes:**
  - `0` -- PASS -- every in-scope AC satisfied, or the unit declares `acceptance: exempt`
  - `1` -- BLOCKED -- at least one in-scope AC unsatisfied
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the verdict plus the unsatisfied set
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Consulted by `st done` and `wp done`. Fail-by-default since ST0048: an empty or missing contract is REFUSED, and the sole escape is a declared `acceptance: exempt`, never inferred from emptiness. Scope-honouring since issue 0024: a `/NN` scope is actually applied -- it used to be silently dropped.
- **Target:** `corrected` -- ratified: vc ruling 2026-08-16, provisional pending hv, recorded on issue 0032 rather than only in an inbox. NOT cited by parity.md -- this row is ratified ELSEWHERE and corrected_check.sh should report it in that bucket.
- **Note:** AC-04.3 requires v3 to reproduce v2 gate verdicts across the corpus contracts. This is the single highest-value parity row in the family.
- **why corrected and not as observed:** The `Corrected` class is defined as `a v2 behaviour that is simply wrong and is fixed rather than faithfully reproduced`, and `as-observed` is for behaviour v3 CHOOSES to reproduce. hv's own wording on issue 0032 is that the combining rule `was chosen by an early-return rather than by a decision` -- **you cannot faithfully reproduce a decision nobody made**, and reproducing it is what parity.md forbids in its own sentence: laundering a v2 defect into a v3 requirement.
- **the conflict ic raised is EMPTY and that is what decides it:** ic recorded this row on 2026-08-15 as two ratified things pointing opposite ways -- AC-04.3 requires v3 to reproduce v2's gate VERDICTS, and correcting 0032 would change verdicts on exactly the contracts AC-04.3 measures against. **It changes none.** vc re-measured all 109 AT rows: the two multi-AT ACs are AC-00.7 (AT-00.5 red + AT-00.7 to-write -> OR unsatisfied, AND unsatisfied) and AC-03.7 (AT-03.7 green + AT-03.9 green -> OR satisfied, AND satisfied). **Zero verdicts move; AC-04.3 is untouched and there is no parity break to ratify.** OR and AND can only diverge on a multi-AT AC holding a green beside a non-green, and no such row exists.
- **the actionable part is TIMING:** Correct it while it is free. AC-03.7 is the near exposure: two greens, so one regression scores it satisfied on the survivor. **The moment any multi-AT AC goes mixed-with-a-green the fix starts MOVING a verdict and will read as a regression rather than a correction.** The window is open and closes on its own.
- **consequence for the AT LEDGER:** Under an OR gate a second covering AT row is worse than neutral -- it is a place a future green can hide a red -- so adding rows to a gate that ORs actively LOWERS the bar it appears to raise. That is why AT-05.5 names its three instruments in one note rather than carrying three rows, and it is the correct form rather than a workaround.
- **ratified in:** vc ruling 2026-08-16 (provisional pending hv), recorded on issue 0032
- **MCP:** exposed as an agent tool -- read-only
- **Wants review -- the classification disagrees with the verb name:** `gate` reads as an enforcement action that stamps a verdict somewhere. It computes and reports; the write lives in the caller (`st done` / `wp done`). vc's own example of why derivation-from-name dies.
- **MCP classification grounded in:** bin/intent_acceptance:973 (cmd_ac_gate) -- no write primitive in the body

### `ac descope`

Record that an AC moved to another thread (non-blocking)

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `acid` (ac-id, arity `1`)
- **Flags:**
  - `--to` `<stid>` (string) -- The thread the requirement moved to
    - **required:** true
    - **disposition:** keep
  - `--by` `<who>` (string) -- Who decided
    - **disposition:** keep
  - `--reason` `<text>` (string) -- Why
    - **disposition:** keep
- **Exit codes:**
  - `0` -- descoped
  - `1` -- missing --to
  - `1` -- AC not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AC> descoped to <ID>`
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **no op:** `ok: <AC> already descoped`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **A deliberate deviation from v2, which refuses at 1, and it DROPS the target the movement line carries**: the movement prints where the criterion went, the no-op prints only that it is descoped. That follows the ruled form `ok: <subject> already <state>` -- `descoped-to ST0002` is not a state -- but it means the no-op answers a strictly smaller question than the movement it stands in for. **One of the two arms 0050's line-oriented enumeration could not see**, because the facade call breaks across lines here.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `ac rescope`

Undo a descope: back in scope, unsatisfied

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `acid` (ac-id, arity `1`)
- **Exit codes:**
  - `0` -- rescoped
  - `1` -- AC not found or not descoped
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AC> back in scope (unsatisfied)`
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `corrected` -- ratified: ic, 2026-08-17, ruling the five voices cc was blocked on -- RATIFY onto `corrected`: `ok: <AC> back in scope (<landing state>)`, with the state COMPUTED FROM KIND. **v2 prints `back in scope (unsatisfied)` for BOTH kinds and BOTH verbs** -- measured, four invocations, one string modulo the id -- and v3 prints the bare `back in scope`. Restoring v2's literal would be wrong for half the cases: **a test-backed criterion lands at `computed`, not `unsatisfied`** (`AcState::entry(kind)`). **Keep the suffix, compute it.** The resulting state is the whole question an undo raises, and `back in scope` alone does not say whether satisfaction survived the round trip. **CORRECTED 2026-08-17, and the first version of this ruling EXCUSED v2.** It said v2 "had no `computed` state to name, so it could only ever say one thing". That is false. v2 has the concept: `bin/intent_acceptance:16` states that a test-backed AC "is satisfied (computed) iff a covering AT is green", and `:1084` refuses to hand-satisfy one on exactly that ground. What v2 lacks is the BRANCH AT THIS CALL SITE -- `:1259` prints the literal `(unsatisfied)` unconditionally, under a comment at `:1252-1254` that reasons only about the non-test case. **So v2 announces `unsatisfied` about a criterion whose satisfaction it computes elsewhere: the message's subject and the row's subject differ**, which is the day's class rather than a limit of v2's model. The deviation is forced by v2 being WRONG here, not by v3 being unable to reproduce it -- same destination, load-bearing difference in the grounds.
- **no op:** **REFUSES A LEGAL SELF-LOOP AT EXIT 1 -- issue 0053, and this is the one place the announced value and the binary disagree.** Measured 2026-08-17 by driving the verb twice through the real binary (ic). On a criterion already holding the verb's target -- `AcState::entry(kind)`, `unsatisfied` for a non-test and `computed` for a test-backed one -- it prints `error: <AC> is in scope, so there is nothing to reinstate` at exit 1, for BOTH verbs and BOTH kinds. 0051's mechanism: a hand-written from-state `match` whose `_` arm raises `NotOffScope` ahead of `set_ac_state`, which is where the shared self-loop test lives (`facade.rs:1764-1795`). **The value cc handed over -- `ok: <AC> already unsatisfied | already computed` -- was reasoned correctly from `AcState::entry(kind)`, which is what the SUCCESS arm passes the setter; the self-loop never reaches that arm.** Written from the binary rather than the handoff, which is the only reason the row is not wrong. **And the refusal names a verb the user did not type**: `NotOffScope` carries no verb, so one hardcoded message serves both entry points and `ac rescope` is answered about `reinstate` twice -- a REGRESSION, since v2 names the right verb in both.
- **ratified in:** ic, 2026-08-17, five-voice ruling recorded at commit `49db5aec` -- NOT parity.md, whose `Corrected` class does not cite these two and should not be extended to reach them (vc ruled: thirteen rows are ratified elsewhere and eleven predate these, so the coherent options are all-thirteen-in or all-thirteen-out). **Carries authority, date and an EXTERNAL record**, which is the grammar vc's `ratified_in` sweep arrived at: the first draft of this field said "on this row's own voice ruling" and a ratification pointing at the row it ratifies is not independent, by the same argument that a check is not independent unless its source is the corpus.
- **stdout exact:**
  - `template`: ok: AC-01.2 back in scope (computed)
  - `basis`: the row's own `ratification`: the landing state computed from kind, and `AcState::entry(kind)` is `Computed` for a test-backed criterion. **NOT measured from v3.** These are the bytes v3 is REQUIRED to print; writing them from the binary would make the assertion say v3 == v3, which is the laundering this field exists to prevent. Expect this to be RED until cc implements the ruled voice -- a red here is an unimplemented ratification, NOT a parity break, and the two have different remedies.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `ac withdraw`

Withdraw an AC outright, with its reason on the record (non-blocking)

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `acid` (ac-id, arity `1`)
- **Flags:**
  - `--reason` `<text>` (string) -- Why it was dropped -- REQUIRED
    - **required:** true
    - **disposition:** keep
  - `--by` `<who>` (string) -- Who decided
    - **disposition:** keep
- **Exit codes:**
  - `0` -- withdrawn
  - `1` -- missing --reason -- the reason is mandatory, by design
  - `1` -- AC not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AC> withdrawn`
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** `--reason` being mandatory is the whole point of the verb: the alternative to withdrawing is deleting the line and losing the audit trail.
- **Target:** `as-observed`
- **no op:** `ok: <AC> already withdrawn`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice, a deliberate deviation from v2's refusal. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **The other arm 0050's enumeration could not see**, for the same reason as `ac descope`: the facade call breaks across lines.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `ac reinstate`

Undo a withdrawal: back in scope, unsatisfied

- **v2:** bin/intent_acceptance:1354-1366 (the `ac` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `acid` (ac-id, arity `1`)
- **Exit codes:**
  - `0` -- reinstated
  - `1` -- AC not found or not withdrawn
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AC> back in scope (unsatisfied)`
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `corrected` -- ratified: ic, 2026-08-17, ruling the five voices cc was blocked on -- RATIFY onto `corrected`, IDENTICAL in form to `ac rescope`. v3 currently prints `ok: <AC> reinstated`, which names the verb and drops the state entirely. **The two verbs differ in which off-scope state they UNDO, not in where they LAND** -- both reach `AcState::entry(kind)` -- so they must print the same thing, and v2 agreed: both said `back in scope (unsatisfied)`, measured. A message naming the verb tells the caller what they just typed; a message naming the state tells them something they did not already know. **Keep the suffix, compute it.** The resulting state is the whole question an undo raises, and `back in scope` alone does not say whether satisfaction survived the round trip. **CORRECTED 2026-08-17, and the first version of this ruling EXCUSED v2.** It said v2 "had no `computed` state to name, so it could only ever say one thing". That is false. v2 has the concept: `bin/intent_acceptance:16` states that a test-backed AC "is satisfied (computed) iff a covering AT is green", and `:1084` refuses to hand-satisfy one on exactly that ground. What v2 lacks is the BRANCH AT THIS CALL SITE -- `:1259` prints the literal `(unsatisfied)` unconditionally, under a comment at `:1252-1254` that reasons only about the non-test case. **So v2 announces `unsatisfied` about a criterion whose satisfaction it computes elsewhere: the message's subject and the row's subject differ**, which is the day's class rather than a limit of v2's model. The deviation is forced by v2 being WRONG here, not by v3 being unable to reproduce it -- same destination, load-bearing difference in the grounds.
- **no op:** **REFUSES A LEGAL SELF-LOOP AT EXIT 1 -- issue 0053, and this is the one place the announced value and the binary disagree.** Measured 2026-08-17 by driving the verb twice through the real binary (ic). On a criterion already holding the verb's target -- `AcState::entry(kind)`, `unsatisfied` for a non-test and `computed` for a test-backed one -- it prints `error: <AC> is in scope, so there is nothing to reinstate` at exit 1, for BOTH verbs and BOTH kinds. 0051's mechanism: a hand-written from-state `match` whose `_` arm raises `NotOffScope` ahead of `set_ac_state`, which is where the shared self-loop test lives (`facade.rs:1764-1795`). **The value cc handed over -- `ok: <AC> already unsatisfied | already computed` -- was reasoned correctly from `AcState::entry(kind)`, which is what the SUCCESS arm passes the setter; the self-loop never reaches that arm.** Written from the binary rather than the handoff, which is the only reason the row is not wrong. **And the refusal names a verb the user did not type**: `NotOffScope` carries no verb, so one hardcoded message serves both entry points and `ac rescope` is answered about `reinstate` twice -- a REGRESSION, since v2 names the right verb in both.
- **ratified in:** ic, 2026-08-17, five-voice ruling recorded at commit `49db5aec` -- NOT parity.md, whose `Corrected` class does not cite these two and should not be extended to reach them (vc ruled: thirteen rows are ratified elsewhere and eleven predate these, so the coherent options are all-thirteen-in or all-thirteen-out). **Carries authority, date and an EXTERNAL record**, which is the grammar vc's `ratified_in` sweep arrived at: the first draft of this field said "on this row's own voice ruling" and a ratification pointing at the row it ratifies is not independent, by the same argument that a check is not independent unless its source is the corpus.
- **stdout exact:**
  - `template`: ok: AC-01.2 back in scope (computed)
  - `basis`: the row's own `ratification`: the landing state computed from kind, and `AcState::entry(kind)` is `Computed` for a test-backed criterion. **NOT measured from v3.** These are the bytes v3 is REQUIRED to print; writing them from the binary would make the assertion say v3 == v3, which is the laundering this field exists to prevent. Expect this to be RED until cc implements the ruled voice -- a red here is an unimplemented ratification, NOT a parity break, and the two have different remedies.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

## Family: `at`

Acceptance tests: the small red-to-green tests that prove ACs

- **v2 source:** `bin/intent_acceptance`
- **v2 help file:** none
- **Owning work package:** WP-04
- **BATS coverage:** 30 burning test(s) across 2 file(s) -- **covered**

- **parity.md's command-level table was wrong about this family and the deep pass corrected it.** The table said `lint [--fix], set, list`. Measured: `list, lint, red, green, na, done, notdone`. There is NO `set` verb -- `cmd_at_set` is an internal function -- and `done`/`notdone` are aliases for `green`/`red`. Evidence: `intent at set` answers `error: unknown at command: set`.
- The AT row has an enforced grammar (issue 0017) with exactly two shapes and nothing else parsing. The reference is the test FILE -- backticked, repo-relative, at least one `/`, no `:`. A test is named by putting the AT id INSIDE the test, which is checkable from both ends and survives rewording; a cited test NAME is not.
- `n-a` is a status for non-test rows ONLY and is NOT green: satisfaction for such a row lives on the AC's own line. v3 reifies this as `AtStatus::Na` with the serde rename `n-a` (native/rust/crates/intentsvcs/src/model.rs).

| command                       | args          | flags | help                                                                  | disposition |
| ----------------------------- | ------------- | ----- | --------------------------------------------------------------------- | ----------- |
| `at`                          | <command>     | --    | Acceptance test commands                                              | keep        |
| `at list`                     | <stid>        | --    | List ATs (id, reference, status)                                      | keep        |
| `at lint`                     | <stid>        | --fix | Check AT rows against the grammar (--fix migrates what is mechanical) | keep        |
| `at green` (alias `at done`)  | <stid> <atid> | --    | Set an AT green (reachable only from red)                             | keep        |
| `at red` (alias `at notdone`) | <stid> <atid> | --    | Set an AT red                                                         | keep        |
| `at na`                       | <stid> <atid> | --    | Set a non-test AT to n-a (the doc / eyeball / gate status)            | keep        |

### `at`

Acceptance test commands

- **v2:** bin/intent_acceptance:1345-1352 + :22 (shared usage)
- **Arguments:**
  - `command` (subcommand, arity `1`)
- **Exit codes:**
  - `1` -- bare -- shared usage block to STDOUT, exit 1
  - `1` -- `--help` parsed as an unknown verb
  - `1` -- unknown verb -- names the valid set
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the shared ac/at usage block (1341B), on STDOUT
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - INV-06 at usage to STDOUT on a failing invocation
  - INV-07 at `at --help` parsed as an unknown verb
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: Defers to INV-07, now `corrected`: `--help` exits 0 to stdout. The v3 binary already does this.
- **build measured:** BUILD HAS ALREADY DECIDED (ic measured, 2026-08-15). See INV-07: the v3 binary exits 0 on `--help` today, deliberately (`spine.rs:193` maps clap's DisplayHelp to EXIT_OK, and `spine.rs:27` names the divergence). This row asks whether to reproduce or correct, and the code has answered `corrected` without the contract recording it. Measurement, not a ruling.
- **ratified in:** parity.md
- **MCP:** not exposed -- read-only

### `at list`

List ATs (id, reference, status)

- **v2:** bin/intent_acceptance:1368-1377 (the `at` dispatch arm)
- **Arguments:**
  - `stid` (st-id[/NN], arity `1`)
- **Exit codes:**
  - `0` -- listed
  - `1` -- thread not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** one row per AT
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `at lint`

Check AT rows against the grammar (--fix migrates what is mechanical)

- **v2:** bin/intent_acceptance:1368-1377 (the `at` dispatch arm)
- **Arguments:**
  - `stid` (st-id[/NN], arity `1`)
- **Flags:**
  - `--fix` (bool) -- Migrate the mechanical part of a legacy row -- and REFUSE what cannot migrate without loss
    - **disposition:** keep
- **Exit codes:**
  - `0` -- all rows parse
  - `1` -- L1-L5 findings present
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** findings, one per offending row, each naming its line
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Scope-honouring since issue 0024: a `/NN` scope is applied, and a scoped `--fix` no longer rewrites rows OUTSIDE the scope.
- **Target:** `as-observed`
- **Note:** **The refuse-lossy discipline is the load-bearing part and must survive into WP-10's migrator.** `--fix` once half-migrated rows and destroyed the only link a row had; the SUGGESTION was lossy before the fixer was, so every human following it lost the same data. A tool that cannot finish a job must not start it.
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review -- the classification disagrees with the verb name:** `lint` is the canonical read-only verb and `intent at lint --fix` migrates rows in place. Classified by the whole entry, not by its default invocation.
- **MCP classification grounded in:** bin/intent_acceptance:1266 (`--fix) fix=1`), at_lint_fix / at_fix_line
- **recoverability:** idempotent

### `at green`

Set an AT green (reachable only from red)

- **v2:** bin/intent_acceptance:1368-1377 (the `at` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `atid` (at-id, arity `1`)
- **Exit codes:**
  - `0` -- set
  - `1` -- AT not found
  - `1` -- cited test file does not exist -- a green AT must resolve against the tree
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AT> -> green`
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** The file-existence check fires on the transition to green specifically (bin/intent_acceptance:1337): a green row whose test does not exist is the exact shape of a vacuous pass.
- **Target:** `corrected` -- ratified: ic, 2026-08-15, ruling cc's parity question -- KEEP THE GUARD. v2 refuses green unless the AT is currently red (bin/intent_acceptance:1325). v3's at_set takes any status from any status. This is v3 more OPEN and less faithful, not more closed.
- **Note:** THE GUARD IS NOT AN ARBITRARY RESTRICTION, WHICH IS WHY THIS IS NOT A DIVERGENCE WORTH BUYING. Requiring green to come from red means an AT cannot be marked passing without first having been recorded as failing -- it is the MECHANISED form of this thread's own central doctrine, that a check which has only ever passed is not verified. Drop it and the discipline survives only as prose, which is rule 12 exactly.
- **consequence:** Three instances on 2026-08-15 alone of a green that proved nothing, none of which had ever been seen red: four vacuous greps that never opened a file, a normaliser that silently did nothing under BSD sed, and a `touch`ed canary whose empty diff sent the run down the wrong branch. v3 restores the from-red guard.
- **open to cc:** v2 carries FOUR guards on `at`, not one, and only the from-guard was raised. The others: `na` refuses on a test-backed AT; a non-`na` status refuses on a `(non-test)` AT; and green/red on a test-backed row refuse unless the CITED TEST FILE RESOLVES on disk (issue 0015 -- catching a rename at the point of the lie rather than after a stale green has counted as coverage for months). Please report whether v3 has those three, because if they went with the from-guard the divergence is four times what was reported.
- **ratified in:** ic, 2026-08-15, ruling recorded at commit `20e8c4b3` (`st(0056): ac unsatisfy recorded, at green RULED, ...`). **The previous value was `ic, with the `at green` row's own basis` -- the self-citation case, and the precedent I copied when writing my own two rows before vc's sweep fixed the grammar's requirement.** A ratification pointing at the row it ratifies is not independent, by exactly the argument that a check is not independent when the same head wrote the needle and the checker: the source has to be something other than the thing under test.
- **no op:** `ok: <AT> already green`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice, a deliberate deviation from v2's refusal. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** one-way
- **recoverability anomaly:** ONE-WAY BY MEASURED BEHAVIOUR, NOT BY DESIGN -- issue 0033. `intent at red|green|na` DESTROYS the row's authored note, so the round trip moves the status back and does NOT restore the prior state. vc measured it on themselves with the issue in working memory: AT-03.12 went from 1,560 bytes to 106, losing 1,447 characters of authored contract, and `intent at lint` reported `ok -- 112 rows conform` immediately afterwards. **The transition graph multiplies it: `to-write -> green` is refused and green is reachable only from `red`, so recording a passing test costs TWO rewrites.** 14,253 characters stand at risk across 34 rows. **Classified against SHIPPED BEHAVIOUR on vc's ruling** -- a field describing what a command is SUPPOSED to do is the `doctor` failure, where the declaration outlived its subject in silence. This row becomes `reversible` when 0033 is fixed, and that is the field tracking reality rather than drifting from it.

### `at red`

Set an AT red

- **v2:** bin/intent_acceptance:1368-1377 (the `at` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `atid` (at-id, arity `1`)
- **Exit codes:**
  - `0` -- set
  - `1` -- AT not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AT> -> red`
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **no op:** `ok: <AT> already red`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **voice ruling:** **BUG: restore the `-> ` arrow.** v2 prints `ok: <AT> -> red`, v3 prints `ok: <AT> red`. **This is drift, not a voice ruling, and v3's own output proves it: `issues close` and `issues open` KEEP the arrow** (`ok: issue 0001 -> CLOSED`) and both reproduce v2 exactly. So the arrow is v3's convention for a state transition and the AT family is the only place it is missing. Restoring costs nothing and fixes two rows; dropping it everywhere for consistency would break two rows that currently pass.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** one-way
- **recoverability anomaly:** ONE-WAY BY MEASURED BEHAVIOUR, NOT BY DESIGN -- issue 0033. `intent at red|green|na` DESTROYS the row's authored note, so the round trip moves the status back and does NOT restore the prior state. vc measured it on themselves with the issue in working memory: AT-03.12 went from 1,560 bytes to 106, losing 1,447 characters of authored contract, and `intent at lint` reported `ok -- 112 rows conform` immediately afterwards. **The transition graph multiplies it: `to-write -> green` is refused and green is reachable only from `red`, so recording a passing test costs TWO rewrites.** 14,253 characters stand at risk across 34 rows. **Classified against SHIPPED BEHAVIOUR on vc's ruling** -- a field describing what a command is SUPPOSED to do is the `doctor` failure, where the declaration outlived its subject in silence. This row becomes `reversible` when 0033 is fixed, and that is the field tracking reality rather than drifting from it.

### `at na`

Set a non-test AT to n-a (the doc / eyeball / gate status)

- **v2:** bin/intent_acceptance:1368-1377 (the `at` dispatch arm)
- **Arguments:**
  - `stid` (st-id, arity `1`)
  - `atid` (at-id, arity `1`)
- **Exit codes:**
  - `0` -- set
  - `1` -- AT not found
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: <AT> -> n/a`
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** `n/a` is v2's token for a `(non-test)` row and the only one its row grammar admits; such a row never satisfies anything, because satisfaction lives on the AC's own line. **v3 spells the same state `n-a` -- that is issue 0056 and NOT v2 vocabulary.** This sentence carried the v3 token until 2026-08-17: the `stdout` above was corrected first and the gloss beside it was not, so the row stated the right value and the wrong vocabulary for one commit (found by vc). **A correction has to reach every field that repeats the value, not the field that was reported.**
- **Target:** `as-observed`
- **no op:** `ok: <AT> already n-a`, exit 0 -- **SHIPPED at `d0f345b5`** in the ruled voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic). **THE STATE TOKEN IS SPELLED DIFFERENTLY FROM v2's: `n-a` here, `n/a` there.** v2 prints `ok: AT-01.2 -> n/a` and the row grammar itself writes `status: n/a`, so the slash is the authored spelling in every `acceptance.md` in the estate. This is the only row in the population whose no-op differs from v2 in the STATE WORD rather than in the sentence around it. **FILED AS 0056, and the no-op line is the smallest of its three faces**: `AtStatus` is the only status enum with no `display()`, so `views.rs:459-461` writes `status: n-a` into the GENERATED `acceptance.md` -- and v2's linter rejects that row L1, `unreadable AT status: 'n-a'`. 23 rows in this estate carry `status: n/a` today. Three of four variants coincide between the two spellings, which is what hid it.
- **voice ruling:** **BUG TWICE OVER, and neither half is ratifiable.** v2 prints `ok: <AT> -> n/a`; v3 prints `ok: <AT> na` on a movement and `ok: <AT> already n-a` on a self-loop. Restore the arrow as on `at red`. **And the token must be `n/a`**: `views.rs` writes it into the generated `acceptance.md` and v2's linter rejects `n-a` at L1, so this is a migration hazard rather than a style choice (issue 0056). **The movement arm must take the word from the ENUM, not from the subcommand name** -- it currently passes `state`, the verb the user typed, which is why a move prints `na` while a self-loop prints `n-a`.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** one-way
- **recoverability anomaly:** ONE-WAY BY MEASURED BEHAVIOUR, NOT BY DESIGN -- issue 0033. `intent at red|green|na` DESTROYS the row's authored note, so the round trip moves the status back and does NOT restore the prior state. vc measured it on themselves with the issue in working memory: AT-03.12 went from 1,560 bytes to 106, losing 1,447 characters of authored contract, and `intent at lint` reported `ok -- 112 rows conform` immediately afterwards. **The transition graph multiplies it: `to-write -> green` is refused and green is reachable only from `red`, so recording a passing test costs TWO rewrites.** 14,253 characters stand at risk across 34 rows. **Classified against SHIPPED BEHAVIOUR on vc's ruling** -- a field describing what a command is SUPPOSED to do is the `doctor` failure, where the declaration outlived its subject in silence. This row becomes `reversible` when 0033 is fixed, and that is the field tracking reality rather than drifting from it.

## Family: `issues`

Track issues without the ceremony of a steel thread

- **v2 source:** `bin/intent_issues`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 20 burning test(s) across 1 file(s) -- **covered**

- **The OPEN/CLOSED directory layout is a ratified deviation.** v2 stores issues at `intent/issues/{OPEN,CLOSED}/NNNN/NNNN-slug.md`, so the directory encodes status. In v3 status is data (`issues/<n>.json`) and index views replace directory browsing (parity.md, D02/D04). Tests asserting the directory shape retire with the layout.
- `new` is an undocumented alias for `add`, and there is an undocumented `help` verb -- both measured, neither in parity.md's original table.

| command                           | args      | flags                               | help                                                | disposition |
| --------------------------------- | --------- | ----------------------------------- | --------------------------------------------------- | ----------- |
| `issues`                          | [command] | --                                  | Track issues without the ceremony of a steel thread | keep        |
| `issues list`                     | --        | --kind open/closed/all              | List issues (default: open)                         | keep        |
| `issues add` (alias `issues new`) | <title>   | --severity critical/high/medium/low | Add a new issue, print its ID:TITLE                 | keep        |
| `issues show`                     | <id>      | --json                              | Show one issue (optionally as JSON)                 | keep        |
| `issues close`                    | <id>      | --                                  | Mark an issue done: OPEN -> CLOSED                  | keep        |
| `issues open`                     | <id>      | --                                  | Reopen an issue: CLOSED -> OPEN                     | keep        |

### `issues`

Track issues without the ceremony of a steel thread

- **v2:** bin/intent_issues:308-316
- **Arguments:**
  - `command` (subcommand, arity `0..1`), default `list`
- **Exit codes:**
  - `0` -- bare -- defaults to `list`, prints `no open issues` and exits 0
  - `0` -- `--help` -- 577B usage to STDOUT, exit 0
  - `1` -- unknown verb -- `error: Unknown issues command '<v>'. Run 'intent issues help' for usage.`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the issue list, or the usage block
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** One of the 17 commands whose `--help` correctly exits 0 -- it is NOT an INV-07 member.
- **Target:** `as-observed`
- **MCP:** not exposed -- read-only

### `issues list`

List issues (default: open)

- **v2:** bin/intent_issues:310, buckets at :207-209
- **Flags:**
  - `--kind` `open|closed|all` (enum) -- Which bucket to list
    - **default:** open
    - **disposition:** keep
- **Exit codes:**
  - `0` -- listed
  - `1` -- unknown --kind value
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** one row per issue
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `issues add`

Add a new issue, print its ID:TITLE

- **v2:** bin/intent_issues:311
- **Arguments:**
  - `title` (string, arity `1`)
- **Flags:**
  - `--severity` `critical|high|medium|low` (enum) -- Severity
    - **default:** medium
    - **disposition:** keep
- **Exit codes:**
  - `0` -- created -- prints TWO lines, `created: <path to the issue's md file>` then `<ID>:<TITLE>` (bin/intent_issues:187-188)
  - `1` -- no title -- `error: Issue title is required`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** TWO lines: `created: <path to the issue's file>` then `<ID>:<TITLE>`. **v2, measured 2026-08-17:** `created: intent/issues/OPEN/0001/0001-probe-two.md` / `0001:Probe two`. **v3 at `34c6a3ae`, measured:** `created: /Users/.../intent/issues/0002.json` / `0002:Probe two`. **THE TWO LINES HAVE DIFFERENT ANSWERS AND THE ROW HAS ONE `target.state` -- see `notes`.**
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** **LINE 2 IS `as-observed` AND LINE 1 IS NOT, AND THE SCHEMA CANNOT SAY SO (0060).** Line 1 differs from v2 in TWO independent ways with OPPOSITE legitimacy. **The layout is ratified**: v3's issue canon is flat, `intent/issues/<NNNN>.json` (`data-model.md:258`, `store.rs:179`), against v2's status-bucketed `OPEN/<NNNN>/<NNNN>-<slug>.md`. v3 cannot print v2's path because v3 does not create it. **The ABSOLUTENESS is a defect** -- v2 prints repo-relative, v3 prints the fully-qualified path including `$HOME` -- and it is forced by nothing. Filed as 0060. **No `stdout_exact` is declared and cannot be while 0060 stands**: a literal template cannot contain a machine's working directory, so this row is excluded from the only instrument that has found parity breaks on this thread, BY the defect that instrument exists to catch. Every other row's coverage question is whether anyone wrote the declaration; this row's is whether one can be written at all. `new` is an undocumented alias, verified by invocation: `intent issues new` answers `error: Issue title is required`, identical to `add`.
- **Target:** `corrected` -- ratified: ic, 2026-08-17 -- CONSEQUENCE OF AN EXISTING RULING, NOT A NEW ONE. The flat issue canon is already ratified in `data-model.md:258` and built in `store.rs:179`, so line 1's path CANNOT reproduce v2's and the deviation is forced by the model rather than chosen -- the same footing as `ac rescope`. Recording the row as `as-observed` asserted a parity that the data model had already made impossible. **This ratifies the LAYOUT half only.** The absoluteness (0060) is a defect and stays one; when it is fixed, line 1 becomes `created: intent/issues/<NNNN>.json`, the row collapses from two deviations to one, and it becomes expressible in the schema that exists today. Fixing 0060 first is therefore the cheaper order.
- **ratified in:** ic, 2026-08-17, on the flat-canon decision recorded at `data-model.md:258` and built at `store.rs:179` -- NOT parity.md, which does not cite this row. Authority, date and an external record, per vc's `ratified_in` grammar.
- **open defect:** 0060 -- v3 prints an ABSOLUTE path where v2 prints repo-relative. Separate from the ratified layout change and must not be resolved with it: one line carrying two deviations of opposite legitimacy is unreadable as either.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `issues show`

Show one issue (optionally as JSON)

- **v2:** bin/intent_issues:312
- **Arguments:**
  - `id` (issue-id, arity `1`)
- **Flags:**
  - `--json` (bool) -- Emit as JSON instead of prose
    - **disposition:** keep
- **Exit codes:**
  - `0` -- printed
  - `1` -- issue not found -- `error: Issue not found: <ID>` (bin/intent_issues:255). **No bucket named here**, unlike `issues close` / `issues open`; see this row's `target.question`.
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the issue body, or JSON
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **Open question for hv:** The v2-spells-it-three-ways / v3-has-one-`NoSuchIssue` deviation is recorded ONCE, on `issues close`. This row carries the condition and not a second copy of the reasoning.
- **MCP:** exposed as an agent tool -- read-only

### `issues close`

Mark an issue done: OPEN -> CLOSED

- **v2:** bin/intent_issues:313
- **Arguments:**
  - `id` (issue-id, arity `1`)
- **Exit codes:**
  - `0` -- closed -- `ok: issue <ID> -> CLOSED` (bin/intent_issues:291)
  - `0` -- ALREADY in the target bucket -- `ok: issue <ID> already CLOSED`, exit 0 (bin/intent_issues:280-283). **This is v2's ONLY `already` arm and the one hv's self-loop ruling cites**; `move_issue` checks the SOURCE bucket, and on a miss checks the TARGET before erroring.
  - `1` -- in NEITHER bucket -- `error: Issue not found in OPEN: <ID>` (bin/intent_issues:285). The message names the BUCKET, not just the id.
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: issue <ID> -> CLOSED` on a move, `ok: issue <ID> already CLOSED` on the self-loop
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Moves the issue directory between OPEN/ and CLOSED/ -- the v2 layout that retires under the ratified deviation
- **Target:** `as-observed`
- **Open question for hv:** **v2 SPELLS ONE CONDITION THREE WAYS AND v3 HAS ONE ERROR, so `as-observed` cannot be true of all three rows at once.** Two source sites produce three rendered messages: `Issue not found in OPEN: <ID>` and `Issue not found in CLOSED: <ID>` from `move_issue` (bin/intent_issues:285, `$from` varying), and `Issue not found: <ID>` with no bucket from `cmd_show` (:255). v3 has a single `FacadeError::NoSuchIssue { number }` -- `no issue {number:04} in this project` (facade.rs:114-115) -- which is bucket-agnostic on purpose and carries a remedy naming the bucket trap directly (`run intent issues list --kind all ...`, facade.rs:286-288). **That is a better answer than any of the three and it is still a deviation**, so it wants deciding rather than discovering as a diff later (cc, 2026-08-17, wiring Machine 4). **`target.state` is deliberately NOT moved off `as-observed` pending that call**: the condition, the exit code and the stream all match, and only the wording differs -- but `as-observed` should not be read as covering the text. Held for vc. `issues open` and `issues show` carry the same condition and point here; this is its one home.
- **no op:** `ok: issue <NNNN> already CLOSED`, exit 0 -- **SHIPPED at `b504d91b`, and this pair is the only place in the table where the v2 spelling and the ruled v3 voice coincide**, which is why hv's ruling cites it. `render.rs:1739-1741` (`fn already`). Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `issues open`

Reopen an issue: CLOSED -> OPEN

- **v2:** bin/intent_issues:314
- **Arguments:**
  - `id` (issue-id, arity `1`)
- **Exit codes:**
  - `0` -- reopened -- `ok: issue <ID> -> OPEN` (bin/intent_issues:291)
  - `0` -- ALREADY in the target bucket -- `ok: issue <ID> already OPEN`, exit 0 (bin/intent_issues:280-283). **This is v2's ONLY `already` arm and the one hv's self-loop ruling cites**; `move_issue` checks the SOURCE bucket, and on a miss checks the TARGET before erroring.
  - `1` -- in NEITHER bucket -- `error: Issue not found in CLOSED: <ID>` (bin/intent_issues:285). The message names the BUCKET, not just the id.
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: issue <ID> -> OPEN` on a move, `ok: issue <ID> already OPEN` on the self-loop
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Moves the issue directory back
- **Target:** `as-observed`
- **Open question for hv:** The v2-spells-it-three-ways / v3-has-one-`NoSuchIssue` deviation is recorded ONCE, on `issues close`. This row carries the condition and not a second copy of the reasoning.
- **no op:** `ok: issue <NNNN> already OPEN`, exit 0 -- **SHIPPED at `b504d91b`, and this pair is the only place in the table where the v2 spelling and the ruled v3 voice coincide**, which is why hv's ruling cites it. `render.rs:1739-1741` (`fn already`). Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

## Family: `todo`

A flat DOING / TODO / DONE view of steel threads and work packages

- **v2 source:** `bin/intent_todo`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 22 burning test(s) across 1 file(s) -- **covered**

- `intent/todo.md` is GENERATED from ST/WP status and is never hand-maintained. In v3 it is a generated view proper (WP-03), so the `update` verb becomes explicit regeneration rather than the thing that keeps it from going stale.
- The DONE bucket is watermarked: `done --flush` advances a `## DONE:<T>` marker so completed threads fall out of the view without being deleted. That watermark is authored state with no home in the reified model yet -- flagged, since it is neither thread data nor a pure view.
- **`todo list` was missing from this table until `drift_check.sh` found it.** The arms in `bin/intent_todo` sit at ZERO indent (`list)` at :384, not `  "list")`), which is the inconsistent-dispatch-formatting trap already recorded in the parity README -- one indentation-anchored regex scored the 1621-line `intent_st` at zero subcommands. The measured inventory had it; the authored table did not. That is the drift check earning its place on its first real run, in the direction that matters: the measurement was right and the judgement had a hole.

| command        | args        | flags            | help                                                           | disposition |
| -------------- | ----------- | ---------------- | -------------------------------------------------------------- | ----------- |
| `todo`         | [command]   | --json           | Show intent/todo.md (generates it if absent)                   | keep        |
| `todo list`    | --          | --json           | Show intent/todo.md (generates it if absent)                   | keep        |
| `todo update`  | --          | --               | Regenerate intent/todo.md from current status                  | keep        |
| `todo done`    | [specifier] | --flush, --prune | Mark a thread/WP done (via intent st/wp done), then regenerate | keep        |
| `todo notdone` | <specifier> | --               | Reopen a thread/WP to WIP, then regenerate                     | keep        |
| `todo toggle`  | <specifier> | --               | Flip done/not-done, then regenerate                            | keep        |

### `todo`

Show intent/todo.md (generates it if absent)

- **v2:** bin/intent_todo:384 (dispatch arms sit at zero indent here, which is what defeated one static enumerator)
- **Arguments:**
  - `command` (subcommand, arity `0..1`), default `list`
- **Flags:**
  - `--json` (bool) -- Emit the DOING/TODO/DONE view as JSON on stdout
    - **disposition:** keep
- **Exit codes:**
  - `0` -- bare -- prints the view, generating the file if absent
  - `1` -- `--help` prints 1077B usage to STDOUT and exits 1
  - `1` -- unknown verb -- `error: Unknown todo command: <v>. Run 'intent todo help' for usage.`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the DOING / TODO / DONE view
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - INV-07 at `todo --help`
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: Defers to INV-07, now `corrected`: `--help` exits 0 to stdout. The v3 binary already does this.
- **build measured:** BUILD HAS ALREADY DECIDED (ic measured, 2026-08-15). See INV-07: the v3 binary exits 0 on `--help` today, deliberately (`spine.rs:193` maps clap's DisplayHelp to EXIT_OK, and `spine.rs:27` names the divergence). This row asks whether to reproduce or correct, and the code has answered `corrected` without the contract recording it. Measurement, not a ruling.
- **ratified in:** parity.md
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review -- the classification disagrees with the verb name:** Bare `intent todo` is the default read of the whole tool and it inherits `list`'s generate-on-absent write.
- **MCP classification grounded in:** bin/intent_todo:380 (`COMMAND="${1:-list}"`) -- bare `todo` IS `todo list`
- **recoverability:** idempotent

### `todo list`

Show intent/todo.md (generates it if absent)

- **v2:** bin/intent_todo:384 -- the `list)` arm; `COMMAND="${1:-list}"` at :380 makes it the default
- **Flags:**
  - `--json` (bool) -- Emit the DOING/TODO/DONE view as JSON on stdout
    - **disposition:** keep
- **Exit codes:**
  - `0` -- printed, generating the file first if absent
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the DOING / TODO / DONE view, or its JSON form under --json
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** The EXPLICIT spelling of the family default. `intent todo` and `intent todo list` are the same code path, verified by invocation (exit 0).
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review -- the classification disagrees with the verb name:** `list` is THE read verb of this table and this one writes a file the first time it is called. It is also the worst shape for a bug: it reads on every run after the first, so the mutation is invisible in testing and appears on a fresh clone.
- **MCP classification grounded in:** bin/intent_todo:384-393 -- the else branch calls generate(), which does `mv "$tmp" "$TODO_FILE"` at :246
- **recoverability:** idempotent

### `todo update`

Regenerate intent/todo.md from current status

- **v2:** bin/intent_todo (update arm)
- **Exit codes:**
  - `0` -- regenerated
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Rewrites intent/todo.md
- **Target:** `as-observed`
- **Note:** In v3 this is a view regeneration on the WP-03 renderer, and the skew check (AC-03.4) makes a stale todo.md detectable rather than merely refreshable.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `todo done`

Mark a thread/WP done (via intent st/wp done), then regenerate

- **v2:** bin/intent_todo (done arm)
- **Arguments:**
  - `specifier` (st-id[/NN], arity `0..1`)
    - omitted when --flush or --prune is given
- **Flags:**
  - `--flush` (bool) -- Advance the DONE watermark to now, clearing the DONE view
    - **disposition:** retire
    - **disposition basis:** RETIRED by D44 (hv, 2026-08-16). Not a deprecation: the operation has no referent in v3. Both flags MUTATED the artefact because in v2 the artefact WAS the record; under D01 the db is the SSOT and `todo.md` is a generated view, so there is nothing to prune and nothing to flush -- only a question of how much of the record to SHOW, which is a display window and is a different flag. hv verbatim: `All of the data is in the db so we can (re)generate whatever we need when we need it.` Reclassified from `keep`, which was correct as a v2 observation and wrong as a v3 target the moment D01 landed.
  - `--prune` (bool) -- Emit the DONE items for archiving, then flush
    - **disposition:** retire
    - **disposition basis:** RETIRED by D44 (hv, 2026-08-16). Not a deprecation: the operation has no referent in v3. Both flags MUTATED the artefact because in v2 the artefact WAS the record; under D01 the db is the SSOT and `todo.md` is a generated view, so there is nothing to prune and nothing to flush -- only a question of how much of the record to SHOW, which is a display window and is a different flag. hv verbatim: `All of the data is in the db so we can (re)generate whatever we need when we need it.` Reclassified from `keep`, which was correct as a v2 observation and wrong as a v3 target the moment D01 landed.
- **Exit codes:**
  - `0` -- marked and regenerated
  - `1` -- specifier not resolvable
  - `1` -- close-gate BLOCKED, propagated from st/wp done
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation, or the pruned DONE items under --prune
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Delegates to `intent st done` / `intent wp done`, so the acceptance close-gate applies transitively.
- **Target:** `as-observed`
- **exposure:** EXPOSURE -- NOT a D42 one, and the first two versions of this note got that wrong (ic, audit 2026-08-15, corrected twice the same day). hv has narrowed D42 twice since I filed it: RETURNING a time is fine, and READING a clock to make a decision or to stamp when a command ran into a GENERATED artefact is fine too -- there is no need to be pathological about it. `--flush` is exactly that permitted case on both counts: v2 `flush_watermark` reads `date -u` and writes the instant into `todo.md`, which is a generated view, and the value is then used to decide DONE-bucket membership. So the clock read is legitimate and I withdraw the D42 complaint. WHAT SURVIVES IS A DIFFERENT AND BETTER DEFECT, found only by reading the v2 source to check the first claim: `generate()` reads the watermark BACK OUT of `todo.md` (`read_done_watermark`, bin/intent_todo:228), so the generated view is the watermark's ONLY store. Under the v3 truth model -- DB rebuildable, `rm` of a derived artefact always safe, md = generated views plus authored prose -- a watermark is neither, and deleting `todo.md` to regenerate it silently resets the flush to zero and resurrects every flushed item. The v3 watermark must live in the store. WP-03 owns the renderer and this is its constraint, not WP-06's.
- **no op:** `ok: <ID> already Completed`, exit 0 -- **SHIPPED at `d0f345b5`**, and **BYTE-IDENTICAL to `st done`'s, which is 0050's actual subject**: the wrapper and the call it delegates to now answer the same question the same way. Before that commit this row was the only arm in the renderer that reported a no-op at all, in the spelling `ok: <spec> was already done` -- honest about the no-op, wrong about the voice. Measured 2026-08-17 by driving the verb twice through the real binary (ic).
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `todo notdone`

Reopen a thread/WP to WIP, then regenerate

- **v2:** bin/intent_todo (notdone arm)
- **Arguments:**
  - `specifier` (st-id[/NN], arity `1`)
- **Exit codes:**
  - `0` -- reopened
  - `1` -- specifier not resolvable
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **no op:** **NO SELF-LOOP IS REACHABLE: the row refuses UNCONDITIONALLY, at exit 1, before any state is read.** Measured 2026-08-17 by driving the verb twice through the real binary (ic). `render.rs:1158-1168` returns the refusal for every invocation -- on a thread already in the target state, on one that would move, on any state at all. So this is not `unexamined` and not a no-op value: **there is no code path from this row to the facade**, and the `no_op` question cannot be asked of it until that changes. cc's reasoning is on the arm and it is sound: the ratified reopen is guarded `ReasonRecorded`, the declared surface for these two carries a specifier and nothing else, and the two alternatives are worse -- synthesising a reason puts a sentence nobody wrote into the permanent record, and bypassing the guard for one caller makes the machine advisory. **The comment routes the decision here by name**: whether these rows grow a reason argument or are withdrawn belongs to the table's owner. **v2 performs the write silently**, so this is a hard parity break either way, and the row currently ships a verb that cannot succeed.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

### `todo toggle`

Flip done/not-done, then regenerate

- **v2:** bin/intent_todo (toggle arm)
- **Arguments:**
  - `specifier` (st-id[/NN], arity `1`)
- **Exit codes:**
  - `0` -- flipped
  - `1` -- specifier not resolvable
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **no op:** **NO SELF-LOOP IS REACHABLE: the row refuses UNCONDITIONALLY, at exit 1, before any state is read.** Measured 2026-08-17 by driving the verb twice through the real binary (ic). `render.rs:1158-1168` returns the refusal for every invocation -- on a thread already in the target state, on one that would move, on any state at all. So this is not `unexamined` and not a no-op value: **there is no code path from this row to the facade**, and the `no_op` question cannot be asked of it until that changes. cc's reasoning is on the arm and it is sound: the ratified reopen is guarded `ReasonRecorded`, the declared surface for these two carries a specifier and nothing else, and the two alternatives are worse -- synthesising a reason puts a sentence nobody wrote into the permanent record, and bypassing the guard for one caller makes the machine advisory. **The comment routes the decision here by name**: whether these rows grow a reason argument or are withdrawn belongs to the table's owner. **v2 performs the write silently**, so this is a hard parity break either way, and the row currently ships a verb that cannot succeed.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** reversible

## Family: `info`

Show the Intent process overview and project status

- **v2 source:** `bin/intent_info`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 23 burning test(s) across 3 file(s) -- **covered**

- Single-action command: no dispatch `case`, no flags parsed at all.

| command | args | flags | help                                                | disposition |
| ------- | ---- | ----- | --------------------------------------------------- | ----------- |
| `info`  | --   | --    | Show the Intent process overview and project status | keep        |

### `info`

Show the Intent process overview and project status

- **v2:** bin/intent_info
- **Exit codes:**
  - `0` -- bare -- 595B to stdout
  - `0` -- `--help` -- IDENTICAL 595B output; the flag is not parsed, merely ignored
  - `0` -- unknown flag -- also 595B, exit 0
  - `0` -- outside a project -- 374B, exit 0; this command does NOT gate
- **stdout:** the overview (595B in a project, 374B outside one)
- **stderr:** --
- **Defects observed in v2:**
  - INV-08 at `intent info --zzz` succeeds silently at exit 0
- **Target:** `corrected` -- ratified: hv 2026-08-14 bounce (the `corrected` class); forced rather than chosen -- clap rejects unrecognised arguments by default -- behaviour: Unknown FLAGS and unknown POSITIONALS both refused, exit 1 per INV-02 -- clap rejects unrecognised arguments by default, which is what the ratification calls forced rather than chosen. HISTORY, kept because the mechanism recurs and this row is the only place it is written down: the row shipped the flag half ONLY, because its own `args[0]` declared a catch-all `ignored` slot (`type: any`, `arity: 0..n`) which instructed clap to accept every positional -- and clap then rendered that slot into the USER-FACING HELP as `Usage: intent info [IGNORED]...` with an empty description, so the slot was not merely inert, it was documented. The correction was defeated by a declaration on the same row. That slot had been carried forward from the v2 measurement in `observed` (where it is correct, and where it stays) into a v3 declaration (where it was the unfixed half), and nobody re-examined it -- the supersession-not-propagated class, same shape as `doctor`'s `read_or_mutate` outliving `--fix`. Measured 2026-08-17 BEFORE the change: `info --zzz` exit 1 at 41B, `info NOSUCHTHING` byte-identical to bare at exit 0 (606B both), `info --help` 149B rendering the `[IGNORED]...` slot. AFTER: `info NOSUCHTHING` exit 1 with `unexpected argument`, bare output byte-identical to before, help down to 107B with the slot gone. THE BOUND: vc's corrected differential sweep in issue 0044 probed 104 commands three ways each and found the unknown-positional arm swallowed by exactly ONE -- this row -- with the unknown-flag arm refused by all 104. So this is the last member of INV-08's positional half and the fix completes the class rather than chipping at it. **DO NOT CITE `flag refused: 103, pos refused: 103` FOR THIS.** That pair is 0044's RETRACTED first run, whose hash included `intent info`'s own `Location: <cwd>` line so no two runs could ever compare equal; the issue prints it as an example of what a dead instrument looks like, and ic cited it here as the bound for several hours before checking it at source. It also collides numerically with the corrected `populations.probeable` of 103, which is a different quantity again. Slot dropped 2026-08-17 by ic under vc's 03:48Z ruling.
- **ratified in:** parity.md
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_info -- no write primitive in the file

## Family: `config`

Display the resolved project configuration

- **v2 source:** `bin/intent_config`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 0 burning test(s) across 0 file(s) -- **HOLE -- nothing in the estate invokes it**

- **This command produces NO OUTPUT AT ALL in a project** -- 0 bytes on both streams, exit 0, measured. `bin/intent_config` is primarily a LIBRARY: `bin/intent:211` sources it for `load_intent_config`, and the executable path is close to vestigial.
- It is the clearest case in the surface of a command whose v3 shape is a decision rather than a port: `intent config` printing nothing is not a behaviour worth reproducing.

| command      | args          | flags | help                                       | disposition |
| ------------ | ------------- | ----- | ------------------------------------------ | ----------- |
| `config`     | [command]     | --    | Display the resolved project configuration | pending     |
| `config get` | <key>         | --    | Print one configuration value              | new-surface |
| `config set` | <key> <value> | --    | Set one configuration value                | new-surface |

### `config`

Display the resolved project configuration

- **v2:** bin/intent_config
- **Arguments:**
  - `command` (subcommand, arity `0..1`)
    - DECLARED AFTER A MEASURED DIVERGENCE (ic, 2026-08-16). This row carried NO args at all, and `config` was the only family in the table that did. `spine.rs`'s `build()` reads the arity off this slot and defaults an ABSENT slot to REQUIRED (`is_none_or(|slot| slot.arity == `1`)`), so v3 answered `intent config` with `requires a subcommand` at exit 1 while v2 exits 0 -- see this row's own `observed.exit`. Found by `implemented_check.sh`, which could not classify the row because clap turned the invocation away before dispatch. `0..1` with NO `default`, unlike `issues` and `todo`: those declare `default: list` because bare means run-the-list-verb, whereas bare `config` is its own action -- this row's help is `Display the resolved project configuration`, which is neither `get` nor `set`. Same shape as `llm`. The wider point is about the ABSENT declaration rather than this row: one missing slot was silently answered by a default nobody chose for it, and `config` was the only row exercising that default, so it was also the only evidence the default existed.
- **Exit codes:**
  - `0` -- bare -- ZERO bytes on both streams
  - `0` -- `--help` -- also zero bytes
  - `0` -- unknown flag -- also zero bytes, exit 0
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** nothing (0B)
- **stderr:** nothing (0B), except the project gate outside a project
- **Defects observed in v2:**
  - INV-08 at `intent config --zzz` succeeds silently at exit 0
  - Produces no output whatsoever in a project. A user cannot distinguish 'ran and printed the empty config' from 'did nothing'.
- **Target:** `undefined` -- ratified: vc ruling, 2026-08-14 -- the fifth parity class, opened on this entry; `config` is its first member -- behaviour: v3 prints the resolved project configuration. This is DESIGNED, not ported and not corrected.
- **Note:** v2 exhibits no behaviour here at all (0B on both streams, exit 0), so there is nothing to be faithful to and nothing to correct. Recording it as `corrected` would have hidden a design decision inside a bug-fix class, and design decisions need a different reviewer.
- **keys backup:**
  - `ratification`: D35 (hv, 2026-08-15) puts the backup schedule and retention in `intent/.config/config.json`, read through `intent config`. Key names are SURFACE, so ic names them and cc implements against these; cc was told not to invent them. Named 2026-08-15 to unblock AC-03.10.
  - `shape`: A nested `backup` object, on the existing `plugins` precedent -- config.json already groups a subsystem's settings under one object rather than flattening them behind a prefix. Single-word snake_case keys are the file's convention throughout (`intent_version`, `project_name`, `st_prefix`, `dft_width`, `intent_dir`).
  - `keys.0.key`: backup.enabled
  - `keys.0.type`: bool
  - `keys.0.default`: true
  - `keys.0.meaning`: Whether the DAEMON takes scheduled snapshots.
  - `keys.0.note`: DELIBERATELY DOES NOT GATE `intent backup`. A manual snapshot always runs, because the moment a user most wants one is the moment they have just discovered the schedule was off. One flag disabling both would turn a preference into a trap.
  - `keys.1.key`: backup.schedule
  - `keys.1.type`: string
  - `keys.1.default`: daily
  - `keys.1.values.0`: hourly
  - `keys.1.values.1`: daily
  - `keys.1.values.2`: weekly
  - `keys.1.meaning`: Base cadence. The daemon takes a snapshot when the newest one is older than this interval.
  - `keys.1.note`: ENUMERATED, NOT A CRON EXPRESSION, and that is a decision rather than a simplification. A cron string is a mini-language embedded in a hand-edited config file -- the shape behind the 0012 quoting scar -- and it is SILENTLY wrong when mistyped rather than refused. D35 fixes the retention tiers at day/week/month, so arbitrary cadences have nowhere to land anyway; a schedule coarser than a tier simply leaves that tier unfilled, which needs no special case.
  - `keys.2.key`: backup.retain.daily
  - `keys.2.type`: integer
  - `keys.2.default`: 7
  - `keys.2.meaning`: How many daily-tier snapshots to keep.
  - `keys.3.key`: backup.retain.weekly
  - `keys.3.type`: integer
  - `keys.3.default`: 4
  - `keys.3.meaning`: How many weekly-tier snapshots to keep. A snapshot enters this tier by being the newest of its ISO week.
  - `keys.4.key`: backup.retain.monthly
  - `keys.4.type`: integer
  - `keys.4.default`: 12
  - `keys.4.meaning`: How many monthly-tier snapshots to keep. A snapshot enters this tier by being the newest of its calendar month.
  - `keys.4.note`: Pruning removes any snapshot held by NO tier. `0` disables a tier explicitly; an ABSENT key means the default. Absence and zero must not be the same value in a retention policy, because one of them deletes backups -- the absence-as-meaning failure this toolchain keeps refusing, in the one place it costs data.
  - `deliberately_not_keys.0`: THE SNAPSHOT DIRECTORY -- fixed at `.backup/db/`. D35 requires DB snapshots to hold their own namespace under `.backup/`, because `intent upgrade` already writes `backup-<TIMESTAMP>/` rollback artefacts there and the two carry different retention rules. A configurable path is exactly how someone points the pruner at the upgrade namespace, making the collision this rule exists to prevent reachable through SUPPORTED configuration.
  - `deliberately_not_keys.1`: ANY SWITCH THAT SILENCES BACKUP FAILURE -- IN-AG-NO-SILENT-001 at its sharpest. This is the backup of the durable SSOT, and D35 records that the natural implementation (best-effort, on a timer, in a daemon nobody watches) is the one that fails silently. A key to turn the warning off MANUFACTURES that failure and gives it a supported name.
  - `resolved_by_hv`: ANSWERED 2026-08-15. I had declined to invent `config get` / `config set` and flagged the reading; hv ruled they ARE new surface and should exist. Rows authored on this family. **hv's accompanying caution is recorded because it is the more useful half: project configuration is USER-FACING surface, and must not be conflated with this repository's own dev/PM apparatus.** Intent dogfoods itself, so one `config.json` serves both roles HERE and nowhere else -- which is precisely what makes the two easy to confuse. A consumer installs `intent` from a tap and gets the command surface; they never get this repo's boards, sweeps or registers.
  - `carry_forward`: vc, 2026-08-15, to hold rather than act on: **if config ever enters the model the way the whiteboard did under D30, the setter question returns as a D32 question rather than a surface preference** -- _a state that can be entered and not left is a missing mutation, not a missing flag_. It is NOT a D32 question today, because `config.json` is project configuration and not model state. Recorded on the row so the trigger is attached to the thing it would change.
- **MCP:** not exposed -- read-only
- **Wants review:**
  - uncertain on `exposed_on_mcp`, `read_or_mutate`
  - The only row uncertain on BOTH fields, and the only row whose parity class is already `undefined`. `bin/intent_config` is dispatched to AND sourced as a library, and it carries a default-config writer (`mkdir -p` + `cat >` at :100-102) that I did not trace to the display path. Leaning mutate under the safe direction rather than guessing the call graph.
- **read or mutate correction:** CORRECTED 2026-08-16 (ic, on vc's finding while they were independently verifying my withheld-26 measurement). Was `mutate`, and it was the only row in the table contradicted by its own help string: `Display the resolved project configuration`, zero args, zero flags. **The convention is settled by the other roots and it is not `roots are reads`** -- there are 14 family roots, 12 `read` and 2 `mutate`, and `todo`'s `mutate` is CORRECT and documented on its own row, because bare `intent todo` inherits `list`'s generate-on-absent write. So a root is marked on its OWN behaviour rather than the union of its children, `todo` is the counter-example that proves the rule has teeth, and `config` was the only row breaking it. Its children are already the correct pair: `config get` is `read`, `config set` is `mutate`, leaving bare `config` no mutating role to play. **The caveat vc raised and could not resolve, resolved here: this row is `disposition: pending` with `target.state: undefined`, so `mutate` MIGHT have been a forward mark for a bare `config` that becomes an editor.** If it was, nobody recorded it -- and `undefined` means v3 is DESIGNING rather than porting, so the help string IS the design statement and the design displays. A field must not carry a forward mark for a decision that exists nowhere; if the design later makes bare `config` an editor, this field changes WITH that decision and cites it. **Worse today than yesterday, which is why it was sent rather than filed: the D45 reorder makes `read_or_mutate` the field an agent reads FIRST, so a zero-arg display command was being announced to agents as a mutation.** It also inflated a denominator I published -- `51 of 65 shipped mutations are exposed` counted a row that does not mutate.

### `config get`

Print one configuration value

- **v2:** new-surface
- **Arguments:**
  - `key` (config-key, arity `1`)
    - A DOTTED PATH addressing a nested value, eg `backup.retain.daily`. Worth stating because v2 cannot do this: `read_config_field` (bin/intent_helpers:75) is a FLAT `.[$key]` lookup, so v2 can read `project_name` and cannot read anything below the first level -- which is why every nested consumer in v2 rolls its own jq. v3 reads the path.
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15, answering D35's `configurable from intent config`: `config get` / `config set` are new surface and should exist.
- **unknown key refuses:** AN UNKNOWN KEY EXITS NON-ZERO AND SAYS SO. It must NOT print an empty line at exit 0, because empty is indistinguishable from a key legitimately set to empty -- the same absence-as-meaning collapse that makes an absent retention count different from `0`. A user scripting against this needs the two cases separable, and the only place that can be decided is here.
- **scope caution:** hv, 2026-08-15: this is USER-FACING project configuration. Intent dogfoods itself so this repo's own `config.json` is also a dev artefact, but that is a coincidence of the project and not a property of the command.
- **MCP:** exposed as an agent tool -- read-only

### `config set`

Set one configuration value

- **v2:** new-surface
- **Arguments:**
  - `key` (config-key, arity `1`)
  - `value` (string, arity `1`)
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: hv, 2026-08-15 -- see `config get`.
- **unknown key refuses:** *** THE LOAD-BEARING DECISION ON THIS ROW: AN UNKNOWN KEY IS REFUSED, NEVER CREATED. *** `intent config set backup.retian.daily 7` -- note `retian`, a TRANSPOSITION OF THE REAL KEY `backup.retain.daily` -- must fail and name the unknown key. **The near-miss is the whole point and a garbage key would not make it**: one transposed character produces a dead key that nothing reads, a config file that LOOKS configured, and a user who believes they set a retention policy they did not. The failure surfaces, if ever, as backups quietly pruned on defaults, long after the typo is forgotten. **The example spells out the misspelling because it failed on its first reader** -- an example whose point depends on the reader spotting a one-character difference is itself a silent failure. This is the whole class this toolchain keeps closing, sitting in the one command whose entire job is to be believed about what is configured.
- **known keys are derived:** THE VALID-KEY SET IS DERIVED FROM THE DECLARED CONFIG SCHEMA, NEVER A HAND-MAINTAINED LIST IN THE SETTER. A hand list is a designed figure: correct the day it is typed, silently wrong at the next key added, because the act that invalidates it (declaring a new setting) is not the act that updates it. Derived, a new key becomes settable the day it is declared and an unknown one stays refused for free. Same rule the drift check applies to new surface, pointed at configuration.
- **typed writes:** VALUES ARE WRITTEN WITH THEIR DECLARED TYPE, NOT AS STRINGS. `config set backup.enabled false` must write JSON `false`, not `"false"` -- a non-empty string is truthy nearly everywhere, so the string form would turn `disable the scheduled backup` into `enable it`, which is the worst available direction for that particular key. The type comes from the same schema that validates the key, so a value that cannot be coerced is a refusal rather than a cast.
- **not model state:** `config.json` is project configuration, NOT model state, so this writes the file rather than mutating through the store -- and D32's every-state-is-leavable reasoning does not apply. vc's carry-forward on the `config` row above records the trigger that would change that: if configuration ever enters the model the way the whiteboard did under D30, this becomes a D32 question rather than a surface one.
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Leaned OPEN against the standing lean, so it wants a look. hv ruled config is legitimate per-project surface; that ruling was about the keys existing, not about who may write them. An agent that can write config can turn `backup.enabled` off.
- **recoverability:** reversible

## Family: `init`

Initialize a new Intent project in the current directory

- **v2 source:** `bin/intent_init`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 55 burning test(s) across 7 file(s) -- **covered**

- Templates are read from `lib/templates/` at `INTENT_HOME`. In v3 they are embedded in the binary (WP-07, rust-embed), which removes the whole class of broken-install failure this command currently has to report.

| command | args           | flags                                   | help                                                     | disposition |
| ------- | -------------- | --------------------------------------- | -------------------------------------------------------- | ----------- |
| `init`  | [project_name] | --with-st0000, --lang <list>, --help/-h | Initialize a new Intent project in the current directory | keep        |

### `init`

Initialize a new Intent project in the current directory

- **v2:** bin/intent_init
- **Arguments:**
  - `project_name` (string, arity `0..1`), default `the current directory name`
- **Flags:**
  - `--with-st0000` (bool) -- Bootstrap all ST0000 deliverables after init
    - **disposition:** keep
  - `--lang` `<list>` (string) -- Comma- or space-separated languages to install canon for
    - Accepts: eg `--lang elixir` or `--lang elixir,rust,shell`
    - **disposition:** keep
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- created
  - `1` -- already an Intent project -- `error: This directory is already an Intent project`
  - `1` -- `--help` prints usage to STDOUT and exits 1
  - `1` -- unknown flag
- **stdout:** progress lines and the created layout
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Writes intent/.config/config.json, the intent/ tree, and root canon (AGENTS.md, CLAUDE.md, usage-rules.md)
- **Defects observed in v2:**
  - INV-07 at `init --help`
- **Target:** `as-observed`
- **Note:** v3 additionally stamps `project_id` (the D15 cloud seam) at init and at migration.
- **MCP:** not exposed -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Creating a project is a human's decision about a directory, not a task step.
- **recoverability:** one-way

## Family: `bootstrap`

First-time setup: create global Intent configuration

- **v2 source:** `bin/intent_bootstrap`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 16 burning test(s) across 2 file(s) -- **covered**

- Runs OUTSIDE a project by design (measured: exit 0, 982B). It is one of the global commands.
- Its own usage block says `Usage: intent_bootstrap [OPTIONS]` and `Initial setup for Intent v2.0.0` -- it names the underlying script rather than the `intent bootstrap` the user typed, and the version is nine minors stale. Both retire when help is generated from this table.

| command     | args | flags                             | help                                                 | disposition |
| ----------- | ---- | --------------------------------- | ---------------------------------------------------- | ----------- |
| `bootstrap` | --   | --force/-f, --quiet/-q, --help/-h | First-time setup: create global Intent configuration | keep        |

### `bootstrap`

First-time setup: create global Intent configuration

- **v2:** bin/intent_bootstrap
- **Flags:**
  - `--force`, `-f` (bool) -- Force recreation of config even if it exists
    - **disposition:** keep
  - `--quiet`, `-q` (bool) -- Suppress informational output
    - **disposition:** pending
    - **disposition basis:** VERBOSITY CLUSTER. Same undecided design question as `doctor --quiet`, and it is one question, not four: does v3 carry per-command verbosity flags or one global pair? Classified together so the answer lands in one place instead of being re-litigated per command.
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- bare -- 1161B in a project, 982B outside one
  - `0` -- `--help` -- 399B usage, exit 0
  - `1` -- unknown flag -- `Unknown option: --zzz-not-a-flag`
- **stdout:** setup progress and instructions
- **stderr:** the unknown-option message
- **Defects observed in v2:**
  - INV-01 at the unknown-option message is `Unknown option: ...` with NO `error:` prefix
  - Usage block says `intent_bootstrap` and `Intent v2.0.0` -- names the script rather than the command, and the version is stale by nine minors.
- **Target:** `pending-hv`
- **Open question for hv:** The missing `error:` prefix is an INV-01 violation and a candidate `corrected` member. Same shape as `doctor` and `fileindex`.
- **MCP:** not exposed -- **mutates**
- **recoverability:** one-way

## Family: `doctor`

Diagnose common Intent configuration issues

- **v2 source:** `bin/intent_doctor`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 11 burning test(s) across 1 file(s) -- **covered**

- Runs outside a project (measured: exit 0, 397B), so it is a global command.
- v3 gains two checks that have no v2 antecedent because they are consequences of the new truth model: the SKEW check (a hand-edited generated view, AC-03.4) and the UNPARSED state (AC-03.5). Both are additions, not deviations.

| command  | args | flags                                         | help                                        | disposition |
| -------- | ---- | --------------------------------------------- | ------------------------------------------- | ----------- |
| `doctor` | --   | --fix/-f, --verbose/-v, --quiet/-q, --help/-h | Diagnose common Intent configuration issues | keep        |

### `doctor`

Diagnose common Intent configuration issues

- **v2:** bin/intent_doctor
- **Flags:**
  - `--fix`, `-f` (bool) -- Attempt to fix issues automatically
    - **disposition:** retire
    - **disposition basis:** vc measured bin/intent_doctor:66 -- v2 really implements --fix, so this is a genuine v2 behaviour v3 is deliberately not carrying. That is the distinction `retire` exists to draw, and AC-06.9 was missing it.
  - `--verbose`, `-v` (bool) -- Show detailed information
    - **disposition:** pending
    - **disposition basis:** vc measured `verbose()` at bin/intent_doctor:133 -- implemented in v2, undecided for v3. Part of the verbosity cluster below.
  - `--quiet`, `-q` (bool) -- Only show errors and warnings
    - **disposition:** pending
    - **disposition basis:** vc measured colour suppression under quiet at bin/intent_doctor:91 -- implemented in v2, undecided for v3. Part of the verbosity cluster below.
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- bare -- 563B, exit 0 whether or not findings exist
  - `0` -- `--help` -- 384B, exit 0
  - `1` -- unknown flag -- `Unknown option: --zzz-not-a-flag`
- **stdout:** check results (checks 1..4e)
- **stderr:** the unknown-option message
- **Defects observed in v2:**
  - INV-01 at the unknown-option message has NO `error:` prefix
  - Exits 0 even with findings, so it cannot be used as a CI gate as it stands. Usage block also names `intent_doctor` rather than `intent doctor`.
- **Target:** `pending-hv`
- **Open question for hv:** Should `doctor` exit non-zero when it finds something? v2 does not, so a CI job cannot gate on it. Changing it is `corrected`; leaving it means the skew and unparsed checks v3 adds are reportable but not enforceable.
- **new obligations:**
  - STATUS-VS-GATE DISAGREEMENT (hv, 2026-08-15, ratifying the state machines): `doctor` reports any unit whose status disagrees with its gate. Refusal on the way in is not enough on its own -- a status that was TRUE when it was set becomes a false green the moment its contract grows, which is how three of five WPs came to disagree with their own gates while every one of them had been closed legitimately.
  - BACKUP STALENESS (vc, 2026-08-15, amending AC-03.10 after ic's `--list` question): `doctor` reports the newest snapshot's age against the configured `backup.schedule`. AC-03.10(d) had said only that a failed backup surfaces -- but A SCHEDULE THAT NEVER FIRES PRODUCES NO FAILURE TO REPORT, so a green implementation could ship where nothing had ever run. Staleness is the two-sided test: it detects never-ran WITHOUT needing anything to have failed, the same construction as the clock guard's check C, which compares two stamps to each other and needs no clock at all.
- **question sharpened by them:** BOTH obligations above make the open exit-code question materially heavier, and this is new evidence for a decision hv has not yet made. When `doctor` only reported configuration tidiness, `reportable but not enforceable` was a mild gap. It now detects (a) tracking data that is actively lying about whether work is finished and (b) a backup of the DURABLE SSOT that may never have run. Those are not advisory findings. A check that cannot fail a CI job is one nobody is obliged to read -- which is the unwired-guard shape dc measured: VISIBLE IS NOT CLOSED.
- **MCP:** exposed as an agent tool -- read-only
- **Wants review -- the classification disagrees with the verb name:** `doctor` is a diagnostic in every other tool that ships one, and v3's IS one -- this row is `read`. It was `mutate` until 2026-08-16, on reasoning that was correct about a command that no longer exists.
- **MCP classification grounded in:** v3: `Facade::doctor(project, ctx, store: Option<&Store>)` ACCEPTS a store and never creates one, so a generate-on-absent write is impossible at the type level (vc's structural proof, stronger than either argument that preceded it); `doctor_changes_nothing_it_looks_at` asserts the reported file is not rewritten and that a second run reports identically. v2: bin/intent_doctor:66 (`-f|--fix`), :216 and :308 both `mv` real files.

## Family: `upgrade`

Upgrade an Intent project to the current version

- **v2 source:** `bin/intent_upgrade`
- **v2 help file:** none
- **Owning work package:** WP-10
- **BATS coverage:** 4 burning test(s) across 1 file(s) -- **covered**

- **REPLACED, not ported.** D09's two-hop policy: v2's own upgrade ledger is never reimplemented in Rust. A project below the v2.19.0 floor runs v2's `intent upgrade` first, then the v3 migrator takes it from there. The two `intent_migrations_*` BATS files retire by design for the same reason.
- `bin/intent_migrations` is mode 644 and is NOT a command -- it exists only to be sourced by the orchestrator, and `bin/intent_help`'s auto-list requires `-x`, so it is correctly absent from help. There is no `intent migrations` in the surface.

| command   | args | flags                                      | help                                             | disposition |
| --------- | ---- | ------------------------------------------ | ------------------------------------------------ | ----------- |
| `upgrade` | --   | --backup-dir <dir>, --no-backup, --help/-h | Upgrade an Intent project to the current version | keep        |

### `upgrade`

Upgrade an Intent project to the current version

- **v2:** bin/intent_upgrade
- **Flags:**
  - `--backup-dir` `<dir>` (string) -- Custom backup directory name
    - **default:** .backup/backup-TIMESTAMP
    - **disposition:** pending
    - **disposition basis:** PENDING, ic 2026-08-16. This flag carried `retire` on the basis "Inherited from the entry: a retired command never reaches clap, so neither can its flags" -- and the entry is no longer `retire` (see the target block: `upgrade` is v3's migration door, migration.md:3). **The basis was the whole justification, so correcting the entry left the value unsupported rather than merely stale.** It was never decided on its own merits. Not defaulted to `keep`: v3's migration rolls back through git (AC-10.6 exercises `git revert`, tree-identical), so v2's backup-directory flags may not apply at all -- which is a judgement nobody has made. `pending` does not ship, so nothing is advertised while it is open.
  - `--no-backup` (bool) -- Skip backup creation (dangerous)
    - **disposition:** pending
    - **disposition basis:** PENDING, ic 2026-08-16. This flag carried `retire` on the basis "Inherited from the entry: a retired command never reaches clap, so neither can its flags" -- and the entry is no longer `retire` (see the target block: `upgrade` is v3's migration door, migration.md:3). **The basis was the whole justification, so correcting the entry left the value unsupported rather than merely stale.** It was never decided on its own merits. Not defaulted to `keep`: v3's migration rolls back through git (AC-10.6 exercises `git revert`, tree-identical), so v2's backup-directory flags may not apply at all -- which is a judgement nobody has made. `pending` does not ship, so nothing is advertised while it is open.
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- bare -- 78B, `Current version: 2.19.0`
  - `0` -- `--help` -- 593B, exit 0
  - `1` -- unknown flag -- `error: Unknown option: ...`
  - `1` -- outside a project -- supplies its OWN message, not the standard gate (INV-03 exception)
- **stdout:** the convergent ledger walk
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Convergent since ST0043: it probes on-disk state, runs only the steps still needed, applies canon via `intent claude upgrade --apply`, and stamps the version once at the end. Safe to re-run after an interruption.
- **Target:** `deviate` -- ratified: vc ruling 2026-08-16: `disposition: keep` (migration.md:3 -- the migrator IS the v3 binary's `intent upgrade`) + `target.state: deviate` citing D09. No new D-number: D09 already covers it.
- **why the old ratification was wrong:** It cited D09, and D09 is one line -- `Migration floor v2.19.0, two-hop` -- which says nothing about the command. What is retired is v2's LEDGER (`the v2 ledger is never reimplemented in Rust`, migration.md:7), the version-walk mechanism, not the verb. `migration.md:3` is explicit in the opposite direction: **`The migrator is the v3 binary's intent upgrade detecting a v2 project`**. So `upgrade` is v3's migration DOOR and keeps its spelling. My own `behaviour` field on the old row named `intent ingest --from-md` as the engine, and I read the ENGINE's succession as the DOOR's retirement: the ingest engine replaces the ledger, the command survives.
- **corroborated by the binary:** v3's unmigrated-project refusal already emits `run intent upgrade to migrate this project to Intent v3`, and AT-06.11 is held red `until WP-10 lands upgrade`. Both were right and this table was the odd one out -- which is why the defect first presented as an AC-06.11 violation, a remedy naming a command the binary cannot do. The remedy was correct and the SURFACE was wrong.
- **state is pending deliberately:** Not a guess dressed as a decision. `keep` is carried by migration.md:3. But WHAT v3's `upgrade` does differs from v2's -- a one-hop v2->v3 migration rather than a convergent version ladder -- and a deliberate behaviour change flowing from v3's design is what `deviate` means in the register vocabulary. `target_states` has no `deviate`, so no value in it can say this. Classifying it is a parity call for vc or hv; `pending-hv` is this file's declared honest blank.
- **why D09 after all:** ic corrected this row off `retire` and then could not find a ratification for what replaces it, having written that D09 `says nothing about the command`. **It says nothing about the command's EXISTENCE and everything about its BEHAVIOUR, and those are two different fields.** `disposition` asks whether the v2 command survives -- migration.md:3 answers it. `target.state` asks what v3 DOES -- D09 answers it: floor at v2.19.0, two-hop, the v2 ledger never reimplemented. A command that refuses below a floor and tells you to install `intent@2` first is precisely a deliberate behaviour change flowing from v3's design. **ic went looking for the command in D09 and found nothing because D09 answers the other question** -- the third instance this week of reading a fact against the wrong one of these two fields, after the three rows carrying `disposition: corrected` that were `keep` all along.
- **MCP:** not exposed -- **mutates**
- **recoverability:** one-way

## Family: `organize`

Organize steel threads into status directories based on their metadata

- **v2 source:** `bin/intent_organize`
- **v2 help file:** none
- **Owning work package:** --
- **BATS coverage:** 3 burning test(s) across 1 file(s) -- **covered**

- **A live Highlander violation, registered in the very file that exists to prevent it.** MODULES.md gives `bin/intent_organize` this job AND gives `bin/intent_st` an `organize` verb. Measured against one thread in a fresh project the two share no output: the top-level form prints `ok: moved 0, kept 0` plus per-directory counts (117B), the `st` form prints `Already organized: ST0001 in intent/st/NOT-STARTED` (73B).
- **`intent organise` is NOT a top-level alias** -- it answers `error: Unknown command 'organise'`. The alias exists only one level down, at `intent st organise` (normalised at bin/intent_st:289-292). Both measured.
- The flags differ between the two faces too: this one takes `--dry-run` (dry by request), `st organize` takes `--write` (dry by default). Opposite polarity for the same operation.

| command    | args | flags                | help                                                                   | disposition |
| ---------- | ---- | -------------------- | ---------------------------------------------------------------------- | ----------- |
| `organize` | --   | --dry-run, --help/-h | Organize steel threads into status directories based on their metadata | retire      |

### `organize`

Organize steel threads into status directories based on their metadata

- **v2:** bin/intent_organize
- **Flags:**
  - `--dry-run` (bool) -- Preview changes without making them
    - OPPOSITE polarity to `st organize --write`: this face acts by default, that one previews by default
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- bare -- 117B
  - `0` -- `--help` -- 575B, exit 0
  - `1` -- unknown flag
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** `ok: moved N, kept M` plus per-directory counts
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `retire` -- ratified: hv, 2026-08-14 -- organize (both faces) is planned vestigial by construction; a strictly structured model cannot hold data in the wrong spot or the wrong format, so the disorder this repairs cannot arise. Confirmed finally at the surface cut (WP-05/06).
- **Note:** Retiring both faces dissolves the Highlander violation rather than resolving it -- nobody has to choose which implementation was right.
- **MCP:** not exposed -- **mutates**
- **Cross-reference:** `st organize` is the other face; see the `st` family.

## Family: `agents`

Manage AGENTS.md -- the primary tool-agnostic LLM config at project root

- **v2 source:** `intent/plugins/agents/ (plugin command)`
- **v2 help file:** none
- **Owning work package:** WP-07
- **BATS coverage:** 86 burning test(s) across 4 file(s), plus 1 file(s) that name it but never reach the CLI -- **covered**

- **parity.md's table said this family was just `sync`. Measured: five verbs** -- init, generate, sync, validate, template.
- A PLUGIN command, so it execs before the project check (bin/intent:188-191) and runs outside a project (measured: exit 0, 984B) despite not being in GLOBAL_COMMANDS. That is INV-03's second exception.

| command           | args                | flags             | help                                            | disposition |
| ----------------- | ------------------- | ----------------- | ----------------------------------------------- | ----------- |
| `agents`          | [command]           | --                | Manage AGENTS.md for Intent projects            | keep        |
| `agents init`     | --                  | --template <name> | Initialize AGENTS.md at project root            | keep        |
| `agents generate` | --                  | --                | Emit generated AGENTS.md content to stdout      | keep        |
| `agents sync`     | --                  | --                | Regenerate AGENTS.md from current project state | keep        |
| `agents validate` | --                  | --                | Validate AGENTS.md shape and required sections  | keep        |
| `agents template` | [subcommand] [name] | --                | Manage AGENTS.md templates                      | keep        |

### `agents`

Manage AGENTS.md for Intent projects

- **v2:** intent/plugins/agents/ dispatch
- **Arguments:**
  - `command` (subcommand, arity `0..1`)
- **Exit codes:**
  - `0` -- bare -- prints 984B usage, exit 0
  - `0` -- `--help` -- same 984B, exit 0
  - `1` -- unknown verb -- 1018B on STDOUT, `Unknown command: ...`, exit 1
- **stdout:** the usage block
- **stderr:** --
- **Defects observed in v2:**
  - INV-06 at the unknown-verb error goes to STDOUT, not stderr
  - INV-01 at `Unknown command: ...` carries no `error:` prefix
- **Target:** `pending-hv`
- **Open question for hv:** Both defects are INV-01/INV-06 members awaiting the same scope ruling.
- **MCP:** not exposed -- read-only

### `agents init`

Initialize AGENTS.md at project root

- **v2:** plugin dispatch
- **Flags:**
  - `--template` `<name>` (string) -- Initial content from a named template
    - **disposition:** keep
- **Exit codes:**
  - `0` -- created
  - `1` -- already exists
- **stdout:** confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- **mutates**
- **MCP classification grounded in:** intent/plugins/agents/bin/intent_agents:196-216 -- cp of AGENTS.md / RULES.md / ARCHITECTURE.md
- **recoverability:** one-way

### `agents generate`

Emit generated AGENTS.md content to stdout

- **v2:** plugin dispatch
- **Exit codes:**
  - `0` -- emitted
- **stdout:** the generated AGENTS.md content
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Pure emit-path -- writes nothing. The composable half of `sync`.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `agents sync`

Regenerate AGENTS.md from current project state

- **v2:** plugin dispatch
- **Exit codes:**
  - `0` -- regenerated
- **stdout:** **THREE lines, measured 2026-08-17 against v2.19.0 with the streams split (130 bytes, stderr 0 bytes):** `Syncing AGENTS.md with latest project state...` / `ok: AGENTS.md updated at project root.` / `    previous version saved to AGENTS.md.bak` (four-space indented continuation). The row previously said `confirmation`, which is true and says nothing a builder could reproduce.
- **stderr:** EMPTY on the success arm -- 0 bytes, measured. No failure arm was reached.
- **Side effects:**
  - Rewrites AGENTS.md at project root
  - Writes AGENTS.md.bak beside it -- the previous version. UNDECLARED until 2026-08-17 and announced by line 3 of stdout. Covered by `*.bak` at `.gitignore:51`, so it does not reach git.
- **Observed notes:** **THIS VOICE IS RECORDED AND MUST NOT BE TIDIED, and the row is where a v3 implementer will read it.** Two of the three lines are against the house style that 0023 spent a release enforcing: line 1 is a BARE CAPITALISED progress line with no `ok:` prefix and a trailing ellipsis, and line 2 carries a TRAILING FULL STOP. Both are v2's observed behaviour on an `as-observed` row, so **reproducing them IS the contract** -- a v3 that cleans them up has broken parity, however much better it reads. A deviation here wants RATIFYING, not fixing. **A third line existed that nobody had recorded**, which is why the warning is written from the measurement rather than from memory of it. **No `stdout_exact` yet, deliberately: the `agents` family is UNBUILT in v3** (exit 2, `error: agents is a known command that is not implemented yet`). A declaration now would red for not-built rather than for a parity break, and the whole point of splitting the as-observed and corrected legs was that a red must name one remedy. The bytes above are measured and ready; declare them when the family ships. AGENTS.md is THE proven generated-committed-view precedent that D04 generalises to info.md, acceptance.md, steel_threads.md and todo.md.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **MCP classification grounded in:** intent/plugins/agents/bin/intent_agents:693 -- backs up AGENTS.md then rewrites it
- **recoverability:** idempotent

### `agents validate`

Validate AGENTS.md shape and required sections

- **v2:** plugin dispatch
- **Exit codes:**
  - `0` -- valid
  - `1` -- findings
- **stdout:** findings
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `agents template`

Manage AGENTS.md templates

- **v2:** plugin dispatch
- **Arguments:**
  - `subcommand` (subcommand, arity `0..1`), default `list` -- one of: `list`, `show`
  - `name` (string, arity `0..1`)
- **Exit codes:**
  - `0` -- listed (bare or `list`), or `show <name>` where the template dir holds an AGENTS.md
  - `1` -- `show` with no name -- `error: template name required` + a `Usage:` line, BOTH ON STDOUT
  - `1` -- `show <name>` where the dir has no AGENTS.md -- `error: template '<name>' not found` ON STDOUT; true for SEVEN of the eight names `list` prints (0058)
  - `1` -- any other subcommand -- `Unknown template subcommand: <x>` + a `Usage:` line, ON STDOUT and with NO `error:` prefix (0058)
- **stdout:** **FOUR ARMS, ONE ROW, and the schema cannot yet say so (0058).** `list` (and the bare invocation, byte-identical) prints `Available AGENTS.md templates:` then `  - <name>` per DIRECTORY under the plugin's `templates/`. `show <name>` cats `templates/<name>/AGENTS.md`. The two failure arms print to stdout too -- see `exit`. **No `stdout_exact` is declared: the list arm's content is the plugin's template DIRECTORY LISTING, so a literal template would assert that v3 reproduces v2's plugin layout, which is a different claim from reproducing its output.**
- **stderr:** **NOTHING. STDERR IS EMPTY ON EVERY ARM, INCLUDING BOTH FAILURES -- an INV-01 violation, measured (0058).** This field previously read "`error: ...` on stderr (INV-01)", a compliance claim nothing had checked, so anything auditing INV-01 from the register rather than from the binary passed this row. Verified by splitting the streams: `intent agents template rust 2>&1 1>/dev/null` prints nothing.
- **Observed notes:** **`show` was absent from `values` until 2026-08-17 and v3's clap surface renders from this row, so the subcommand was on course to be dropped silently (0058).** v2's own usage line names it: `Usage: intent agents template [list|show <name>]`. **`default: "list"` was also absent** -- `intent_agents:769` is `local subcommand="${1:-list}"`, and the bare invocation is byte-identical to `list`, measured. **AND `list` ADVERTISES SEVEN NAMES `show` CANNOT DELIVER**: `list` enumerates directories (`-d`), `show` requires `<dir>/AGENTS.md` (`-f`), and only `elixir/` has one -- the other seven hold the ST0035 per-language canon packs (`ARCHITECTURE.md` + `RULES.md`). `list`'s subject is "directories under templates/" and `show`'s is "directories containing AGENTS.md"; the output cannot tell them apart. `list` did not break -- its meaning was changed underneath it by an unrelated feature and nothing re-read it.
- **Target:** `corrected` -- ratified: ic 2026-08-17, on vc's split -- **THE CLASS IS DECIDABLE AND THE REMEDY IS NOT, AND ONLY THE CLASS WAS EVER MINE.** The test for `corrected` is whether the v2 behaviour is simply wrong, and `list` printing `rust` while `show rust` answers `not found` IS wrong -- not as an implementation error but as an OUTPUT MAKING A FALSE CLAIM ABOUT ITS SIBLING. How it came to be wrong is a question about blame; `corrected` is a question about the behaviour. Reproducing it would launder a v2 defect into a v3 requirement, which `parity.md` forbids in as many words. **Ruling the class NOW is the point rather than a formality**: `as-observed` left the row BUILDABLE AS-IS, so the one outcome nobody wants was the one the register was licensing while the question sat open. All three candidate remedies satisfy `corrected`, so nothing waits on the remedy being chosen. **The INV-01 half needs no ruling from anyone** -- errors on stdout, one with no `error:` prefix, is `corrected` on its face.
- **open question:** **THE REMEDY, AND IT IS hv's -- NOT THE CLASS, WHICH IS RULED ABOVE.** Three answers exist and they differ in what the command DOES, not in how it speaks: narrow `list` to directories carrying an `AGENTS.md`; widen `show` to serve the ST0035 canon packs; or split the namespace so the two populations stop sharing a directory. **That is past what the five voices were convened for and past vc's standing grant, which covers folding a class and not deciding what a command enumerates.** v3 must not reproduce the false claim; which of the three it does instead is open.
- **ratified in:** ic, 2026-08-17, class ruled on vc's split; measurement and argument recorded on issue 0058 at commit `632d9d86`. Authority, date and an external record, per vc's `ratified_in` grammar.
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** intent/plugins/agents/bin/intent_agents:768-801 -- `list` reads the templates dir, `show` cats one file; no write primitive on any arm, so `read` is now grounded rather than assumed

## Family: `claude`

Claude Code integration: subagents, skills, rules, hooks, workstreams

- **v2 source:** `bin/intent (claude arms) + intent/plugins/claude/`
- **v2 help file:** lib/help/rules.help.md (documents `claude rules`, misfiled as a top-level command)
- **Owning work package:** WP-07
- **BATS coverage:** 296 burning test(s) across 30 file(s), plus 2 file(s) that name it but never reach the CLI -- **covered**

- The largest family in the surface and the only one needing an explicit arm in `bin/intent` -- every other `bin/intent_<name>` auto-dispatches via the `*)` default case.
- **`intent claude` bare, `--help`, an unknown flag and outside-a-project all produce the SAME 189B error** (`error: Unknown claude subcommand. Try: ...`). Four distinct conditions, one message: a user who typed `--help` is told they used an unknown subcommand. It also means the family never reaches the project gate (INV-03's first exception).
- **`intent claude rules` bare does not print usage -- it LISTS rules**, defaulting to the `list` verb. Measured.
- `claude hook <name>` must stay byte-compatible on day one (parity.md): issue 0016's runtime-resolved hooks plus byte-identical settings.json is what makes the v2-to-v3 binary swap invisible at the consumer hook layer. It propagates the hook's own exit code, including 2, by design (INV-04).

| command            | args             | flags         | help                                            | disposition |
| ------------------ | ---------------- | ------------- | ----------------------------------------------- | ----------- |
| `claude`           | <subcommand>     | --            | Claude Code integration                         | keep        |
| `claude subagents` | <verb> [name]... | -v            | Manage Claude Code subagents                    | keep        |
| `claude skills`    | <verb> [name]... | -v            | Manage Claude Code skills                       | keep        |
| `claude rules`     | [verb] [id]      | --lang <lang> | List and show rule-library rules                | keep        |
| `claude hook`      | <name>           | --            | Run a named Intent hook                         | keep        |
| `claude upgrade`   | --               | --apply       | Apply Claude canon to the project               | keep        |
| `claude prime`     | --               | --            | Generate MEMORY.md content for a Claude session | keep        |
| `claude ws`        | <verb> [wsid]    | --            | Manage whiteboard workstreams                   | keep        |
| `claude start`     | <ws>             | --            | Launch a Claude session bound to a workstream   | keep        |

### `claude`

Claude Code integration

- **v2:** bin/intent (explicit `claude` arm)
- **Arguments:**
  - `subcommand` (subcommand, arity `1`) -- one of: `subagents`, `skills`, `rules`, `hook`, `upgrade`, `prime`, `ws`, `start`
- **Exit codes:**
  - `1` -- bare / `--help` / unknown flag / outside a project -- all FOUR produce the identical 189B `error: Unknown claude subcommand. Try: ...`
- **stdout:** --
- **stderr:** the 189B subcommand list (INV-01 voice, correct stream)
- **Defects observed in v2:**
  - INV-07 at `claude --help` answered as an unknown subcommand
  - Four distinct conditions collapse to one message, so the error cannot tell the user which mistake they made. This is the same-text-for-different-causes collapse AC-04.4 forbids in v3.
- **Target:** `as-observed` -- ratified: AC-04.4 -- every facade error is typed and renders a remedy with its full cause chain, with no same-text-for-different-causes collapses
- **Note:** **Governed, not unexamined.** The four-conditions-one-message collapse is already forbidden by AC-04.4, so this row needs no separate hv ruling: v3 cannot reproduce it and stay inside its own contract. Cited here so the next reader sees it was decided rather than missed. (vc, 2026-08-14.)
- **MCP:** not exposed -- read-only

### `claude subagents`

Manage Claude Code subagents

- **v2:** bin/intent claude arm -> plugin
- **Arguments:**
  - `verb` (subcommand, arity `1`) -- one of: `init`, `list`, `install`, `sync`, `uninstall`, `show`, `status`
  - `name` (string, arity `0..n`)
- **Flags:**
  - `-v` (bool) -- Show full descriptions in `list`
    - **disposition:** keep
- **Exit codes:**
  - `0` -- listed / installed / synced
  - `1` -- unknown verb
  - `1` -- named subagent not found
- **stdout:** the subagent table or detail
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** `intent/plugins/claude/subagents/.manifest/` tracks global-agents.json but NOT its sibling installed-agents.json, and .gitignore names neither, so running `install` inside a project leaves a permanent untracked file holding absolute machine paths. Pre-existing; wants an issue.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `claude skills`

Manage Claude Code skills

- **v2:** bin/intent claude arm -> plugin
- **Arguments:**
  - `verb` (subcommand, arity `1`) -- one of: `list`, `install`, `sync`, `uninstall`, `show`
  - `name` (string, arity `0..n`)
- **Flags:**
  - `-v` (bool) -- Show full descriptions in `list`
    - **disposition:** keep
- **Exit codes:**
  - `0` -- listed / installed / synced
  - `1` -- unknown verb
  - `1` -- named skill not found
- **stdout:** the skill table or detail
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** `sync` checksums SKILL.md ONLY, so a change confined to a skill's scripts/ does not propagate -- it needs `install --force` (or touching SKILL.md). Known trap, worth not reproducing in v3.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `claude rules`

List and show rule-library rules

- **v2:** bin/intent claude arm
- **Arguments:**
  - `verb` (subcommand, arity `0..1`), default `list` -- one of: `list`, `show`, `validate`, `index`
  - `id` (rule-id, arity `0..1`)
- **Flags:**
  - `--lang` `<lang>` (string) -- Filter to one language
    - **disposition:** keep
- **Exit codes:**
  - `0` -- listed / shown
  - `1` -- unknown rule id
- **stdout:** the rule table, or one rule's full text
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Bare `intent claude rules` LISTS rather than printing usage -- measured.
- **Defects observed in v2:**
  - `intent claude rules index` MUTATES `INTENT_HOME` -- it rewrote a tracked file (intent/plugins/claude/rules/index.json) in the worktree under test. A verb that reads like a query modifies the installation.
- **Target:** `pending-hv`
- **Open question for hv:** In v3 rules are embedded in the binary (WP-07), so `index` has no installation to mutate and arguably retires with the on-disk rules root. Decide at the surface cut.
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review:**
  - uncertain on `read_or_mutate`
  - Leaned mutate against the obvious reading. `rules list` / `rules show` are plainly reads and are the ones agents want, but `intent_claude_rules` carries write primitives I did not attribute to an arm. If they all live in an unrelated arm this is a `read`.
- **recoverability:** idempotent

### `claude hook`

Run a named Intent hook

- **v2:** bin/intent claude arm
- **Arguments:**
  - `name` (string, arity `1`)
- **Exit codes:**
  - `0` -- hook succeeded
  - `1` -- hook failed
  - `2` -- the hook's OWN exit code, propagated verbatim -- by design (INV-04)
- **stdout:** the hook's stdout, verbatim
- **stderr:** the hook's stderr, verbatim
- **Target:** `as-observed`
- **Note:** **Byte-compatible on day one is a hard requirement, not a preference** (parity.md). Consumer `.claude/settings.json` files reference `intent claude hook <name>` by runtime resolution (issue 0016); if this path changes shape, every consumer's hooks break on the binary swap. The single most parity-critical entry in the family.
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Installs and removes Claude Code lifecycle hooks -- an agent editing the hooks that police it is the shape of the problem, even when each step is legitimate.
- **recoverability:** idempotent

### `claude upgrade`

Apply Claude canon to the project

- **v2:** bin/intent claude arm
- **Flags:**
  - `--apply` (bool) -- Write the canon; without it the command reports only
    - **disposition:** keep
- **Exit codes:**
  - `0` -- applied or reported
  - `1` -- canon source missing
- **stdout:** the canon convergence report
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Called by `intent upgrade` as its single canon step (ST0043).
- **Target:** `as-observed`
- **MCP:** not exposed -- **mutates**
- **recoverability:** one-way

### `claude prime`

Generate MEMORY.md content for a Claude session

- **v2:** bin/intent claude arm
- **Exit codes:**
  - `0` -- emitted
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the primed memory content
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Consumes `intent/.config/learnings.md` (written by `intent learn`).
- **Defects observed in v2:**
  - INV-06 at intent_claude_prime:212 writes `Error:` to stdout -- one of the parked plugin-bin sites
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **MCP classification grounded in:** intent/plugins/claude/bin/intent_claude_prime:250-251 -- `mkdir -p "$memory_dir"` then `echo "$content" > "$memory_path"`
- **recoverability:** idempotent

### `claude ws`

Manage whiteboard workstreams

- **v2:** bin/intent claude arm (ST0047)
- **Arguments:**
  - `verb` (subcommand, arity `1`) -- one of: `new`, `list`, `archive`, `hygiene`
  - `wsid` (string, arity `0..1`)
- **Exit codes:**
  - `0` -- scaffolded / listed / archived / linted
  - `1` -- unknown verb
  - `1` -- workstream not found
- **stdout:** the workstream table, or scaffold confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** `hygiene` enforces the line-oriented `key: value` header contract (D13) and says nothing about YAML validity, because validity is not the contract.
- **Target:** `as-observed`
- **Note:** D14: the whiteboard stays md-authored through 3.0.0/3.1 and is restructured in the 3.2 bus ST. So this family ports as-is rather than being reified.
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `claude start`

Launch a Claude session bound to a workstream

- **v2:** bin/intent claude arm (ST0047)
- **Arguments:**
  - `ws` (string, arity `1`)
- **Exit codes:**
  - `0` -- launched
  - `1` -- unknown workstream
- **stdout:** session launch output
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- **mutates**
- **MCP note:** Not a data operation at all -- it launches an interactive Claude Code session bound to a node. An agent that can call it can spawn agents.
- **recoverability:** one-way

## Family: `critic`

Run Intent rule-library critics against source files without invoking an LLM

- **v2 source:** `bin/intent_critic`
- **v2 help file:** none
- **Owning work package:** WP-07
- **BATS coverage:** 19 burning test(s) across 2 file(s) -- **covered**

- **The only command in the shipped surface that legitimately uses exit code 2** (bin/intent_critic:89,95) -- findings-present, distinct from failure. INV-04's named exception, and INV-02 must not flatten it.
- Strict-proxy contract since ST0039: the headless runner enforces ONLY rules publishing a simple `Greppable proxy`, and REFUSES non-simple proxies with a once-per-rule stderr note rather than approximating them. A critic that silently approximates a rule reports findings the rule does not actually make.
- `author` and `content` are accepted as a clean no-op: prose critique is on-demand via the critic-prose subagent, not this runner.

| command  | args   | flags                                                                                                         | help                                                 | disposition |
| -------- | ------ | ------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- | ----------- |
| `critic` | <lang> | --files <path> ..., --staged, --severity-min <lvl>, --format text/json, --rules <dir>, --languages, --help/-h | Run Intent rule-library critics against source files | keep        |

### `critic`

Run Intent rule-library critics against source files

- **v2:** bin/intent_critic
- **Arguments:**
  - `lang` (enum, arity `1`) -- one of: `elixir`, `rust`, `swift`, `lua`, `shell`, `author`, `content`
- **Flags:**
  - `--files` `<path> ...` (string) -- Explicit file list
    - default: scan nothing unless --staged
    - **disposition:** keep
  - `--staged` (bool) -- Scan files in the git staging area (pre-commit mode)
    - **disposition:** keep
  - `--severity-min` `<lvl>` (enum) -- Minimum severity to report
    - Accepts: critical | warning | recommendation | style
    - **default:** warning
    - **disposition:** keep
  - `--format` `text|json` (enum) -- Output format
    - **default:** text
    - **disposition:** keep
  - `--rules` `<dir>` (string) -- Alternative rules root, overriding canon discovery
    - **disposition:** keep
  - `--languages` (bool) -- List languages with a headless code critic, one per line, and exit
    - consumed by the pre-commit gate so the gate and the runner cannot drift
    - **disposition:** keep
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- clean scan, or `--languages` / `--help`
  - `1` -- outside a project (INV-03)
  - `2` -- findings present at or above --severity-min -- THE MEANINGFUL ONE (INV-04)
  - `2` -- bare invocation -- 1588B usage printed to STDOUT
  - `2` -- unknown flag -- 657B of grep's own error on stderr
  - `2` -- bad positional after a valid lang -- `error: unknown flag ...`
- **stdout:** the findings report, grouped by severity
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - **Exit 2 means FOUR different things** -- findings-present (the meaningful one), a bare invocation, an unknown flag, and a bad positional. Independently measured by vc; my first pass reported three and undercounted. The pre-commit gate reads this exit code, so a caller genuinely cannot distinguish 'findings' from 'you typed it wrong'.
  - The unknown-flag path leaks `grep: unrecognized option` as the command's own voice -- a raw tool error surfacing as Intent's.
  - INV-06 at the bare-invocation usage block goes to STDOUT on a failing (exit 2) invocation
- **Target:** `pending-hv`
- **Open question for hv:** **Highest priority of the 19 pending rows, and a different risk class from the other 18: this one has a LIVE CONSUMER.** Exit 2 must keep meaning findings-present (INV-04), which requires the other three conditions to move to exit 1 per INV-02. That is a `corrected` change the pre-commit gate reads today.
- **Note:** Confirmed independently by vc at the same revision; escalate to hv ahead of the usage-convention bundle.
- **wp07 owes:** MEASURED DIVERGENCE, RECORDED NOT RULED (cc measured it building 0038; ic recorded it 2026-08-16). **`intent critic` with NO language exits 2 in v2 and 1 in v3.** v2's 2 comes from `critic`'s OWN argument parsing -- it is already in `observed.exit` above as `bare invocation -- 1588B usage printed to STDOUT`. v3's 1 comes from clap under INV-02, which is correct for a usage error everywhere else in the surface. **So this is not a defect in either binary: it is the one place v2 used 2 for a usage error, and WP-07 has to decide whether to reproduce it.** Recorded here because the decision is invisible at the moment it gets made -- whoever wires `critic`'s language validation will be looking at clap's behaviour, which is already right by the general rule, and nothing on the path will mention that this command is the exception. `target.state` stays `pending-hv`: a divergence that needs a ruling is not settled by writing it down. **The wider point 0038 established, and the reason this matters more than one exit code: an exit code is a property of the CALLER's contract, not of the tool.** `critic`'s 2 has a live consumer -- the pre-commit gate reads it today as findings-present -- which is why it is the only unit in this register whose exit code is load-bearing before v3 ships.
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_critic -- no write primitive in the file; it reports and sets an exit code

## Family: `lang`

Per-language canon: install language-specific RULES + ARCHITECTURE templates

- **v2 source:** `bin/intent_lang`
- **v2 help file:** none
- **Owning work package:** WP-07
- **BATS coverage:** 57 burning test(s) across 3 file(s) -- **covered**

- **parity.md's table said `init, remove, list`. Measured: six verbs** -- list, show, init, remove, sync, plus `rm` as an alias of `remove`. Evidence: `intent lang sync` answers `ok: no declared languages; nothing to sync`.
- The `languages` array in config.json is authoritative (ST0037); filesystem-marker detection was retired because filesystem presence is unreliable evidence.

| command                         | args      | flags   | help                                                                      | disposition |
| ------------------------------- | --------- | ------- | ------------------------------------------------------------------------- | ----------- |
| `lang`                          | [command] | --      | Per-language canon management                                             | keep        |
| `lang list`                     | --        | --      | List languages with available templates                                   | keep        |
| `lang show`                     | <lang>    | --      | Show what `intent lang init <lang>` installs                              | keep        |
| `lang init`                     | <lang>... | --      | Install per-language canon (idempotent; multi-lang)                       | keep        |
| `lang remove` (alias `lang rm`) | <lang>... | --      | Remove per-language canon (idempotent; multi-lang)                        | keep        |
| `lang sync`                     | --        | --check | Converge the Language Packs block in RULES.md for every declared language | keep        |

### `lang`

Per-language canon management

- **v2:** bin/intent_lang
- **Arguments:**
  - `command` (subcommand, arity `0..1`)
- **Exit codes:**
  - `0` -- bare -- 1149B usage, exit 0
  - `0` -- `--help` -- same, exit 0
  - `1` -- unknown verb -- `error: unknown lang subcommand '<v>'`
  - `0` -- outside a project -- usage, exit 0
- **stdout:** the usage block
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- read-only

### `lang list`

List languages with available templates

- **v2:** bin/intent_lang list arm
- **Exit codes:**
  - `0` -- listed
- **stdout:** one row per language
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `lang show`

Show what `intent lang init <lang>` installs

- **v2:** bin/intent_lang show arm
- **Arguments:**
  - `lang` (string, arity `1`)
- **Exit codes:**
  - `0` -- shown
  - `1` -- unknown language
- **stdout:** the file list
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only

### `lang init`

Install per-language canon (idempotent; multi-lang)

- **v2:** bin/intent_lang init arm
- **Arguments:**
  - `lang` (string, arity `1..n`)
- **Exit codes:**
  - `0` -- installed
  - `1` -- unknown language
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation per language
- **stderr:** `error: ...` on stderr (INV-01)
- **Side effects:**
  - Writes intent/llm/RULES-<lang>.md + ARCHITECTURE-<lang>.md
  - Adds the language to config.json's `languages` array
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

### `lang remove`

Remove per-language canon (idempotent; multi-lang)

- **v2:** bin/intent_lang remove arm
- **Arguments:**
  - `lang` (string, arity `1..n`)
- **Exit codes:**
  - `0` -- removed
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation per language
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Removes per-language canon files. Deletion of authored canon is the one shape where a wrong call is not recoverable from the tool.
- **recoverability:** one-way

### `lang sync`

Converge the Language Packs block in RULES.md for every declared language

- **v2:** bin/intent_lang sync arm
- **Flags:**
  - `--check` (bool) -- Report without writing; exit 1 if any entry is missing or stale
    - **disposition:** keep
- **Exit codes:**
  - `0` -- converged, or `ok: no declared languages; nothing to sync`
  - `1` -- --check found a missing or stale entry
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the convergence report
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Touches ONLY the Language Packs block -- never the RULES-<lang>.md files, which `init` overwrites.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** idempotent

## Family: `llm`

LLM-related commands for working with AI assistants

- **v2 source:** `bin/intent_llm`
- **v2 help file:** none
- **Owning work package:** WP-06 + WP-09
- **BATS coverage:** 5 burning test(s) across 1 file(s) -- **covered**

- **The `intent llm` agent guide is regenerated from the dispatch table at WP-09** (design.md:85, the Lamplight DD-6 pattern), so this family is both a parity subject and a CONSUMER of this file.

| command           | args         | flags           | help                                                                                             | disposition |
| ----------------- | ------------ | --------------- | ------------------------------------------------------------------------------------------------ | ----------- |
| `llm`             | [subcommand] | --              | LLM-related commands                                                                             | keep        |
| `llm usage_rules` | --           | --symlink [dir] | Display the Intent usage rules for LLMs                                                          | keep        |
| `llm guide`       | --           | --              | Render the agent guide: the command reference generated from this table, plus the authored prose | new-surface |

### `llm`

LLM-related commands

- **v2:** bin/intent_llm
- **Arguments:**
  - `subcommand` (subcommand, arity `0..1`)
- **Exit codes:**
  - `0` -- bare -- 596B usage, exit 0
  - `0` -- `--help` -- same
  - `1` -- unknown subcommand -- `error: Unknown subcommand: <v>`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the usage block
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- read-only

### `llm usage_rules`

Display the Intent usage rules for LLMs

- **v2:** bin/intent_llm usage_rules arm
- **Flags:**
  - `--symlink` `[dir]` (string) -- Create a symlink to usage-rules.md in the current or specified directory
    - the directory argument is OPTIONAL, defaulting to the current directory -- an optional-value flag, which clap models differently from v2's positional peek
    - **disposition:** keep
- **Exit codes:**
  - `0` -- displayed or symlinked
  - `1` -- symlink target unwritable
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the usage-rules content, or the symlink confirmation
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **Note:** The optional-value flag is a genuine clap modelling decision (`num_args(0..=1)`), not a free port. Worth naming here so WP-05 does not discover it as a parse bug.
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review -- the classification disagrees with the verb name:** Reads as a display command, and its default IS display. The flag is what makes the entry a mutation -- the same shape as `at lint`, `doctor`, and `todo list`, which is why the field is defined over the entry rather than the default.
- **MCP classification grounded in:** bin/intent_llm:65 is `cat "$USAGE_RULES_FILE"`; :88-100 is the `--symlink` path, which creates a symlink and warns when one exists
- **recoverability:** idempotent

### `llm guide`

Render the agent guide: the command reference generated from this table, plus the authored prose

- **v2:** new-surface
- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run
- **Target:** `new-surface` -- ratified: DECLARED BY ic 2026-08-15, ratification OUTSTANDING. AC-09.4 says `intent llm` renders the agent guide and the `llm` family's own notes say it is regenerated from this table at WP-09 -- and no entry declared the command. The family knew; the surface did not. Declared now rather than at WP-09 because `guide_refs_check.sh` and the eventual `llm_guide_gen.rs` both need a command to test against, and an AC whose subject has no row cannot be tested at all.
- **Note:** A SECOND PROJECTION OF ONE FILE, which is exactly what vc's 2026-08-15 no-split ruling calls for: `intent llm` projects differently rather than reading a different file. `llm usage_rules` stays the human-facing DO/NEVER projection and keeps its name, which `--symlink` and the Elixir `usage_rules.sync` habit both depend on; this is the agent projection, and it carries what a human projection has no reason to -- the generated command reference, the `exposed_on_mcp` and `read_or_mutate` constraints ahead of each description, and the exit-code contract once for the surface rather than per row.
- **MCP:** exposed as an agent tool -- read-only
- **Wants review -- the classification disagrees with the verb name:** Exposing to an agent the guide that tells an agent what it may call looks circular and is the opposite: it is the only row on this surface whose whole content is the answer to `what am I allowed to do`, so withholding it is what would be strange. It is also the safest kind of row here -- it renders to stdout and writes nothing, which is why it is `read` where its sibling `llm usage_rules` is `mutate` (that one carries `--symlink`).
- **MCP classification grounded in:** AC-09.4 plus this family's own `family_notes`; no v2 antecedent, so there is nothing measured to ground it in and this row says so rather than implying a measurement

## Family: `learn`

Capture project-specific learnings for future LLM sessions

- **v2 source:** `bin/intent_learn`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 18 burning test(s) across 1 file(s) -- **covered**

- Storage is `intent/.config/learnings.md`, consumed by `intent claude prime` for MEMORY.md injection.
- Unusually for this surface, the primary action takes a POSITIONAL description and the verbs are expressed as flags (`--list`) rather than subcommands.

| command | args          | flags                    | help                                                       | disposition |
| ------- | ------------- | ------------------------ | ---------------------------------------------------------- | ----------- |
| `learn` | [description] | --category <cat>, --list | Capture project-specific learnings for future LLM sessions | keep        |

### `learn`

Capture project-specific learnings for future LLM sessions

- **v2:** bin/intent_learn
- **Arguments:**
  - `description` (string, arity `0..1`)
    - required unless --list is given
- **Flags:**
  - `--category` `<cat>` (enum) -- Which kind of learning
    - Accepts: footgun (default), worked, failed
    - **default:** footgun
    - **disposition:** keep
  - `--list` (bool) -- Show all learnings
    - **disposition:** keep
- **Exit codes:**
  - `0` -- captured, or listed
  - `0` -- bare -- 904B usage, exit 0
  - `1` -- unknown flag -- `error: Unknown option: <f>. Run 'intent learn help' for usage.`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** confirmation, or the learnings list
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- **mutates**
- **MCP classification grounded in:** bin/intent_learn:82 (append_learning), called at :201
- **recoverability:** idempotent

## Family: `modules`

Module registry guardrails and enforcement

- **v2 source:** `bin/intent_modules`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 20 burning test(s) across 1 file(s) -- **covered**

- One of the few v2 commands with DOCUMENTED exit codes in its own help: 0 clean, 1 issues found. Most of the surface documents none.
- `check` honours `file::function` rows since v2.11.12, so a helper registered against a function name is not reported as missing.

| command         | args      | flags      | help                                           | disposition |
| --------------- | --------- | ---------- | ---------------------------------------------- | ----------- |
| `modules`       | [command] | --         | Module registry guardrails                     | keep        |
| `modules check` | --        | --register | Compare MODULES.md registry against filesystem | keep        |
| `modules find`  | <term>    | --         | Search MODULES.md for a term                   | keep        |

### `modules`

Module registry guardrails

- **v2:** bin/intent_modules
- **Arguments:**
  - `command` (subcommand, arity `0..1`)
- **Exit codes:**
  - `0` -- bare -- 673B usage, exit 0
  - `0` -- `--help` -- same
  - `1` -- unknown verb -- `error: Unknown modules command: <v>. Run 'intent modules help' for usage.`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the usage block
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- read-only

### `modules check`

Compare MODULES.md registry against filesystem

- **v2:** bin/intent_modules check arm
- **Flags:**
  - `--register` (bool) -- Interactively register unregistered modules
    - INTERACTIVE -- the only interactive path in the surface, and a shape a non-tty caller cannot use
    - **disposition:** keep
- **Exit codes:**
  - `0` -- registry clean
  - `1` -- issues found -- unregistered or stale entries
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the findings
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **Note:** The interactive `--register` needs an explicit non-tty behaviour in v3; v2 has none stated.
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_modules -- no write primitive in the file

### `modules find`

Search MODULES.md for a term

- **v2:** bin/intent_modules find arm
- **Arguments:**
  - `term` (string, arity `1`)
- **Exit codes:**
  - `0` -- found, or no match
  - `1` -- no term given
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** matching rows
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_modules -- no write primitive in the file

## Family: `plugin`

Discover installed Intent plugins and their commands

- **v2 source:** `bin/intent_plugin`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 35 burning test(s) across 3 file(s) -- **covered**

- Runs outside a project (measured: exit 0, 1076B).

| command       | args      | flags | help                                                 | disposition |
| ------------- | --------- | ----- | ---------------------------------------------------- | ----------- |
| `plugin`      | [command] | --    | Discover installed Intent plugins and their commands | keep        |
| `plugin list` | --        | --    | List all plugins and their commands                  | keep        |
| `plugin show` | <name>    | --    | Show detailed information for a plugin               | keep        |

### `plugin`

Discover installed Intent plugins and their commands

- **v2:** bin/intent_plugin
- **Arguments:**
  - `command` (subcommand, arity `0..1`), default `list`
- **Exit codes:**
  - `0` -- bare -- defaults to `list`, 1076B
  - `0` -- `--help` -- 371B, exit 0
  - `1` -- unknown verb -- `error: Unknown plugin subcommand '<v>'. Run 'intent plugin help' for usage.`
  - `0` -- outside a project -- lists, exit 0
- **stdout:** the plugin list
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** not exposed -- read-only

### `plugin list`

List all plugins and their commands

- **v2:** bin/intent_plugin list arm
- **Exit codes:**
  - `0` -- listed
- **stdout:** one block per plugin
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_plugin -- no write primitive in the file

### `plugin show`

Show detailed information for a plugin

- **v2:** bin/intent_plugin show arm
- **Arguments:**
  - `name` (string, arity `1`)
- **Exit codes:**
  - `0` -- shown
  - `1` -- plugin not found
- **stdout:** the plugin detail
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_plugin -- no write primitive in the file

## Family: `ext`

Manage Intent user extensions at ~/.intent/ext/<name>/

- **v2 source:** `bin/intent_ext`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 41 burning test(s) across 3 file(s) -- **covered**

- Extensions live OUTSIDE the project, under `~/.intent/ext/`, so this family runs outside a project (measured: exit 0).
- Its help block still marks two verbs with development-session tags -- `validate [Session 3]` and `new [Session 4]` -- which are internal scheduling notes leaking into user-facing help.

| command        | args      | flags                            | help                                            | disposition |
| -------------- | --------- | -------------------------------- | ----------------------------------------------- | ----------- |
| `ext`          | [command] | --                               | Manage Intent user extensions                   | keep        |
| `ext list`     | --        | --                               | List installed extensions                       | keep        |
| `ext show`     | <name>    | --                               | Show manifest + contributions for one extension | keep        |
| `ext validate` | [name]    | --                               | Validate extension manifests                    | keep        |
| `ext new`      | <name>    | --skill, --subagent, --rule-pack | Scaffold a new extension                        | keep        |

### `ext`

Manage Intent user extensions

- **v2:** bin/intent_ext
- **Arguments:**
  - `command` (subcommand, arity `0..1`), default `list`
- **Exit codes:**
  - `0` -- bare -- defaults to `list`; `ok: no extensions installed (<path> does not exist)`
  - `0` -- `--help` -- 672B, exit 0
  - `1` -- unknown verb -- `error: unknown ext subcommand '<v>'`
  - `0` -- outside a project -- lists, exit 0
- **stdout:** the extension list
- **stderr:** `error: ...` on stderr (INV-01)
- **Observed notes:** Honours INTENT_EXT_DIR (override the default root, used by tests) and INTENT_EXT_DISABLE=1 (suppress ext discovery entirely).
- **Target:** `as-observed`
- **MCP:** not exposed -- read-only

### `ext list`

List installed extensions

- **v2:** bin/intent_ext list arm
- **Exit codes:**
  - `0` -- listed
- **stdout:** one row per extension
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_ext -- every write primitive in the file is inside ext_new (:743-759)

### `ext show`

Show manifest + contributions for one extension

- **v2:** bin/intent_ext show arm
- **Arguments:**
  - `name` (string, arity `1`)
- **Exit codes:**
  - `0` -- shown
  - `1` -- extension not found
- **stdout:** the manifest and contributions
- **stderr:** `error: ...` on stderr (INV-01)
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_ext -- writes confined to ext_new (:743-759)

### `ext validate`

Validate extension manifests

- **v2:** bin/intent_ext validate arm
- **Arguments:**
  - `name` (string, arity `0..1`)
    - omitted validates all
- **Exit codes:**
  - `0` -- valid
  - `1` -- findings
- **stdout:** findings
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - Help text marks this `[Session 3]` -- an internal development-scheduling tag in user-facing help.
- **Target:** `as-observed`
- **MCP:** exposed as an agent tool -- read-only
- **MCP classification grounded in:** bin/intent_ext -- writes confined to ext_new (:743-759)

### `ext new`

Scaffold a new extension

- **v2:** bin/intent_ext new arm
- **Arguments:**
  - `name` (string, arity `1`)
- **Flags:**
  - `--skill` (bool) -- Scaffold a skill extension
    - **disposition:** keep
  - `--subagent` (bool) -- Scaffold a subagent extension
    - **disposition:** keep
  - `--rule-pack` (bool) -- Scaffold a rule-pack extension
    - **disposition:** keep
- **Exit codes:**
  - `0` -- scaffolded
  - `1` -- missing --type
  - `1` -- name already exists
- **stdout:** the created layout
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - Help says `new <name> --type` but the parsed flags are `--skill` / `--subagent` / `--rule-pack`; there is no `--type` flag. The documented invocation cannot work.
- **Target:** `corrected` -- ratified: vc, 2026-08-15 -- and deliberately NOT an hv ruling, because none was required. NOTED ON THE ROW SO NOBODY RE-OPENS IT LOOKING FOR THE RULING THAT NEVER EXISTED: v3 generates help from this table, so a help/implementation mismatch CANNOT arise by construction. That is not a decision anyone has to make; it is a defect the SSOT retires wholesale, and this row was the last place still treating it as open. -- behaviour: Help text and implementation agree because both derive from this row. The v2 mismatch is not reproduced and needs no per-site correction.
- **ratified in:** vc, 2026-08-15, ruling applied at commit 803ff46d
- **MCP:** exposed as an agent tool -- **mutates**
- **recoverability:** one-way
- **recoverability anomaly:** DISAGREES WITH `exposed_on_mcp` AND IS REPORTED RATHER THAN ABSORBED (vc's ruling, 2026-08-16). `one-way` here, exposed on MCP in the table. **The anomaly is an INCOMPLETE FAMILY, not a misclassified row:** `ext` ships `list`, `show`, `validate` and `new`, and there is no `ext remove` anywhere in the table, so nothing undoes a scaffold. It unifies with `backup` shipping without a `restore`: both are a family shipping the creating half without the undoing half. **The field is not failing here, it is DETECTING a real gap on its first run** -- a derived field that merely reproduces the partition it was fitted to tells you nothing. Resolution is a SCOPE call for hv (build `ext remove`, and this row becomes `reversible`; or withhold `ext new`, and it becomes an ordinary agreement), never a label anyone adjusts.

## Family: `treeindex`

Generate LLM-oriented directory summaries

- **v2 source:** `bin/intent_treeindex`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 6 burning test(s) across 1 file(s), plus 1 file(s) that name it but never reach the CLI -- **covered**

- **53 of the 53 tests in `tests/unit/treeindex_commands.bats` exec `bin/intent_treeindex` DIRECTLY, bypassing the dispatcher entirely -- burn ratio zero.** The file reads as CLI-shaped and is not; that discovery is the reason the register classifies by burn measurement rather than by reading assertions.
- D21: the treeindex cache location is unchanged until WP-06 ports the command. If it moves under `intent/.cache/`, that is its own register entry.

| command     | args  | flags                                                                           | help                                      | disposition |
| ----------- | ----- | ------------------------------------------------------------------------------- | ----------------------------------------- | ----------- |
| `treeindex` | <dir> | --depth/-d <n>, --check, --dry-run, --force, --prune, --model <name>, --help/-h | Generate LLM-oriented directory summaries | retire      |

### `treeindex`

Generate LLM-oriented directory summaries

- **v2:** bin/intent_treeindex
- **Arguments:**
  - `dir` (path, arity `1`)
    - NEVER the project root -- always a subdirectory
- **Flags:**
  - `--depth`, `-d` `<n>` (integer) -- How deep to descend
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--check` (bool) -- Report without writing
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--dry-run` (bool) -- Preview
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--force` (bool) -- Regenerate even if current
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--prune` (bool) -- Remove stale index entries
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--model` `<name>` (string) -- Model to summarise with
    - **disposition:** retire
    - **disposition basis:** Inherited from the entry: a retired command never reaches clap, so neither can its flags. Mechanically checkable, and the refusal checks it.
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `1` -- bare -- prints 1548B usage to STDOUT and exits 1
  - `1` -- `--help` -- identical, exit 1
  - `1` -- unknown flag -- 1548B on stdout AND 33B on stderr, exit 1
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the usage block, or per-directory progress
- **stderr:** the unknown-option note
- **Defects observed in v2:**
  - INV-07 at `treeindex --help`
  - INV-06 at the unknown-flag path writes to BOTH streams -- one of only two such cases in 108 probes
- **Target:** `retire` -- ratified: hv, 2026-08-15 -- treeindex retires WHOLE (command, `intent/.treeindex/` cache, `/in-essentials` rules 3 and 4, every canon reference), together with the `in-handoff` skill. The source tree index in the DB obviates treeindex, and the DB model obviates handover: state moves out of per-session `.md` files shared between workstreams into durable state in the intentdb. Settles AC-13.1, which had been vc-specced under standing authorisation and contradicted by D21.
- **Note:** D21 (design.md:195) still reads "the treeindex cache location is unchanged until WP-06 ports the command" -- which assumes a port. Its DECISION (`intent/.cache/` gitignored whole-dir, DB inside) is unaffected and AC-01.4 does not reopen; the subordinate forward-looking clause needs striking. Flagged by ic, surfaced by vc following the register's UNRATIFIED marker.
- **consequence:** Removes 762 lines of bash from WP-06's port list, and INV-07 (`--help` exits non-zero here) is moot rather than pending-hv: there is no v3 command to correct.
- **MCP:** not exposed -- **mutates**

## Family: `fileindex`

Maintain checkbox file indexes

- **v2 source:** `bin/intent_fileindex`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 2 burning test(s) across 1 file(s) -- **THIN -- 2 burning test(s)**

- The widest short-flag surface in the CLI (-C -U -X -f -h -i -r -v), and the only family where short flags carry meaning beyond an alias for a long form.
- 45 of 47 tests in `tests/unit/fileindex_commands.bats` bypass the dispatcher (burn 2/47), the same shape as treeindex.
- **The thinness number is evidence in a live decision, not just a register row.** 2 burning tests against 47 in the file: hv has not yet ruled whether `fileindex` follows `treeindex` into retirement under WP-13, and 45 of 47 tests bypassing the dispatcher means the conformance suite would barely notice either way. (vc, 2026-08-14.)

| command     | args                  | flags                                                                                                                                               | help                           | disposition |
| ----------- | --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ | ----------- |
| `fileindex` | [startdir] [filespec] | --check/-C, --uncheck/-U, --toggle/-X, --file/-f <path>, --index/-i <path>, --index-dir <path>, --intent-dir <path>, --no-intent, -r, -v, --help/-h | Maintain checkbox file indexes | keep        |

### `fileindex`

Maintain checkbox file indexes

- **v2:** bin/intent_fileindex
- **Arguments:**
  - `startdir` (path, arity `0..1`)
  - `filespec` (string, arity `0..1`)
- **Flags:**
  - `--check`, `-C` (bool) -- Mark checked
    - **disposition:** keep
  - `--uncheck`, `-U` (bool) -- Mark unchecked
    - **disposition:** keep
  - `--toggle`, `-X` (bool) -- Flip the mark
    - **disposition:** keep
  - `--file`, `-f` `<path>` (string) -- Target file
    - **disposition:** keep
  - `--index`, `-i` `<path>` (string) -- Index file
    - **disposition:** keep
  - `--index-dir` `<path>` (string) -- Index directory
    - **disposition:** keep
  - `--intent-dir` `<path>` (string) -- Intent directory
    - **disposition:** keep
  - `--no-intent` (bool) -- Operate without an Intent project
    - **disposition:** keep
  - `-r` (bool) -- Recurse
    - **disposition:** keep
  - `-v` (bool) -- Verbose
    - **disposition:** pending
    - **disposition basis:** VERBOSITY CLUSTER -- log-level verbosity, same open question as `doctor --verbose`. NOT the same thing as `claude skills -v` / `claude subagents -v`, which are a list display mode and ship.
  - `--help`, `-h` (bool) -- Print the usage block
    - **disposition:** intrinsic
    - **disposition basis:** PROPOSED FOURTH VALUE, vc to rule. clap supplies help itself and `spine.rs:145-151` ALREADY skips these spellings, so the flag ships and the renderer is not expected to read it -- which `keep` (ships AND must be read) and `retire` (never reaches clap) both state falsely. The spine currently gets this right by matching on the spelling, which is exactly the inference-from-name that EXP-05 exists to replace with a declaration.
- **Exit codes:**
  - `0` -- bare -- prints `[ ]`, 5B, exit 0
  - `1` -- `--help` -- 1518B usage to STDOUT, exit 1
  - `1` -- unknown flag -- 1551B on STDOUT, `Unknown option: ...`, exit 1
  - `0` -- outside a project -- prints `[ ]`, exit 0
- **stdout:** the checkbox line, or the usage block
- **stderr:** --
- **Defects observed in v2:**
  - INV-07 at `fileindex --help`
  - INV-06 at the unknown-flag error goes to STDOUT
  - INV-01 at `Unknown option: ...` carries no `error:` prefix
- **Target:** `corrected` -- ratified: hv, 2026-08-14 at the bounce, via parity.md's `Corrected` ratified deviation class -- which names this exact defect as a known member: "`--help` reporting failure on 10 of 27 commands". APPLIED by vc, 2026-08-15, who ruled it independently from INV-04 (0 is success, 1 is every failure) before either of us noticed the earlier ratification. Two routes, same answer: the requirement governs v3 and the gap is a parity class. hv ratification is therefore NOT outstanding -- it predates the question. -- behaviour: Defers to INV-07, now `corrected`: `--help` exits 0 to stdout. The v3 binary already does this.
- **build measured:** BUILD HAS ALREADY DECIDED (ic measured, 2026-08-15). See INV-07: the v3 binary exits 0 on `--help` today, deliberately (`spine.rs:193` maps clap's DisplayHelp to EXIT_OK, and `spine.rs:27` names the divergence). This row asks whether to reproduce or correct, and the code has answered `corrected` without the contract recording it. Measurement, not a ruling.
- **ratified in:** parity.md
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Writes an index an agent would plausibly want to refresh before searching. Leaned open; the cost of being wrong is a stale-index rebuild, not lost work.
- **recoverability:** idempotent

## Family: `help`

Show usage for Intent or one of its commands

- **v2 source:** `bin/intent_help`
- **v2 help file:** none
- **Owning work package:** WP-05
- **BATS coverage:** 89 burning test(s) across 7 file(s) -- **covered**

- **This family is RETIRED AND REPLACED in v3, not ported: help is generated from this dispatch table (AC-05.1).** That retires the entire `lib/help/` mechanism along with its drift.
- The drift is the argument. `lib/help/` holds 11 help files for 27 `bin/intent_*` scripts, so 17 commands have none. Its `@usage:` / `@options:` / `@arguments:` / `@examples:` grammar is used exactly ONCE each. `stzero.help.md` is named against `bin/intent_st_zero`, and `rules.help.md` documents a `claude` SUBcommand as though it were top-level. At v2.19.0 it still describes `upgrade` as an STP migration.
- `bin/intent_help` also hand-maintains its command list behind a skip list rather than enumerating the surface, so the list and the surface are two things that must be kept in agreement by hand. **`lib/help/` therefore cannot be used as the v2 spec to port from** -- which is precisely why this table was measured from `bin/**` and by invocation instead.

| command | args      | flags | help                                         | disposition |
| ------- | --------- | ----- | -------------------------------------------- | ----------- |
| `help`  | [command] | --    | Show usage for Intent or one of its commands | retire      |

### `help`

Show usage for Intent or one of its commands

- **v2:** bin/intent_help; the no-help fallthrough is at bin/intent_help:37
- **Arguments:**
  - `command` (string, arity `0..1`)
- **Exit codes:**
  - `0` -- bare -- 2320B command list, exit 0
  - `1` -- `--help` -- `error: Unknown command '--help'`, exit 1
  - `1` -- unknown argument -- `error: Unknown command '<arg>'`
  - `0` -- outside a project -- lists, exit 0
- **stdout:** the command list, or one command's help
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - INV-07 at `intent help --help` fails outright -- asking help for help is an error
  - 17 of 26 commands have no help file, so `intent help <cmd>` silently falls through to a no-help path for most of the surface.
- **Target:** `retire` -- ratified: AC-05.1 -- the dispatch table is the SSOT and help text is generated from it, asserted by test -- behaviour: The `help` COMMAND survives as a surface; its v2 IMPLEMENTATION (lib/help/ + the hand-maintained list + the skip list) does not.
- **MCP:** not exposed -- read-only
- **Wants review -- the classification disagrees with the verb name:** Classified `false` despite being the single most harmless command in the table. In v3 help RENDERS FROM this file, so an MCP client already holds every string `help` would print; exposing it would be a second copy of the surface description, which is the thing this file exists to prevent.

## Family: `st_zero`

Retrofit ST0000 deliverables into brownfield projects

- **v2 source:** `bin/intent_st_zero`
- **v2 help file:** `lib/help/stzero.help.md`
- **Owning work package:** WP-06
- **BATS coverage:** 32 burning test(s) across 1 file(s) -- **covered**

- NAME DRIFT in the help file: `lib/help/stzero.help.md` against `bin/intent_st_zero`. One of the reasons `lib/help/` cannot serve as the v2 spec to port from -- see the `help` family.
- Reachable by two spellings: `intent st_zero` (top level, auto-dispatched) and `intent st zero` (bin/intent_st:1610 execs this binary). Its own usage block says `intent st zero install`, so it documents only the second.
- parity.md flags this family as a candidate for a ratified RETIRE if the fleet does not use it -- decided at port time, in the register. That one ruling decides both spellings.

| command   | args      | flags                                       | help                                                  | disposition |
| --------- | --------- | ------------------------------------------- | ----------------------------------------------------- | ----------- |
| `st_zero` | [command] | --audit-only, --dry-run, --deliverable <id> | Retrofit ST0000 deliverables into brownfield projects | retire      |

### `st_zero`

Retrofit ST0000 deliverables into brownfield projects

- **v2:** bin/intent_st_zero
- **Arguments:**
  - `command` (subcommand, arity `0..1`) -- one of: `install`
- **Flags:**
  - `--audit-only` (bool) -- Show gap analysis only, no changes
    - **disposition:** retire
    - **disposition basis:** The root `st_zero` spelling dies (hv ratified, 2026-08-15) -- these three flags reach clap via `st bootstrap`, whose row carries its own live copies. Recorded from v2, never built.
  - `--dry-run` (bool) -- Show what would change, no writes
    - **disposition:** retire
    - **disposition basis:** The root `st_zero` spelling dies (hv ratified, 2026-08-15) -- these three flags reach clap via `st bootstrap`, whose row carries its own live copies. Recorded from v2, never built.
  - `--deliverable` `<id>` (string) -- Target a single deliverable (D2-D11)
    - **disposition:** retire
    - **disposition basis:** The root `st_zero` spelling dies (hv ratified, 2026-08-15) -- these three flags reach clap via `st bootstrap`, whose row carries its own live copies. Recorded from v2, never built.
- **Exit codes:**
  - `0` -- bare -- prints `Usage: intent st zero install [options]`, 879B, exit 0
  - `0` -- `--help` -- same, exit 0
  - `1` -- unknown verb -- `error: Unknown st zero command: <v>. Run 'intent st zero help' for usage.`
  - `1` -- outside a project -- `error: not in an Intent project directory` (INV-03)
- **stdout:** the gap analysis and install report
- **stderr:** `error: ...` on stderr (INV-01)
- **Defects observed in v2:**
  - A bare invocation that printed only usage exits 0, where every other family in this table exits 1 for the same shape. Inconsistent in the opposite direction to INV-07.
- **Target:** `retire` -- ratified: hv, 2026-08-15: "`st_zero` is wrong and the root spelling dies." APPLIED as `retire` per vc's ruling of 2026-08-15, which chose this over teaching the spine to read `target.spelling`. Two reasons, and the second decides it: building a general rename facility for a population of ONE reads as foresight and ships as unused surface; and more importantly, the alternative makes this row assert something hv did NOT ratify. `corrected` says "this v2 command survives, renamed" -- a different and softer claim than "the root spelling dies", adopted for mechanical convenience. A mechanism that changes what a ratified row MEANS is not a neutral choice of mechanism. Reconsider at the SECOND instance, not before.
- **Note:** MEASURED 2026-08-15: before this change the v3 binary offered BOTH `intent st_zero` and `intent st bootstrap`, because `is_shipped()` reads `disposition` and the command name comes from `entry.path` -- so `target.spelling` was declared and unread. `target.spelling` STAYS on this row as the human-readable record of the successor; it is simply not machine-read. The successor `st bootstrap` carries the behaviour and its own live copies of these three flags.
- **spelling:** intent st bootstrap
- **consequence:** The divergence cost is ZERO for anyone following the command's own documentation: its usage block only ever said `intent st zero install`, never the root spelling. So the face that dies is the one no user was told to use, and the face that survives is a rewording of the one that was.
- **face:** deleted
- **never built:** true
- **MCP:** not exposed -- **mutates**

## Family: `version`

Print the Intent version

- **v2 source:** `bin/intent (global command)`
- **v2 help file:** none
- **Owning work package:** WP-06
- **BATS coverage:** 65 burning test(s) across 2 file(s) -- **covered**

- A global command: runs outside a project.
- `get_intent_version` in bin/intent_helpers is THE single fallback site for version resolution (consolidated in v2.11.12); v3 bakes GIT_HASH into the version string (design.md:74).

| command   | args | flags | help                     | disposition |
| --------- | ---- | ----- | ------------------------ | ----------- |
| `version` | --   | --    | Print the Intent version | keep        |

### `version`

Print the Intent version

- **v2:** bin/intent (GLOBAL_COMMANDS); resolution via `get_intent_version` in bin/intent_helpers
- **Exit codes:**
  - `0` -- printed
  - `0` -- unknown flag -- ACCEPTED SILENTLY, exit 0
- **stdout:** the version string
- **stderr:** --
- **Defects observed in v2:**
  - INV-08 at `intent version --zzz` succeeds silently at exit 0
- **Target:** `corrected` -- ratified: hv 2026-08-14 bounce (the `corrected` class); forced -- clap rejects unrecognised arguments by default -- behaviour: Unknown arguments refused, exit 1 per INV-02. The version string itself gains a baked GIT_HASH.
- **ratified in:** parity.md
- **MCP:** exposed as an agent tool -- read-only

## Known exposures -- defects this file does not have, and is not protected against

### EXP-01 -- The generated view is formatter-stable today by accident, not by design

- **Detail:** cc found a third formatter class: prettier normalises the AUTHOR's own emphasis (`*major*` -> `_major_`). No renderer discipline can reach it, because a renderer is forbidden to rewrite authored prose -- and the prose here comes from the canon. This view currently contains ZERO single-asterisk emphasis spans, measured, so it does not trip the class. That is luck: one canon note written with `*emphasis*` breaks the skew check on this file tomorrow, and the break would look like drift on a file nobody touched.
- **Resolution:** AC-07.6 (vc, 2026-08-14): generated views are EXCLUDED from the formatter repo-wide, converged at `init` and migration so consumer repos inherit it -- a local `.prettierignore` would fix this tree and leave the shipped skew check crying wolf in Lamplight, Utilz and Baize forever. Deliberately not landing today: the pre-commit hook rewrites every staged `.md` and `git add`s it, and three nodes hold uncommitted work.
- **Consequence for the generator:** The table aligner has TWO justifications and only one of them expires. (1) The skew justification -- matching the formatter's column widths so regeneration reproduces the committed bytes -- becomes unnecessary the moment AC-07.6 lands. (2) The house-rule justification -- `in-standards`: all markdown tables must be column-aligned -- does not expire, and the formatter was correcting a real defect rather than imposing a preference. So the aligner STAYS after AC-07.6; what goes away is the need for it to match the formatter's choices exactly. Written down because a future reader finding align logic in a renderer whose output the formatter no longer touches would reasonably assume it was vestigial and delete it.

### EXP-02 -- Two artefacts now describe the command surface, and nothing checks they still agree

- **Detail:** `parity/cmd-<command>.md` (26 files, GENERATED by `gen_inventory.sh` from probes and static passes) and this file (AUTHORED) both carry the verb and flag sets. They were built in that order -- the table was derived from the inventory plus per-arm source reads, then enriched with target / disposition / invariants -- so it is a derive-then-enrich relationship rather than a copy-paste, which is normal and fine. What is NOT fine is that nothing keeps them in agreement afterwards. Spot-checked at the stamped revision: `lang`, `plugin` and `modules` agree exactly (the inventory lists `rm` as its own row where this file records it as an alias on `lang remove` -- consistent, differently represented). Agreement today with no mechanism is the divergent-copy shape, and it is the shape THIS artefact exists to eliminate elsewhere in the surface. Named against my own work rather than left for someone else to find.
- **Resolution:** The next re-sweep re-runs `gen_inventory.sh`, which regenerates the 26 inventory files from live measurement. **That pass must include a drift check between the regenerated verb/flag sets and this file's**, and a disagreement must be reported rather than resolved by picking a winner: the inventory is measurement and this file is judgement, so a divergence means either the surface moved or a judgement here was wrong, and those need different responses. Without that check the re-sweep silently updates one of the two descriptions and leaves the other stale, which is worse than today's position -- they agree now.
- **Consequence for the generator:** None; the drift check belongs to the inventory toolchain, not the renderer. Recorded here because this file is the one that would be silently wrong.

### EXP-03 -- Two ACs say the MCP tool tier and the agent guide are GENERATED from this file, and this file cannot answer the first question either generator will ask

- **Detail:** AC-09.1 -- _the typed tool tier is generated from the dispatch table_ -- and AC-09.4 -- _`intent llm` renders the agent guide from the dispatch table; NO HAND-MAINTAINED COMMAND LIST EXISTS_. Measured 2026-08-15 against all 103 rows: **no row says whether it is exposed on the MCP surface, and no row says whether it reads or mutates.** Neither fact is derivable from what is here. `observed.side_effects` exists on 10 rows of 103, so its absence means `not recorded`, not `no side effects` -- reading it as a read/write flag would be absence-as-meaning in the one place it decides whether an agent may close a steel thread. Nothing is broken today because WP-09 has not started; the exposure is that the ACs already assume a capability this artefact does not have.
- **Resolution:** Two declared fields per entry -- exposed-on-MCP, and read-or-mutate -- **declared per row rather than derived from the verb**, because deriving from a name is the sniffing this toolchain keeps refusing (`st sync` and `sync` are different commands sharing a spelling; `ac gate` reads while `wp done` consults the same gate and writes). Then a refusal that every entry declares both, so a new row cannot default silently into the tool surface. NOT landed unilaterally today: adding the fields is authoring, which is ic's, but CLASSIFYING 103 rows is a judgement with a safety edge and belongs with vc and cc as WP-09 opens. Raised now, before WP-09 starts, so it is a specification rather than a rediscovery -- the same row-before-surface order that worked for `ac unsatisfy` and the seven lifecycle verbs.
- **Consequence for the generator:** None yet. When the fields land, the completeness refusal already added to `gen_dispatch_table.sh` will carry them into the view for free, because it renders every authored field rather than an enumerated set.

### EXP-04 -- A `keep` disposition is honest about the SURFACE and silent about the SEMANTICS

- **Detail:** vc, 2026-08-15, generalising from `st new -s|--start`. The whole disposition vocabulary -- `keep · retire · deviate · corrected · new-surface · pending` -- classifies what happens to a COMMAND'S SURFACE. It has no way to say _the spelling, the flags and the observed behaviour are all unchanged, and the meaning moved anyway_. That is exactly what happened to `-s|--start`: it is v2 parity, its help text still matches, its v2 behaviour still matches, and **a ratified decision changed the state space underneath it** so that one transition became two. **Nothing in this file could have shown that**, and a reader scanning dispositions would have seen `keep` and moved on.
- **Resolution:** **Deliberately NOT solved today, and vc explicitly did not ask for a mechanism.** Recorded here rather than left as a note in an inbox because an inbox entry gets archived and this section is read by anyone who reads the table -- and because the honest status of this exposure is _known, unprotected_, which is precisely what `known_exposures` exists to say. The cheap partial already in use: when a row's semantics move under a ruling, say so ON the row in its own field (`st new` now carries `start_flag_ruled` and `composition_constraint`), so the fact is at least greppable even though the disposition cannot carry it. If it recurs often enough to need a mechanism, the shape is a semantics stamp -- the ruling a row was last checked against -- and that is a contract decision for vc, not a renderer change.

### EXP-05 -- Entries declare whether they ship; FLAGS could not -- CLOSED in this file, still open in the spine

- **Detail:** `is_shipped()` reads an entry's `disposition` and `target.state`, so a retired COMMAND never reaches clap. There is no equivalent one level down: `spine.rs` builds every declared flag on every shipped entry, unconditionally, and the flag schema carries no field that could say otherwise -- measured, the union of all flag keys across this table is: accepts, default, help, note, required, spellings, type, value. So the table can withdraw a command from the v3 surface and CANNOT withdraw a flag. That is the structural gap behind AC-06.8, and it is the same shape as EXP-03: a fact a generator needs, that this file does not carry, and that cannot be derived -- whether a flag is READ is a property of the renderer, never of its spelling.
- **Resolution:** CLOSED HERE, AND ONLY HERE. EVERY flag declares a disposition -- **the count is deliberately not written here.** It said `All 93 flags (63 keep, 14 retire, 10 intrinsic, 6 pending)` and went stale the first time a flag was added, which was `schema --versions` on 2026-08-15, hours later. **A tally in prose is a hand-maintained roster inside the artefact that exists to abolish hand-maintained rosters** -- the same shape as AC-02.6's eight-vs-nine table list, and the reason that one is now enumerated from the generated face. The live numbers come from the file: `jq '[.families[].entries[], .new_surface[]] | map(.flags // []) | add | group_by(.disposition) | map({(.[0].disposition): length}) | add'`. Three refusals enforce it, and a shared renderer plus a COUNT-based check puts every one of them in the view. THE SPINE DOES NOT YET HONOUR IT: `spine.rs:142` still builds every declared flag on every shipped entry, so today the declaration is documentation with a guard on it. That half is cc's and the first user is `doctor --fix`, whose `retire` is the one classification vc measured in v2 source rather than inferred. Until it lands, this file can say a flag does not ship and the binary will ship it -- which is a smaller gap than not being able to say it at all, and is not the same as closed.
- **Consequence for the generator:** Nothing yet: the generator renders flags faithfully and has no opinion about whether they ship. It gains one the moment the field exists -- every flag must declare a disposition or the run refuses, the same construction as the MCP fields, so a flag cannot join the surface by being typed. Until then this register is the only place the gap is written down. The 44 further declared-and-unread flags all sit on commands with NO renderer arm at all, so they are future violations rather than current ones: they become AC-06.8 breaches one at a time as each command is wired, which is the worst arrival schedule available for a defect nobody is watching for.

### EXP-06 -- D42 audit of the declared surface: it came back CLEAN, and the two findings that survive are not about clocks

- **Detail:** Audited the dispatch table and the 27 command inventories against D42 and filed three findings. hv then narrowed the rule twice within the day and BOTH narrowings cut against me. First: the rule is ASYMMETRIC -- no CLI or intentsvcs function TAKES a time, but functions may RETURN times, because those went end-to-end through the DB where SQLite set them; vc has since sharpened this into its strongest form, a property of the SIGNATURE rather than of a value, so a time-typed input parameter is a defect by inspection and nobody has to trace provenance. Second: reading a clock to MAKE A DECISION is fine, and so is stamping WHEN A COMMAND RAN into a GENERATED artefact -- what D42 forbids is confecting a time and injecting it into a source document. Under the rule as it now stands the declared surface has NO D42 exposure at all: zero of three findings survive as D42, and the inventories were already clean because v2 declares no time-bearing flag or argument anywhere.
- **Resolution:** None adopted here; each row names its own owner (`todo done` is WP-03's renderer constraint, `backup` is AC-02.8). The finding worth keeping is about auditing, not about time. THE AUDIT WAS RIGHT TO RUN AND WRONG IN EVERY VERDICT IT FIRST REACHED, and it still produced two real defects -- because the only way to check a claim about a surface is to read the code under it, and the reading is what found them. An audit that had trusted its own pattern match would have filed three wrong findings and stopped.
- **Consequence for the generator:** None today; the generator renders `exposure` through the generic `targetextra` path like any other authored sub-key, and the entry-level completeness check proves it reaches the view. Two standing obligations, and they are different sizes. The small one: re-audit when new_surface changes. The large one: THIS CENSUS CANNOT SEE THE THING THE RULE ACTUALLY FORBIDS. vc's caution is exact -- a clean result here is about DECLARED FLAGS AND ARGUMENTS, and hv's rule is about function PARAMETERS, which need never appear in a flag inventory. A green from this file is not evidence about signatures, and the needle that checks them lives in cc's guard, not here.

### EXP-07 -- The flag schema declares five properties the CLI cannot see, and one of them is a live defect on an implemented command

- **Detail:** Measured while classifying for EXP-05, by reading `dispatch.rs` rather than reasoning about the renderer. `pub struct Flag` at dispatch.rs:126-133 has exactly THREE fields -- `spellings`, `kind`, `help`. The canon authors eight: those three plus `accepts` (4 instances), `default` (6), `required` (3), `value` (35) and `note` (9). So `accepts`, `default`, `required` and `value` DO NOT DESERIALIZE AT ALL. This is not a renderer that forgets to read them; it is a type with no field for them, which is the same shape EXP-05 diagnosed for `disposition` -- except EXP-05 said the schema was missing ONE field and the measurement says it is missing FOUR that are already authored.
- **Resolution:** Not adopted here, and deliberately NOT folded into EXP-05. EXP-05 was ruled as a specific mechanism and vc anchors review to the proposal, so widening it silently while implementing it is the way a reviewed change becomes an unreviewed one. Recorded whole instead. The cheap half is this file's and is already done -- `default` and `required` now RENDER, so a reader of the view can at least see the claim. The rest is cc's: four fields on `Flag`, `.required()`/`.default_value()`/`.value_parser()`/`.value_name()` on the clap side, and a refusal rather than a `continue` for a flag the spine cannot build. `ac satisfy --evidence` is the one worth doing first because it is live and silent.
- **Consequence for the generator:** Two, both now closed. The flag level was the LAST population with an enumerated renderer and no generic remainder -- `extras` had closed the entry and `target` levels and nothing descended into `flags` -- so `default` and `required` had been in the canon and out of the view for as long as they had existed, and no check anywhere could say so. `flagextra` closes it. The second is sharper and is a lesson about checks rather than about flags: the flag completeness loop I wrote to close this class WENT GREEN ON ITS FIRST RUN while five flags rendered nothing, because it greps for the LABEL and 88 of 93 flags supplied it. Presence-of-label and completeness-of-population are different questions. It is now a COUNT, matched against the declared total, and the count is what found the fact that new_surface flags had never rendered in detail at all.

### EXP-08 -- The root command's help is the one help string in the surface that does not come from this file

- **Detail:** This file's own `about` opens by claiming that the clap surface, THE HELP TEXT, the MCP tool list and the `intent llm` agent guide all render FROM it. Measured 2026-08-16 and it is true 107 times out of 108. `spine.rs` sets `.about(entry.help)` for every family, entry and verb, and `.help(flag.help)` for every flag -- so help is table-driven BY CONSTRUCTION for the whole command surface, which is also why the SSOT test spot-checking a single command (`st new`) is defensible rather than thin. The exception is `spine.rs:26`, the ROOT command, whose about is a Rust string literal: `Intent: steel threads, work packages and the acceptance contract`. It is the only `.about(` literal in the entire CLI, and grepping this file for that sentence returns zero. **It is also the first line an agent reads from `intent --help`.** The general shape is the one this contract has hit from four sides in one day: an artefact asserting a property it very nearly has, where the near-miss is invisible because the assertion is true everywhere the reader checks.
- **Resolution:** CLOSED 2026-08-16, both halves in one commit. `root_help` is declared here and `spine.rs:26` reads it, so there is no longer a `.about(...)` string literal anywhere in the CLI and this file's `about` claim -- that the clap surface and THE HELP TEXT render from it -- is true without exception. Landed by ic across both halves rather than waiting for the handoff, on hv's steer of 2026-08-16 (`the sooner we can get this project onto v3, the better`) and because `spine.rs` was clean while cc was working elsewhere; raised with cc the same minute with an offer to revert if they would rather own the wiring. **The sequencing constraint that shaped this is still the right one and was not waived: the field was never declared while nothing read it**, because a declared value nothing renders is exactly what AC-06.8 exists to prevent -- the two halves landed together, which is what the constraint asks for, rather than the field landing first and waiting.

## Parity holes -- what the BATS estate does NOT cover

A command family with no burning coverage is a parity hole: v3 can change it freely and the conformance suite stays green. Produced by `parity/tools/coverage_map.sh`, which joins these families against `burn-baseline.tsv` -- the join matters, because a naive grep reports `treeindex` as well covered when all 53 of its tests exec `bin/intent_treeindex` directly and the dispatcher never sees them.

| family      | files (real) | files (vacuous) | burning tests | verdict                                  |
| ----------- | ------------ | --------------- | ------------- | ---------------------------------------- |
| `st`        | 22           | 0               | 267           | covered                                  |
| `wp`        | 8            | 0               | 79            | covered                                  |
| `ac`        | 4            | 0               | 57            | covered                                  |
| `at`        | 2            | 0               | 30            | covered                                  |
| `issues`    | 1            | 0               | 20            | covered                                  |
| `todo`      | 1            | 0               | 22            | covered                                  |
| `info`      | 3            | 0               | 23            | covered                                  |
| `config`    | 0            | 0               | 0             | HOLE -- nothing in the estate invokes it |
| `init`      | 7            | 0               | 55            | covered                                  |
| `bootstrap` | 2            | 0               | 16            | covered                                  |
| `doctor`    | 1            | 0               | 11            | covered                                  |
| `upgrade`   | 1            | 0               | 4             | covered                                  |
| `organize`  | 1            | 0               | 3             | covered                                  |
| `agents`    | 4            | 1               | 86            | covered                                  |
| `claude`    | 30           | 2               | 296           | covered                                  |
| `critic`    | 2            | 0               | 19            | covered                                  |
| `lang`      | 3            | 0               | 57            | covered                                  |
| `llm`       | 1            | 0               | 5             | covered                                  |
| `learn`     | 1            | 0               | 18            | covered                                  |
| `modules`   | 1            | 0               | 20            | covered                                  |
| `plugin`    | 3            | 0               | 35            | covered                                  |
| `ext`       | 3            | 0               | 41            | covered                                  |
| `treeindex` | 1            | 1               | 6             | covered                                  |
| `fileindex` | 1            | 0               | 2             | THIN -- 2 burning test(s)                |
| `help`      | 7            | 0               | 89            | covered                                  |
| `st_zero`   | 1            | 0               | 32            | covered                                  |
| `version`   | 2            | 0               | 65            | covered                                  |

### `config` -- HOLE

- **Finding:** NOTHING in the BATS estate invokes `intent config`. Zero files, zero tests.
- **Why it matters:** This family already has no v2 behaviour to be faithful to (0B on both streams, exit 0) and now also has nothing that would notice a change. Both halves of the safety net are absent at the same site: v3 can do anything here and the suite stays green. It is the strongest possible argument for the `undefined` class being separate from `corrected` -- there is neither an antecedent nor a guard.
- **The trap:** `tests/unit/config.bats` EXISTS and burns 5 of 7, which makes the hole invisible in any file listing. It tests config LOADING -- through `intent info`, `intent doctor` and `intent st list` -- and never invokes `intent config` once. A file named after a command that does not test that command is worse than no file, because it answers the question 'is this covered?' wrongly and confidently.
- **Action:** WP-06 must land a conformance test for `intent config` BEFORE changing its behaviour, or the `undefined` ruling is unverifiable by construction.

### `fileindex` -- THIN

- **Finding:** 2 burning tests. `tests/unit/fileindex_commands.bats` holds 47 tests and 45 of them bypass the dispatcher entirely.
- **Why it matters:** The dispatcher path for `fileindex` is almost untested, so most of what the register counts as fileindex coverage does not constrain the v3 binary at all.
- **Action:** Either accept it as a named thin spot in the register, or add dispatcher-level tests at WP-06. Naming it is the minimum; silence would let 47 tests read as coverage.

### `*` -- METHOD

- **Finding:** The zero for `config` was calibrated before being believed: the same needle returns 3 files for `doctor` (a known-covered control), and a direct grep for `intent config` / `run_intent config` returns nothing.
- **Why it matters:** A measuring instrument that reports zero is indistinguishable from a broken one until it is shown to report non-zero somewhere it should. This is the calibration rule that came out of the zsh probe artefact earlier in the same session, applied to the very next instrument built.

### `*` -- METHOD

- **Finding:** Three rows carried `evidence_class: measured` on a probe matrix that could not have reached the field they filled. The matrix is bare / --help / unknown flag / outside-a-project -- four invocations, no positional argument -- and `issues add`, `wp list` and `critic` each REQUIRE an argument to succeed, yet each declared a success-path `stdout`. Found by cc on `issues add` while reading v2 for Machine 4; generalised and bounded to three by ic the same day.
- **Why it matters:** **An `evidence_class` names a METHOD, and nothing checked that the method reached the fields the row went on to assert.** The class was honest -- it says exactly which four invocations were run -- so the defect is not dishonesty but an unstated coverage claim: a reader takes `measured` as being about the ROW, when it is only ever about the PROBE. That is the same shape as a criterion stated more broadly than the one applied, which is what cost `populations.not_probed` a member the day before. It is also why the count matters: the first reading of this was 25 rows, and 22 of those are fine because bare IS their success path (`issues`, `todo`, `info`, `version`, the usage-block families). Only a row whose success needs an argument is exposed.
- **The trap:** `measured` outranks `read` in every reading, so the three exposed rows looked like the BEST-evidenced rows in the table. `issues add` in particular carried a four-invocation matrix in its own `why`, which reads as diligence and is precisely the text that should have raised the question.
- **Action:** All three now carry a direct observation of the success path alongside the matrix, and one of the three was WRONG when measured (`critic` declared only the findings report; the clean case prints a one-line `ok:` summary). No sweep beyond the three: the discriminator is `a required arg the matrix never supplied`, and it is mechanical, so it can be re-run rather than remembered.

## Families outstanding

**None.** Every v2 command family is authored.

## New surface (no v2 antecedent, no parity obligation)

| command  | args         | flags                 | help                                                                                         | owning WP | basis                                                                                                                                                                                                                                                                                                                        |
| -------- | ------------ | --------------------- | -------------------------------------------------------------------------------------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `search` | <query>      | --                    | Full-text search across all authored prose                                                   | WP-06     | design.md:68 -- FTS5 across all bodies, from CLI and MCP. There is no bin/intent_search.                                                                                                                                                                                                                                     |
| `sync`   | --           | --to-disk, --to-store | Sync this machine's store with the committed extract, in both directions                     | WP-06     | hv, 2026-08-14: 'Syncing back-and-forth from disk to db is something that is triggered manually (eg via intent sync) or periodically (eg via intentd running its async synchronisation process).' Daily-driver commands answer from the store and never scan the tree; this is the expensive, infrequent half of that split. |
| `schema` | [face]       | --versions            | Print the generated schema faces (JSON Schema, DDL, GraphQL SDL)                             | WP-06     | design.md:43                                                                                                                                                                                                                                                                                                                 |
| `export` | --           | --format <fmt>        | Extract the store into a portable format usable without Intent                               | WP-06     | design.md:57 -- YAML/md/anything else are export projections                                                                                                                                                                                                                                                                 |
| `ingest` | [path]       | --from-md             | Ingest markdown into the store through the API gate (the recovery path, and the v2 migrator) | WP-03     | design.md:66; WP-03 deliverable, shared with the WP-10 migrator                                                                                                                                                                                                                                                              |
| `backup` | --           | --list                | Snapshot this machine's store -- prunes expired snapshots; one-way, no restore verb ships    | WP-03     | D35 (hv, 2026-08-15): 'a `.backup/` dir in the project that is gitignored could have a per-{day,week,month} rolled-up snapshot of the actual SQLite db file, configurable from `intent config`'.                                                                                                                             |
| `daemon` | <subcommand> | --                    | Manage the machine-level intentd                                                             | WP-08     | design.md:73-74, D07/D08/D19                                                                                                                                                                                                                                                                                                 |
| `mcp`    | --           | --                    | Serve the MCP surface over stdio                                                             | WP-09     | design.md:84, D11                                                                                                                                                                                                                                                                                                            |

### `search`

- **v2:** new-surface
- **acceptance:** AC-06.4 (added by vc, 2026-08-14, on the finding that all 62 ACs had zero coverage of search)
- **disposition:** new-surface
- **MCP:** exposed as an agent tool -- read-only

### `sync`

- **direction selector:** REQUESTED BY cc (2026-08-15): the bare verb refuses and names both directions, and needed a selector row. FLAGS NAME THE DESTINATION, NOT THE SOURCE, because the destination is the side that gets overwritten and therefore the side a user needs to be sure about. `--to-disk` / `--to-store` are symmetric and each says which way the bytes move without the reader having to hold D34's endpoint vocabulary in their head. **The bare verb continuing to REFUSE is right and should stay** -- the two directions have opposite blast radii, so there is no safe default, and picking one would make the dangerous case reachable by typing the short form. This is INV-01 territory rather than convenience.
- **boundary ruled:** RULED by ic, 2026-08-15, after cc asked for it. **THE BOUNDARY IS THE INPUT DOMAIN, NOT THE DIRECTION.** `sync` moves bytes between this machine's store and THE EXTRACT THIS TOOL PRODUCED -- a self-produced artefact, round-trip-guaranteed and lossless by AC-02.6. `ingest` takes FOREIGN markdown -- v2 trees, hand-authored files, anything `sync --to-disk` did not write. **The two have different ERROR CONTRACTS, which is what makes them two commands rather than one**: a `sync --to-store` that drops a field is a bug, because the extract is supposed to round-trip; an `ingest` that cannot parse a v2 file is EXPECTED and must report per-file. cc's proposal was `sync` owns both directions and `ingest` retires, on the ground that both write the store through the same gate. **That reads the MECHANISM as the identity** -- by the same argument `st new` is also `ingest`. The gate is what they share, not what they are. Measured before ruling, they differ on five axes: input domain, error contract, owning WP (WP-06 vs WP-03, the latter shared with WP-10's migrator), MCP exposure (`sync` true, `ingest` false), and implementation state (`sync --to-store` is built; `intent ingest` is declared, reachable and refuses with `is a known command that is not implemented yet`).
- **boundary open to vc:** OPEN TO vc, NARROWED. The ruling above is a command-surface call and is mine (AC-05.1); what is NOT mine is that AC-02.3's ratified prose names _the disk-to-db ingest direction_, so retiring `ingest` would touch a ratified criterion. **I did not need to reach that question** -- I have a positive case for keeping both, so nothing blocks and cc is unblocked either way. If vc reads AC-02.3 as naming the COMMAND rather than the direction, the rows change and NOTHING IN THE BINARY DOES, because `ingest` is unimplemented today. That is the cheapest possible moment to disagree with me.
- **v2:** new-surface
- **note:** NOT the same command as `st sync`, and NOT a superset of it either. v2's `st sync` composes `list` and PRINTS the thread table; only `--write` persists `steel_threads.md` (bin/intent_st:1145-1211, verified by ic). Reconciling the store from canon is a different job, so the two are two commands sharing a name and v3 treats them as such. Added by cc at build time (2026-08-14); second clause originally read "v2's job is a strict subset of this reconciliation and both spellings run it", corrected 2026-08-15 after cc found their own test could not catch it -- it was written from the same misreading as the code, asserted the two spellings produce identical bytes, and passed.
- **truth model correction:** 2026-08-15, ic, under hv's ratified db-is-SSOT model. The help read `Reconcile the runtime store with committed canon on disk` and was backwards in BOTH halves: the store is not runtime, it is the DURABLE SSOT, and disk is not canon, it is a secondary artefact. Corrected here rather than filed because this string is USER-FACING -- it renders to `--help`, the MCP tool list and the `intent llm` guide, so the retracted model would have been the sentence a user READS, in the help for the very command the model is about. `Reconcile` went too: it implies two authorities being arbitrated, and the model is ONE authority with two-way transport.
- **d34 wording:** FINAL wording, released by D34 (hv, 2026-08-15) after I held it pending the multi-machine question. **The DB is per-machine truth and is never committed; the committed extract IS the interchange between nodes**, and a fresh clone reconstitutes its DB by passing that extract through the ingest gate. So the help names both endpoints exactly: `this machine's store` (per-machine, authoritative locally) and `the committed extract` (what travels). D34 adopts the formulation _authority is not bidirectional just because transport is_ -- which is why the string says `in both directions` about the MOVEMENT and says nothing about precedence. A help line implying the file could win would describe a different architecture.
- **disposition:** new-surface
- **recoverability:** idempotent
- **Flags:**
  - `--to-disk` (bool) -- Write the store out to the committed extract
    - **disposition:** keep
  - `--to-store` (bool) -- Read the committed extract into the store, through the API gate
    - **disposition:** keep
    - **disposition basis:** RULED `keep` by ic, 2026-08-15, on the boundary declared at `boundary_ruled` below. It was `pending` on the ground that `sync --to-store` and `ingest` might be two spellings for one act; they are not, and the flag ships. cc had already implemented it and stopped rather than let the code answer the open question -- which was right, and is the reason this row is ruled rather than discovered.
- **MCP:** exposed as an agent tool -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Leaned OPEN against the standing lean because sync is ordinary workflow, and it is the one row where that lean is least comfortable: it moves truth in BOTH directions, so a wrong `--to-store` can overwrite this machine's store from a stale extract. **The stated closing condition was TESTED and DID NOT FIRE**: it said _if the boundary with `ingest` is drawn so that `sync --to-store` is the recovery path, this should close_, and `boundary_ruled` draws it the other way -- `sync` is routine reconciliation of a self-produced artefact and `ingest` is the recovery path. So the lean STAYS OPEN, and it stays open because a condition was checked rather than because nobody came back to it. Recorded rather than deleted: a closing condition that was evaluated and not met is a different fact from one still pending.

### `schema`

- **v2:** new-surface
- **disposition:** new-surface
- **Flags:**
  - `--versions` (bool) -- Print each face's INTENT_VER and SCHEMA_<TYPE>_VER instead of the face bodies
    - **disposition:** keep
    - **disposition basis:** REQUESTED BY cc (2026-08-15 17:47Z) for AC-06.10(b) / D41: _a surface reports both parts for every face, so a consumer deciding whether to upgrade can ASK rather than diff_. `keep` from the moment it is authored, because the decision is made and the obligation to read it is cc's to discharge -- `pending` would say the flag is UNDECIDED, which is the one thing it is not.
    - **ordering note:** cc asked which of us goes first and the answer is the table, which is why this row exists before the reader. **Both orderings are the EXP-05 defect in opposite directions**: reader-first makes `--versions` a flag clap REJECTS, row-first makes it a flag clap ACCEPTS and the renderer ignores. Row-first is the recoverable one -- the flag appears in `--help` and does nothing for the hours it takes cc to land the reader, and `surface_check.sh` reports it as PRESENT-vs-`keep` the entire time, so the window is INSTRUMENTED rather than merely short. Reader-first fails at the clap layer where no instrument of mine can see it.
    - **composition with face arg:** `--versions` selects the OUTPUT MODE; the existing `face` arg selects WHICH faces. So `intent schema --versions` prints one line per face and `intent schema ddl.sql --versions` prints that face's line only -- the two compose without a new rule, and no arm has to special-case the other. Declared here rather than left to build time because an undeclared composition is how two authors end up with two answers, which is the same defect this row's own boundary question was.
    - **spelling note:** `--versions` PLURAL, and the singular is not merely a style choice: `-V, --version` is a clap intrinsic on the root command. MEASURED 2026-08-15 -- it does NOT propagate to subcommands (`intent schema --version` gives `unexpected argument`), so the singular would not have collided today. It is still the wrong name, because it would print several lines and because a future `propagate_version(true)` would make the collision real without anyone touching this row.
- **MCP:** exposed as an agent tool -- read-only

### `export`

- **v2:** new-surface
- **not backup:** SEE `backup`, which carries the full distinction. In short: this is the INTERCHANGE -- lossless text, usable without Intent, the artefact that travels between machines under D34 and reconstitutes a DB through the ingest gate. `backup` is a binary SQLite snapshot for fast local restore, carrying the derived index. Both help strings carry their own distinguishing clause deliberately, because a user choosing between them is not reading them side by side.
- **truth model correction:** 2026-08-15, ic. The help read `Project the canon into another format`, which named the DISK side as the canon and so read as one on-disk format converting to another. Under hv's ratified model the store is the truth and this command is the OPENNESS half of it -- hv, verbatim: 'I can get my data out of the db and use it somewhere else LOSSLESSLY.' The `usable without Intent` clause is in the help deliberately: AC-02.6 requires the file form to be usable without this tool, and a promise a user cannot read is a promise nobody can hold us to. This is the surface half of AC-02.6; vc owns whether the contract wants it cited on this row.
- **disposition:** new-surface
- **read or mutate correction:** CORRECTED 2026-08-16 (ic). Was `mutate`. Found by cc while building the renderer and deferred here rather than resolved there, because the table is ic's -- `render.rs:1215` states the case in full. **The old defence was reasoned about a command that does not exist as declared**: it justified `mutate` by working-tree clobbering, which requires an output path this row has never carried. Sound reasoning, wrong subject. Two independent routes agree on `read`, which is why this is a correction rather than a preference: (1) the definition quantifies over EVERY flag, and the only flag selects a format rather than a destination; (2) `schema` is structurally the same command -- one flag, a face printed to stdout, zero destinations -- and is declared `read`, so counting shell redirection as mutation would make `schema` a mutation too and leave the table disagreeing with itself about one shape. **The MUTATE lean does not preserve the old value.** The lean governs UNCERTAINTY -- it is the tie-break when the answer is genuinely unclear -- and nothing is unclear once the flag set is read; treating a lean as a floor would make the field unfalsifiable in one direction. The cost of leaving it wrong is the failure the agent guide exists to prevent: an agent reading `mutate` treats `intent export` as capable of clobbering its working tree, and either avoids the command or copies files defensively before calling it. That is a capability the tool does not have and a workaround built against a phantom.
- **Flags:**
  - `--format` `<fmt>` (string)
    - **disposition:** keep
- **MCP:** exposed as an agent tool -- read-only
- **Wants review -- the classification disagrees with the verb name:** `export` sounds like it produces a FILE and it does not -- it writes the document to STDOUT, so `intent export > estate.json` is the SHELL creating the file, not the command. Under this table's own definition the row is a `read`: `read` means no invocation of the entry, UNDER ANY FLAG, changes durable state, and this entry declares exactly one flag (`--format`, which chooses a projection) and zero arguments. Quantifying over the flag set is the whole test, and there is one flag to quantify over.
- **MCP classification grounded in:** This row's own `flags` and `args`, plus `render.rs`'s `export()`, which calls the facade and `print!`s the result.

### `ingest`

- **input selector gap:** CLOSED at the `path` arg above (ic declared, cc ruled the default), 2026-08-15. Kept as the record of what the gap WAS: **`intent ingest` had no way to say what to ingest** -- measured, `Usage: intent ingest [OPTIONS]`, one bool flag and no positional. The recovery path and the v2 migrator both inherently take a source, and the migrator's source is ANOTHER project's tree, which cannot be implied by the working directory.
- **v2:** new-surface
- **truth model correction:** 2026-08-15, ic. The help read `Rebuild the canon from markdown`, which under the retracted model meant reconstructing the durable thing and so read as an authority-restoring act. Under hv's ratified model it is the opposite: markdown is a secondary artefact and ingest is the path INTO the truth, well-formed ONLY because it passes the hard gate of the intentsvcs API. `through the API gate` is in the user-facing string on purpose -- the gate is what makes the result trustworthy, so hiding it would let a reader assume a file's own format was sufficient. Recreation from an extract stays a CAPABILITY and is not a licence to treat the store as disposable.
- **disposition:** new-surface
- **recoverability:** one-way
- **Flags:**
  - `--from-md` (bool)
    - **disposition:** keep
    - **disposition basis:** RULED `keep` BY vc, 2026-08-15, on my own proposal to withdraw it -- against the proposal. It was `pending` first as half of the undeclared `sync`/`ingest` boundary (settled at `sync.boundary_ruled`: input domain, not direction), then on the narrower ground that **`--from-md` is a mode flag with ONE mode** -- reading md into the store is what `ingest` IS, so the flag distinguishes nothing. **vc: that objection is REAL and is not resolved by a table edit.** `acceptance.md` does not merely mention this flag, it RULES that the scaffolding ships in WP-03 with acceptance at AC-10.2/10.3, so **withdrawing it for tidiness would put this table in contradiction with a ratified row -- AC-05.5's exact class, which is what I caught myself about to do.** The objection belongs at AC-10.2/10.3 where its acceptance actually lands, raised against the thing itself rather than against a row describing it.
    - **why not dropped by ic:** I RULED IT DROPPED AND THEN REVERSED MYSELF ON EVIDENCE, 2026-08-15, and the evidence is the point. **`--from-md` is cited outside this table in FIVE live places across FOUR artefacts I do not own**: `design.md` (design canon), `acceptance.md` (ratified -- and it explicitly says _the scaffolding still ships in WP-03_, with acceptance at AC-10.2/10.3), `parity.md` (the WP-01 register's new-in-v3 row), `WP/03/info.md`, and `intentsvcs/tests/prose_ingest_fts.rs`. Dropping it here would have made this table disagree with the ratified contract -- **the exact decision-drift class AC-05.5 was landed for an hour earlier, manufactured by me, in my own artefact, on the same day.** The command surface is mine (AC-05.1); a spelling the contract NAMES is not mine alone. Proposed to vc instead.
    - **count corrected twice:** THE COUNT IS FIVE AND IT WAS WRONG TWICE, WHICH IS WORTH MORE THAN THE COUNT. I first wrote SEVEN and **listed six** -- an off-by-one in my own sentence, against my own list, in the same breath. cc then corrected me to SIX, because `intentsvcs/src/ingest.rs:280` genuinely stopped naming the flag under their D37 rewrite -- but cc reached six by SUBTRACTING ONE FROM MY SEVEN rather than recounting, so the correction inherited my error. **Re-measured: five.** Neither of us was careless; both of us trusted a number that had been written down. That is the same failure as every citation defect today, at the smallest possible scale. **LINE NUMBERS DELIBERATELY REMOVED FROM THIS ROW.** They were `design.md:67`, `acceptance.md:298`, `parity.md:70` -- and `acceptance.md:298` had already rotted to `:302` within the hour, because vc added rows above it. **A citation that names a line is stale as soon as anyone edits the file above it, and it goes stale SILENTLY.** This is live evidence for the AC-05.5 citation format being IDS rather than locations, which is what I proposed to vc before finding it.
- **MCP:** not exposed -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - The recovery path and the v2 migrator behind one verb. Both are operations a human decides to run after something went wrong; neither is a task step. Closed on the strength of what it is FOR rather than what it costs.

### `backup`

- **v2:** new-surface
- **config:** Reads `backup.enabled` / `backup.schedule` / `backup.retain.{daily,weekly,monthly}` -- named on the `config` entry in this table. `backup.enabled` gates the DAEMON's schedule and deliberately does NOT gate this command.
- **not export:** *** `backup` AND `export` ARE NOT SYNONYMS, AND CONFLATING THEM COSTS A USER THE THING THEY WERE TRYING TO SAVE. *** `export` is AC-02.6 OPENNESS: lossless, text, usable WITHOUT Intent, and under D34 it is THE INTERCHANGE -- the artefact that travels between machines and reconstitutes a DB through the ingest gate. `backup` is a binary SQLite snapshot: NOT usable without SQLite, NOT the interchange, and it carries the DERIVED INDEX so a restore is immediate with no re-ingest and no re-index. Different jobs, and neither is redundant. The failure mode is directional and asymmetric: a user who reaches for `backup` when they wanted portability gets a file no other tool can read, and a user who reaches for `export` when they wanted a fast restore gets a correct artefact that costs a full re-index -- so BOTH help strings must carry their own distinguishing clause rather than relying on a reader comparing them side by side, which is exactly what nobody does at the moment they need one.
- **disposition:** new-surface
- **recoverability:** one-way
- **note:** ONE-WAY IS RULED, NOT AN OVERSIGHT (hv, 2026-08-17: `Backup is fine to be one-way for now`). The family ships the creating half without the undoing half and that is now a decision with a ratification rather than a gap. **The help string changed with the ruling and that is the point of recording this**: it read `Snapshot this machine's store for fast local restore`, which promised a `restore` verb that does not exist anywhere in the tool -- exactly 1 of 107 shipped rows promising a capability with no implementation, and the one my register CANNOT mechanise, because a derived detector matches a promised thing against a shipped row and this promise has no row to match. So the measurement was hand-made and the fix is a help string, not a check. `for now` is hv's word and is kept: the ruling is that no restore is owed TODAY, not that one never will be. The sibling `ext` family is one-way on different and firmer grounds -- what `ext new` creates is a directory in the operator's own HOME, so its inverse is a filesystem operation the user already owns, where a backup's inverse would have been a real capability.
- **Flags:**
  - `--list` (bool) -- List retained snapshots and when each was taken
    - RATIFIED -- vc, 2026-08-15: `--list` STANDS. Proposed by ic on the grounds that D35 requires a failed or skipped backup to SURFACE, and with no read path a user cannot tell a working schedule from one that has silently never run -- the nothing-is-wrong / nothing-ran ambiguity again. **The question turned out to expose a hole in AC-03.10 itself**: the clause said only that a FAILED backup surfaces, and A SCHEDULE THAT NEVER FIRES PRODUCES NO FAILURE TO REPORT, so a green implementation could have shipped where nothing had ever run -- the ambiguity living inside the clause written to prevent it. AC-03.10 is amended and `doctor` now reports backup STALENESS, which detects never-ran without needing anything to have failed.
    - **disposition:** keep
- **surfacing:** RATIFIED -- vc, 2026-08-15: the split is the ruling. A failed or skipped scheduled backup must surface (IN-AG-NO-SILENT-001), and **`doctor` is THE ONE PLACE health is reported** -- not a second status surface here -- because `doctor` already exists for exactly that and Highlander says one place, not two. `--list` answers only _what snapshots exist_, which is inventory rather than health. `doctor`'s obligation is recorded on its own row as `target.new_obligations`, so the requirement lives where the command that owns it is described.
- **exposure:** PROVENANCE EXPOSURE (ic, audit 2026-08-15; twice corrected). NOT a D42 exposure -- I filed it first for emitting a time, which is legal, and the residue is not about clocks at all. `--list` promises to show WHEN EACH SNAPSHOT WAS TAKEN and answers from the file mtime of a snapshot under .backup/db/, so a filesystem attribute is DISPLAYED AS a record fact. vc reached the identical ruling from the contract end and has absorbed it into AC-02.8: `threads.created` and `issues.created` are authored dates, `file_index.mtime` is a file mtime, `event_log.ts` was an argument, and this snapshot mtime is the fourth instance -- in all four the display was never the problem and the provenance always was. The fix is unchanged and now correctly reasoned: taking a snapshot WRITES A RECORD and `--list` reads that record stamp. Owned under AC-02.8; recorded here so the surface row carries its own obligation.
- **MCP:** not exposed -- **mutates**
- **Wants review:**
  - uncertain on `exposed_on_mcp`
  - Genuinely harmless -- it writes a snapshot into `.backup/db/` and touches nothing else. Closed only by the standing lean, which is the weakest reason on this list.

### `daemon`

- **v2:** new-surface
- **disposition:** new-surface
- **recoverability:** one-way
- **MCP:** not exposed -- **mutates**
- **MCP note:** The canonical example behind the whole exposure field: `daemon start|stop|run` is machine-level process control, and it is the row that makes 'lean closed' the right default rather than a nicety.

### `mcp`

- **v2:** new-surface
- **disposition:** new-surface
- **recoverability:** one-way
- **MCP:** not exposed -- **mutates**
- **MCP note:** The MCP server's own launcher. Listing it inside the tool surface it serves is recursive, and an agent that can call it can spawn a second server against the same store.
