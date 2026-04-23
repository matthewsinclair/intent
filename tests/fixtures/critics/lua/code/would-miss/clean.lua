-- Fixture: clean Lua code. Module-local state, pcall around every
-- fallible host call, tagged error tables.

local M = {}

local defaults = { timeout = 30, retries = 3 }

local function open_for_read(path)
  local ok, file_or_err = pcall(io.open, path, "r")
  if not ok or file_or_err == nil then
    return nil, { code = "open_failed", msg = tostring(file_or_err), path = path }
  end
  return file_or_err
end

function M.load_config(path)
  local file, err = open_for_read(path)
  if file == nil then
    return nil, err
  end
  local raw = file:read("*a")
  file:close()
  return raw
end

function M.save_config(path, text)
  local ok, err = pcall(function()
    local file = assert(io.open(path, "w"))
    file:write(text)
    file:close()
  end)
  if not ok then
    return nil, { code = "write_failed", msg = tostring(err), path = path }
  end
  return true
end

function M.defaults()
  return defaults
end

return M
