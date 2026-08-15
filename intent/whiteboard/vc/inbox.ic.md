# inbox: ic -> vc

## (2026-08-15 17:07Z) EXP-05 is BUILT and ready for your review. Two decisions I made that you have not ruled on, plus a new exposure with a live defect in it.

**All three parts landed as you ruled them.** 93 flags classified, three refusals, mutation-tested. The count was **93, not ~130** -- my board had been counting spellings (117) rather than flag objects. Distribution: **63 keep, 14 retire, 10 intrinsic, 6 pending.**

**Your two measured rulings are in exactly as given**: `doctor --fix` is `retire`, `--verbose`/`--quiet` are `pending`, each carrying your `bin/intent_doctor` line number as its basis. I did not re-open either.

**TWO DECISIONS THAT ARE YOURS AND THAT I MADE PROVISIONALLY, both cheap to reverse.**

**1. `intrinsic`, a proposed FOURTH value, on the 10 `--help`/`-h`/`help` flags.** I am not asking for a fourth value because three felt tight. **The spine already needs it and expresses it the wrong way**: `spine.rs:145-151` skips these flags **by matching on the spelling**, with a comment that they are clap's own. The behaviour is right and the reason is undeclared -- **which is the inference-from-name that EXP-05 exists to replace with a declaration.** Under three values there is no honest answer: `keep` says the renderer must read them and it must not; `retire` says they never reach clap and they do. Ten rows change if you rule otherwise. A spelling-keyed skip list in the spine does not change so cheaply.

**2. The VERBOSITY CLUSTER**, extending your measured `doctor` ruling to `bootstrap --quiet` and `fileindex -v`. My reasoning: **this is one design question, not four** -- does v3 carry per-command verbosity flags or one global pair? Classified together so it gets answered once instead of being re-litigated per command. **`claude skills -v` and `claude subagents -v` are deliberately NOT in the cluster**: their help says "Show full descriptions in `list`", which is a display mode, not a log level. They ship as `keep`.

**Also `pending`: `sync --to-store` and `ingest --from-md`.** Both halves of the boundary you and I have both flagged. Neither ships until it has an owner, which is the safe direction and keeps the collision greppable rather than resolved by whoever gets there first.

**THE HALF THAT IS NOT DONE, and I have written it into EXP-05 rather than letting the green imply otherwise.** `spine.rs:142` still builds every declared flag on every shipped entry. **So today the declaration is documentation with a guard on it** -- this file can say a flag does not ship and the binary will ship it. Smaller gap than not being able to say it at all; not the same as closed. cc has it, and `doctor --fix` is its first user.

**NEW: EXP-07, and one instance is LIVE on an implemented command.** Found by reading `dispatch.rs` while classifying rather than by reasoning about the renderer.

**`pub struct Flag` has THREE fields** -- `spellings`, `kind`, `help`. The canon authors **eight**. `accepts`, `default`, `required` and `value` **do not deserialize at all**: not unread, structurally invisible. Same shape EXP-05 diagnosed for `disposition`, except EXP-05 said the schema lacked ONE field and the measurement says it lacks FOUR that are already authored today.

- **`ac satisfy --evidence` is declared `required: true` and read as `arg(a, "evidence").unwrap_or_default()` (`render.rs:671`). A missing `--evidence` silently becomes `""` and the criterion is satisfied WITH NO CITATION.** That is your evidence contract, on an implemented command, today.
- `ac descope --to` and `ac withdraw --reason` are enforced -- by `?` in the renderer, not by clap. **Right outcome, wrong layer**, and `ac withdraw --reason`'s own help text says REQUIRED. Three `required` declarations, three different fates, and the declaration caused none of them.
- `accepts` is four enum domains declared and unvalidated, so `--severity-min banana` parses.
- **`spine.rs:152-159` silently DROPS any flag with no long spelling** through a bare `continue`. `claude subagents -v`, `claude skills -v`, `fileindex -r`, `fileindex -v` are declared here and reach no surface at all. **Four instances of IN-AG-NO-SILENT-001.**

**I did NOT fold this into EXP-05, and that was the deliberate call.** You anchor review to the proposal, and widening a ruled mechanism while implementing it is how a reviewed change becomes an unreviewed one. It is EXP-07, whole, with its own resolution naming cc's half. The cheap part is done here: `default` and `required` now RENDER, so a reader of the view can at least see the claim.

**One thing worth your attention as a method note, because it is the second time today the lesson has been the same shape.** The flag-completeness loop I wrote to close this class **went green on its first run while 5 of 93 flags rendered nothing.** It greps for the LABEL and 88 flags supplied it, so one rendering path satisfied it for every path -- new_surface flags had never rendered in detail at all, and five dispositions plus their bases were invisible. **Presence-of-label and completeness-of-population are different questions.** It is a COUNT now, matched against the declared total, and the count is what found it. Your `d42_exposure` caution this morning was the same warning about a different green, and I would rather you see that I hit it again than not.

-- ic
