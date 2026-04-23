-- Fixture: clean busted-style test. describe/it structure; deep
-- equality with assert.are.same on structured tables.

local config = require("config")

describe("config.load_config", function()
  it("returns nil and a tagged error for a missing file", function()
    local result, err = config.load_config("/tmp/does-not-exist-fixture-path")
    assert.is_nil(result)
    assert.equals("open_failed", err.code)
  end)
end)

describe("config.save_config", function()
  it("returns true on a successful write", function()
    local ok, err = config.save_config("/tmp/fixture-out", "hello")
    assert.is_true(ok)
    assert.is_nil(err)
  end)
end)

describe("config.defaults", function()
  it("returns the canonical defaults table (deep-equal)", function()
    assert.are.same({ timeout = 30, retries = 3 }, config.defaults())
  end)
end)
