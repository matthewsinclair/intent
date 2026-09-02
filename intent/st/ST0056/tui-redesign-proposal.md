# TUI redesign proposal -- lower bar + mode model (Claude Code alignment)

**Status: PROPOSAL, awaiting hv + vc ratification. Nothing here is landed.** Authored by ic 2026-09-02 on hv's direct instruction to make `intent explore` "much more like the Claude Code UX/UI", aimed at the lower bar. It revises `tui-design.md` sections 2 (screen), 3 (mode machine), 4 (keys) and 5 (menus, one line).

**Why a separate file and not an edit to `tui-design.md`.** Section 3's table is parsed by `mode.rs`'s `the_transcription_carries_every_row_the_design_ratifies`, and section 2's shape is pinned by `AC-17.11`. Editing either in place breaks the suite the moment it lands and before the code follows -- the coupled-change hazard. So this is ruled on paper first and executed as ONE coordinated commit (design + code + criteria) second.

## What hv asked, and the three rulings

hv, 2026-09-02: operations are right and the edit window is sound; it is the lower bar that is not right. hv's model is two modes -- Omnibox and Menu -- where in Omni a `/command` fires up the menu and anything else is omni-dispatched from the Omni. The rulings on the three gaps I raised:

1. **Section 2's "rules" are reopenable.** hv: _those 'rules' are only rules to the extent that I made them at the time; we can change them if we need to, and this is an opportunity to do that._ So two-rules-no-borders and the minimal-chrome foot collapse are both back on the table.
2. **The status content comes back, matched to Claude Code's coherence.** hv agrees what we had was less consistent and coherent than Claude Code, and we should up our game to match.
3. **The mode model goes simpler.** hv: _'cleverer' isn't necessarily 'betterer'._

## 1. The mode model -- recommendation (the load-bearing decision)

**Recommendation: collapse to two modes the operator holds in their head -- OMNI and MENU -- plus EDIT as the transient act-on-a-row state. Kill the 3-way `/` ring and the Esc toggle. Fold NAV out of the machine into a focus guard.**

The current machine is five modes (OMNIBOX, NAV, MENU, FIELD, EMBED) with two clever devices: `/` cycles a ring NAV -> OMNIBOX -> MENU -> NAV, and Esc toggles the home pair {OMNIBOX, NAV}. Both are coherent and both cost the operator: the Lotus menu is TWO `/` presses from the body, and Esc does double duty as a mode toggle. Claude Code's coherence comes from three things this trades away -- ONE input that is always home, ONE meaning for `/` (open the menu), and editing being a place you go and return from rather than a mode you steer between.

**The simplification, stated as the operator sees it:**

- **OMNI is the one home.** The composer always holds the text cursor. This is unchanged from hv's 2026-08-30 ruling that the omnibox is the rest state -- the new part is that it is the ONLY home, with no NAV to toggle to.
- **The body is browsed, not entered.** A row in the body is always "current" (highlighted). When the composer buffer is EMPTY, the arrows move the current row -- you browse the list without leaving home. When the buffer is NON-EMPTY, the arrows pick among the fuzzy matches, exactly as today. One guard -- buffer empty or not -- drives the difference, and it is the same guard that already governs `/`.
- **`/` opens MENU in a single press**, always, from an empty composer. The ring is gone. `st/ST0056` stays a legal address because the empty-buffer guard already distinguishes the menu key from a slash mid-spelling.
- **Enter means "act on the current selection."** With a query in the buffer, that selection is the picked match, so Enter navigates to it. With an empty buffer, the selection is the current body row, so Enter descends (door row), edits in place (FIELD), or hands off to `$EDITOR` (EMBED) -- the guarded triple, resolved by the ROW exactly as it is today.
- **Esc means "back to the composer", never a mode toggle and never quit.** In MENU it closes the palette; in EDIT it discards; with a query typed it clears the buffer; on an already-empty composer it is a no-op because you are already home. `Ctrl-C` quits from anywhere; `:q` survives as an omnibox command. (Open call O3 below: whether Esc on an empty composer should instead pop the view stack for a faster "back out".)
- **The chip shows three lamps, not five: OMNI / MENU / EDIT.** FIELD and EMBED are both "editing a row" to the operator, so they share the EDIT lamp; the machine keeps them as distinct internal states because their exits differ (EMBED's is the child exiting, and that exemption stays).

**Revised section 3 edge table (replaces the current one):**

```
| from  | trigger              | to    | notes                                             |
| ----- | -------------------- | ----- | ------------------------------------------------- |
| OMNI  | Typing               | OMNI  | into the composer; a printable in the body seeds it |
| OMNI  | Move   (buffer empty)| OMNI  | browse the body list (the current row moves)      |
| OMNI  | Move   (buffer set)  | OMNI  | pick among the fuzzy matches                      |
| OMNI  | Enter  (buffer set)  | OMNI  | go -- address or picked match; pushes the view    |
| OMNI  | Enter  (buffer empty)| OMNI  | descend -- door rows (the guarded triple, arm 1)  |
| OMNI  | Enter  (buffer empty)| FIELD | edit in place -- editable rows (arm 2)            |
| OMNI  | Enter  (buffer empty)| EMBED | hand off -- prose rows -> $EDITOR (arm 3)         |
| OMNI  | `/`    (buffer empty)| MENU  | open the command palette, one press               |
| OMNI  | Esc    (buffer set)  | OMNI  | clear the buffer                                  |
| OMNI  | Esc    (buffer empty)| OMNI  | no-op -- already home                             |
| OMNI  | Back                 | OMNI  | pop the view stack (buffer empty)                 |
| MENU  | Hotkey / Move        | MENU  | select or drill in                                |
| MENU  | Enter                | OMNI  | run the command / land its view                   |
| MENU  | Esc / Cancel / `/`   | OMNI  | close the palette                                 |
| FIELD | Typing               | FIELD | in-place edit, one keymap                          |
| FIELD | Enter                | OMNI  | commit                                            |
| FIELD | Esc                  | OMNI  | discard                                           |
| EMBED | Typing               | EMBED | forwarded to the child                            |
| EMBED | ChildExit            | OMNI  | read the file back                                |
```

**The invariants survive the change and get simpler to state.** Every state is leavable; every state is reachable from OMNI (the rest state); one Esc from any state that owns its escape lands in OMNI (MENU and FIELD do; EMBED is the one exemption, because the child owns the terminal). The `Enter (buffer empty)` triple is the one guarded ambiguity, resolved by row kind and never by table order -- unchanged in substance, re-sourced from OMNI-empty instead of NAV. The `arm`/`BY_ROW_KIND` resolver in `mode.rs` carries over verbatim.

## 2. The lower bar -- revised section 2

hv wants the Claude Code look, which is three stacked pieces: a framed composer, a persistent status-segment row, and a mode line. The current foot is the omnibox line plus one hint line. The proposal grows the foot back out, but as SEGMENTS matching Claude Code rather than the old STATUS/COMMAND/INFO trio.

**Proposed screen (foot mirrors Claude Code's composer -> status -> mode order):**

```
 ST0056  Add a Rust-based CLI with a local SQLite DB…            /thread/ST0056
────────────────────────────────────────────────────────────────────────────
 ▸ title       Add a Rust-based CLI with a local SQLite DB…
   status      wip
   objective   673 bytes
   documents   297   2.3 MB
   work pkgs   17
──────────────────────────── detail ─────────────────────────────────────────
   kind   test
   text   **A CHECKER VERIFIES MEMBERSHIP IN A VOCABULARY** and never that…
──────────────────────────────────────────────────────────────────────────────
╭──────────────────────────────────────────────────────────────────────────╮
│ ❯ 56▏                                                                      │
╰──────────────────────────────────────────────────────────────────────────╯
 Intent · ⎇ main +12−6 · gate 110/135 · bin @361eff9 ⚠ behind HEAD
 OMNI   ⏎ go   / menu   ↑↓ browse   ⌫ back   esc clear
```

- **APP ROW** -- unchanged: the entity's id, name, and view trail. It carries the way back when nested.
- **BODY** -- unchanged: the two-aligned-columns `{name, value, type}` list, the detail split, the alignment guarantee. The fuzzy dropdown still borrows the body's last rows while the composer has matches.
- **COMPOSER (framed)** -- the one framed element on the screen, and the signature Claude Code affordance: a bordered box that reads unmistakably as "the place you type." Prompt glyph, buffer, cursor. Bright while it holds the keyboard (always, now that it is the one home). This is the deliberate relaxation of "no borders anywhere" -- see O1 for the lighter alternative.
- **STATUS SEGMENTS** -- persistent, session/project-level context, the analogue of Claude Code's `Model · branch · diff`: the project name, git branch + diff stat, the release gate figure, and -- genuinely useful given this estate's currency saga -- a binary-currency segment that says when the delivered binary is behind HEAD. These are dotted segments, coloured, and they do NOT change as the cursor moves (that is the hint line's job).
- **MODE / HINT ROW** -- the mode lamp first and unconditional (OMNI / MENU / EDIT, coloured per mode, reversed -- the lamp stays), then what the keys do right now: the row position, the one verb Enter means on this row, the crossings that exist. A standing notice takes this line when something happened.

**Revised layout rules (delta from the current four):**

- The two-aligned-columns guarantee, render-time clipping, and no-wrap all STAY exactly as ratified. They are the part that works.
- "Two rules are the only chrome" is RETIRED. The chrome is now: the two body rules, the detail rule, the composer's box, and the segment styling. Borders are allowed on the composer and nowhere else -- stated as a positive rule so a future reader does not read the relaxation as open season.
- Colour-as-roles-computed-by-layout STAYS (the pure-data seam is what makes `AC-17.11` assertable, and it does not change). The palette gains status-segment roles and the box border role; the mode lamp and semantic status colours are unchanged.

## 3. Keys (revised section 4) and menus (section 5)

**Keys.** The `/`-ring row is gone; `/` from an empty composer opens MENU in one press, and `/` from MENU closes it. Esc is "clear the buffer, else no-op" rather than a toggle. Arrows are buffer-guarded (browse the body when empty, pick matches when set). `:` is retired as a live command sigil in favour of `/` (hv's `/commands`), keeping `:q`/`:w`/`:q!` only as omnibox spellings for muscle memory -- open call O2 on whether to keep them at all. Everything hv already ruled STAYS: typing anywhere seeds the composer, one in-place keymap, `C-w` and the vi field keymap retired, `Ctrl-C` quits.

**Menus (section 5) are almost untouched.** The nested Lotus tree, the coloured-in-place accelerator found by position, the selectable `[←]`/`[X]` positions, unique-accelerators-per-level, and destinations-not-directions all stay. The only change is how you get here: one `/` from the composer instead of a ring stop. The palette can reuse the existing fuzzy dropdown mechanism -- a `/` filters COMMANDS the way a bare query filters ENTITIES -- which is the most direct read of hv's "`/commands` fire up the menus".

## 4. What is explicitly KEPT (so the review is about the delta, not the whole design)

Operations and the verbs; the edit window (FIELD in-place + EMBED `$EDITOR` handoff, including the re-read-on-return discipline and the restore-on-every-exit guard); the pure-data layout/draw seam; the two-aligned-columns alignment guarantee and its assertion; the fuzzy omnibox matcher and dropdown; the view stack and `§6` navigation; the mode-lamp-as-colour and the semantic status palette; `Ctrl-C`-quits and quitting-is-an-act. hv named operations and the edit window as right; this proposal does not touch them.

## 5. Blast radius if ratified -- one coordinated change

- **Design:** `tui-design.md` §2, §3, §4, §5 rewritten to the above (this doc merges in and is deleted). Size S.
- **Code, all with thorough tests to rewrite alongside:** `mode.rs` (the `Mode` enum drops `Nav`; `EDGES` becomes the new table; the invariant tests and the design-parse test move with it), `keys.rs` (no ring, `/`-one-press, Esc semantics, buffer-guarded arrows), `layout.rs` (`FOOT` grows; new status-segment section; the composer-box allowance; the degradation order re-examined for the taller foot), `draw.rs` (the box border role + palette; the "no border" assertion relaxed to "only the composer is framed"), `omnibox.rs` (the `/` command path routes to the MENU-filtered palette; `Go`'s `:` command arm retired or kept per O2). Size M.
- **Criteria, through vc:** `AC-17.11` reworded from "five sections / two rules / one modeline" to the composer-box + status-segments + mode-line shape; `AC-17.9` reworded from the {OMNIBOX, NAV} home pair + Esc-toggle to the OMNI/MENU/EDIT machine with Esc-to-home. `AC-17.4`, `AC-17.10`, `AC-17.12` and `AC-17.1` are UNAFFECTED (they are about the editor handoff, the view stack, and model-equivalence, none of which this changes). Size S.

Total: roughly L, and it is the TUI-divergence rework hv flagged on 2026-08-30, not a tweak.

## 6. Open calls for hv (each changes the design, none blocks ruling the rest)

- **O1 -- framed composer, or a lighter prompt line?** The framed box is the Claude Code look and the recommendation, but it costs the foot two extra lines (top and bottom border), which matters on a short terminal and re-opens the degradation order. The lighter option is a single prompt line with a distinct background/underline -- less Claude-Code, one line instead of three. Recommendation: framed, and revisit degradation.
- **O2 -- retire `:` entirely, or keep `:q`/`:w`/`:q!` as omnibox spellings?** hv said `/commands`; `:` muscle memory is cheap to keep and cheap to drop. Recommendation: keep the three as hidden aliases, advertise only `/`.
- **O3 -- Esc on an empty composer: no-op, or pop the view stack?** No-op is the pure Claude Code reading (Esc clears, does not navigate); pop-stack gives a faster "back out" but overloads Esc with navigation that `⌫`/Back already owns. Recommendation: no-op, keep navigation on Back.
- **O4 -- the status segments' exact contents.** Proposed: project · branch+diff · gate · binary-currency. The binary-currency segment is the one non-obvious add and I think it earns its place given how often stale-binary bit this estate; hv may want it, cut it, or swap in thread counts.

## 7. Provenance

ic, 2026-09-02, on hv's direct instruction. The mode-model recommendation (section 1) is ic's, offered as the answer to hv's "what's the rec?"; the section-2 status-segment direction is hv's ("bring it back, match Claude Code"); the reopening of section 2's rules is hv's explicit ruling. Ratification is hv's, with vc stewarding the criteria rewording. Until ratified, `tui-design.md` §2-§5 stand as written and the code matches them.
