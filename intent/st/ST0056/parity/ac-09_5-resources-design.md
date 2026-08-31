# AC-09.5 — MCP resources design

**Criterion:** _MCP resources serve the read surfaces (wip, whiteboard boards, ST docs) and their contents match what the equivalent CLI read returns._
**Test:** `AT-09.5` → `native/rust/crates/intent-cli/tests/mcp_resources.rs` (to-write).
**Status:** bound as option (A) by vc, 2026-08-31, under hv's standing pen; the wip/boards scope half goes to hv costed. Build on this.

## The measurement that shapes the design

AC-09.5's second clause — _contents match the equivalent CLI read_ — is only satisfiable for a surface that HAS an equivalent CLI read behind a facade door. Measured at HEAD:

| surface                      | facade read door                             | CLI read                                                                  | match target exists? |
| ---------------------------- | -------------------------------------------- | ------------------------------------------------------------------------- | -------------------- |
| **ST / WP / issue entities** | `Facade::st_show` / `wp_show` / `issue_show` | `intent st show <ID>` etc.                                                | **YES**              |
| **wip.md** (`intent/wip.md`) | none                                         | none — `todo` reads todo.md, `info` reads the overview, neither is wip.md | **NO**               |
| **whiteboard boards**        | none                                         | none — `claude ws` manages boards, no read-a-board verb                   | **NO**               |

**And what `st show` actually returns is the ENTITY MODEL, not a doc file** (measured 2026-08-31): `intent st show ST0056` and `intent st show ST0056 design` print the SAME thing — `id: title / status / reason / created / completed` — because the show arm calls `f.st_show(&id)` and never reads the `FILE` argument. `FILE` is `st edit`'s argument (it selects a doc file to open), not `st show`'s. So "ST docs" here means the ST ENTITY read through `st_show`, and the equivalent CLI read is `st show <ID>`. The doc FILES (design.md, impl.md) have no CLI read that returns their content — `st edit` returns a path — so they are not a resource with a match target either.

So the resource set is exactly the entities with a `*_show` facade door, and "contents match the CLI read" is a comparison against `<entity> show`. wip.md and the boards have no door and no CLI read, so serving them would leave the row asserting agreement between a resource and nothing — the register-overclaim class, and the reason (C) below is rejected.

## The bound: option (A), ruled

**Resources serve model entities through EXISTING facade doors. Zero new facade surface.**

- `intent:///threads/<id>` → `Facade::st_show(id)` → matches `intent st show <id>`
- `intent:///threads/<st>/wp/<n>` → `Facade::wp_show(st, seq)` → matches `intent wp show`
- `intent:///issues/<nnnn>` → `Facade::issue_show(n)` → matches `intent issues show`

The URIs are the `intent://` address scheme's own — **plural, four-digit issue ids, `address.rs`'s grammar**, not spelled by this surface.

**(B) build file-read doors + CLI verbs for wip and boards** was declined for the tag: it is new surface (`Facade::wip`, `Facade::board(node)` plus their CLI verbs) and the choice of building it before the tag is hv's, taken to hv costed — the same division as the GraphQL hatch, where vc bounded under the pen and hv ruled the build. **(C) serve wip/boards as raw file resources with no CLI equivalent** was rejected: a read beside the facade breaks the one-door rule AND makes the match clause vacuous — two defects for one.

**The generalisation, recorded because it is `AC-09.6`'s shape one surface over:** _served implies there is a CLI read to agree with_ — the same principle as _a tool that cannot be served must not be published_, applied to resources. Two instances now, two directions, one tier.

## The three design rules

1. **One facade door per resource** (`AC-09.1` / `AC-09.6`). A resource reads through a `Facade` method, never a file read beside the facade, so the resource and the CLI read are the SAME read and "contents match" holds by construction rather than by a second implementation kept in sync. The MCP tier calls the facade, never the CLI dispatch arm (vc's MCP ruleset (a)).

2. **The URI is an `intent://` address, PARSED AND RENDERED BY `address.rs`** — the estate's one home for the scheme (`D57-8`), guarded by `address_resolution_single_home::no_second_resolver_exists`. `resource_list` renders each URI from an `address::Entity`; `resource_read` recovers the entity with `address::parse`; the three forms with a `*_show` door become resources and every other address is refused. **CORRECTION, recorded rather than overwritten (ic, 2026-08-31, on cc's guard):** the first draft of this design and its first code cut spelled a SINGULAR `intent:///thread/<id>` "derived from `nav.rs`'s kinds". That was wrong twice — it hand-spelled the scheme (a second resolver the guard reds), and it used the wrong grammar. The `intent://` scheme is `address.rs`'s and is PLURAL (`intent:///threads/ST0056`); the singular `/thread/…` is `nav.rs`'s TUI/web path, a DIFFERENT single-home grammar. Two legitimate grammars, and the design proposed a third by hand between them. The reason is kept because a later reader tempted to "align the resource URI with the TUI path" would re-make exactly this mistake.

3. **`resources/list` == `resources/read`, both directions** — the same agreement `mcp_surface.rs` pins for tools. A listed resource that read cannot serve, or a readable resource not listed, is the tool-roster defect on the resource surface.

## The one build decision: the render seam

"Contents match `st show`" is strongest as byte-identity, and byte-identity needs ONE renderer. Today the entity→text rendering lives inside `render.rs`'s show arms; the MCP tier cannot call `render.rs` (it calls the facade). So the entity→text renderer is extracted to one shared function both faces call:

- a `fn` taking `&Thread` (`&WorkPackage`, `&Issue`) and returning the exact text `st show` prints,
- `render.rs`'s show arm rewired to call it (Highlander — the CLI keeps one renderer),
- `mcp.rs`'s resource read calling `st_show` then the same function.

Then `mcp_resources.rs` asserts the resource's content equals `intent st show <id>`'s stdout, and the equality is true by construction rather than by two renderers agreeing today. This is `AC-09.4`'s "agreement is not derivation" applied to the two faces of one read: the test must not pass merely because two renderers coincide on today's data.

## What `mcp_resources.rs` (AT-09.5) witnesses

- `resources/list` over a real stdio session enumerates one resource per entity the facade can `*_show`, and every listed URI round-trips through `address::parse` (the plural `intent://` grammar), not a grammar spelled by this surface.
- `resources/read(uri)` returns content byte-identical to the equivalent CLI read (`st show <id>`), driven for a thread, a work package and an issue.
- list and read agree both ways (no listed-but-unreadable, no readable-but-unlisted).
- **Scope stated, not discovered:** wip.md and the whiteboard boards are NOT resources under this bound, because they have no facade door and no CLI read to match; they are the hv-costed follow-on.

## The criterion reword — landed

**AC-09.5's TEXT read "wip, whiteboard boards, ST docs" until 2026-08-31.** Greening AT-09.5 against a test that covers only entity docs would have marked the criterion satisfied by a test that does not touch two of the three surfaces it named — the overclaim class. vc reworded it under the pen at `f27829df`, before the green rather than after: the row now scopes to _the read surfaces THAT HAVE A CLI READ TO AGREE WITH — ST/WP/issue docs_, with the `wip`/boards half named as a CLI-gap follow-on (they need `Facade::wip` / `Facade::board(node)` and their verbs, which is a product question about the CLI, not an MCP gap). The build does not depend on the reword; the green does, and the reword is landed, so `AT-09.5` is clear to green when `mcp_resources.rs` lands.
