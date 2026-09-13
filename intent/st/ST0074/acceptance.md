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

### WP-03 -- The project registry: explore registers its project, intent discover registers compatible ones, intentd watches the file (status: Done)

- AC-03.1 The project registry is one file that a human can edit by hand and the tool reads and rewrites without losing what the human wrote. -- satisfied: yes (computed)
- AC-03.2 `intent explore` run inside an Intent project ensures that project is in the registry and its entry is current. -- satisfied: yes (computed)
- AC-03.3 `intent discover [fromdir]` registers every config-compatible Intent project it finds under the directory, and names each project it does not register with the reason. -- satisfied: yes (computed)
- AC-03.4 intentd refreshes what it knows of the machine's projects when the registry file changes, without a restart. -- satisfied: yes (computed)

### WP-04 -- The explorer's project picker: /projects, and intent explore outside a project (status: Done)

- AC-04.1 `/projects` in the explorer lists the registry's projects and opens the one picked. -- satisfied: yes (computed)
- AC-04.2 `intent explore` outside an Intent project opens the project picker, and leaving the picker returns to the shell. -- satisfied: yes (computed)

### WP-05 -- Where intentd's durable configuration lives: a standards-compliant home, ruled by hv (status: Done)

- AC-05.1 (non-test) hv has ruled where intentd's durable configuration lives, and design.md records the ruling. -- evidence: f2452077a -- satisfied: yes

## Acceptance Tests

### WP-01 -- The explorer handles /threads and /issues itself (status: Done)

- AT-01.1 `native/rust/crates/intent-cli/src/tui/app.rs` -- covers AC-01.1 -- status: green -- cargo test -p intent-cli --lib --no-fail-fast: 287 passed, 0 failed, at the worktree base 1f8a912cf with 793984a50's bytes; red control: the collection act's push made a no-op fails slash_threads_and_slash_issues_open_their_collections. Literal id added at 5f62b2633.

### WP-02 -- The menubar's one status line, in Gtools' shape (status: Done)

- AT-02.1 `native/macos/Intent/IntentTests/HealthTests.swift` -- covers AC-02.1 -- status: green -- bin/int macos app-test at 133061d7f's Swift bytes (ids added as comments after the run): 32 tests, 0 failures, IntentTests passed.
- AT-02.2 `native/macos/Intent/IntentTests/HealthTests.swift` -- covers AC-02.2 -- status: green -- bin/int macos app-test at 133061d7f's Swift bytes (ids added as comments after the run): 32 tests, 0 failures, IntentTests passed.
- AT-02.3 `native/macos/Intent/IntentTests/HealthTests.swift` -- covers AC-02.3 -- status: green -- bin/int macos app-test at 133061d7f's Swift bytes (ids added as comments after the run): 32 tests, 0 failures, IntentTests passed.

### WP-03 -- The project registry: explore registers its project, intent discover registers compatible ones, intentd watches the file (status: Done)

- AT-03.1 `native/rust/crates/intentsvcs/src/projects.rs` -- covers AC-03.1 -- status: green -- projects::tests::a_rewrite_keeps_what_the_operator_wrote, green at 2123f7c08
- AT-03.2 `native/rust/crates/intentsvcs/src/projects.rs` -- covers AC-03.2 -- status: green -- projects::tests::a_root_is_added_once_and_a_newer_schema_is_left_alone, green at 2123f7c08
- AT-03.3 `native/rust/crates/intentsvcs/src/projects.rs` -- covers AC-03.3 -- status: green -- projects::tests::discover_registers_the_compatible_and_names_the_rest, green at 2123f7c08
- AT-03.4 `native/rust/crates/intentd/tests/the_daemon_lists_the_project_registry.rs` -- covers AC-03.4 -- status: green -- red with listed::start disabled (never listed in 10s), green at 2123f7c08

### WP-04 -- The explorer's project picker: /projects, and intent explore outside a project (status: Done)

- AT-04.1 `native/rust/crates/intent-cli/src/tui/picker.rs` -- covers AC-04.1 -- status: green -- tui::picker::tests::enter_opens_the_project_under_the_cursor: red with Enter inert, green at 2a121e359
- AT-04.2 `native/rust/crates/intent-cli/src/tui/picker.rs` -- covers AC-04.2 -- status: green -- tui::picker::tests::esc_leaves_and_an_empty_registry_says_how_to_fill_it: red with Esc inert, green at 2a121e359

### WP-05 -- Where intentd's durable configuration lives: a standards-compliant home, ruled by hv (status: Done)

_(no tests in this group)_

---

_Generated by Intent v3.0.1 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
