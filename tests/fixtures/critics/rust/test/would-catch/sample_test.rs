// Fixture: triggers IN-RS-TEST-002 (assert_matches-for-variants) by
// comparing a whole multi-field struct literal with assert_eq!.

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
fn test_load_user_full_struct_match() {
  let result = load_user();
  assert_eq!(
    result,
    Ok(User {
      id: 42,
      name: "Alice".to_string(),
      email: "alice@example.com".to_string(),
      active: true,
    })
  );
}

#[test]
fn test_load_user_field_walk() {
  match load_user() {
    Ok(u) => {
      assert_eq!(u.id, 42);
      assert_eq!(u.name, "Alice");
      assert_eq!(u.email, "alice@example.com");
      assert_eq!(u.active, true);
    }
    _ => panic!("expected Ok"),
  }
}
