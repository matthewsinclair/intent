# Intent v3.0.1

**v3.0.1 is a correctness release for people already running v3.0.0.** It repairs a packaging fault that left a Homebrew install without part of its support tree, and two defects in the criteria surface that destroyed authored text without failing.

**If you installed v3.0.0 from Homebrew, upgrade.** Two of the three faults below are in the build you have, and one of them is silent.

## Provenance

**These notes were written before the cut, which is the only way they can ship inside it.** Every claim below was verified against the tree the release is being cut from, and against the published `v3.0.0` tag (`80d8b2ca`) for the "before" half. Anything ruled but not yet landed is deliberately absent rather than described in advance: a release note that documents a verb the release does not contain is the same defect as a manual page that does, and it is harder to withdraw.

## Fixed

**A Homebrew install of v3.0.0 has no rule library and no skills.** The formula's copy list did not match what the binary resolves at runtime, so the keg is missing `intent/plugins/claude/rules/` and `intent/plugins/claude/skills/`. The failure is per-command rather than global, which is why it survived: `intent st` and the rest work normally, and only `intent claude rules list`, `intent claude rules show`, `intent critic <lang>` and the skills verbs fail. **There is no workaround in v3.0.0 short of a source install**; the fix ships here, and the copy list is now checked against its consumer rather than maintained beside it.

**The same keg cannot run `intent claude ws` or `intent claude start`.** The support tree also omitted `intent/plugins/claude/bin/intent_claude_cwi`, which both resolve against, so the whiteboard provisioner is unavailable on a Homebrew install of v3.0.0. **The keg contradicts itself on this**: the whiteboard skill it ships says scaffolding a node is the job of `intent claude ws new`, and the same keg cannot run that command. It is one fix with the trees above, not a second one.

**`intent ac new` on an id that already exists destroyed the row it collided with, and there was no edit verb to reach for instead.** In v3.0.0 the create overwrote the criterion's text, kind and state, reporting success. It now refuses, names the id it would have overwritten, and points at the verb that does the thing you meant. **This was the most expensive command in the tool**: the ordinary way to hit it is to retype a criterion you meant to reword, which is exactly when the row you destroy is the one you were being careful about. `intent at new` carried the same shape and is refused the same way.

**Migrating a criterion authored unsatisfied destroyed its evidence clause, by construction, while exiting 0.** A criterion standing unsatisfied with a note saying what was measured or what was blocking lost that note on the way in. **This was not a parsing fault and no parsing fix could have reached it**: the model's `Unsatisfied` state was a unit variant with nowhere to put the text, so a wildcard match arm routed the case to a state that could not represent it. The state now carries the note, and the arm is written as three explicit cases rather than two and a catch-all, so a fourth case cannot be added without someone deciding what happens to the prose.

**That distinction is worth stating plainly, because the defect was first recorded as ingest damage and it is not.** Ingest damage is a migration artefact you survey and repair once. A state the model cannot represent destroys the same data every time anything hops, including hops that have not happened yet. The earlier framing survives in the issue record because Intent has no verb that can rewrite an issue body, which is itself an open defect.

## Added

**`intent ac edit`** — change a criterion's text without touching its satisfaction. This is the verb `ac new` now points at, and its absence is what made the destructive create reachable.

**`intent at edit`** — the acceptance-test sibling: re-cite a test's file or coverage while keeping the status and the note that a re-create would have reset.

## Changed

**`intent st edit` opens an editor on a terminal and prints the path into a pipe.** In v3.0.0 it printed the path in both cases. `--editor` and `--path` force either branch, and they exist for a stated cost rather than for symmetry: a bare terminal test makes behaviour depend on an invisible property of the environment, so a wrapper, a CI job or an editor plugin gets a different result with nothing in the command saying why. **If you have scripted `intent st edit` expecting a path on stdout, you are already in the branch that still prints one** — a script's stdout is not a terminal — but the override is there to say so explicitly.

## Removed

**Nothing that worked in v3.0.0 is removed here.** If you have a working script, this release does not break it.

**`intent st repair` is now declared retired, and it never ran.** In v3.0.0 it was a declared command with no implementation: invoking it answered `intent st repair is a known command that is not implemented yet`. It now answers `intent st repair was retired in Intent v3 and is not a command in this build`. **The message changed and the capability did not, because there was none.** A script calling it failed before this release and fails after it.

**An earlier draft of this section said the opposite** -- that `st repair` shipped in v3.0.0 and that this was the one command in the release that works in the version you are on. That was wrong, and it is worth saying how, because the same mistake is available to anyone reading our register. The register's `shipped` population is derived as everything declared minus everything retired; `st repair` was declared and not retired, so it was counted. **`shipped` answers whether a row is declared and live, not whether the command was ever built** -- and nothing in the count says which question it answered.

## Upgrading

```
  $ brew upgrade matthewsinclair/intent/intent
  $ intent --version
  $ intent claude rules list
```

**The third line is the check that the support tree actually arrived**, which is the fault this release exists to fix. It is the direct test rather than a proxy: that command reads the rule library out of the install, so it fails on exactly the packaging fault described above and succeeds only if the tree is there. Run it once rather than assuming.

**No migration is required and no project data changes.** A project already readable by v3.0.0 is readable by v3.0.1.

**Evidence already destroyed by the third fault is not recovered by upgrading.** The fix stops the loss; it cannot reconstruct text the store never held. If you migrated an estate under v3.0.0 and criteria authored unsatisfied carried evidence clauses, that text is in your v2 history and not in your store.
