# WP04: CLI Core - Remove Backlog Commands and Config

## Scope

Delete backlog-specific scripts, clean remaining scripts.

## Files to DELETE

- bin/intent_backlog (~448 lines)
- bin/intent_bl (~9 lines)
- bin/intent_migrate (~217 lines)
- bin/intent_status (~24 lines)
- bin/intent_task (~143 lines)

## Files to Edit

- bin/intent - Remove bl) case block
- bin/intent_config - Remove backlog config vars
- bin/intent_helpers - Remove backlog from helpers
- bin/intent_init - Remove backlog init
- bin/intent_upgrade - Remove backlog migration
- bin/intent_bootstrap - Remove backlog from defaults
- bin/intent_info - Remove backlog display
- bin/intent_doctor - Remove backlog check

## Acceptance Criteria

- Backlog commands return "Unknown command"
- `intent init` creates projects without backlog/
- All remaining bin/ scripts load without errors
