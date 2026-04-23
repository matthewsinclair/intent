-- Fixture: triggers IN-LU-CODE-001 (local-over-global) and
-- IN-LU-CODE-004 (pcall-for-errors) by design.

-- Module-level assignments missing `local` -- these leak as globals.
defaults = { timeout = 30, retries = 3 }
current = nil

-- Function at module scope without `local` -- also a global.
function load_config(path)
  local file = io.open(path, "r")
  local raw = file:read("*a")
  file:close()
  current = raw
  return raw
end

function save_config(path, text)
  local file = io.open(path, "w")
  file:write(text)
  file:close()
end

-- pcall return values discarded -- the error is swallowed entirely.
function try_parse(text)
  pcall(load, text)
  return current
end

-- Error raised with bare string; no structure for the caller to inspect.
function require_key(t, key)
  if t[key] == nil then
    error("missing key")
  end
  return t[key]
end

return {
  load_config = load_config,
  save_config = save_config,
  try_parse = try_parse,
  require_key = require_key,
}
