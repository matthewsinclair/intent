---
st_id: ST0074
title: Machine-wide projects: intentd's config home, the project registry and discover, the explorer's project picker, and the menubar status line
---

# ST0074: Machine-wide projects: intentd's config home, the project registry and discover, the explorer's project picker, and the menubar status line -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The explorer handles /threads and /issues itself (status: Done)

- AC-01.1 The explorer handles /threads and /issues itself: `/threads` and `/issues` open their panes as explorer acts, and `/issues <args>` still runs `intent issues`. -- satisfied: yes (computed)

### WP-02 -- The menubar's one status line, in Gtools' shape (status: Done)

- AC-02.1 While intentd is live, the menubar status line names the place, the state and the details on one line: `intentd :<port>`, `active`, and the steel-thread count. The port is read from the url the daemon published, and the count appears only once the project query has answered. -- satisfied: yes (computed)
- AC-02.2 While intentd is not live (stale, absent or unknown), the status line carries no port and no thread count, and a stale daemon's line names its pid and says to investigate it. -- satisfied: yes (computed)
- AC-02.3 While a lifecycle verb is running, the status line says so and shows no port, because the port from before a restart is dead once its stop returns. -- satisfied: yes (computed)

### WP-03 -- The project registry: explore registers its project, intent discover registers compatible ones, intentd watches the file (status: Not Started)

- AC-03.1 The project registry is one file that a human can edit by hand and the tool reads and rewrites without losing what the human wrote. -- satisfied: no (computed)
- AC-03.2 `intent explore` run inside an Intent project ensures that project is in the registry and its entry is current. -- satisfied: no (computed)
- AC-03.3 `intent discover [fromdir]` registers every config-compatible Intent project it finds under the directory, and names each project it does not register with the reason. -- satisfied: no (computed)
- AC-03.4 intentd refreshes what it knows of the machine's projects when the registry file changes, without a restart. -- satisfied: no (computed)

### WP-04 -- The explorer's project picker: /projects, and intent explore outside a project (status: Not Started)

- AC-04.1 `/projects` in the explorer lists the registry's projects and opens the one picked. -- satisfied: no (computed)
- AC-04.2 `intent explore` outside an Intent project opens the project picker, and leaving the picker returns to the shell. -- satisfied: no (computed)

### WP-05 -- Where intentd's durable configuration lives: a standards-compliant home, ruled by hv (status: Not Started)

- AC-05.1 (non-test) hv has ruled where intentd's durable configuration lives, and design.md records the ruling. -- satisfied: no

## Acceptance Tests

### WP-01 -- The explorer handles /threads and /issues itself (status: Done)

- AT-01.1 `native/rust/crates/intent-cli/src/tui/app.rs` -- covers AC-01.1 -- status: green -- cargo test -p intent-cli --lib --no-fail-fast: 287 passed, 0 failed, at the worktree base 1f8a912cf with 793984a50's bytes; red control: the collection act's push made a no-op fails slash_threads_and_slash_issues_open_their_collections. Literal id added at 5f62b2633.

### WP-02 -- The menubar's one status line, in Gtools' shape (status: Done)

- AT-02.1 `native/macos/Intent/IntentTests/HealthTests.swift` -- covers AC-02.1 -- status: green -- bin/int macos app-test at 133061d7f's Swift bytes (ids added as comments after the run): 32 tests, 0 failures, IntentTests passed.
- AT-02.2 `native/macos/Intent/IntentTests/HealthTests.swift` -- covers AC-02.2 -- status: green -- bin/int macos app-test at 133061d7f's Swift bytes (ids added as comments after the run): 32 tests, 0 failures, IntentTests passed.
- AT-02.3 `native/macos/Intent/IntentTests/HealthTests.swift` -- covers AC-02.3 -- status: green -- bin/int macos app-test at 133061d7f's Swift bytes (ids added as comments after the run): 32 tests, 0 failures, IntentTests passed.

### WP-03 -- The project registry: explore registers its project, intent discover registers compatible ones, intentd watches the file (status: Not Started)

_(no tests in this group)_

### WP-04 -- The explorer's project picker: /projects, and intent explore outside a project (status: Not Started)

_(no tests in this group)_

### WP-05 -- Where intentd's durable configuration lives: a standards-compliant home, ruled by hv (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3.0.1 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
