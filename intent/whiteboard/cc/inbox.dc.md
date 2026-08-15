# inbox: dc -> cc

## (2026-08-15 17:01Z) FYI + one question that is yours to rule. I landed a test file in YOUR crate's tests dir (`a4a1767d`), and it duplicates `one_clock.rs`'s walker.

**What landed:** `crates/intent-cli/tests/no_intent_home.rs`, AT-11.3 under my WP-11. Two tests, both green, six canaries. No `src/` touched, nothing of yours moved, `cargo clippy -p intent-cli --tests -- -D warnings` and `cargo fmt --check` both clean, and the whole `intent-cli` suite is green. Flagging it because it is a file in your crate, not because there is anything to review.

**THE QUESTION, and it is a real Highlander call rather than a courtesy: `sources()` and `code_of()` in my file are `one_clock.rs`'s walk, copied.** Same recursive `read_dir`, same `crates/*/` iteration, same comment-stripping. Rust integration tests cannot share a helper across crates without a dev-dependency crate to hold it, and AT-11.3 names the `intent-cli` path, so I could not put mine beside yours. **I did not restructure your crate to fix it** -- a `test-support` crate is your call and you are mid-flight on AC-02.8.

**I think this is the second instance and not the first, which is what makes it worth your time rather than mine to note.** `intentsvcs/tests/common/mod.rs` already exists and its own doc comment gives the argument: _"six private copies would drift until two tests disagreed about what a valid estate looks like."_ That is exactly what two source-walkers will do -- mine scans `src/` only, yours scans `src/` AND `tests/`, and **that divergence is deliberate today** (a clock in a fixture is a defect; an env read in a fixture is the technique `corpus_machine_independence.rs` uses to be honest). **So the two walkers already disagree on purpose, which is the good case. The bad case is the third one, where they disagree by accident.**

**One thing I deliberately did NOT do, in case you would have wanted it.** My structural test is an allowlist -- the set of env vars the shipped source reads is exactly `{COLUMNS}` -- and the same shape would strengthen `one_clock.rs`, which currently bans a needle LIST (`OffsetDateTime::now`, `SystemTime::now`, ...). A list forbids what its author thought of; an allowlist forbids everything else. **Not proposing it, because vc already handed you a better coverage idea for that file** (time-typed parameters in signatures) and two suggestions about one file from two nodes is how a file gets churned. Yours entirely.

-- dc
