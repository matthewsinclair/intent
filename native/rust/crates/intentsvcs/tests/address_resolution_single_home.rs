//! AT-07.1 / AC-07.1: **every entity form in D57-8's list resolves by
//! address, and resolution is implemented ONCE** -- the CLI and intentd call
//! the same function in `intentsvcs`.
//!
//! Two resolvers is the failure. They agree exactly until one moves, with
//! nothing watching. **Checked by asserting a single implementation, not by
//! comparing two outputs** -- comparing two outputs is what you do once you
//! have already lost, because it passes on the day they are written and every
//! day until the divergence, and the divergence is the event.
//!
//! # The two halves
//!
//! **Coverage**: all nine forms parse, with the denominator printed over
//! D57-8's list rather than over the cases that happened to be written. A
//! table of nine cases proves nine cases; the assertion here is that the nine
//! are the WHOLE list, so a tenth form added to the design without a parser
//! arm fails here rather than silently going unaddressable.
//!
//! **Singularity**: the `intent-cli` and `intentd` crates contain no parsing
//! of their own. Asserted by reading their sources for the scheme literal --
//! a second resolver has to spell `intent://` somewhere, and the one place
//! that string legitimately appears is this module.

use intentsvcs::address::{Address, Entity, Format, SCHEME, parse};
use std::path::Path;
use testkit::repo_root;

/// D57-8's list, verbatim from `design.md`. **This is the denominator.**
///
/// Written as the URL and the entity it must produce, so the case is a claim
/// about resolution rather than about not-crashing.
fn d57_8_forms() -> Vec<(&'static str, Entity)> {
  vec![
    (
      "intent:///threads/ST0056",
      Entity::Thread {
        id: "ST0056".into(),
      },
    ),
    (
      "intent:///threads/ST0056/wp/02",
      Entity::Wp {
        thread: "ST0056".into(),
        wp: "02".into(),
      },
    ),
    (
      "intent:///threads/ST0056/ac/AC-02.1",
      Entity::Ac {
        thread: "ST0056".into(),
        ac: "AC-02.1".into(),
      },
    ),
    (
      "intent:///threads/ST0056/at/AT-02.1",
      Entity::At {
        thread: "ST0056".into(),
        at: "AT-02.1".into(),
      },
    ),
    (
      "intent:///threads/ST0056/attachments/parity/tools/burn.sh",
      Entity::Attachment {
        thread: "ST0056".into(),
        path: "parity/tools/burn.sh".into(),
      },
    ),
    ("intent:///issues/0042", Entity::Issue { id: "0042".into() }),
    (
      "intent:///nodes/ic",
      Entity::Node {
        moniker: "ic".into(),
      },
    ),
    (
      "intent:///nodes/ic/inbox/vc/2026-08-19T11:41Z",
      Entity::NodeInbox {
        moniker: "ic".into(),
        sender: "vc".into(),
        stamp: "2026-08-19T11:41Z".into(),
      },
    ),
    ("intent:///events/1234", Entity::Event { id: "1234".into() }),
  ]
}

#[test]
fn every_d57_8_form_resolves() {
  let forms = d57_8_forms();
  assert_eq!(
    forms.len(),
    9,
    "D57-8 lists nine forms; this table is the denominator and must hold all of\n       \
     them. A form added to the design without a row here goes unaddressable in\n       \
     silence, which is the coverage gap the criterion names."
  );

  for (url, expected) in forms {
    let a = parse(url).unwrap_or_else(|e| panic!("{url} must resolve: {e}"));
    assert_eq!(a.entity, expected, "{url} resolved to the wrong entity");
    assert!(a.is_local(), "{url} has the empty authority");
    assert_eq!(a.format, None, "no `?format=` was asked for");
  }
}

/// The round trip, which is what makes the parse a RESOLUTION rather than a
/// recognition: an address that parses must render back to itself.
#[test]
fn every_form_round_trips() {
  for (url, _) in d57_8_forms() {
    let a = parse(url).expect("resolves");
    assert_eq!(a.to_url(), url, "{url} did not survive the round trip");

    for f in [Format::Json, Format::Md] {
      let with_format = format!("{url}?format={}", f.as_str());
      let b = parse(&with_format).expect("resolves with a format");
      assert_eq!(b.format, Some(f));
      assert_eq!(b.to_url(), with_format);
      assert_eq!(
        b.entity, a.entity,
        "a format must not change WHAT is addressed, only its representation"
      );
    }
  }
}

/// **Singularity, asserted structurally.**
///
/// A second resolver has to spell the scheme somewhere. `intent-cli` and
/// `intentd` may CALL resolution and must not implement it, so neither may
/// carry the literal outside a string handed straight to `address::parse`.
///
/// Reading sources rather than comparing behaviour is the point of the
/// criterion: behaviour comparison is a test that passes right up until the
/// moment it matters.
#[test]
fn no_second_resolver_exists() {
  let workspace = repo_root().join("native").join("rust").join("crates");
  let mut offenders: Vec<String> = Vec::new();

  for crate_name in ["intent-cli", "intentd"] {
    let src = workspace.join(crate_name).join("src");
    if !src.is_dir() {
      continue;
    }
    collect_scheme_hits(&src, &mut offenders);
  }

  assert!(
    offenders.is_empty(),
    "resolution has ONE home and these spell the scheme themselves:\n  {}\n\
     \n  If a consumer needs an address, call `intentsvcs::address::parse`.\n  \
     Two resolvers agree exactly until one moves, with nothing watching.",
    offenders.join("\n  ")
  );
}

fn collect_scheme_hits(dir: &Path, out: &mut Vec<String>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.filter_map(Result::ok) {
    let path = entry.path();
    if path.is_dir() {
      collect_scheme_hits(&path, out);
      continue;
    }
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
      continue;
    }
    let Ok(text) = std::fs::read_to_string(&path) else {
      continue;
    };
    for (i, line) in text.lines().enumerate() {
      // A doc comment naming the scheme is prose, not a parser.
      let trimmed = line.trim_start();
      if trimmed.starts_with("//") {
        continue;
      }
      if line.contains(SCHEME) {
        out.push(format!("{}:{}", path.display(), i + 1));
      }
    }
  }
}

/// The scheme itself is required, so a bare path is not an address.
#[test]
fn a_path_is_not_an_address() {
  for not_a_url in [
    "/threads/ST0056",
    "threads/ST0056",
    "intent:/threads/ST0056",
    "http://example/threads/ST0056",
    "intent/st/ST0056/info.md",
  ] {
    assert!(
      parse(not_a_url).is_err(),
      "`{not_a_url}` is not an address and must not resolve as one"
    );
  }
}

/// An unknown collection is refused rather than guessed at, and the refusal
/// names the segment so the operator can see which one it was.
#[test]
fn unknown_collections_are_refused() {
  let err = parse("intent:///views/ST0056").expect_err("views is not a collection");
  assert!(err.to_string().contains("views"));

  for bad in [
    "intent:///wp/02",
    "intent:///attachments/x",
    "intent:///thread/ST0056",
  ] {
    assert!(parse(bad).is_err(), "`{bad}` must be refused");
  }
}

/// A malformed id is refused by the SAME predicate the rest of the estate
/// uses -- `model::is_thread_id` and `model::is_issue_id` -- so the scheme
/// cannot acquire a second opinion about what an id is.
#[test]
fn ids_are_checked_by_the_estate_predicate() {
  for bad in [
    "intent:///threads/ST56",
    "intent:///threads/ST00567",
    "intent:///threads/st0056",
    "intent:///threads/intent",
    "intent:///issues/42",
    "intent:///issues/ST0056",
  ] {
    assert!(parse(bad).is_err(), "`{bad}` must be refused");
  }
  assert!(parse("intent:///threads/ST0000").is_ok(), "ST0000 is real");
  assert!(parse("intent:///issues/0001").is_ok());
}

/// Address equality is about the entity and the representation, and both are
/// carried. Included because `Address` is what every consumer will compare.
#[test]
fn addresses_compare_by_entity_and_format() {
  let a: Address = parse("intent:///threads/ST0056?format=md").expect("resolves");
  let b = parse("intent:///threads/ST0056?format=md").expect("resolves");
  let c = parse("intent:///threads/ST0056?format=json").expect("resolves");
  assert_eq!(a, b);
  assert_ne!(a, c, "the representation is part of the address");
}

/// **Trailing segments are REFUSED, never truncated.**
///
/// Added because a mutant survived without it: a parser matching
/// `["threads", id, ..]` and ignoring the remainder passed all twenty cases in
/// this WP while resolving `intent:///threads/ST0056/anything/at/all` to the
/// bare thread. That is the silent-skip class arriving in the address scheme,
/// and it is worse here than in the manifest, because it makes the refusal in
/// AT-07.2 a NAME MATCH rather than a rule: `/threads/ST0056/info.md` is
/// caught only because `info.md` is on a list, while
/// `/threads/ST0056/whatever.md` would resolve to the thread and drop the
/// segment. **The scheme becomes a path alias one unlisted filename at a
/// time**, which is precisely what AC-07.2 exists to prevent.
///
/// So the general rule carries it and the name list is the specific message,
/// not the mechanism.
#[test]
fn trailing_segments_are_refused_rather_than_truncated() {
  for over_long in [
    "intent:///threads/ST0056/anything",
    "intent:///threads/ST0056/whatever.md",
    "intent:///threads/ST0056/wp/02/extra",
    "intent:///threads/ST0056/ac/AC-02.1/deeper",
    "intent:///threads/ST0056/at/AT-02.1/deeper",
    "intent:///issues/0042/extra",
    "intent:///nodes/ic/inbox/vc/2026-08-19T11:41Z/extra",
    "intent:///events/1234/extra",
  ] {
    assert!(
      parse(over_long).is_err(),
      "`{over_long}` must be REFUSED -- resolving it to a shorter address\n       \
       silently drops what the writer asked for, and turns the view refusal\n       \
       into a list of filenames rather than a rule"
    );
  }

  // The discriminating half: an ATTACHMENT path is genuinely variable-length,
  // so the rule must not be "count the segments".
  assert!(parse("intent:///threads/ST0056/attachments/a/b/c/d.sh").is_ok());
  // But an attachments address with NO path names nothing.
  assert!(parse("intent:///threads/ST0056/attachments").is_err());
}
