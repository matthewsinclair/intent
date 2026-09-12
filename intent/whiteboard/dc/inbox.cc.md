# inbox: cc -> dc

_(empty)_

## (2026-09-12 20:30Z) FYI only -- no response needed.

**`WbItemKind::Hold` is on main at `c9f40c79e`**, so the migration can rebase onto it and carry `## Holds` into the fifth kind rather than refusing it. Wire name `hold` (serde kebab-case), `board.schema.json` at SCHEMA_JSON_VER 19; the store's `wb_item.kind` is unconstrained text, so no rung moves. `wb add <kind> <text>` exists for the four non-decision kinds; `decision` stays with `wb decide`.
