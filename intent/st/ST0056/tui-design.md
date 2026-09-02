# TUI and form-DSL design (WP-17, with WP-08's web realiser)

**Status: design, ratified in conversation with hv on 2026-08-29, not yet built.** Everything here was arrived at by driving a working strawman against real ST0056 data (154 criteria, 161 tests, 297 attachments, 17 work packages) rather than by sketching. **Where a decision reversed, the reversal and its cause are recorded, because the reason is the part that does not survive in the code.**

The strawman is Rust + ratatui 0.29 + portable-pty + tui-term. It is READ-ONLY against the model and is not the deliverable; this document is.

## 1. What is being built

`intent edit <kind> <id>` opens an entity in a TUI. `--browser` opens the same model in a browser served by `intentd`. Both are realisers over one declaration; neither is a second implementation of the model.

**The form is declared, not coded.** A YAML declaration carries LAYOUT — order, label, widget, editability — and takes field existence and type from the entity's schema face. **It never enumerates the field set.** A hand-authored field list is a second home for what `schema/*.json` already declares and goes stale exactly the way `populations.shipped` did; that is `AC-17.2`, and the converse is held too — an editable property appearing in no form is named rather than silently unreachable.

Covers steel threads, work packages and issues.

## 2. Screen layout

Three sections, named by hv, separated by two rules. **There are no borders anywhere; those two rules are the only chrome.** _(Revised with the omnibox machine, hv 2026-08-30: the foot collapsed from three standing rows to two — the omnibox line and one mode-chipped hint line — on the minimal-chrome ruling, and the screen boots into the threads list rather than a kind lobby.)_

```
 ST0056   Add a Rust-based CLI with a local SQLite DB with bidirectional sync…   /thread/ST0056   ESC back
──────────────────────────────────────────────────────────────────────────────────────────────────
 ▸ title        Add a Rust-based CLI with a local SQLite DB with bidirectional…
   status       wip                                    ⏎ choose
   objective    673 bytes                              ⏎ $EDITOR
   documents    297   2,318,041 bytes                  ⏎ open
   work pkgs    17                                     ⏎ open
──────────────────────────────── detail ──────────────────────────────────────────────────────────
   kind    test
   text    **A CHECKER VERIFIES MEMBERSHIP IN A VOCABULARY** and never that the …
──────────────────────────────────────────────────────────────────────────────────────────────────
 ❯ 56▏
 NAV  4/12  ⏎ open  ←→↑↓ move · ⌫ back · type to find
```

- **APP ROW** — the entity's id and name; when nested, the view trail and the key that leaves.
- **BODY** — a flat `{name, value, type}` column. Splits into list + detail where the selected row has detail. **The dropdown borrows the body's last rows** while the omnibox has matches: best match nearest the input, matched letters highlighted, the pick wearing the caret.
- **OMNIBOX ROW** — always present: caret + buffer, bright while it holds the keyboard, dim in NAV; the menu bar borrows this line in MENU.
- **HINT ROW** — the mode chip first and unconditional, coloured per mode; then a standing notice, or what the keys do right now (position, the one verb ⏎ means on this row, the crossing where one exists).

### Layout rules

- **Two aligned columns: names in one, values in another.** That alignment IS the design, so it is asserted, not eyeballed. The gutter is **computed from the row set**, never hardcoded — the first strawman pinned it at 13 and real data collided on the first render.
- **Values are clipped at RENDER time to the terminal width**, with an ellipsis. Never truncated when the row is built: a value truncated into the model is truncated for every width forever.
- **A value that does not fit is clipped, never wrapped into a second row** — a wrapped value breaks the one guarantee the layout makes.
- Unicode box-drawing for the rules. **Colour is carried as ROLES computed by layout and mapped to a palette by the printer** — the mode chip per-mode (OMNIBOX cyan, NAV green, MENU magenta, FIELD yellow, EMBED red, reversed), doors cyan, field names dim, statuses semantic from the model's own display vocabulary (wip yellow, done green, blocked red), selection reversed, notices amber, matched dropdown letters bold accent.

## 3. The mode machine

**Declared as a table; the controller reads it.** This is the estate's own idiom — `transitions.rs` declares edge tables and a machine check holds the code to them — applied to the controller, because a controller finagled into life by trial and error has no table anyone can review.

**Revised by hv 2026-09-02, superseding the five-mode omnibox machine at the same provenance strength -- driven, then ruled.** hv's instruction was to make `explore` read like Claude Code, aimed at the lower bar: _I see two modes: Omnibox and Menu. And in OMNI mode, /commands fire up the menus, and anything else is omni-dispatched from the Omni._ On the mode model specifically: _'cleverer' isn't necessarily 'betterer'._

**THE COMPOSER IS THE ONE HOME, AND `NAV` IS GONE.** Claude Code's coherence comes from three things the five-mode machine traded away: ONE input that is always home, ONE meaning for `/`, and editing being a place you go and return from rather than a mode you steer between. So `OMNIBOX` and `NAV` collapse into **OMNI**: the composer always holds the text cursor, and the body is BROWSED rather than entered.

**WHAT WAS A MODE IS NOW A GUARD, WHICH IS WHY THE TABLE DID NOT GROW.** The superseded machine told _arrows pick matches_ from _arrows move the cursor_ by being in two different modes. One guard replaces it -- **is the composer buffer empty?** -- and it is the same species of guard as pane focus: it changes what a key DOES without changing which mode you are in.

**SUPERSEDED, KEPT INLINE SO A LATER READER MEETS THE ARGUMENT RATHER THAN RE-RUNNING IT.** The machine this replaces read: _OMNIBOX is the rest state ... NORMAL is renamed NAV, hv's own word for it_ (hv, 2026-08-30); **`/` was the single mode-advance key cycling NAV -> OMNIBOX -> MENU -> NAV** (hv Option A, 2026-08-31, _`/` just cycles thru the three_); and **Esc toggled the two home modes {OMNIBOX, NAV} and never quit**. All three are retired together, and they fell together for one reason: each was a device for moving between places the operator now never leaves. The ring cost the Lotus menu two keystrokes and gave `/` a meaning that depended on where you already were; the toggle gave Esc a second job.

**THE TRIGGER COLUMN'S SPELLING CHANGED AFTER hv RATIFIED IT, AND HERE IS WHY.** The accepted proposal wrote the buffer guard INTO the trigger cell -- `Enter (buffer set)`, `Enter (buffer empty)`, `Move (buffer empty)`. **That cannot be built.** `keys.rs` rests on a stated invariant -- the keymap cannot see the buffer -- so it emits a bare trigger and the app applies the guard, which is how `/` has always worked here. A guarded trigger is one no keystroke can ever produce: `step(OMNI, "Enter")` would answer _nothing_ on the commonest key on the screen. So **triggers are BARE in this table and the guard is stated in the notes column**; the SEMANTICS hv ratified are unchanged in every particular. The correction also keeps the invariants simpler, which is the argument for it rather than a consolation: spelling the guard in would have split the one guarded pair into two and made _exactly one guarded ambiguity_ stop meaning anything.

| from  | trigger            | to    | notes                                                |
| ----- | ------------------ | ----- | ---------------------------------------------------- |
| OMNI  | Typing             | OMNI  | into the composer -- it always holds the keyboard    |
| OMNI  | Move               | OMNI  | empty buffer browses the body; a query picks matches |
| OMNI  | Enter              | OMNI  | go to the picked match, or descend a door row        |
| OMNI  | Enter              | FIELD | edit in place -- editable rows                       |
| OMNI  | Enter              | EMBED | hand off -- prose rows to `$EDITOR`                  |
| OMNI  | `/`                | MENU  | open the palette, one press, empty buffer only       |
| OMNI  | Esc                | OMNI  | clear the query; a no-op when already empty          |
| OMNI  | Back               | OMNI  | pop the view stack, empty buffer only                |
| MENU  | Typing / Move      | MENU  | filter the palette; pick among the hits              |
| MENU  | Enter              | OMNI  | run the picked command                               |
| MENU  | Esc / Cancel / `/` | OMNI  | close the palette                                    |
| FIELD | Typing             | FIELD | in-place edit, one keymap                            |
| FIELD | Enter              | OMNI  | commit                                               |
| FIELD | Esc                | OMNI  | discard                                              |
| EMBED | Typing             | EMBED | forwarded to the child                               |
| EMBED | ChildExit          | OMNI  | read the file back                                   |

**MENU'S ROWS WERE REVISED AGAIN ON 2026-09-02, AFTER hv DROVE THE BUILD, AND THE TWO RETIREMENTS ARE STATED AS BEHAVIOURS RATHER THAN AS ABSENCES.**

- **`MENU Typing -> MENU` is ADDED**: the palette filters as you type. This is the row that makes `/quit` reach quit.
- **`MENU Hotkey -> MENU` is RETIRED, removed rather than rehomed.** It was the Lotus bar's accelerator, and it was **declared, emitted by the keymap, answered by this table, and consumed by no realiser for its entire life** -- a bound, reachable, inert key. In a palette a letter has an obvious job, so the trigger is gone rather than given a handler. A dead trigger rehomed is a dead trigger with an alibi.
- **`MENU Back -> OMNI` is RETIRED, and what replaces it is affirmative: `Backspace` ERASES in the palette, and erasing back past the `/` IS the exit.** The palette therefore needs no exit key of its own. Stated this way round because a retirement recorded only as a missing row is indistinguishable from one nobody noticed dropping.

**LEAVABILITY SURVIVES THE RETIREMENT, AND IT IS ASSERTED HERE SO THE NEXT READER DOES NOT HAVE TO RE-DERIVE IT.** Removing `MENU Back` removes one of MENU's exits, so the invariant is worth re-checking rather than assuming: MENU still leaves by `Esc`, `Cancel` and `/`, all three landing in OMNI, so **every state that owns its escape still reaches the home mode in ONE press**, and EMBED remains the single named exemption.

**FOUR MACHINE STATES, THREE LAMPS.** `FIELD` and `EMBED` stay distinct in the machine because their EXITS differ -- `EMBED`'s is the child exiting -- but the chip shows both as `EDIT`, because which of the two you are in is a fact about who owns the terminal and not something the operator can act on. Showing a lamp per internal state would advertise a distinction nobody can use.

**Two invariants, asserted at startup and checkable headlessly:**

- **Every mode is leavable.** A mode you can enter and not leave is the trap `no_state_can_be_entered_and_not_left` already refuses for entities.
- **Every mode is reachable from OMNI.** An unreachable mode is dead code that reads as a feature.

**ESC IS TOTAL, WITH EMBED AS THE ONE NAMED EXEMPTION, AND IT LANDS IN THE HOME MODE IN ONE PRESS.** Esc means _back to the composer_: in MENU it closes the palette, in FIELD it discards, with a query typed it clears the buffer. **The exemption is EMBED and it is declared rather than filtered out** -- a child process owns the terminal while it runs, so Esc reaches `$EDITOR` and not us, and EMBED's only exit is the child ending. That is the whole narrowing: `total` still means every state the TUI owns the keyboard in, and the one state it does not is named here rather than dropped silently.

**ESC ON AN ALREADY-EMPTY COMPOSER IS A NO-OP, BECAUSE YOU ARE ALREADY HOME.** Stated affirmatively: this is the behaviour, not a case nobody got round to. Esc never navigates -- popping the view stack is `Back`'s job, and overloading Esc with it would give it the second job the retired toggle was retired for. **Home is now ONE mode**, so Esc converges rather than oscillating between two rooms. **Quitting stays an act, never an accident** -- `Ctrl-C` from anywhere. Save protection still names its own escape: `unsaved edits -- :w to write, :q! to discard`.

**The `OMNI + Enter` triple is the one guarded ambiguity**, resolved by the ROW and never by table order: a row with a door descends, an editable row edits in place, a prose row hands off. The door arm is the strawman's worst defect fixed -- Enter on a `button` row used to reach FIELD, so the one navigation verb on screen navigated nowhere, which is what hv drove into on 2026-08-30. **That it is still exactly ONE guarded pair after the collapse is the evidence that folding NAV in removed a mode rather than smuggling a second ambiguity in behind it.**

**Pane focus (list / detail) is a GUARD on OMNI's edges, not a fifth mode.** It changes where Move and Enter land; it does not change what the keys mean. The buffer condition is the same species, and it now governs three keys: mid-query `/` is a character (`st/ST0056` is a legal spelling), `Backspace` deletes rather than walking up the model, and the arrows pick among matches rather than browsing the body. **One rule asked at three keystrokes, never three rules.**

## 4. Keys

**ONE HOME MEANS THE COLUMNS ARE NO LONGER MODES.** The superseded table split every key by _in OMNIBOX_ against _in NAV_. There is one mode now, so what a key does depends on the composer's BUFFER -- the same guard the machine's notes column states, read from the operator's side.

| key         | composer empty -- browsing                 | query typed                 |
| ----------- | ------------------------------------------ | --------------------------- |
| printable   | into the composer                          | into the composer           |
| `⏎`         | act on the row: descend, edit, or hand off | go: address or picked match |
| `↑` `↓`     | browse the body                            | pick among the matches      |
| `Tab`       | switch panes (list ⟷ detail)               | --                          |
| `Backspace` | pop the view stack                         | delete a character          |
| `ESC`       | a no-op -- you are already home            | clear the query             |
| `/`         | open the MENU, one press                   | (a character)               |
| `Ctrl-C`    | quit                                       | quit                        |

**Typing always lands in the composer** -- the Claude Code affordance: you never select an input before typing, the input is simply where unclaimed keystrokes go. **The superseded machine implemented this as a SEED**, carrying a character out of NAV into the omnibox along with a mode change; with the composer permanently focused there is nothing to seed FROM, so the affordance survives and the machinery under it does not. The cost is stated and unchanged: **no single-letter bindings outside MENU** (`e`-to-edit and `hjkl` died for this), because a letter bound to a verb is a letter the composer never receives.

**`/` OPENS THE MENU IN ONE PRESS AND THAT IS ITS ONLY MEANING**, superseding hv Option A's three-way ring (`NAV -> OMNIBOX -> MENU -> NAV`, 2026-08-31), under which the Lotus menu was two `/` away and `/` meant different things from different places. The empty-buffer guard is unchanged and is what keeps it safe: `st/ST0056` is a legal address, so mid-address `/` is a character.

**`:` IS RETIRED AS A LIVE SIGIL** (hv's `/commands`). `:q`, `:w` and `:q!` survive as **hidden aliases** -- typed into the composer like any other text, honoured by the resolver, and advertised nowhere. The vi muscle memory keeps working for the people who have it; the surface teaches only `/`.

**`C-w` is retired with the vi field keymap** (hv, 2026-08-30: _we're handing the text off to a dedicated editor, not trying to recreate it inside_). In-place editing keeps ONE keymap — readline defaults — and everything longer goes to `$EDITOR`.

## 5. The command palette

**RULED BY hv 2026-09-02, AFTER DRIVING THE BUILD, SUPERSEDING THE LOTUS 1-2-3 MENU.** `/` opens a FILTERED LIST of commands: typing narrows it, the arrows pick, `⏎` runs, `esc` closes. The Lotus tree survives as the GROUPING of the vocabulary and not as a widget.

```
╭──────────────────────────────────────────────────────────╮
│ ❯ /qu▏                                                   │
╰──────────────────────────────────────────────────────────╯
  File   quit   leave explore

 MENU   type to filter · ↑↓ pick · ⏎ run · esc close
```

**WHAT WAS HERE, AND WHY IT WENT.** This section specified a nested horizontal bar -- `Go: [←] Back Threads Issues Packages Criteria [X]`, arrows moving along it, the accelerator letter **coloured in place, not bracketed** and **found by position** rather than assumed to be the first character, `[←]` and `[X]` as **selectable POSITIONS** rather than decorations, accelerators **unique within a level, asserted at startup**, and entries naming **DESTINATIONS, never DIRECTIONS** (`Up`/`Down` were removed because `[←]` already moves up the MENU tree while `Up` would mean up the MODEL tree -- one word, two motions). **None of that was wrong, and none of it was ever built.** hv rebuilt at `a8981480`, opened the menu and found three things: the arrows did nothing, `:q` was still the only way out, and `/quit` did not work. All three were one defect -- **the bar was a hardcoded string with no model behind it**, so there was nothing to select, nothing to move, and no command vocabulary in the program at all.

**THE PALETTE IS PREFERRED FOR A REASON, NOT MERELY BECAUSE IT IS EASIER.** It puts the operator's own spelling first: hv reached for `/quit` unprompted, which is what a filtered list rewards and what a bar cannot answer. It reuses the fuzzy matcher, the pick, the dropdown and the composer that already exist, so the palette is a second VOCABULARY rather than a second WIDGET. And it makes discovery cheap in the one place a menu is supposed to be good: pressing `/` shows the whole vocabulary.

### `/` is for things to DO; typing is for places to GO

**Navigation is deliberately NOT in the palette**, which is hv's own frame: _`/commands` fire up the menus, and anything else is omni-dispatched from the Omni._

The retired bar had a `Go` group listing the entity kinds, and rebuilding it here was the obvious move. **The code refused it**: the omnibox index already carries one entry per declared kind, so `thread` reaches the threads collection by typing today. A `Go` group would be a second route to a destination the composer reaches better -- **and it would bury the commands that have no other route under a list of ones that do.**

### Only what is wired is offered

**A COMMAND THAT CANNOT RUN MUST NOT APPEAR.** The retired tree also listed `Docs` (Browse, Open, New), `File` (Write, Reload) and `Help`; **none has a realiser**, and declaring them would ship a palette advertising a menu of errors -- which is the defect hv drove into, one layer up. So the vocabulary is `quit` and `back`, and **the small honest set beats the large one with holes**. It grows when an act lands, never before; there is deliberately no `Unimplemented` state for an offer to sit in.

**The resting palette lists its whole vocabulary**, which is the opposite of the composer at rest and deliberate: the body is already the listing of the model, but nothing else lists the ACTS.

**There is still no `Edit` entry.** Editing a field is `⏎` on the row; reaching it through a menu was scope that did not belong there, and that survives the change unaltered.

## 6. Navigation and views

Navigation is a **stack**: `⏎` pushes, `Backspace`/`ESC` pops. Cursor and scroll reset with the view, because a row index means nothing once the row set changes.

Views: `Thread`, `Issue`, `Threads`, `Issues`, `Packages`, `Criteria`, `Tests`, `Docs`.

- **A view builder returns the id, the title AND the rows together**, so a view physically cannot render a heading that disagrees with its content.
- **A row's door is DECLARED on the row, not inferred from its kind.** Working out where `documents` goes from the fact that it looks like a pane is the same guess-from-shape that makes `intent edit st 68` misparse today.
- **Opening a real file is a separate action from navigating.** Modelling it as a view was wrong and the compiler said so immediately.
- **When nested, the APP ROW carries the trail and the exit key.** A way back that is wired and unlabelled is a way back nobody finds — this was a real defect in the strawman: `Backspace` worked and nothing on screen said so, so every key a user tried was a reasonable guess and none was the one.

### Documents are not fields

**ST0056 carries 297 attachments, 2.3MB, nested three deep, longest path 81 characters. ST0058 carries one.**

Inlining them makes the form 325 rows of which 297 are files, and it breaks the alignment guarantee outright: a single aligned name column cannot serve `title` and `parity/tools/conservation_check.sh` at once. **So documents are ONE row that opens its own pane.**

### List + detail

Where the selected row carries detail, the BODY splits: list above, detail below, focus shown on the rule between them.

**The split is triggered by the row CARRYING detail, not by a hardcoded list of view kinds.** A list of kinds is a second place to update when a new view arrives, and it is the half that gets forgotten.

- **Criteria** — kind, state, evidence, and the full text.
- **Tests** — status, covers, file, note.
- **Work packages** — title, scope, status, objective.
- **A row that cites a project file shows THE FILE, read-only.** An AT's whole job is to name a test; reading the citation without the cited thing is how a row keeps pointing at a file that no longer says what the row claims.

**The detail pane renders markdown** — inline `code`, `**bold**`, `*em*`; block headings and bullets. The list uses the SAME renderer. Stripping markup in one place and parsing it in the other is two encodings of one fact.

## 7. Editing

### In-place fields

**Editing happens inline, in the value column, where the field is displayed** — not in a footer. `⏎` on an editable row opens the edit **seeded with the RAW value read through the same `Model::read` the `$EDITOR` handoff uses** — the display value may be truncated or derived, and an editor seeded from a rendering writes the rendering back. `⏎` commits through `Model::write` (`facade.set`, whose refusals reach the notice line in the model's own words); `ESC` discards and says so.

**ONE keymap** (hv, 2026-08-30: _we're handing the text off to a dedicated editor, not trying to recreate it inside_): characters and Backspace. The emacs/vi in-field keymaps from the earlier design are retired unbuilt — two keymaps inside one-line fields is recreating an editor inside, and the measured `INTENT_EDIT_MODE`/`~/.inputrc` resolution notes move to the prose-handoff section where the real editor is chosen.

**A `select` row opens the same collector for now and the write door adjudicates** — a status typed against the machine is refused in the store's words. The picker (offering the legal transitions, as the design always intended) is the recorded next step; what ships never silently accepts an illegal state, because `facade.set` will not.

### Prose fields — the external editor

`⏎` on a prose row, `⏎` in the detail pane, or `e` anywhere hands the field to `$VISUAL`/`$EDITOR`. **All three go through one helper**, so the shortcut and the long way cannot drift.

**Two spikes were built and both work.**

- **EMBED** — the editor runs on a pty rendered into the BODY area; the APP/STATUS/COMMAND/INFO rows stay. hv's preference after driving it.
- **FULL-PANE** — the editor owns the terminal; the TUI is restored on exit.

**Verified against real editors:** `vi`, `vim`, `nvim`, `nano` and `emacs -nw` all render correctly in the embedded pane.

**Binding constraints, all measured:**

1. **`$EDITOR` is a COMMAND LINE, not a binary name.** `EDITOR="emacs -nw"` and `EDITOR="code -w"` are ordinary spellings; exec'ing the whole string looks for a file called `emacs -nw`. Split on whitespace. **Stated limit: a quoted argument containing spaces will not survive.**
2. **Reuse the existing resolver and the realise-then-open path.** `$VISUAL`-before-`$EDITOR` is already resolved once in the CLI; a second resolver is the Highlander defect in the one place this estate can least afford it.
3. **THE RETURN IS THE DANGEROUS HALF, NOT THE DEPARTURE.** After the editor exits the store is behind the file. **Re-read that artefact and treat the editor as its authority BEFORE painting anything derived from it.** A TUI that repaints from its stale in-memory model and then saves writes the old bytes over what the operator just wrote — the destroys-authored-prose class this estate already carries five open issues for, reduced to two keystrokes.
4. **Restore the terminal on EVERY exit path, including a panic** — drop guard plus panic hook, never a tidy happy path.
5. **State what happens to an unsaved form at handoff**, rather than letting the two edits interleave unpredictably.
6. **Ask the editor to SOFT WRAP; never hard-wrap the model.** `-c 'setlocal wrap linebreak breakindent'` for the vim family, `--eval '(visual-line-mode 1)'` for emacs. An earlier design hard-wrapped on the way out and unwrapped on the way back; **it was only reversible for 439 of 444 real criteria**, and it solved a display problem by transforming data. Deleted.
7. **A slow editor needs a "starting…" line.** Measured first paint: **`vi` 104ms, `nano` 105ms, `nvim` 315ms, `emacs -nw` 2723ms.** Embed mode otherwise gives an emacs user nearly three seconds of blank pane with nothing saying why.

**The one cost of EMBED, stated rather than discovered: the child owns every key including ESC, so the only way out is the child exiting.** It is the one mode the TUI cannot get you out of, and it is visible in the machine table as a mode whose single exit is not ours. Full-pane does not have this property; it costs a screen flash instead.

### Raw artefact editing

A row opens the realised file itself — `intent/st/<ID>/info.md` — exactly as `vi ST0056/info.md` would. **Deliberately distinct from the field rows, which edit the MODEL.**

**Hazard to be deliberate about:** that writes the project. `info.md` is a generated view, so an edit there creates canon drift, and one unsynced attachment aborts `cargo test -p intentsvcs` for every node in a shared checkout. It should not default to `info.md` in the shipped tool.

## 8. Addressing and failure

- **An operator's spelling of an id resolves to the id.** `56`, `ST0056`, `st56`, `ST56`, `st/ST0056` all name one thread — `AC-06.12`, and the CLI already reports `1 names both a steel thread and an issue`, which is what the `<kind>` argument exists to disambiguate.
- **A spelling that names nothing is refused AS A SPELLING**, saying what was tried and what it resolved to — never as a missing file the operator never asked for.
- **No panics after the terminal is initialised.** A panic in raw mode leaves the user with no cursor and no shell echo.
- **A view that cannot load renders an ERROR ROW, never an empty form.** Swapping a panic for a blank screen trades a loud failure for a silent one, which is the worse of the two and is what this project's contract forbids.

## 9. The `intent edit` surface

`intent edit` is an existing verb — `edit <ADDRESS> [FILE]`, FILE defaulting to `info` — so `intent edit st 68` currently parses `st` as the address and `68` as the file, and refuses about the wrong thing. **The shape change is a deliberate surface decision and belongs to the interface owner; the register row is authored before either realiser is built.**

The ruled shape keeps the TTY-aware design already in force:

```
intent edit st 58                  # terminal → TUI ;  pipe → path
intent edit st 58 --editor[=code]  # force the editor
intent edit st 58 --browser        # force the browser
intent edit st 58 --path           # force the path
```

All four mutually exclusive; `--editor --path` is already refused and `--browser` joins that set.

- **`--path` must survive.** `$EDITOR "$(intent st edit ST0001 design)"` is in `docs/getting-started.md` under test — a documented OUTPUT contract. Command substitution makes stdout a pipe, so that invocation keeps receiving the path.
- **The terminal branch becoming the TUI is a behaviour change for every interactive user.** Taken deliberately under hv's no-3.1.0 ruling: all v3 pain lands under one rubric.
- **`--editor` needs `=`.** An optional-value flag followed by a positional is ambiguous to clap.
- **`--browser` uses localhost HTTP, not a custom `intent://` scheme.** A custom scheme needs per-platform OS registration and buys nothing over `http://127.0.0.1:<port>/threads/ST0058?mode=edit`, while adding a URL-handler surface anything on the machine can invoke.
- **The URL addresses the ENTITY, not a filename.** The TUI and browser edit the model; only `--editor` opens a file. Mixing them is what makes the current argument shape misparse.
- **`--browser` refuses when the daemon is not running**, naming `intent daemon start`, rather than spawning a process the user did not ask for.
- **`intent browse` DOES ship, and the earlier text here refused it by MISREADING the criterion it cited.** `ST0058 AC-00.6` reads _a flag and its subcommand twin AGREE ABOUT WHETHER THE CAPABILITY EXISTS_ -- it was driven off `intent3 --version` returning rc=0 while `intent3 version` returned _known command that is not implemented yet_. **It refuses DISAGREEMENT, not duplication**, and two spellings that agree are exactly what it asks for. hv asked for both forms in their own words -- _`intent edit st 68`, or browse to something like `intent browse st 68`_ -- so the deleted bullet contradicted the ask it was written to serve. `intent browse <kind> <id>` is `intent edit <kind> <id> --browser`, it gets a register row, and **the register row is where the agreement is asserted**: the twin cannot be present by one spelling and absent by the other. Raised by ic 2026-08-29 while authoring that row, who declined to green `AC-17.6` by reinterpreting it.

## 10. What the data actually looks like

Measured across the corpus, because the design has to survive it:

```
446 criteria      432 single-line      14 multi-paragraph
chars: median 224   p90 2,482   max 59,061
91 criteria exceed 1,000 chars on a single line
largest: ST0057 AC-08.5 -- 59,061 chars, zero line breaks
```

**Multi-line text is durable**: a criterion authored with paragraph breaks and bullets survives `sync --to-store` then `--to-disk` byte-identical, corroborated by the 14 criteria already carrying newlines. _(Measured against the delivered binary `30a2dd81`; unverified against HEAD, whose gap includes `180fb4a3` touching the read path.)_

**So reformatting stored criterion text for readability is possible and durable — but it is a content change to ratified rows and is hv's call, not a build decision.** A 59KB single-line paragraph is not readable in any editor at any wrap width, and no display fix reaches it.

## 10a. The renderers (D56 -- `intentd` emits JSON only)

**Ruled by hv 2026-08-29 and recorded as `D56` in `design.md`; this section is the working detail, not a second copy of the decision.**

```
                        intentsvcs              <- the ONLY thing that knows the model
                       /     |     \
            CLI      TUI   intentd              <- coordinators
                            |
                    GraphQL / JSON              <- ONE output contract
                   /        |        \
            browser    SwiftUI     MCP          <- generic renderers
             (ES)     (ST0064)
```

**`intentd` renders no HTML beyond a single static shell page.** Everything else is JSON over GraphQL, on the socket and over HTTP. **The deciding argument is the menubar app: SwiftUI cannot consume server-rendered HTML**, so a daemon that serves HTML to browsers ends up serving JSON to the menubar as well -- two output contracts, which is the drift a single face exists to prevent.

### What makes a renderer generic

**The daemon RESOLVES the form declaration server-side and emits a generic description.** The renderer never sees the DSL and never learns the domain:

```json
{ "entity": "ST0056",
  "title":  "Add a Rust-based CLI with a local SQLite DB ...",
  "fields": [
    {"name": "objective", "label": "objective", "widget": "prose",
     "value": "...", "editable": true}
  ] }
```

**The JS renders `{label, value, widget}` triples. So does SwiftUI. So does the TUI when it routes through the daemon.** None of them knows what a criterion is, and **that property is now enforced by the wire boundary rather than by discipline** -- which is the whole reason this shape beats server-rendered HTML.

**If a renderer ever needs a domain concept, the field description is missing something. Add it to the description, never to the renderer.**

### Stack

- **`axum`** -- HTTP and routing. Costs nothing extra: `intentd` needs tokio anyway for the socket server, the debounced watcher and MCP's streamable HTTP.
- **One static shell page**, embedded in the binary.
- **Plain ES modules, served same-origin from the binary.** No npm, no bundler, no wasm, nothing in CI.

### Copy Conflab's bridge rather than re-deriving it

`../Conflab/assets/js/hooks/daemon_bridge.js` is the working precedent and carries operational knowledge that is expensive to rediscover:

- **Endpoints come from configuration, never hardcoded** in the client.
- **The auth token lives in `localStorage`, not `sessionStorage`** -- the latter is per-tab and made Safari re-prompt on every cross-link.
- **Hysteresis on liveness: three consecutive failed probes before declaring the daemon dead, one success resets the counter.** Without it the UI oscillates on a momentary hiccup. **This is the part most likely to be left out and most annoying to add later.**

### Where the renderers legitimately differ

**In the WIDGET, never in the model.** `prose` means _this field is long_, and each renderer answers it in its own idiom: the TUI hands it to `$EDITOR`, the browser uses a `<textarea>`, SwiftUI uses a `TextEditor`. **A difference anywhere else is a defect.**

The browser splits **vertically** -- list left, detail right -- because a browser window is tall, where the TUI splits horizontally because a terminal is short and wide. Same information architecture, each medium's own shape.

**`--browser` refuses when the daemon is not running**, naming `intent daemon start`, rather than spawning a process the operator did not ask for.

### The binding test

**`AC-17.1` diffs the MODEL.** The same edit through any renderer must reach an identical store state. **There is now ONE wire format to diff rather than two renderings to compare for similarity**, which makes the criterion cheaper to satisfy and harder to fake. `AC-08.2`'s in-process-versus-daemon requirement is the same property one layer down and is unchanged.

## 10b. Making the pages look good

**INHERIT `docs/design/design-system.md`. DO NOT INVENT A SECOND ONE.** Its founding principle is _"the site renders like the tool"_ and its semantic palette is _"taken from the CLI's own output vocabulary rather than invented"_ -- which applies more strongly here, because these pages ARE the tool.

**Cite it, never restate it.** The tokens live in that file. What follows is only what the renderers need on top, and why.

- **The colour tokens as written** -- ground, ink, rules, the steel accent, and the semantic set (`--ok`, `--error`, `--warning`, `--note`, `--residue`) already carrying the CLI's prefix vocabulary. Its own instruction applies: _lift these directly -- a token that exists only in a prose table gets retyped and drifts._
- **Dark as a designed palette, not an inversion.** Already specified there.
- **Newsreader for prose, IBM Plex Mono for machine text, no sans-serif.** Self-hosted, subset, same-origin.

**SELF-HOSTING IS A CONSTRAINT, NOT A PREFERENCE.** `intentd` serves localhost and must render identically with no network. **No CDN, no Google Fonts, no off-origin anything** -- for the daemon, a network dependency makes the tool fail in the case it exists to serve.

**One stylesheet, embedded in the binary**, so a running daemon cannot serve a page whose CSS is missing or stale -- the same reason `intent init` works offline from embedded canon (`AC-07.1`).

### Three patterns the site never needs

The site renders prose. The renderers render a MODEL at sizes the site never meets (section 10): 154 criteria in one list, 297 attachments, a 3KB criterion, one 59KB paragraph.

1. **The form** -- one field per row, label column and value column. **Long prose gets a readable measure (~70ch), not a full-bleed box.**
2. **The list** -- dense, monospace ids, sticky header, and the same truncation rule as the TUI: **clip with an ellipsis, never wrap into a second line.** A 154-row list that reflows is unscannable.
3. **List + detail**, split as described in 10a.

**Markdown renders the same four constructs the TUI renders** -- inline code, bold, emphasis, headings, bullets. **Same four, not a superset**, so a criterion cannot read differently in two renderers.

**Semantic HTML, no ARIA theatre.** A form is a `<form>`, a list is a `<ul>` or `<table>`, and the browser's own affordances survive.

**THE TEST OF "GOOD" IS NOT TASTE, IT IS THE 59KB PARAGRAPH.** A page that looks handsome on `ST0058` and is unreadable on `ST0057 AC-08.5` has not been designed against the corpus. **Build every screen against the largest real thread** -- the strawman found four defects that way and none was visible on a small one.

## 11. Open, and whose

- **The shipped default for prose editing: EMBED or FULL-PANE.** hv preferred embed after driving it; the machine-level cost above is the argument against. **hv's.**
- **Reflowing the 91 over-1000-character criteria.** Content change to ratified rows. **hv's.**
- **Whether nav descends into criteria as a flat list when ST0056 has 154.** Paging or filtering is a different widget from anything in the form vocabulary. **vc, then build.**
- **`body` and `preamble` on the thread form at all** — empty everywhere measured, and the `info.md` round-trip has just changed which sections are authored. **Recommend leaving them out of v1.**
- **The renderers consume `intentd`'s JSON face** (`AC-08.9`), ruled as `D56`; see section 10a. **The SwiftUI menubar app (ST0064) is the third renderer and is what decided the shape.**

## 12. Provenance

Designed with hv on 2026-08-29 by building and driving a strawman, not by specification. The reversals are recorded above because they cost the most to find: the field list, the truncation, the hard wrap, the `**` strip, `TRAVERSE`, `Up`/`Down`, `/`-as-a-prefix, `View::Raw`, the invisible way back, and the alignment gutter were each wrong first and corrected against real data.
