# ARCHITECTURE-lua.md

Lua-specific architectural patterns and conventions for this project.

## Module Layout

Standard Lua project layout (LuaRocks / Neovim plugin / generic):

- `<project>-scm-1.rockspec` -- LuaRocks rockspec (when applicable)
- `lua/<modname>/` -- per-module source
- `lua/<modname>/init.lua` -- module entry; returns the public table
- `tests/` -- test source (busted, plenary.nvim, or similar)
- `.luarc.json` -- LSP configuration (when applicable)

## Module Organisation

- One concern per module. Module-local tables are private; `return M` exposes the public surface.
- Internal helpers prefix with `_` and are not documented in the module README.
- Avoid global state; configuration is passed explicitly or stored in module-local tables.

## Error Handling

- Use `pcall` / `xpcall` at API boundaries to convert exceptions to `(false, err)` returns.
- Multi-return `nil, err` is the canonical error idiom for non-exceptional failures (file I/O, parsing).
- Always check the second return; never write `local x = io.open(path)` without inspecting the second value.

## Testing

- Use `busted` for standalone Lua, `plenary.nvim` for Neovim plugins.
- Tests live under `tests/`; one file per module (`tests/foo_spec.lua` for `lua/foo.lua`).
- Test helpers in `tests/helpers/`; never alongside production code.

## Build / CI

- `luarocks make` for LuaRocks projects; `make test` for ad-hoc.
- `luacheck` is gating in CI when `.luacheckrc` is present.
- `stylua` for formatting (gating in CI when `stylua.toml` is present).
- Pre-commit critic gate runs `intent critic lua` on staged files (per Intent canon).
