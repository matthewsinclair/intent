# Design - ST0053: Content (web-content) project-type pack

Status: **scope framing only.** No WP is authored until hv ratifies the decisions below, the same discipline ST0052 followed (D1-D7 ratified before build). Proposals are cc's recommendation; the `?` items are genuine open questions for hv.

## The crux: reuse, not copy (Highlander)

The author pack (ST0052) already built a mechanical prose surface:

- `IN-AU-STYLE-001` banned-filler-and-house-style
- `IN-AU-STYLE-002` no-vanity-metrics
- `IN-AU-STYLE-003` front-matter-and-objectives
- `IN-AU-STYLE-004` heading-hygiene
- `IN-AU-STYLE-005` mechanical-trope-pass (references the single `in-detrope` catalogue)

Most of that surface is discipline-agnostic prose hygiene -- it applies to a landing page as much as to a book chapter. Copying it into `IN-WC-STYLE-*` would be a textbook Highlander violation: two divergent copies of banned-filler / trope-pass / heading-hygiene that drift within a quarter.

**Proposed resolution (for ratification):** extract the shared mechanical surface into a common **prose base** that both `author` and `content` reference, and let each pack own only its genuinely distinct rules. Author keeps `front-matter-and-objectives` (book/course IA); content adds its own IA + craft rules; the shared mechanical rules (filler, vanity-metrics, heading-hygiene, trope-pass) live once. This makes ST0053 partly a **refactor of ST0052** (lift the shared rules to the base), not a greenfield copy.

## Design Decisions (hv RATIFIED 2026-07-07)

- **D1 -- language code: `CO` (content).** RATIFIED `CO` (overrides cc's `WC` lean). Rationale (hv): the pack is literally _content_ -- books, courses, etc -- not web-specific. Rule prefix `IN-CO-*`; the code threads through the rule-id validator + `LANG_SUBDIRS` enumerator in `rules_lib.sh` (ST0052/WP01's 5th-site discovery). NB: rename the thread's framing from "web-content" to "content".
- **D2 -- prose base pack: `IN-PR-*`, shared, NOT agnostic.** RATIFIED option (a): a new `IN-PR-*` "prose" base pack that both `author` and `content` depend on. Rationale (hv): prose hygiene is not generic to code; keep it out of the code-agnostic pack. If a code project ever needs a prose rule, it can add `IN-PR-*` to itself. This makes ST0053 partly a **refactor of ST0052** (lift the shared mechanical rules -- filler, vanity-metrics, heading-hygiene, trope-pass -- from `IN-AU-*` into `IN-PR-*`, and re-point `author` at the base).
- **D3 -- one `critic-prose`, parameterised by declared language.** RATIFIED: a single `critic-prose` serves both disciplines, parameterised by the declared language (not a separate `critic-content`). One two-form-detrope wiring, one critic; the author/content difference is the craft-tier rule set it loads.
- **D4 -- what content genuinely owns (IN SCOPE, not deferred).** RATIFIED list stands: web voice / scannability, link + CTA hygiene, page meta/front-matter (title, description, canonical), image alt-text presence, reading-level target. These are `content`'s `craft` tier (authored in the pack's rule-tier WP); the `style` tier is mostly the shared `IN-PR-*` base. "Not blocking" earlier meant only that D4 does not gate WP _authoring_ (D1-D3 shape the structure) -- D4 ships _with_ the pack, not later.
- **D5 -- release framing: cut 2.15.1 first (hv RATIFIED (a)).** The pending fixes ship as the **2.15.1 patch** now; the content pack is built after and ships as **2.16.0** (its own minor). ST0053 build does NOT start until 2.15.1 is tagged, so the release tree stays clean.

## Architecture

Reuses the language-pack machinery end-to-end (the ST0052 / ST0037 axis): `intent lang init content` writes config `languages`, installs canon templates, and the pack activates through the same `/in-session` fan-out and `/in-review` critic dispatch as any code language. The only net-new structural work is the **prose base extraction** (D2) -- everything else is the ST0052 shape applied to a second discipline.

## Provisional WP outline (contingent on D1-D3)

Mirrors ST0052's six-WP shape, adjusted for the base-pack extraction. Illustrative, **not created** -- `intent wp new` runs only after D1-D3 are ratified:

1. Extract the shared mechanical surface into the `prose` base (D2); re-point `author` at it (the ST0052 refactor half).
2. `content` style + craft rule tiers (only the genuinely distinct rules; `WC` code plumbing per D1).
3. `critic-content` (or the content mode of `critic-prose`, per D3).
4. `intent lang init content` canon (`RULES-content.md` + `ARCHITECTURE-content.md`).
5. `/in-content-essentials` skill + `content -> critic-content` dispatch in `/in-review` + `/in-session`.
6. Dogfood (a real web-content target), docs, close.

## Alternatives Considered

- **Copy the author pack (`IN-WC-*` = duplicate of `IN-AU-*`).** Rejected: direct Highlander violation; the shared mechanical surface would drift. This is the anti-pattern D2 exists to avoid.
- **Fold web-content into the author pack (one `author` pack covering both books and pages).** Rejected: the craft tiers genuinely diverge (long-form continuity vs page scannability/CTA), and a project may want one without the other. Separate packs over a shared base is the right granularity.
