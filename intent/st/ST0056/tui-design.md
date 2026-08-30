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

**Revised by hv 2026-08-30 (in conversation, superseding the NORMAL-rest machine below at the same provenance strength — driven, then ruled).** hv's frame, verbatim: _an omnibox style input that autonavigates to entity by its address_ ... _the omnibox is the starting point, but pressing ESC or / takes you into those other modes._ So: **OMNIBOX is the rest state** — the input is where a session opens, where addresses are typed, and where `:` commands live (COMMAND is gone; the omnibox absorbed it, because a `:` line beside an always-present input is two prompts for one keyboard). NORMAL is renamed **NAV**, hv's own word for it.

| from    | trigger   | to      | notes                             |
| ------- | --------- | ------- | --------------------------------- |
| OMNIBOX | Typing    | OMNIBOX | the address buffer                |
| OMNIBOX | Move      | OMNIBOX | pick among matches                |
| OMNIBOX | Enter     | NAV     | go — address, pick or `:` command |
| OMNIBOX | Esc       | NAV     | leave the input as it is          |
| OMNIBOX | `/`       | MENU    | empty buffer only                 |
| NAV     | Move      | NAV     | in the focused pane               |
| NAV     | Enter     | NAV     | descend — rows with a door        |
| NAV     | Enter     | FIELD   | editable rows                     |
| NAV     | Enter     | EMBED   | prose rows -> `$EDITOR`           |
| NAV     | Back      | NAV     | pop the view stack                |
| NAV     | Esc       | OMNIBOX | home to the input                 |
| NAV     | `:`       | OMNIBOX | seed the buffer with `:`          |
| NAV     | Typing    | OMNIBOX | a printable seeds the buffer      |
| NAV     | `/`       | MENU    |                                   |
| MENU    | Hotkey    | MENU    | select or drill in                |
| MENU    | Move      | MENU    | select or drill in                |
| MENU    | Enter     | NAV     |                                   |
| MENU    | Back      | NAV     |                                   |
| MENU    | Cancel    | NAV     |                                   |
| MENU    | Esc       | NAV     |                                   |
| FIELD   | Typing    | FIELD   |                                   |
| FIELD   | Enter     | NAV     | commit                            |
| FIELD   | Esc       | NAV     | discard                           |
| EMBED   | Typing    | EMBED   | forwarded to the child            |
| EMBED   | ChildExit | NAV     | read the file back                |

**Two invariants, asserted at startup and checkable headlessly:**

- **Every mode is leavable.** A mode you can enter and not leave is the trap `no_state_can_be_entered_and_not_left` already refuses for entities.
- **Every mode is reachable from OMNIBOX.** An unreachable mode is dead code that reads as a feature.

**ESC LANDS IN A HOME MODE IN ONE PRESS AND NEVER QUITS.** Home is the pair {OMNIBOX, NAV} — the omnibox is home for the keyboard, NAV is home for the cursor, and Esc toggles between them. This retires the ratified _at the root it QUITS_ deliberately: **quitting is now an act, never an accident** — `Ctrl-C` from anywhere, `:q` from the omnibox. The modern agent idiom (Claude Code's own Esc never quits) is exactly this shape, and it keeps what the old invariant was for: an operator who does not know where they are presses Esc and lands somewhere fully operable, every time. Save protection still names its own escape: `unsaved edits -- :w to write, :q! to discard`.

**The `NAV + Enter` triple is the one guarded ambiguity**, resolved by the ROW and never by table order: a row with a door descends, an editable row edits in place, a prose row hands off. The door arm is this revision's fix for the strawman's worst defect — Enter on a `button` row used to reach FIELD, so the one navigation verb on screen navigated nowhere, which is what hv drove into on 2026-08-30.

**Pane focus (list / detail) is a GUARD on NAV's edges, not a sixth mode.** It changes where Move and Enter land; it does not change what the keys mean. The omnibox's empty-buffer condition on `/` is the same species of guard: mid-address, `/` is a character (`st/ST0056` is a legal spelling), so the menu key fires only when there is nothing it could be part of.

## 4. Keys

| key         | in OMNIBOX                        | in NAV                                |
| ----------- | --------------------------------- | ------------------------------------- |
| printable   | the buffer                        | seeds the omnibox with that character |
| `⏎`         | go: address, pick, or `:` command | descend / edit / hand off, by the row |
| arrows      | `↑↓` pick among matches           | move within the focused pane          |
| `Tab`       | —                                 | switch panes (list ⟷ detail)          |
| `Backspace` | delete                            | pop the view stack                    |
| `ESC`       | to NAV, buffer kept               | to OMNIBOX                            |
| `:`         | (a character)                     | to OMNIBOX, seeded with `:`           |
| `/`         | MENU when the buffer is empty     | MENU                                  |
| `Ctrl-C`    | quit                              | quit                                  |

**Typing anywhere in NAV lands in the omnibox** — the Claude Code affordance: you never select an input before typing, the input is simply where unclaimed keystrokes go. The cost is stated: NAV has no single-letter bindings (`e`-to-edit and `hjkl` died for this), because a letter that does something in NAV is a letter the omnibox never receives. Enter on the row already edits; arrows already move.

**`/` is the MENU key**, the real Lotus 1-2-3 binding, guarded in the omnibox by the empty buffer since `st/ST0056` is a legal address. There is ONE typed vocabulary and it lives in the omnibox; `:` survives only as a seed character for it, so `:w` and `:q` keep their vi muscle memory without a COMMAND mode to host them.

**`C-w` is retired with the vi field keymap** (hv, 2026-08-30: _we're handing the text off to a dedicated editor, not trying to recreate it inside_). In-place editing keeps ONE keymap — readline defaults — and everything longer goes to `$EDITOR`.

## 5. Menus — Lotus 1-2-3

Menus **nest**. The accelerator letter is **coloured in place, not bracketed**, and is **found by position** in the label rather than assumed to be the first character — a hotkey that is not the initial otherwise marks the wrong letter, which happens the first time two entries at one level share a first letter.

```
 Go: [←]  Back  Threads  Issues  Packages  Criteria  [X]
```

- **`[←]` and `[X]` are selectable POSITIONS**, not decorations. The arrows land on them and `⏎` does what the glyph says. A control you can see and cannot select is a label pretending to be a button.
- **Accelerators are unique within a level, asserted at startup**, not left to whoever adds the next entry.
- The INFO row describes whichever position is selected, including `[←]` and `[X]`.

**Menu entries name DESTINATIONS, never DIRECTIONS.** `Up` and `Down` were removed: `[←]` already moves up the MENU tree, so a menu item called `Up` reads as the same motion while meaning up the MODEL tree — one word, two motions. Naming the destination loses nothing, because a thread's parent _is_ `Threads`.

**`Go > Back` is history, not a direction** — the same act as `Backspace`, and it does not reintroduce the confusion `Up`/`Down` caused.

Current tree: `Go` (Back, Threads, Issues, Packages, Criteria) · `Docs` (Browse, Open, New) · `File` (Write, Reload, Quit) · `Help`.

**There is no `Edit` submenu.** Editing a field is `⏎` on the row; reaching it through a menu was scope that did not belong there.

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

**Editing happens inline, in the value column, where the field is displayed** — not in a footer. The COMMAND row shows the active keymap and its bindings, never a second copy of the text.

**Keymap is emacs or vi**, resolved: `INTENT_EDIT_MODE` → `~/.inputrc` `set editing-mode` → **default emacs**.

> **The shell does NOT export `set -o`, and the obvious fallback is measured wrong.** On the reference machine `set -o` reports `emacs on / vi off` and nothing in `env` carries it — a child process cannot read the shell's editing mode, only be told it. And inferring from `$EDITOR` gives `vi` there (it is `nvim`) on a shell that is in emacs mode. **If the mode must genuinely follow the shell, the shell has to export it via a one-line rc shim. That is the only honest route.**

- **emacs** — `C-a C-e C-b C-f C-d C-k C-u C-w`.
- **vi** — starts in insert; `ESC` drops to normal (`h l 0 $ i a I A x D`); `ESC` again leaves the field. **Two presses to exit is vi's own shape** and keeps ESC meaningful at each depth rather than doing two jobs at once. The caret colour distinguishes insert from normal.

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
