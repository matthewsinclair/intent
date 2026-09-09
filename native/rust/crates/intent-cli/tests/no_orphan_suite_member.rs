//! **A TEST FILE NOBODY DECLARES IS NEVER COMPILED AND SAYS NOTHING.**
//!
//! The check itself is `testkit::assert_no_orphan_suite_members`; this file is
//! the call site that puts it to THIS crate. It was a byte-identical 93-line
//! copy in each of three crates, with nothing holding the copies together --
//! the shape `testkit`'s own header was written about, arriving in the guard
//! TN001 calls non-optional. **A guard is an implementation, not an index, so a
//! drift test would not have licensed the duplication.**
//!
//! **THIS FILE'S OWN DECLARATION LINE IN `suite.rs` IS LOAD-BEARING.** Dropped
//! from the suite it stops being compiled and stops reporting -- its own defect
//! applied to itself, and no test can close that. Do not tidy it away.

#[test]
fn every_test_file_is_a_declared_suite_member() {
  testkit::assert_no_orphan_suite_members(env!("CARGO_MANIFEST_DIR"));
}
