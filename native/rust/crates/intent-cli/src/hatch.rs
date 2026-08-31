//! The GraphQL escape hatch's bridge: `intent graphql` and the MCP tool
//! `intent_graphql` are two faces of this one round trip (`AC-00.4`,
//! `AC-09.2`, WP-09).
//!
//! **THE HATCH EXECUTES NOTHING IN THIS PROCESS, AND THAT IS A RULING RATHER
//! THAN A LIMITATION.** Executing a document needs an async runtime, and the
//! CLI carries none -- `cargo tree -p intent-cli -i tokio` matches no package
//! -- because the stdio MCP tier's zero-dependency loop was affordable only
//! while every tool stayed synchronous open-per-call (vc, 2026-08-31, under
//! hv's pen). An in-process executor for this one tool would be that runtime
//! arriving through the back door, one ruling later. So both faces ship the
//! document to intentd over [`wire::ask`] -- synchronous std sockets, the
//! same round trip `--daemon st list` makes -- and when no daemon is
//! answering they REFUSE and name `intent daemon start`. Never a fallback,
//! never a hang: the probe is a completed round trip with a deadline.
//!
//! **ONE ROUND TRIP FOR TWO FACES.** The terminal arm in `render.rs` and the
//! MCP arm in `mcp.rs` both call [`graphql`]; discovery, the request, the
//! refusal texts and the variables parse live here once, so the two faces
//! cannot disagree about what a daemon-down looks like. What differs is only
//! how the answer is carried: pretty JSON on stdout at the terminal, a value
//! in the tool result on MCP.
//!
//! **READS ONLY, AND THE BOUND IS NOT ENFORCED HERE.** `EmptyMutation` ships
//! in the schema (`intentsvcs::graphql`), so a mutation document fails the
//! schema's own validation and the refusal travels back INSIDE the answer's
//! `errors` -- the spec's channel -- rather than as a wire error. This module
//! does not read the document at all.

use std::path::Path;

use intentsvcs::daemon::{self, Route};
use intentsvcs::remedy::Remedy;
use intentsvcs::wire::{self, AskError, Op, Request, Response};
use serde_json::Value;
use thiserror::Error;

use crate::spine::Failure;

/// Verb paths that ONLY a daemon answers.
///
/// **A SEPARATE DECLARATION FROM `render::SERVED_BY_DAEMON`, BECAUSE THE TWO
/// LISTS MAKE DIFFERENT CLAIMS.** `SERVED_BY_DAEMON` says *this verb answers
/// identically in-process and through a daemon*, and the `AC-08.2` conformance
/// harness reads it to make exactly that comparison. `graphql` has ONE path,
/// so the identity claim cannot be made for it -- putting it there would hand
/// the harness a verb whose local half refuses by design, and the harness
/// would be right to go red. What `graphql` shares with that roster is only
/// that `--daemon` must not be REFUSED on it: the global flag is a request for
/// the daemon, and this verb grants it by existing. `render::run` consults
/// both.
///
/// It grows when a second verb has no in-process twin, and the reason it has
/// none belongs on that verb's table row, as `graphql`'s is on its own.
pub const DAEMON_ONLY: &[&str] = &["graphql"];

/// Does this path go to a daemon whether or not `--daemon` was said?
pub fn daemon_only(path: &str) -> bool {
  DAEMON_ONLY.contains(&path)
}

/// Why the hatch did not return an answer.
///
/// **EVERY VARIANT CARRIES A REMEDY, AND ONE OF THEM OVERRIDES THE REMEDY IT
/// WRAPS.** [`AskError::Unreachable`]'s own advice ends *with no daemon
/// running, this command executes in-process* -- true for every verb on
/// `SERVED_BY_DAEMON` and false for this one, so repeating it here would send
/// an operator to a fallback that does not exist. See [`HatchError::remedy`].
#[derive(Debug, Error)]
pub enum HatchError {
  /// The candidate addresses could not be listed at all.
  #[error("{0}")]
  Discovery(#[from] daemon::DaemonError),
  /// No daemon answered the probe at any candidate address.
  #[error("`intent graphql` executes in intentd, and no daemon is answering")]
  NoDaemon,
  /// A daemon answered the probe and the request still failed.
  #[error("{0}")]
  Ask(#[from] AskError),
  /// The daemon understood the request and refused it -- about the project,
  /// so rendered as the project's refusal, remedy and all.
  #[error("{message}")]
  Refused { message: String, remedy: String },
  /// `--variables` was not a JSON object.
  #[error("`--variables` must be a JSON object, given as text: {why}")]
  Variables { why: String },
  /// The daemon answered a different variant -- a routing fault, named.
  #[error("intentd answered a GraphQL request with `{result}`, which is not a GraphQL response")]
  Unexpected { result: String },
}

impl HatchError {
  pub fn remedy(&self) -> String {
    match self {
      HatchError::Discovery(e) => e.remedy(),
      HatchError::NoDaemon => {
        "start one with `intent daemon start`. The escape hatch never executes in this process -- the CLI carries no async runtime by ruling -- so there is no in-process answer to fall back to.".to_string()
      }
      // The wrapped remedy promises an in-process fallback this verb does not
      // have; every other `AskError` remedy is right as written.
      HatchError::Ask(AskError::Unreachable(_)) => {
        "the daemon answered the liveness probe a moment ago and is not answering now, so it has stopped or is no longer reachable. Run `intent daemon status`, then `intent daemon start`; the escape hatch has no in-process answer.".to_string()
      }
      HatchError::Ask(e) => e.remedy(),
      HatchError::Refused { remedy, .. } => remedy.clone(),
      HatchError::Variables { .. } => {
        "pass an object, eg `--variables '{\"id\":\"ST0000\"}'`; the document names what it takes with `query($id: String!)`.".to_string()
      }
      HatchError::Unexpected { .. } => {
        "this is a fault inside intentd's dispatch, not in the document. Stop the daemon and start it again from the same build as this CLI, and report it if it recurs.".to_string()
      }
    }
  }

  /// The operator-facing text, in the estate's `error:` / `remedy:` shape.
  pub fn render(&self) -> String {
    format!("error: {self}\n  remedy: {}", self.remedy())
  }

  /// The exit the terminal face takes: 2 when nothing could answer, 1 when
  /// something answered no.
  ///
  /// **`NoDaemon` IS `Unavailable`, NOT `Error`, FOR THE REASON `served()`
  /// GIVES**: the operator's project is fine; the thing in their hand could not
  /// answer, and rc=2 is what says that rather than returning a verdict about
  /// their work.
  pub fn failure(self) -> Failure {
    match self {
      HatchError::Discovery(_) | HatchError::NoDaemon | HatchError::Ask(_) => {
        Failure::Unavailable(self.render())
      }
      HatchError::Refused { .. } | HatchError::Variables { .. } | HatchError::Unexpected { .. } => {
        Failure::Error(self.render())
      }
    }
  }
}

/// Parse `--variables` as the hatch accepts it: absent is none; present must
/// be a JSON object.
///
/// **SHARED BY BOTH FACES AND RUN BEFORE ANY DAEMON IS LOOKED FOR**, so a bad
/// value is refused in one voice, and refused the same way on a machine with
/// a daemon and one without -- which is also what makes it drivable from a
/// unit test that must not reach for a daemon.
pub fn variables(text: Option<&str>) -> Result<Option<Value>, HatchError> {
  let Some(text) = text else {
    return Ok(None);
  };
  match serde_json::from_str::<Value>(text) {
    Ok(Value::Object(map)) => Ok(Some(Value::Object(map))),
    Ok(other) => Err(HatchError::Variables {
      why: format!("got {}", kind(&other)),
    }),
    Err(e) => Err(HatchError::Variables { why: e.to_string() }),
  }
}

fn kind(value: &Value) -> &'static str {
  match value {
    Value::Null => "null",
    Value::Bool(_) => "a boolean",
    Value::Number(_) => "a number",
    Value::String(_) => "a string",
    Value::Array(_) => "an array",
    Value::Object(_) => "an object",
  }
}

/// Ship one document to the daemon that owns this project's answers.
///
/// Discovery is the shipped one -- [`daemon::candidates`] then
/// [`daemon::route`], the same pair `--daemon st list` uses -- so the hatch
/// cannot find a daemon the rest of the CLI would not, or miss one it would.
pub fn graphql(root: &Path, query: &str, variables: Option<Value>) -> Result<Value, HatchError> {
  let candidates = daemon::candidates()?;
  let Route::Daemon(endpoint) = daemon::route(&candidates) else {
    return Err(HatchError::NoDaemon);
  };
  let request = Request {
    root: root.to_path_buf(),
    op: Op::Graphql {
      query: query.to_string(),
      variables,
    },
  };
  match wire::ask(&endpoint, &request)? {
    Response::Graphql { response } => Ok(response),
    Response::Error { message, remedy } => Err(HatchError::Refused { message, remedy }),
    other => Err(HatchError::Unexpected {
      result: variant_name(&other),
    }),
  }
}

/// The variant's name off its `Debug` form, so the refusal can name what came
/// back without this module keeping a list of the wire's variants.
fn variant_name(response: &Response) -> String {
  format!("{response:?}")
    .split([' ', '{', '('])
    .next()
    .unwrap_or("?")
    .to_string()
}

/// Did the face answer with errors? The terminal face exits 1 on this, with
/// the answer already on stdout where machines read it.
pub fn has_errors(answer: &Value) -> bool {
  answer
    .get("errors")
    .and_then(Value::as_array)
    .is_some_and(|errors| !errors.is_empty())
}

#[cfg(test)]
mod tests {
  use super::*;
  use serde_json::json;

  #[test]
  fn variables_accept_an_object_and_refuse_everything_else_by_name() {
    assert_eq!(variables(None).expect("absent is none"), None);
    assert_eq!(
      variables(Some(r#"{"id":"ST0000"}"#)).expect("an object"),
      Some(json!({"id": "ST0000"}))
    );
    for (text, why) in [
      ("[1]", "an array"),
      ("42", "a number"),
      ("\"s\"", "a string"),
      ("null", "null"),
    ] {
      match variables(Some(text)) {
        Err(HatchError::Variables { why: got }) => assert!(got.contains(why), "{text}: {got}"),
        other => panic!("{text} was accepted or refused otherwise: {other:?}"),
      }
    }
    let garbage = variables(Some("{ not json")).expect_err("garbage is refused");
    assert!(
      garbage.render().contains("--variables"),
      "{}",
      garbage.render()
    );
    assert!(garbage.render().contains("remedy:"), "{}", garbage.render());
  }

  #[test]
  fn has_errors_reads_only_a_non_empty_errors_list() {
    assert!(!has_errors(&json!({"data": {"threads": []}})));
    assert!(!has_errors(&json!({"data": null, "errors": []})));
    assert!(has_errors(
      &json!({"data": null, "errors": [{"message": "x"}]})
    ));
  }

  #[test]
  fn the_exit_splits_could_not_answer_from_answered_no() {
    assert!(matches!(
      HatchError::NoDaemon.failure(),
      Failure::Unavailable(_)
    ));
    assert!(matches!(
      HatchError::Ask(AskError::ClosedWithoutAnswering).failure(),
      Failure::Unavailable(_)
    ));
    assert!(matches!(
      HatchError::Refused {
        message: "no".into(),
        remedy: "so".into()
      }
      .failure(),
      Failure::Error(_)
    ));
    assert!(matches!(
      HatchError::Variables { why: "x".into() }.failure(),
      Failure::Error(_)
    ));
  }

  #[test]
  fn the_daemon_down_remedies_name_the_start_verb_and_never_an_in_process_fallback() {
    // The positive control first: the WRAPPED remedy really does promise the
    // fallback, so the override below is changing something.
    let inner = AskError::Unreachable(std::io::Error::other("gone"));
    assert!(inner.remedy().contains("in-process"), "{}", inner.remedy());

    for e in [HatchError::NoDaemon, HatchError::Ask(inner)] {
      let text = e.render();
      assert!(text.contains("intent daemon start"), "{text}");
      assert!(!text.contains("executes in-process"), "{text}");
    }
  }

  #[test]
  fn the_daemon_only_roster_names_graphql_and_nothing_served_both_ways() {
    assert!(daemon_only("graphql"));
    for path in crate::render::daemon_servable_paths() {
      assert!(
        !daemon_only(path),
        "`{path}` is on both rosters, and the two make contradictory claims"
      );
    }
  }
}
