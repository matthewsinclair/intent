# Intent — design system

**This is the design specification for `intent.laksa.io`.** `/design` at Laksa reads it, and laksa-{vc,cc} implement it as a Laksa custom theme (`theme/theme.yaml`, `theme/layout.liquid`, `assets/`) in `../Sites/intent`. `../Sites/appendix3` is the working reference for the theme structure.

**It specifies a one-page site.** Intent's documentation lives in this repository at [`docs/`](../) and the site links to it rather than reproducing it. Sections marked _docs shell_ apply only if the documentation is later hosted; everything else is needed for the one page.

**This document was consolidated from two independent drafts** — cc's, briefed by hv as simple / technical / coherent / self-contained and modelled on best-in-class tool home pages, and vc's, briefed by hv's selection of an editorial direction. Both were written without knowing the other existed. The measured prefix vocabulary in §1, the references in §2, the forbidden affordances in §8 and the budgets in §9 are cc's and are kept largely as written.

---

## 1. The founding principle: the site renders like the tool

**The semantic palette is inherited from the CLI's output contract, not chosen.** Intent v3 emits one vocabulary of line prefixes. These are measured counts across `intent-cli/src` at `HEAD`, 2026-08-29:

| Prefix     | Uses | What it means                                    |
| ---------- | ---- | ------------------------------------------------ |
| `error:`   | 88   | The operation did not happen                     |
| `ok:`      | 25   | The operation happened                           |
| `created:` | 10   | A new artefact exists                            |
| `warning:` | 9    | It happened, and something about it needs saying |
| `note:`    | 7    | Context the operator wants and did not ask for   |
| `residue:` | 1    | State left behind that nothing owns              |
| `done:`    | 1    | A sequence finished                              |

All lowercase, all colon-terminated, no banners, no unicode decoration.

**That vocabulary is the site's semantic colour system.** A callout is not a "tip" or an "info box" — it is a `note:`, and it is styled as one. Where the page shows a failure the tool prints as `error:`, it renders in the colour the terminal uses.

**This is the coherence argument and it is the reason nobody has to defend a palette on taste.** A reader who has used the tool for ten minutes already knows what the colours mean.

**It is also the one fact in this document.** Everything below is a design intention. The prefix table is a measurement of the tool; nothing else here has been built or measured.

## 2. References, and what to take from each

What matters is the specific thing to take, not general admiration.

| Site                  | Take this                                                                                                                    | Do not take                                                            |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| **sqlite.org**        | Longevity and zero dependency. It has outlived every framework used to build its contemporaries. Content first, chrome last. | Its 1998 layout. Density is the lesson; the table-based layout is not. |
| **esbuild.github.io** | The home page makes one claim and proves it immediately, above the fold, with a measurement.                                 | The single-page-for-everything structure; Intent has real docs.        |
| **htmx.org**          | Code in the first screen. The reader sees the thing working before reading a sentence about it.                              | The jokey register. Intent's voice is drier.                           |
| **Astral (uv, ruff)** | Restrained typography, one accent, benchmarks treated as content rather than marketing.                                      | The launch-announcement energy.                                        |
| **Ghostty**           | A terminal tool whose site looks like the tool without kitsch. Honest terminal rendering.                                    | Nothing much; this is the closest peer.                                |
| **curl.se**           | Utilitarian to the point of being unfashionable, and completely trustworthy for it.                                          | The navigation.                                                        |
| **Zed**               | Dark mode as a designed palette rather than an inverted light one.                                                           | The motion.                                                            |
| **Stripe docs**       | _(docs shell)_ Structure only: the code sample beside every concept.                                                         | The visual language, far richer than this brief allows.                |

**The counter-examples are as instructive**: animated gradient meshes, autoplaying typing animations, scroll-jacking, "Trusted by" logo walls, floating chat widgets, cookie banners, parallax, custom cursors. Every one appears on sites that are otherwise good. None survives the brief. §8 makes the list checkable.

## 3. Colour

Near-monochrome ground and ink, one accent, and the semantic set from §1. Lift these directly — a token that exists only in a prose table gets retyped and drifts.

```css
  :root {
    /* ground and ink -- never pure black on pure white */
    --bg:            #fcfcfd;
    --bg-raised:     #ffffff;
    --bg-sunken:     #f4f5f7;
    --ink:           #16181d;
    --ink-muted:     #5c6370;
    --ink-faint:     #868d99;
    --rule:          #e3e5e9;
    --rule-strong:   #cdd1d8;

    /* one accent, and it is steel -- the thread metaphor, not a mood */
    --accent:        #35618f;
    --accent-hover:  #274b71;
    --accent-wash:   #eef3f9;

    /* semantic, INHERITED FROM THE CLI PREFIX VOCABULARY (section 1) */
    --ok:            #1f7a44;
    --created:       #1f7a44;
    --warning:       #92650e;
    --error:         #a32b2b;
    --note:          #55606f;
    --residue:       #6b4d94;

    /* geometry -- restrained on purpose. Nothing is a pill. */
    --radius:        3px;
    --radius-lg:     5px;
    --border:        1px;
  }
```

**Dark is a designed palette, not an inversion.** The semantics shift lightness and keep hue, so a reader switching modes does not relearn the vocabulary.

```css
  @media (prefers-color-scheme: dark) {
    :root:not([data-theme="light"]) {
      --bg:            #0e1013;
      --bg-raised:     #16191e;
      --bg-sunken:     #0a0c0f;
      --ink:           #e6e8ec;
      --ink-muted:     #9aa1ac;
      --ink-faint:     #6d7581;
      --rule:          #252a31;
      --rule-strong:   #363d46;

      --accent:        #74a6dd;
      --accent-hover:  #93bcea;
      --accent-wash:   #16202b;

      --ok:            #4aab72;
      --created:       #4aab72;
      --warning:       #c9973a;
      --error:         #e06767;
      --note:          #8b96a5;
      --residue:       #a586cf;
    }
  }
```

**Define every colour on bare `:root` first and redefine only what changes inside the media query.** A token whose only definition lives in a media block vanishes in the other mode. Redefine the same tokens again under `:root[data-theme="dark"]` if a toggle is built, so the toggle wins in both directions.

### OPEN DECISION A — the accent, and whether it can be red at all

**This is not settled and it must not be settled by whoever holds the pen on this document.** The `--accent` values above are a placeholder so the spec is buildable; they are one of the two candidates, not a ruling.

**The decision.** Is the site's accent **steel `#35618f`** or **rust `#A03E1E`**?

**Why it is decidable rather than a preference.** The editorial direction has an obvious accent — **rubrication**, the red ink a scribe used to mark the parts of a manuscript that told you how to read it. That is a good description of what Intent does to a codebase. But `--error` is already red, inherited from the tool (§1). Measured in CIELAB, ΔE76:

| Candidate       | Hue    | ΔE vs `error:` | ΔE vs `warning:` |
| --------------- | ------ | -------------- | ---------------- |
| rust `#A03E1E`  | 45.1°  | **13.8**       | 30.8             |
| steel `#35618f` | 269.6° | 77.1           | 81.2             |

Rust sits 14 degrees of hue from the `error:` red at ΔE 13.8 — the bottom of the distinguishable band, and that band is calibrated on large swatches. **A link inline in prose and an `error:` line in a terminal block are both small text, where discrimination is worse still.** Rust against `warning:` is fine; it is the error token specifically that it lands on.

**The constraint it must respect.** §1 is the founding principle: the semantic palette is inherited from the tool's output vocabulary and is not ours to choose. A terminal renders errors red.

**What breaks if it goes the other way.** If the accent is rust, then **either `error:` moves off red — which kills the inheritance principle and with it the whole coherence argument — or the two collide on the page.** Both cannot hold. So the real question is not which accent; it is **whether the accent can be red at all**, and answering that settles the hue as a consequence.

**A note on the instrument, because it nearly went the other way.** WCAG contrast ratio was tried first and returned rust-vs-error at 1.09 and steel-vs-error at 1.11 — near-identical, which would have condemned both accents equally. That reading is false on its face: a dark blue and a dark red do not look alike. **WCAG contrast is a lightness metric and is structurally blind to hue, which is the only axis this question is about.** It was caught because both candidates happened to sit in the same comparison table — **the control was a by-product of the format, not a precaution anybody took.** That is the more useful form of the lesson: measure the case you are not arguing for as a habit, because you will not notice you needed it until it contradicts you.

**If the accent moves, `--note` moves with it.** See Decision F.

## 4. Typography

**Serif for prose, mono for everything machine-shaped, and no sans-serif anywhere.** This is the most distinctive decision in the system and it should not be softened later by adding a "UI font".

| Role            | Family            | Licence |
| --------------- | ----------------- | ------- |
| Prose, headings | **Newsreader**    | OFL     |
| Machine text    | **IBM Plex Mono** | OFL     |

**Both are self-hosted, subset, and served same-origin.** Not from Google Fonts, not from any CDN. Self-containment (§9) is a hard constraint and an off-origin font request breaks it before the bytes are even counted. Subset to latin, ship the variable weight axis on Newsreader, and ship one weight of Plex Mono.

**The type is the largest single cost in the page budget and that is the trade the editorial direction buys.** Budget it explicitly at §9 rather than letting it arrive by surprise.

```css
  --font-prose: "Newsreader", "Iowan Old Style", Palatino, Georgia, serif;
  --font-mono:  "IBM Plex Mono", ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
```

**Fallbacks are in the same genre deliberately** — a font that fails to load must degrade to something that still reads as set text, not to Helvetica.

**Newsreader carries an optical-size axis and it must be used.** `font-optical-sizing: auto`. This is most of what makes the page look set rather than styled, and it costs one line.

### Mono earns its place; it is not the leftover

Monospace carries anything the tool would print, and everything that is a machine fact rather than an argument: commands, output, file paths, flags, identifiers (`ST0056`, `AC-08.6`), version numbers, navigation, table headers, figure labels, the footer, and the wordmark.

**Body prose is never monospace.** All-mono body text reads as costume and measurably slows long-form reading, which is what the docs are.

Mono runs at `0.875em` of its context with `letter-spacing: 0.01em`; Plex Mono's x-height sits high against Newsreader and will otherwise look a size too big.

### Scale and rhythm

A 1.2 minor third, rounded to whole pixels, with one deliberate jump at the display size so the opening statement has presence on a page that is otherwise quiet.

```css
  --t-xs: 12px;  --t-sm: 14px;  --t-base: 17px;  --t-lg: 20px;
  --t-xl: 24px;  --t-2xl: 29px; --t-3xl: 35px;
  --t-display: clamp(2.5rem, 6vw, 4rem);
```

Line height is `1.6` for prose, `1.45` for code, `1.2` for headings, `1.05` for display. Display and the largest heading set at `letter-spacing: -0.02em`; Newsreader opens up at size and needs pulling in. Body sets at `0`, mono sets loose.

Headings step `--t-2xl` / `--t-xl` / `--t-lg` and stop. **There is no `h5`.** A page needing a fifth level needs splitting, which is a content finding rather than a type-scale gap.

## 5. Layout

One content column, capped, centred.

```css
  --measure:    68ch;   /* running text -- not negotiable */
  --width-wide: 46rem;  /* code, tables, figures may break out */
  --width-page: 68rem;  /* header and footer rules only */

  --s-1: 4px; --s-2: 8px; --s-3: 12px; --s-4: 16px;
  --s-6: 24px; --s-8: 32px; --s-12: 48px; --s-16: 64px;
```

**Prose is capped at `--measure` and this is the reason the layout is single-column.** Code, tables and figures may break out to `--width-wide` and scroll inside themselves; running text may not.

Section separation is **space, not lines**. The only horizontal rules are the one under the header and the one above the footer. Margins are `--s-6` at mobile and `--s-12` from `48rem` up.

**Nothing is sticky except the top bar**, and the top bar is a wordmark and three links.

_(docs shell)_ A hosted docs set adds a left nav — static above `900px`, a disclosure below, a list of links, no animation — and an optional right-side contents above `1200px`. **The contents may not be sticky-with-active-highlighting** unless implemented without scroll-jank; a jittering highlight is worse than none.

## 6. Components

### 6.1 Header

Mono, `--t-sm`, on a hairline. Left: the wordmark. Right: `docs` and `github`. No logo image and no hamburger — three items fit at every width. Nav links are `--ink-muted` going to `--ink` on hover, **and are not underlined**; the underline is reserved for prose links so it keeps meaning something.

### 6.2 The terminal block — the one component that carries the whole voice

**It shows real captured output. Never simulated, never animated, never a screenshot.**

```
  $ intent st new "Port the acceptance gate"
  created: ST0069
  ok:      intent/st/ST0069/info.md
```

Four rules, each closing a real failure:

1. **The prompt character is not selectable and is not copied.** A copy control copies the command without the `$`. A reader who pastes `$ intent ...` and gets `command not found` was failed by the page.
2. **Output is styled by its prefix**, using the semantic tokens. The colour comes from the prefix, so it cannot disagree with the terminal.
3. **Never truncate a sample in a way that removes a caveat.** This is `intent/st/ST0056/output-contracts.md`'s finding applied to the page: _the reader's view is not the author's string_. A sample that cuts a `warning:` line to fit is the same defect class as an instrument that emits its caveat into a comment. If a sample is too long, show less of the **start**, or show it whole and let it scroll.
4. **No typing animation.** It delays the information, it cannot be copied mid-run, and it is the most common silly affordance on tool sites.

**No window chrome, no title bar, no traffic lights, no language tag.** The block sits in the page; it is not a picture of a terminal.

### 6.3 Code blocks and inline code

Code blocks: `--bg-sunken`, `--border` in `--rule`, `--radius`, `overflow-x: auto`, no line numbers by default. **Syntax highlighting is optional and must be build-time**, never a client-side highlighter — that is JavaScript the page does not need plus a flash of unstyled code for everyone.

Inline code: mono at `0.875em`, `--ink`, on `--bg-sunken`, `0.15em 0.35em` padding, `--radius`. **No border** — inline code appears often enough that a border would speckle the page.

### 6.4 Callouts are the prefix vocabulary

There is no "tip", no "info", no "danger". There are **`note:`, `warning:` and `error:`**, rendered as a left rule in the semantic colour with the prefix set in mono. They match what the tool prints, which is the entire point.

### 6.5 The rubric

**One pulled-out passage type, distinct from a callout**, for the sentence a reader most needs and would otherwise skim. A `2px` `--accent` left rule, `--s-4` padding, no fill, no icon, no label.

**One or two per page at most.** It is a rubricator's mark and must stay rare enough to mean something; a page with four of them has none.

### 6.6 The thread figure

One diagram, on the home page, as **inline SVG drawn to these tokens** — not an image file, not a library, not a screenshot. Hairlines in `--rule`, labels in mono at `--t-sm`, satisfied criteria marked with the accent.

It shows one thread resolving into work packages and criteria:

```
  ST0042  Rate-limited cache
    |
    +-- WP01  Cache layer      AC-01.1  evicts under load    [x]
    |                          AC-01.2  survives restart     [x]
    +-- WP02  Rate limiter     AC-02.1  refuses over quota   [ ]
```

**Draw it as a diagram, not as this ASCII** — the ASCII states the content and the relationships. It carries a text alternative describing the same structure, and it must not be the only place any of that information appears.

### 6.7 Tables

Mono headers at `--t-sm` in `--ink-muted` on a `--rule` bottom border. Rules between rows, **none between columns**, no zebra striping, no outer border. Numerics right-aligned and in mono. Tables scroll inside an `overflow-x: auto` wrapper; the page body never scrolls sideways at any width.

### 6.8 Links

`--accent`, underlined, `text-underline-offset: 0.15em`, `text-decoration-thickness: 1px`. Hover changes shade and thickness — **not size, not position**. Never colour-only: that fails for the colour-blind and it fails in print. **Link text must describe its destination** — no "click here", no bare URLs in running text.

## 7. The home page

The site is a single scroll. The copy below is a **draft to design against**, reconciled against the shipped command surface before launch.

**1. Header.** Wordmark, `docs`, `github`.

**2. One sentence saying what Intent is**, in prose, about fifteen words. Not a tagline with the verb missing. Then one paragraph, then the install line.

> ### Capture why the code exists.
>
> Your code says what it does. It does not say what you were trying to achieve, what you ruled out, or what breaks if someone changes it. Intent gives that a place to live — in the repository, next to the code, in a form both your team and your coding agents can read.
>
> `brew install matthewsinclair/intent/intent`

**3. One real terminal capture proving the claim**, above the fold. This is esbuild's move: the proof is immediate, and it is a capture rather than an adjective.

**4. The problem, stated once.**

> ### Code archaeology is a tax you pay forever
>
> Every codebase accumulates decisions nobody wrote down. Six months later the reasoning is gone and all that is left is the artefact, which cannot be interrogated. New engineers spend weeks reconstructing it. AI agents cannot reconstruct it at all — they read what the code does and build confidently on assumptions you would have rejected in a sentence.
>
> Comments rot because they sit beside the code and nothing checks them. Design docs rot because they sit apart from it and nothing links them. Intent puts the reasoning in the repository as tracked, checkable structure.

**5. The model, in about four sentences.** Carries the thread figure (§6.6). A reader who stops here should still know what the tool believes.

> ### A steel thread is one intention, followed end to end
>
> Not a ticket and not a task. A steel thread names something you are trying to achieve, records why it matters, breaks into work packages, and states the acceptance criteria that decide when it is genuinely done. Criteria are not prose promises — each is backed by an acceptance test, and Intent computes whether the thread is satisfied rather than asking you to assert it.

**6. Working with agents.**

> ### Written for the way you actually work now
>
> Intent generates the context files coding agents read — `AGENTS.md` for the tool-agnostic contract, `CLAUDE.md` for Claude Code — from the project's real state rather than from a file someone remembered to update. It ships a rule library agents can be held to, per-language critics that check work against those rules, and commit-time gates that refuse changes contradicting what the project said it was doing.
>
> The point is not that an agent reads your docs. It is that the docs are generated from something that cannot silently drift.

**7. Install**, with the platforms named and nothing hidden behind a tab that defaults wrong.

**8. Footer.** Docs, GitHub, changelog, licence, author. Mono, `--ink-muted`, on a hairline.

**What the home page does not have**: a hero image, a logo wall, testimonials, a metrics counter, a newsletter capture, a comparison table against named competitors, or any call to action that is not "install it" or "read the docs". **One primary action, and it is `brew install matthewsinclair/intent/intent`.**

## 8. Forbidden affordances

This list is checkable. A review can run it.

- Autoplaying or typing terminal animations
- Scroll-jacking, scroll-driven reveals, parallax
- Animated gradients, mesh backgrounds, glow effects
- Custom cursors, cursor followers
- Floating chat widgets, feedback tabs, "was this helpful" thumbs
- Cookie banners (there are no cookies; see §9)
- Third-party analytics, fonts, embeds, or scripts of any kind
- Logo walls, testimonials, star counts as decoration
- Carousels
- Modal dialogs on entry or exit
- Motion that does not respect `prefers-reduced-motion`
- A dark-mode toggle that animates the whole page
- Emoji as interface

**Transitions are permitted only on `color`, `background-color`, `border-color` and `opacity`, at 120ms or less. Nothing moves position.**

## 9. Accessibility and performance — both are budgets, not aspirations

- **Contrast.** Body text at or above 7:1 against its ground; large text and UI at or above 4.5:1. The semantic colours were chosen to clear this in both modes and **must be re-measured after any palette edit** — a hue nudge that keeps the character of a colour can lose a whole ratio point.
- **Colour is never the only carrier.** Links are underlined as well as coloured. `error:` is a word before it is a colour. Satisfied criteria in the figure carry a mark, not just an accent.
- **Focus is always visible**: a `2px` accent outline at `2px` offset. Never `outline: none` without a replacement.
- **Semantic HTML.** One `h1`, no skipped levels, real `nav` / `main` / `article`, real `button` for actions and real `a` for navigation.
- **Keyboard.** Everything reachable, in document order, no traps.
- **JavaScript is progressive enhancement only.** Copy buttons and an optional theme toggle are enhancements. With JS off, every page reads and every link works.
- **Budget: under 160KB on the home page, uncompressed, including everything.** The two subset webfonts are the largest single item at roughly 45–60KB together, and that is the cost the editorial direction buys — it is stated here so it is a decision rather than a surprise. If the budget cannot be met, the fonts are what gets revisited, not the accessibility items.
- **Zero off-origin requests. This is the hard constraint and it is checkable in one line of devtools.**

## 10. Voice

The site inherits Intent's house style, which is enforced in this repository. The site is not exempt.

- **`eg`, never `e.g.`**
- **No vanity metrics.** No test counts, no star counts, no "trusted by" strip, no download numbers.
- **No banners, no unicode decoration, no emoji.**
- **Say the thing, then stop.** Front-load every section: the point in the heading, the argument in the first sentence, the qualification after. A reader scanning only the headings should get the whole case.
- **Claims are checkable or they are cut.** No "10x", no "revolutionary", no "seamless".

## 11. Open decisions

**These go to the design agent at Laksa, not to hv in a workstream session** (hv's ruling, 2026-08-29). **Nothing here has been quietly settled by whoever held the pen**, and if you find something in this document that reads as decided but appears below, the section above is a placeholder and this is the authority.

**A flag that says "open" is not the same as one that says what would close it.** Each decision below carries three things: the decision itself, the constraint it must respect, and what breaks if it goes the other way. A decision presented without those gets resolved on taste, which is the same ambiguity in a tidier format.

### A. The accent, and whether it can be red at all

Specified in full at §3. **Steel `#35618f` vs rust `#A03E1E`.** The constraint is §1's inheritance principle; the consequence of choosing rust is that either `error:` leaves red or the two collide at ΔE 13.8. **Resolve the red question and the hue follows.** The token values in §3 are a placeholder.

### B. Search

**Decision.** Build-time static index, or no search at all with strong navigation?

**Constraint.** §9's zero-off-origin rule forbids a hosted search index. That is not negotiable — it is the same rule that forbids third-party fonts and analytics.

**What breaks either way.** A build-time index adds bytes to a page budget already carrying webfonts (§9), and it has to be regenerated whenever the docs change or it lies. No search at all puts the entire burden on navigation, which is survivable for a one-page site and gets steadily worse if the docs are ever hosted. **This decision is cheap now and expensive after the docs land**, so it wants deciding before the docs shell exists rather than after.

### C. The wordmark

**Decision.** Is there a mark, and what is it?

**Constraint.** There is no logo in the repository and none has been invented. Mono-set `intent` in lowercase is the obvious placeholder.

**What breaks either way.** The placeholder may simply be right permanently — a tool whose voice is "no banners, no unicode decoration" is not obviously improved by a glyph. A drawn mark then has to earn its place against §8's forbidden-affordance list, which rules out most of what a mark is usually for.

### D. Versioned docs, and the canonicals

**Decision.** Does the v3 documentation get a version switcher, and what happens to the `docs/v2/` canonicals?

**Constraint, and this one is not purely a design question.** v2 is frozen under `docs/v2/` with its canonical URLs **deliberately still pointing at the old `docs/blog/` locations** — hv ruled that on 2026-08-29 to preserve a year of inbound links and their search ranking. The archive is intentionally not authoritative for its own content, and `docs/v2/README.md` records why so nobody "repairs" it.

**What breaks if it goes the other way.** A switcher that rewrites those canonicals to match their new paths discards every inbound link and whatever ranking those posts have. **That is not recoverable once reindexed.** A design agent should be told this rather than left to discover it, because the tidy answer is the destructive one.

### E. Syntax highlighting theme

**Decision.** If build-time highlighting is used, is its palette derived from the tokens in §3 or imported?

**Constraint.** §6.3 already rules out client-side highlighting — that is JavaScript the page does not need plus a flash of unstyled code for everyone. This decision is only about the palette.

**What breaks if it goes the other way.** An imported theme gives the site **two colour systems**, one of which knows nothing about the semantic vocabulary in §1. The first time a highlighted keyword lands near an `error:` line, the page is saying two things with colour.

### F. The `--note` token

**Decision.** Does `note:` stay a colour, or become a weight or a mark?

**Constraint.** `note:` is the lowest-emphasis prefix in §1 and must not compete with the accent — which, under the steel placeholder, is also blue.

**What breaks if it goes the other way.** Check it against `--accent` and `--ink-muted` side by side on a real page. **If it reads as either of them it is doing no work**, and a colour that carries no distinction is worse than no colour because it implies one. **This decision is downstream of Decision A** — if the accent moves off blue, `--note` may be fine exactly as it is.

### G. The thread figure's composition

**Decision.** How is §6.6 actually drawn?

**Constraint.** §6.6 specifies its content, its tokens, its text alternative, and that it is inline SVG rather than an image, a library or a screenshot.

**What breaks if it goes the other way.** Nothing structural — this is a genuine drawing decision and belongs to whoever draws it. It is listed so it is visibly a decision rather than an omission.

### H. The theme toggle

**Decision.** Is there one, or does the site respect `prefers-color-scheme` alone?

**Constraint.** Both palettes are fully specified in §3 either way, and §8 forbids a toggle that animates the whole page.

**What breaks if it goes the other way.** A toggle needs the `[data-theme]` selectors in §3 wired in both directions, and it is JavaScript — permitted under §9 only as progressive enhancement, so the page must be correct with it off.

## 12. What this document is not

**It is not ratified, no page has been built against it, and eight of its decisions are open (§11).** Every claim is a design intention rather than a measurement — with the single exception of the prefix table in §1, measured at `HEAD` on 2026-08-29, which is the one thing here that is a fact about the tool rather than a proposal about the site.

---

_Consolidated by vc for ST0068 from cc's and vc's independent drafts. The copy in §7 is a draft to design against and is reconciled against the shipped command surface before launch._
