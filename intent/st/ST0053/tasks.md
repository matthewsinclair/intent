# Tasks - ST0053: Content project-type pack

## Tasks

- [x] WP01 -- IN-PR-* prose base + author refactor
- [ ] WP02 -- content (CO) rule tiers
- [ ] WP03 -- critic-prose (rename critic-author)
- [ ] WP04 -- content lang canon
- [ ] WP05 -- in-content-essentials skill + dispatch
- [ ] WP06 -- dogfood + docs + close

## Task Notes

Build order and per-WP acceptance live in `acceptance.md`; design decisions D1-D5 (hv-ratified) in `design.md`. D5=(a): 2.15.1 shipped first (done), the pack ships as 2.16.0.

## Dependencies

- WP02..05 depend on WP01 (the `IN-PR-*` base + the lang-code plumbing pattern).
- WP06 (dogfood + close) depends on WP02..05 and on hv providing a content target + release framing.
