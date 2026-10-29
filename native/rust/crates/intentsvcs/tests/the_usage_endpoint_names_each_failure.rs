//! ST0086 AT-02.6
//!
//! **THE USAGE ENDPOINT'S READER, DRIVEN THROUGH STUBS.** [`read_endpoint`]
//! takes its tools, so each arm hands it a Keychain stub and a curl stub in a
//! temporary directory, with fixture tokens only: no arm reads the operator's
//! Keychain or reaches the account's endpoint (vc's ruling on ST0086 AC-02.6).
//!
//! The Keychain stub answers only for the one service name it is written for,
//! so the naming condition is driven both ways: the bare name when no config
//! dir is set, the suffixed name when one is, and never a fallback between
//! them. The curl stub records its argv, so the arms can say the token never
//! rode on it.
//!
//! **AND WHAT FOLLOWS A READ** (issue 0658): a 429 named with its
//! `Retry-After`, the last good reading kept beside a later failure until a
//! newer statusline report replaces it, and the wait before the next read.

use std::path::{Path, PathBuf};

use intentsvcs::estate::usage::{
  Endpoint, EndpointFailure, EndpointTools, READ_EVERY, Reading, Source, Usage, WAIT_CAP, Window,
  carry, next_wait, read_endpoint, report, resolve, service,
};
use intentsvcs::userstate::Dirs;

const TOKEN: &str = "fixture-token-not-a-real-one";

/// A minimal answer in the endpoint's shape.
const GOOD: &str = r#"{"five_hour":{"utilization":21.0,"resets_at":"2026-10-25T20:00:00+00:00"},"seven_day":{"utilization":6.0,"resets_at":null}}"#;

/// An executable shell script at `dir/name`, run once before it is handed
/// out. Linux refuses to exec a file that another thread's fork still holds
/// open for writing (the mechanism is in [`testkit::output_retrying_busy`]),
/// and the reader execs these itself with no retry, so this first run waits
/// that out.
fn stub(dir: &Path, name: &str, body: &str) -> PathBuf {
  use std::os::unix::fs::PermissionsExt;
  let path = dir.join(name);
  std::fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("the stub is written");
  std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755))
    .expect("the stub is executable");
  testkit::output_retrying_busy(|| std::process::Command::new(&path), name);
  path
}

/// A Keychain stub that answers `creds` for `-s <service>` and fails as
/// `security` does for any other name.
fn security_for(dir: &Path, service: &str, creds: &str) -> PathBuf {
  stub(
    dir,
    "security",
    &format!(
      "while [ $# -gt 0 ]; do\n  if [ \"$1\" = \"-s\" ]; then name=\"$2\"; fi\n  shift\ndone\nif [ \"$name\" = '{service}' ]; then\n  printf '%s' '{creds}'\nelse\n  exit 44\nfi"
    ),
  )
}

/// A curl stub that keeps its argv and its stdin config beside it, then
/// prints `body` and the status line the way
/// `-w "\n%{http_code} %header{retry-after}"` does for an answer that sent no
/// `Retry-After`.
fn curl_answering(dir: &Path, status: &str, body: &str) -> PathBuf {
  let argv = dir.join("curl.argv");
  let config = dir.join("curl.config");
  stub(
    dir,
    "curl",
    &format!(
      "printf '%s\\n' \"$@\" > '{}'\ncat > '{}'\nprintf '%s\\n%s' '{body}' '{status}'",
      argv.display(),
      config.display()
    ),
  )
}

fn creds() -> String {
  format!(r#"{{"claudeAiOauth":{{"accessToken":"{TOKEN}"}}}}"#)
}

/// Tools over the stubs, the URL a fixture one.
fn tools(security: PathBuf, curl: PathBuf) -> EndpointTools {
  EndpointTools {
    security,
    curl,
    url: "https://usage.invalid/api/oauth/usage".to_string(),
  }
}

#[test]
fn a_good_answer_is_read_and_the_token_rides_stdin_never_argv() {
  let dir = tempfile::tempdir().expect("tempdir");
  let claude = dir.path().join("claude");
  let t = tools(
    security_for(dir.path(), &service(&claude, false), &creds()),
    curl_answering(dir.path(), "200", GOOD),
  );
  let usage = read_endpoint(&t, &claude, false).expect("a 200 with a usage body reads");
  assert_eq!(usage.five_hour.as_ref().map(|w| w.percent), Some(21.0));
  assert_eq!(usage.seven_day.as_ref().map(|w| w.percent), Some(6.0));
  assert!(
    usage.read_at.is_some(),
    "the reading carries when it was read"
  );
  let argv = std::fs::read_to_string(dir.path().join("curl.argv")).expect("argv kept");
  let config = std::fs::read_to_string(dir.path().join("curl.config")).expect("config kept");
  assert!(
    !argv.contains(TOKEN),
    "the token rode on curl's argv: {argv}"
  );
  assert!(
    config.contains(&format!("Bearer {TOKEN}")) && config.contains("usage.invalid"),
    "the token and the URL are not in curl's stdin config: {config}"
  );
}

/// The naming condition, both ways: each setting reads its own item, and a
/// dir whose own item is missing fails rather than borrowing the other.
#[test]
fn the_keychain_item_is_named_by_whether_a_config_dir_is_set_with_no_fallback() {
  let dir = tempfile::tempdir().expect("tempdir");
  let claude = dir.path().join("claude");
  let (bare, suffixed) = (service(&claude, false), service(&claude, true));
  assert_ne!(bare, suffixed);
  let curl = curl_answering(dir.path(), "200", GOOD);

  let only_bare = tools(security_for(dir.path(), &bare, &creds()), curl.clone());
  assert!(read_endpoint(&only_bare, &claude, false).is_ok());
  assert_eq!(
    read_endpoint(&only_bare, &claude, true),
    Err(EndpointFailure::Keychain {
      why: format!("no item under `{suffixed}`")
    }),
    "a set config dir fell back to the bare item"
  );

  let only_suffixed = tools(security_for(dir.path(), &suffixed, &creds()), curl);
  assert!(read_endpoint(&only_suffixed, &claude, true).is_ok());
  assert_eq!(
    read_endpoint(&only_suffixed, &claude, false),
    Err(EndpointFailure::Keychain {
      why: format!("no item under `{bare}`")
    }),
    "an unset config dir fell back to a suffixed item"
  );
}

#[test]
fn a_keychain_that_cannot_run_or_holds_no_token_is_named() {
  let dir = tempfile::tempdir().expect("tempdir");
  let claude = dir.path().join("claude");
  let name = service(&claude, false);
  let curl = curl_answering(dir.path(), "200", GOOD);

  let absent = tools(dir.path().join("no-such-security"), curl.clone());
  assert!(
    matches!(read_endpoint(&absent, &claude, false), Err(EndpointFailure::Keychain { why }) if why.contains("could not run")),
    "a Keychain tool that cannot run is not named as one"
  );
  let not_json = tools(security_for(dir.path(), &name, "not json"), curl.clone());
  assert_eq!(
    read_endpoint(&not_json, &claude, false),
    Err(EndpointFailure::Keychain {
      why: format!("the `{name}` item is not JSON")
    })
  );
  let no_token = tools(security_for(dir.path(), &name, "{}"), curl);
  assert_eq!(
    read_endpoint(&no_token, &claude, false),
    Err(EndpointFailure::Keychain {
      why: format!("the `{name}` item has no `claudeAiOauth.accessToken`")
    })
  );
}

#[test]
fn a_curl_that_cannot_run_or_reaches_nothing_is_named() {
  let dir = tempfile::tempdir().expect("tempdir");
  let claude = dir.path().join("claude");
  let security = security_for(dir.path(), &service(&claude, false), &creds());

  let missing = dir.path().join("no-such-curl");
  assert!(
    matches!(
      read_endpoint(&tools(security.clone(), missing.clone()), &claude, false),
      Err(EndpointFailure::NoCurl { curl, .. }) if curl == missing.display().to_string()
    ),
    "a curl that cannot run is not named by its path"
  );
  // curl's own failure: no status line, and its reason on stderr.
  let unreached = stub(
    dir.path(),
    "curl-unreached",
    "cat > /dev/null\necho 'curl: (6) Could not resolve host' >&2\nexit 6",
  );
  assert_eq!(
    read_endpoint(&tools(security, unreached), &claude, false),
    Err(EndpointFailure::Unreached {
      why: "curl: (6) Could not resolve host".to_string()
    })
  );
}

#[test]
fn a_status_other_than_200_and_an_unparsable_answer_are_named() {
  let dir = tempfile::tempdir().expect("tempdir");
  let claude = dir.path().join("claude");
  let security = security_for(dir.path(), &service(&claude, false), &creds());

  let refused = tools(
    security.clone(),
    curl_answering(dir.path(), "401", r#"{"error":"unauthorized"}"#),
  );
  assert_eq!(
    read_endpoint(&refused, &claude, false),
    Err(EndpointFailure::Status { status: 401 })
  );
  let garbled = tools(
    security.clone(),
    curl_answering(dir.path(), "200", "<html>"),
  );
  assert!(
    matches!(
      read_endpoint(&garbled, &claude, false),
      Err(EndpointFailure::Unparsable { .. })
    ),
    "an answer that is not JSON is not named unparsable"
  );
  let not_object = tools(security, curl_answering(dir.path(), "200", "[1,2]"));
  assert_eq!(
    read_endpoint(&not_object, &claude, false),
    Err(EndpointFailure::Unparsable {
      why: "it is not an object".to_string()
    })
  );
}

/// A 429 is its own failure, carrying the `Retry-After` it sent in whole
/// seconds, or none when it sent none.
#[test]
fn a_429_is_named_with_the_retry_after_it_sent() {
  let dir = tempfile::tempdir().expect("tempdir");
  let claude = dir.path().join("claude");
  let security = security_for(dir.path(), &service(&claude, false), &creds());
  // The last line as `-w "\n%{http_code} %header{retry-after}"` prints it
  // when the answer sent the header.
  let throttled = stub(
    dir.path(),
    "curl-throttled",
    "cat > /dev/null\nprintf '%s\\n%s' '{\"error\":\"rate_limited\"}' '429 120'",
  );
  let failure = read_endpoint(&tools(security.clone(), throttled), &claude, false)
    .expect_err("a 429 is a failure");
  assert_eq!(
    failure,
    EndpointFailure::Throttled {
      retry_after: Some(120)
    }
  );
  assert_eq!(
    failure.to_string(),
    "the endpoint answered 429, too many requests"
  );
  let bare = tools(security, curl_answering(dir.path(), "429", "{}"));
  assert_eq!(
    read_endpoint(&bare, &claude, false),
    Err(EndpointFailure::Throttled { retry_after: None })
  );
}

/// A good endpoint reading, read at 05:44Z on 2026-10-27.
fn good_reading() -> Usage {
  Usage {
    source: Some(Source {
      kind: "endpoint".to_string(),
      session_id: None,
    }),
    read_at: Some("2026-10-27T05:44:00.000Z".to_string()),
    five_hour: Some(Window {
      percent: 1.0,
      resets_at: None,
    }),
    ..Usage::default()
  }
}

/// A failed read after a good one keeps that reading, still stamped when it
/// was read, with the failure named; a good read replaces whatever was kept.
#[test]
fn a_failed_read_keeps_the_last_good_reading_beside_its_failure() {
  let good: Endpoint = Ok(good_reading());
  let throttled = || -> Endpoint { Err(EndpointFailure::Throttled { retry_after: None }) };
  assert_eq!(
    carry(None, throttled()),
    throttled(),
    "with nothing good behind it, the failure is kept"
  );
  let kept = carry(Some(&good), throttled()).expect("the good reading is kept");
  assert_eq!(kept.read_at.as_deref(), Some("2026-10-27T05:44:00.000Z"));
  assert_eq!(kept.five_hour.as_ref().map(|w| w.percent), Some(1.0));
  assert_eq!(
    kept.error.as_deref(),
    Some("the endpoint answered 429, too many requests")
  );
  let again = carry(
    Some(&Ok(kept)),
    Err(EndpointFailure::Status { status: 503 }),
  )
  .expect("a second failure keeps the same reading");
  assert_eq!(again.read_at.as_deref(), Some("2026-10-27T05:44:00.000Z"));
  assert_eq!(again.error.as_deref(), Some("the endpoint answered 503"));
  assert_eq!(carry(Some(&Ok(again)), good.clone()), good);
}

/// The wait after a good read is `READ_EVERY`; a failure doubles the wait
/// that led to it, up to `WAIT_CAP`, and a 429 waits at least its
/// `Retry-After`.
#[test]
fn a_good_read_waits_read_every_and_a_failure_backs_off() {
  let good: Endpoint = Ok(Usage::default());
  let refused: Endpoint = Err(EndpointFailure::Status { status: 503 });
  assert_eq!(next_wait(&good, WAIT_CAP), READ_EVERY);
  assert_eq!(next_wait(&refused, READ_EVERY), READ_EVERY * 2);
  assert_eq!(next_wait(&refused, READ_EVERY * 2), READ_EVERY * 4);
  assert_eq!(next_wait(&refused, WAIT_CAP), WAIT_CAP);
  let asked = |secs: u64| -> Endpoint {
    Err(EndpointFailure::Throttled {
      retry_after: Some(secs),
    })
  };
  // Never sooner than the Retry-After, past the cap when it asks for more,
  // and the doubling stands when it asks for less.
  let long = WAIT_CAP * 3;
  assert_eq!(next_wait(&asked(long.as_secs()), READ_EVERY), long);
  assert_eq!(next_wait(&asked(1), READ_EVERY), READ_EVERY * 2);
}

/// A reading kept beside a failure stands until a statusline report is
/// newer, and the report then shows, naming the same failure.
#[test]
fn a_kept_reading_stands_until_a_newer_statusline_report() {
  let home = tempfile::tempdir().expect("tempdir");
  let dirs = Dirs::at_home(home.path());
  let mut unreadable = Vec::new();
  let kept = Usage {
    // Older than any report this arm writes.
    read_at: Some("2000-01-01T00:00:00.000Z".to_string()),
    error: Some("the endpoint answered 429, too many requests".to_string()),
    ..good_reading()
  };
  let endpoint = Reading::Read(Ok(kept.clone()));
  assert_eq!(
    resolve(&endpoint, &dirs, &mut unreadable),
    kept,
    "with no report, the kept reading stands"
  );
  report(
    &dirs,
    r#"{"session_id":"00000000-0000-4000-8000-000000000000","rate_limits":{"five_hour":{"used_percentage":30}}}"#,
  )
  .expect("the statusline report is kept");
  let usage = resolve(&endpoint, &dirs, &mut unreadable);
  assert_eq!(
    usage.source.as_ref().map(|s| s.kind.as_str()),
    Some("statusline")
  );
  assert_eq!(usage.five_hour.as_ref().map(|w| w.percent), Some(30.0));
  assert_eq!(usage.error, kept.error);
  assert!(unreadable.is_empty(), "{unreadable:?}");
}
