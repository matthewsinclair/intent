-- Fixture: triggers IN-LU-TEST-001 (busted-describe-it-structure)
-- by using top-level asserts without describe/it and shallow
-- equality on structured tables.

local config = require("config")

-- No describe/it at all -- these are bare top-level asserts.
local result = config.load_config("/tmp/does-not-exist-fixture-path")
assert(result == nil)

local ok = pcall(config.save_config, "/tmp/fixture-out", "hello")
assert(ok == true)

-- Shallow `==` on structured tables -- passes only by reference, not value.
assert.equals({ timeout = 30, retries = 3 }, config.defaults())
