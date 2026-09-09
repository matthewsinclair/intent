//! **A TEST FILE NOBODY DECLARES IS NEVER COMPILED AND SAYS NOTHING.**
//!
//! The check itself is `testkit::assert_no_orphan_suite_members`; this file is
//! the call site that puts it to THIS crate. A crate that hosts the guard is not
//! thereby guarded -- testkit went unguarded for exactly as long as it owned no
//! `autotests` key, which is the window TN001's four parts never cover.
//!
//! **THIS FILE'S OWN DECLARATION LINE IN `suite.rs` IS LOAD-BEARING.** Dropped
//! from the suite it stops being compiled and stops reporting. Do not tidy it away.

#[test]
fn every_test_file_is_a_declared_suite_member() {
  testkit::assert_no_orphan_suite_members(env!("CARGO_MANIFEST_DIR"));
}
