// Fixture: clean Rust test. Uses assert_matches! on tagged variants
// with precisely the field(s) being asserted; each test pins one
// behaviour and is resilient to additive struct evolution.

use assert_matches::assert_matches;

#[derive(Debug, PartialEq)]
pub struct User {
  pub id: u64,
  pub name: String,
  pub email: String,
  pub active: bool,
}

pub fn load_user() -> Result<User, String> {
  Ok(User {
    id: 42,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
    active: true,
  })
}

#[test]
fn test_load_user_returns_requested_id() {
  assert_matches!(load_user(), Ok(User { id: 42, .. }));
}

#[test]
fn test_load_user_returns_active_user() {
  assert_matches!(load_user(), Ok(User { active: true, .. }));
}
