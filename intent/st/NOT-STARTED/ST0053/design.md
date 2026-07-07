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

## Design Decisions (hv to rule -- proposals, not ratified)

- **D1 -- language code.** `WC` (web-content) or `CO` (content)? cc leans `WC` (unambiguous). The code threads through the rule-id validator + `LANG_SUBDIRS` enumerator in `rules_lib.sh` (ST0052/WP01's 5th-site discovery).
- **D2 -- base-pack shape (load-bearing).** How is the shared prose surface expressed so both packs reference it without copy? (a) a new `IN-PR-*` "prose" base pack that author + content depend on; (b) promote the mechanical rules to the **agnostic** pack as prose-applicable rules; (c) a `concretised_by`-style cross-reference. cc leans (a) -- a `prose` base pack -- as the cleanest Highlander home that stays out of the code-agnostic pack. **The WP plan depends on this.**
- **D3 -- critic reuse.** One `critic-prose` serving both disciplines (parameterised by declared language), or a distinct `critic-content` mirroring `critic-author`? cc leans a shared `critic-prose` core with a thin content-specific craft mode, to avoid a second copy of the two-form detrope wiring (ST0052/WP03).
- **D4 -- what content genuinely owns.** Draft for hv: web voice / scannability, link + CTA hygiene, page meta/front-matter (title, description, canonical), image alt-text presence, reading-level target. These are the `craft` tier; the `style` tier is mostly the shared base.
- **D5 -- release framing.** Ships as **2.16.0** (minor; new project-type surface), its own release, not folded into the 2.15.x patch line. (Confirmed direction; listed for completeness.)

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
