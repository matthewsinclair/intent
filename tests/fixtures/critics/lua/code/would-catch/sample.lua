-- Fixture: triggers IN-LU-CODE-001 (local-over-global),
-- IN-LU-CODE-004 (pcall-for-errors), and IN-LU-CODE-006
-- (dispatch-table-over-if-chain) by design.

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

-- Dispatch on `perturbation.tag` via a chain of sibling `if` blocks --
-- each branch returns a different downstream function call. Should be
-- expressed as a HANDLERS table-of-functions instead.
function apply_perturbation(state, perturbation)
  if perturbation.tag == "phase_entered" then
    return on_phase_entered(state, perturbation)
  end
  if perturbation.tag == "option_selected" then
    return on_option_selected(state, perturbation)
  end
  if perturbation.tag == "character_died" then
    return on_character_died(state, perturbation)
  end
  return state
end

return {
  load_config = load_config,
  save_config = save_config,
  try_parse = try_parse,
  require_key = require_key,
  apply_perturbation = apply_perturbation,
}
