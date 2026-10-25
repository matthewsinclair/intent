//! Issue 0646: the web face's page, opened without a token the daemon admits,
//! says in one sentence how to open it, and shows nothing of the `/op`
//! refusal's `Authorization: Bearer` text. hv met that refusal through
//! Intent.app's Open, which opened a bare address.
//!
//! The page is the shell intentd serves at every path, `include_str!`'d from
//! `src/shell.html`, so these arms read those bytes. They pin the page's
//! script, not a browser's rendering of it: no browser runs here.

const SHELL: &str = include_str!("../src/shell.html");

/// The sentence the page shows, read from its own declaration so the arms
/// cannot drift from it.
fn no_token() -> &'static str {
  let start = SHELL
    .find("var NO_TOKEN = \"")
    .expect("the page declares its no-token sentence")
    + "var NO_TOKEN = \"".len();
  let len = SHELL[start..].find("\";").expect("the declaration ends");
  &SHELL[start..start + len]
}

#[test]
fn the_sentence_names_the_ways_in_and_no_bearer_text() {
  let sentence = no_token();
  assert!(
    sentence.contains("`intent estate status --browser`"),
    "{sentence}"
  );
  assert!(sentence.contains("Intent.app"), "{sentence}");
  assert!(sentence.contains("`intent browse`"), "{sentence}");
  assert!(!sentence.contains("Bearer"), "{sentence}");
  assert!(!sentence.contains("Authorization"), "{sentence}");
  assert_eq!(
    sentence.matches(". ").count(),
    0,
    "one sentence: {sentence}"
  );
}

#[test]
fn a_page_with_no_stored_token_shows_the_sentence() {
  assert!(
    SHELL.contains("if (!token || !root) {\n        failed(NO_TOKEN, \"\");"),
    "the no-credentials branch shows the sentence"
  );
  assert!(!SHELL.contains("This page has no credentials"));
}

/// A token stored from an earlier run of the daemon is refused at 401; the
/// page names that before the refusal's own text could reach the reader.
#[test]
fn a_refused_token_shows_the_sentence_before_the_refusal_text() {
  let at_401 = SHELL
    .find("if (answer.status === 401) {\n          failed(NO_TOKEN, \"\");")
    .expect("a 401 shows the sentence");
  let refusal = SHELL
    .find("if (answer.body && answer.body.result === \"error\")")
    .expect("the refusal branch");
  assert!(
    at_401 < refusal,
    "the 401 is named before the refusal text is shown"
  );
}
