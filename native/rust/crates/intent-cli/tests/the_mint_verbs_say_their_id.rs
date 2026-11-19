//! **`st new`, `wp new` and `issues add` say what they minted in one form**
//! (ST0095 AC-02.1). Each prints exactly one stdout line, `created: <id>`, and
//! the value is the whole id that the verbs taking its kind accept back as
//! printed, so a script or a test that reads it never has to know how an id is
//! made. `crate::common::created` is the one reader, and it fails unless there
//! is exactly one such line.

use crate::common::ok;

#[test]
fn each_mint_prints_one_created_line_whose_id_its_doors_take_back() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "mintproj"]);

  let st = crate::common::created(&ok(root, &["st", "new", "A minted thread"]));
  let wp = crate::common::created(&ok(root, &["wp", "new", st.as_str(), "A minted package"]));
  let issue = crate::common::created(&ok(root, &["issues", "add", "A minted issue"]));

  assert!(
    ok(root, &["st", "show", st.as_str()]).contains("A minted thread"),
    "`st show` takes `{st}` back as printed"
  );
  assert!(
    ok(root, &["wp", "show", wp.as_str()]).contains("A minted package"),
    "`wp show` takes `{wp}` back as printed"
  );
  assert!(
    ok(root, &["issues", "show", issue.as_str()]).contains("A minted issue"),
    "`issues show` takes `{issue}` back as printed"
  );
  assert!(
    wp.starts_with(&format!("{st}/")),
    "a package's id is its whole address, its thread's id first: `{wp}`"
  );
}
