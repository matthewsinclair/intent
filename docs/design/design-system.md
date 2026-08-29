# Intent — design system

**This document is the design specification for `intent.laksa.io`.** It is written to be built from: `/design` at Laksa reads it, and laksa-{vc,cc} implement it as a Laksa custom theme (`theme/theme.yaml` + `theme/layout.liquid` + `assets/`) in `../Sites/intent`.

It specifies a **one-page site**. Intent's documentation lives in this repository at [`docs/`](../) and the site links to it rather than reproducing it. If the docs are later hosted, every token and component below extends to them unchanged — that is why the type scale and components are specified beyond what one page needs.

---

## 1. The idea the design is built on

**Rubrication.** In a manuscript, the scribe wrote the body in black and then went back with red ink to mark the parts that told you how to read it — the headings, the initials, the instructions. The red was not decoration. It was the layer that said _this is what matters and this is why_.

That is what Intent does to a codebase. The code is the body text. Intent is the red ink.

**So the site is a document, not an interface.** Paper ground, ink text, one red accent used the way a rubricator used it: sparingly, structurally, and always to mark meaning rather than to attract attention. Nothing glows, nothing floats, nothing animates on scroll. A reader should feel they have been handed a well-set page.

**This is also the visual form of a rule Intent already holds.** The CLI ships Rust-style prefixes — `ok:`, `error:`, `created:`, `done:` — with no banners and no unicode decoration. Restraint is the house voice. The site should not be louder than the tool.

### What this rules out

Stated explicitly, because these are the defaults every developer-tool site reaches for and every one of them would break the idea:

- **No gradient hero, no mesh, no glow, no glassmorphism.** The ground is flat.
- **No dark terminal window mock with traffic-light dots.** Code sits in the page, not in a fake window.
- **No emoji as interface.** The current README uses them; the site does not inherit that.
- **No animated typing effect on the command example.** It is a printed line.
- **No card grid of feature tiles.** Features are prose with headings, because the product's whole argument is that prose carries what tiles cannot.
- **No sans-serif.** See §3.

---

## 2. Colour

Two grounds, one ink, one accent. Everything else is derived.

### Light (default)

| Token           | Value     | Use                                           |
| --------------- | --------- | --------------------------------------------- |
| `--ground`      | `#FBFAF7` | Page background. Warm off-white, never `#FFF` |
| `--ground-sunk` | `#F3F1EB` | Code blocks, table header fill, inset panels  |
| `--ink`         | `#1A1815` | Body text. Warm near-black, never `#000`      |
| `--ink-muted`   | `#6B6558` | Metadata, captions, table labels, footer      |
| `--rule`        | `#DDD8CC` | Hairlines, borders, dividers                  |
| `--accent`      | `#A03E1E` | The rubric. Links, the mark, `error:`         |
| `--accent-sunk` | `#7A2E15` | Accent on hover / visited                     |

### Dark

Not an inversion — a different paper. Ink-on-vellum becomes chalk-on-slate, and the accent lifts so it stays legible without vibrating against the dark ground.

| Token           | Value     |
| --------------- | --------- |
| `--ground`      | `#16150F` |
| `--ground-sunk` | `#1F1E16` |
| `--ink`         | `#E8E4DA` |
| `--ink-muted`   | `#9C9585` |
| `--rule`        | `#33312A` |
| `--accent`      | `#D9764F` |
| `--accent-sunk` | `#E89570` |

**Implementation.** Define the light palette on bare `:root`. Redefine only the changed tokens under `@media (prefers-color-scheme: dark)` and again under an explicit `[data-theme="dark"]` selector so a toggle wins in both directions. Never give a colour its only definition inside a media query.

### The accent has three jobs and no others

1. **Links in prose**, underlined at 1px with a `0.15em` offset. Not a different weight, not a background.
2. **The rubric mark** — a `2px` vertical rule in `--accent` at the left of a pulled-out passage. This is the design's signature; see §6.4.
3. **The `error:` prefix** inside terminal output blocks.

`ok:`, `created:` and `done:` prefixes render in `--ink-muted`, not in green. **Intent does not use colour to mean success**, and the site should not teach a vocabulary the tool does not have.

---

## 3. Typography

**Two families. Serif for prose, mono for everything machine-shaped.** There is no sans-serif anywhere in the system — not in nav, not in labels, not in table headers. This is the single most distinctive decision in the design and it should not be softened by adding a "UI font" later.

| Role           | Family            | Source                                 |
| -------------- | ----------------- | -------------------------------------- |
| Display + body | **Newsreader**    | Google Fonts, variable, optical sizing |
| Machine text   | **IBM Plex Mono** | Google Fonts                           |

**Fallback stacks**, because a font that fails to load must degrade to something in the same genre rather than to Helvetica:

```
  --font-serif: "Newsreader", "Iowan Old Style", "Palatino Linotype", Palatino, Georgia, serif;
  --font-mono:  "IBM Plex Mono", "SF Mono", "Menlo", "Consolas", monospace;
```

**Newsreader carries an optical-size axis and it must be used.** Set `font-optical-sizing: auto` and let the display sizes take the high-contrast cut and the body take the sturdy one. This is most of what makes the page look set rather than styled, and it costs one line.

### Mono earns its place, it is not the leftover

Mono is not only for code. It carries every element that is a machine fact rather than an argument: **navigation, the version string, table headers, figure labels, the footer, command names in running prose, and metadata.** That division is what gives the page its structure without a second sans-serif weight ladder doing the work.

Mono runs at `0.875em` of its context with `letter-spacing: 0.01em`, because Plex Mono's x-height sits high against Newsreader and it will otherwise look a size too big.

### Scale

A 1.25 modular scale, with one deliberate jump at the display size so the opening statement has real presence on a page that is otherwise quiet.

| Step      | Size                          | Line height | Use                                  |
| --------- | ----------------------------- | ----------- | ------------------------------------ |
| `display` | `clamp(2.75rem, 7vw, 4.5rem)` | `1.05`      | The one statement at the top         |
| `h1`      | `2.25rem`                     | `1.15`      | Page title, if separate from display |
| `h2`      | `1.5rem`                      | `1.25`      | Section heads                        |
| `h3`      | `1.125rem`                    | `1.35`      | Subsection heads                     |
| `body`    | `1.125rem`                    | `1.65`      | Prose                                |
| `small`   | `0.9375rem`                   | `1.5`       | Captions, footer, metadata           |

**Display and `h1` set tight** — `letter-spacing: -0.02em` — because Newsreader at size opens up and needs pulling in. Body sets at `0`. Mono sets loose.

### Measure

**Prose is capped at `65ch` and this is not negotiable.** It is the whole reason the layout is single-column. Code blocks, tables and figures may break out to `--width-wide`; running text may not.

---

## 4. Layout

Single column, centred, generous. No sidebar on the one-page site.

```
  --width-prose: 65ch;      /* running text                    */
  --width-wide:  46rem;     /* code, tables, figures           */
  --width-page:  72rem;     /* header and footer rules only    */
```

**Vertical rhythm** on a `1.5rem` base: `0.75 / 1.5 / 3 / 4.5 / 7.5rem`. Section separation is **space, not lines**. The only horizontal rules in the design are the one under the header and the one above the footer.

**Margins.** `1.5rem` at mobile, `3rem` from `48rem` up. The page should never feel like it is touching the edge of the window.

---

## 5. The one page

The site is a single scroll. Six sections, in this order, with the copy below as the starting content. **The copy is a draft for the design to be built against, not final wording** — it will be reconciled against the shipped tool once the command surface is measured.

### 5.1 Header

Mono, small, sitting on a hairline. Left: `intent`. Right: `docs` and `github`. No logo image, no hamburger — three items fit at every width.

### 5.2 Opening

Display type, alone, with a lot of air. Below it one paragraph at body size, then the install line.

> ## Capture why the code exists.
>
> Your code says what it does. It does not say what you were trying to achieve, what you ruled out, or what will break if someone changes it. Intent gives that a place to live — in the repository, next to the code, in a form both your team and your coding agents can read.
>
> `brew install intent`

### 5.3 The problem, stated once

> ## Code archaeology is a tax you pay forever
>
> Every codebase accumulates decisions nobody wrote down. Six months later the reasoning is gone and all that is left is the artefact, which cannot be interrogated. New engineers spend weeks reconstructing it. AI agents cannot reconstruct it at all — they read what the code does and confidently build on assumptions you would have rejected in a sentence.
>
> Comments rot because they sit beside the code and nothing checks them. Design docs rot because they sit apart from it and nothing links them. Intent puts the reasoning in the repository as tracked, checkable structure.

### 5.4 The idea — steel threads

**This section carries the one diagram on the page.** See §6.5.

> ## A steel thread is one intention, followed end to end
>
> Not a ticket and not a task. A steel thread names something you are trying to achieve, records why it matters, breaks into work packages, and states the acceptance criteria that decide when it is genuinely done. The thread stays in the repository as the work happens, so the reasoning and the code age together.
>
> Criteria are not prose promises. Each one is backed by an acceptance test, and Intent computes whether the thread is satisfied rather than asking you to assert it.

### 5.5 Working with agents

> ## Written for the way you actually work now
>
> Intent generates the context files coding agents read — `AGENTS.md` for the tool-agnostic contract, `CLAUDE.md` for Claude Code — from the project's real state rather than from a file someone remembered to update. It ships a rule library agents can be held to, per-language critics that check work against those rules, and commit-time gates that refuse changes which contradict what the project said it was doing.
>
> The point is not that an agent reads your docs. It is that the docs are generated from something that cannot silently drift.

### 5.6 Getting started

Three steps, mono-numbered, each one line of command and one line of prose. Then a closing pair of links to the docs and the repository.

> ## Three commands
>
> `intent init` — set up a project, declare its languages, generate its agent contract.
> `intent st new "..."` — open a steel thread and write down why.
> `intent st show ST0001` — read it back, with its criteria and their current state.

### 5.7 Footer

Mono, `--ink-muted`, on a hairline. Version, licence, repository link, author. One line if it fits.

---

## 6. Components

### 6.1 Header nav

Mono `small`. Items separated by `1.5rem`. Links are `--ink-muted`, going to `--ink` on hover. **No underline in the nav** — the underline is reserved for prose links so that it keeps meaning something.

### 6.2 Code and terminal blocks

Two variants, and the difference matters.

**Command** — a thing you type. Mono on `--ground-sunk`, `1px` `--rule` border, `4px` radius, `1rem 1.25rem` padding. A `$ ` prefix in `--ink-muted`, non-selectable (`user-select: none`) so a copy takes the command and not the prompt.

**Output** — a thing the tool said. Same box. Prefixes render at their own weight: `error:` in `--accent`, everything else in `--ink-muted`, the message body in `--ink`.

```
  $ intent st new "Rate-limited cache for API protection"
  created: ST0042
  ok:      intent/st/ST0042/info.md
```

**Neither variant gets window chrome, a title bar, traffic lights, or a language tag.** Both scroll horizontally inside themselves (`overflow-x: auto`); the page body never scrolls sideways at any width.

### 6.3 Inline code

Mono at `0.875em`, `--ink`, on a `--ground-sunk` fill with `0.15em 0.35em` padding and a `3px` radius. **No border and no accent colour** — inline code appears often enough that a border would speckle the page.

### 6.4 The rubric

The design's signature element, and the only pulled-out passage type. A `2px` `--accent` rule on the left, `1.25rem` of padding, no background fill, no icon, no bold "Note:" label.

**Use it for the sentence a reader most needs and would otherwise skim past** — one or two per page at most. It is a rubricator's mark, so it must stay rare enough to still mean something. If a page has four of them it has none.

### 6.5 The thread figure

One diagram, in section 5.4, and it should be **inline SVG drawn to the system's own tokens** — not an image file, not a library, not a screenshot. Hairlines in `--rule`, labels in mono at `small`, satisfied criteria marked with the accent.

It shows one thread resolving into work packages and criteria:

```
  ST0042  Rate-limited cache
    |
    +-- WP01  Cache layer            AC-01.1  evicts under load      [x]
    |                                AC-01.2  survives restart       [x]
    +-- WP02  Rate limiter           AC-02.1  refuses over quota     [ ]
```

**Draw it as a diagram, not as this ASCII.** The ASCII states the content and the relationships; the SVG should render them as thin ruled lines with real typographic labels. It must carry a text alternative describing the same structure, and it must not be the only place any of that information appears.

### 6.6 Tables

Mono headers at `small` in `--ink-muted`, sitting on a `1px` `--rule` bottom border. No vertical rules, no zebra striping, no outer border. Rows separated by hairlines. Tables may break out to `--width-wide` and scroll inside an `overflow-x: auto` wrapper.

### 6.7 Links

In prose: `--accent`, `text-decoration: underline`, `text-underline-offset: 0.15em`, `text-decoration-thickness: 1px`. On hover the colour goes to `--accent-sunk` and the thickness to `2px`. **Link text must describe its destination** — no "click here", no bare URLs in running text.

---

## 7. Accessibility

Not a checklist appended at the end; several of these constrain the palette above and were chosen to satisfy them.

- **Contrast.** `--ink` on `--ground` is well past AA. `--accent` on `--ground` clears AA for body text, which is why the accent is a mid-dark rust rather than a bright red — a brighter one would have failed and the design would have had to stop using it for links.
- **Colour is never the only carrier.** Links are underlined as well as coloured. `error:` is a word before it is a colour. Satisfied criteria in the figure carry a mark, not just an accent.
- **Focus is visible and it is the accent.** A `2px` `--accent` outline at `2px` offset. Never `outline: none`.
- **One `h1`, no skipped heading levels**, and the heading text carries the argument so the document outline reads as a summary.
- **Motion.** There is essentially none by design. Whatever remains respects `prefers-reduced-motion: reduce`.
- **The figure has a text alternative** that describes the structure, not the picture.

---

## 8. Voice

The site inherits Intent's house style. These are enforced in this repository and the site is not exempt.

- **`eg`, never `e.g.`**
- **No vanity metrics.** No test counts, no star counts, no "trusted by" strip, no download numbers. The tool's value is not a number.
- **No banners, no unicode decoration, no emoji.**
- **Say the thing, then stop.** Front-load every section: the point in the heading, the argument in the first sentence, the qualification after. A reader scanning only the headings should get the whole case.
- **One primary call to action**, and it is `brew install intent`. The `docs` and `github` links are secondary and live in the header and footer, not competing in the body.
- **Claims are checkable or they are cut.** No "10x", no "revolutionary", no "seamless".

---

## 9. Build notes for laksa-{vc,cc}

- **Target:** Laksa custom theme in `../Sites/intent` — `laksa.yaml`, `index.md`, `theme/theme.yaml`, `theme/layout.liquid`, `assets/`. `../Sites/appendix3` is the working reference for the structure.
- **Host:** `intent.laksa.io`. No dedicated domain yet, so **do not hard-code absolute URLs to the site's own pages** — a domain move should not mean a content edit.
- **Fonts:** Newsreader and IBM Plex Mono from Google Fonts, `display=swap`, with the fallback stacks in §3 declared so the first paint is in the right genre.
- **Tokens:** every value in §2 and §4 becomes a CSS custom property on `:root`. Nothing in a component may hard-code a colour or a width.
- **Meta:** `title`, `description` and `canonical` are required on the page (`IN-CO-STYLE-001`). Every content image needs meaningful alt text (`IN-CO-STYLE-002`).
- **No build-time JavaScript is required and none should be added** beyond an optional theme toggle. The page must be complete and readable with scripting off.

### Open, and deliberately left to the implementer

- **Whether the theme toggle exists at all.** Respecting `prefers-color-scheme` alone is a defensible answer; both palettes are specified either way.
- **The exact SVG rendering of the thread figure.** §6.5 specifies its content, its tokens and its text alternative. Its composition is a drawing decision and should be made by whoever is drawing it.

---

_Written by vc for ST0068. The content in §5 is a draft to design against and is reconciled against the shipped command surface before launch._
