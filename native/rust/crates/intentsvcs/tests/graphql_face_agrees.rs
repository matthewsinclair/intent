//! AT-02.2 / AC-02.2: the GraphQL SDL face carries the same model the JSON
//! Schema face does.
//!
//! The SDL is exported from the authored master, so most of it cannot drift by
//! construction. One thing can: [`intentsvcs::graphql::AcStateView`], the
//! hand-written projection GraphQL needs because its type system cannot express
//! a tagged enum with per-variant fields. A projection that silently stops
//! matching the master is the failure this whole release has been about, so it
//! is held shut from both ends here rather than by anyone remembering.
//!
//! What these assert, and deliberately what they do not: that every field
//! reaches BOTH faces ON ITS OWN TYPE, and that the projection carries exactly
//! the data the serde form does. They do NOT require the two faces to spell
//! things the same way -- serde renders `AtStatus::Na` as `n-a` and GraphQL
//! renders it `NA`, one vocabulary in two wire conventions.
//!
//! Each of the three was proven by a mutation that makes it red, and two were
//! rewritten because the first cut did not go red when it should have: the
//! field check compared against a flat set of every name in the SDL, so hiding
//! `Thread.slug` passed on the strength of `Issue.slug`; and the enum check
//! turned out to guard something that cannot happen (see its own comment).

use serde_json::Value;

fn face(name: &str) -> String {
  intentsvcs::faces::faces()
    .into_iter()
    .find(|(n, _)| *n == name)
    .unwrap_or_else(|| panic!("no committed face named {name}"))
    .1
}

/// One `type` or `enum` block in the SDL, with its member names.
///
/// ONE parser for both kinds, because the hard part is shared: a description
/// block spans several lines and its CONTENT does not start with a quote, so
/// skipping quoted lines is not enough. The first cut of this probe counted two
/// lines of AtStatus's `n-a` description as enum values and reported 6 where the
/// model has 4.
fn sdl_blocks(sdl: &str) -> Vec<(String, String, Vec<String>)> {
  let mut out = Vec::new();
  let mut lines = sdl.lines();
  while let Some(line) = lines.next() {
    let (kind, rest) = match (line.strip_prefix("type "), line.strip_prefix("enum ")) {
      (Some(r), _) => ("type", r),
      (_, Some(r)) => ("enum", r),
      _ => continue,
    };
    let name = rest.trim_end_matches(" {").trim().to_string();
    let mut members = Vec::new();
    let mut in_description = false;
    for body in lines.by_ref() {
      let t = body.trim();
      if t.starts_with("\"\"\"") {
        // A single-line `"""..."""` opens and closes at once.
        if !(t.len() > 3 && t.ends_with("\"\"\"")) {
          in_description = !in_description;
        }
        continue;
      }
      if in_description {
        continue;
      }
      if t == "}" {
        break;
      }
      if t.is_empty() || t.starts_with('#') {
        continue;
      }
      // A field is `name: Type` or `name(args): Type`; an enum value is bare.
      let member = t
        .split(':')
        .next()
        .unwrap_or(t)
        .split('(')
        .next()
        .unwrap_or(t)
        .trim();
      if !member.is_empty() {
        members.push(member.to_string());
      }
    }
    out.push((kind.to_string(), name, members));
  }
  out
}

/// The named block's members, or a panic naming what the SDL does carry -- an
/// absent type must fail loudly rather than vacuously compare against nothing.
fn members_of<'a>(
  blocks: &'a [(String, String, Vec<String>)],
  kind: &str,
  name: &str,
) -> &'a [String] {
  blocks
    .iter()
    .find(|(k, n, _)| k == kind && n == name)
    .map(|(_, _, m)| m.as_slice())
    .unwrap_or_else(|| {
      let present: Vec<&str> = blocks
        .iter()
        .filter(|(k, _, _)| k == kind)
        .map(|(_, n, _)| n.as_str())
        .collect();
      panic!("SDL has no {kind} {name}; {kind}s present: {present:?}")
    })
}

/// snake_case -> the camelCase async-graphql renders it as.
fn camel(s: &str) -> String {
  let mut out = String::new();
  let mut up = false;
  for c in s.chars() {
    if c == '_' {
      up = true;
    } else if up {
      out.push(c.to_ascii_uppercase());
      up = false;
    } else {
      out.push(c);
    }
  }
  out
}

/// `(type name, its property names)` for a JSON Schema face: the root object
/// plus every `$defs` entry that has properties.
fn schema_types(face_json: &str) -> Vec<(String, Vec<String>)> {
  let schema: Value = serde_json::from_str(face_json).expect("a face is valid JSON");
  let mut out = Vec::new();
  let props_of = |v: &Value| -> Option<Vec<String>> {
    v.get("properties")
      .and_then(Value::as_object)
      .map(|p| p.keys().cloned().collect())
  };
  if let (Some(Value::String(title)), Some(props)) = (schema.get("title"), props_of(&schema)) {
    out.push((title.clone(), props));
  }
  if let Some(defs) = schema.get("$defs").and_then(Value::as_object) {
    for (name, def) in defs {
      if let Some(props) = props_of(def) {
        out.push((name.clone(), props));
      }
    }
  }
  out
}

#[test]
fn every_model_field_reaches_the_sdl_on_its_own_type() {
  let blocks = sdl_blocks(&face("schema.graphql"));

  // PER TYPE, not against a flat set of every field name in the SDL. The flat
  // form was the first cut and it passed a mutation that hid `Thread.slug`
  // from the SDL -- because `Issue` also has a `slug`, so the name was still
  // "present". A guard that cannot tell which type a field belongs to cannot
  // see a field move or vanish.
  let mut missing = Vec::new();
  let mut checked = 0;
  for schema_name in ["thread.schema.json", "issue.schema.json"] {
    let types = schema_types(&face(schema_name));
    assert!(
      !types.is_empty(),
      "{schema_name} yielded no types -- the probe is not reading the schema"
    );
    for (type_name, props) in types {
      // AcState is the one type with no SDL counterpart by that name: GraphQL
      // cannot express it, so it is projected as AcStateView and checked by
      // the projection test below.
      if type_name == "AcState" {
        continue;
      }
      let fields = members_of(&blocks, "type", &type_name);
      for p in props {
        checked += 1;
        if !fields.contains(&camel(&p)) {
          missing.push(format!("{type_name}.{p} (SDL {type_name} has {fields:?})"));
        }
      }
    }
  }
  assert!(
    checked > 20,
    "only {checked} fields checked -- the probe is not finding the model"
  );
  assert!(
    missing.is_empty(),
    "fields in the JSON Schema face that never reach the SDL face:\n  {}\n\
     a field added to the model must appear in BOTH generated faces",
    missing.join("\n  ")
  );
}

#[test]
fn every_enum_reaches_the_sdl_with_all_its_variants() {
  let sdl = face("schema.graphql");
  let blocks = sdl_blocks(&sdl);
  let count = |name: &str| members_of(&blocks, "enum", name).len();

  // What this actually guards, established by mutation rather than assumed.
  // async-graphql's `Enum` derive has NO `skip`, so a variant cannot be hidden
  // from the SDL -- attempting it does not compile. Every variant therefore
  // reaches the SDL by construction, and this is not a drift detector.
  //
  // It is a review tripwire in the other direction: adding or removing a model
  // variant changes a count here, so the vocabulary cannot grow or shrink
  // without someone landing the change in this file and looking at what the
  // wire contract now says. Proven by adding a seventh ThreadStatus.
  //
  // The two faces are NOT required to spell values identically -- serde renders
  // `AtStatus::Na` as `n-a` and GraphQL as `NA`, one vocabulary in two wire
  // conventions.
  assert_eq!(count("ThreadStatus"), 6, "ThreadStatus variants");
  assert_eq!(count("WpStatus"), 3, "WpStatus variants");
  assert_eq!(count("AtStatus"), 4, "AtStatus variants");
  assert_eq!(count("AcKind"), 2, "AcKind variants");
  assert_eq!(count("AtKind"), 2, "AtKind variants");
  assert_eq!(count("IssueStatus"), 2, "IssueStatus variants");
  assert_eq!(count("Tshirt"), 6, "TShirt variants");
  assert_eq!(count("AcceptanceMode"), 1, "AcceptanceMode variants");
  // The projection's own discriminant, one per AcState variant.
  assert_eq!(count("AcStateName"), 5, "AcStateName variants");
}

#[test]
fn the_ac_scope_projection_carries_exactly_the_serde_form() {
  use intentsvcs::graphql::AcStateView;
  use intentsvcs::model::AcState;

  let cases = vec![
    AcState::Computed,
    AcState::Unsatisfied,
    AcState::Satisfied {
      evidence: "the render itself".into(),
    },
    AcState::Descoped {
      to: "ST0057".into(),
      by: Some("hv".into()),
      reason: Some("moved".into()),
    },
    AcState::Descoped {
      to: "ST0057".into(),
      by: None,
      reason: None,
    },
    AcState::Withdrawn {
      reason: "not doing it".into(),
      by: Some("hv".into()),
    },
    AcState::Withdrawn {
      reason: "not doing it".into(),
      by: None,
    },
  ];

  for scope in cases {
    let json = serde_json::to_value(&scope).expect("AcState serialises");
    let obj = json
      .as_object()
      .expect("internally-tagged form is an object");
    let view = AcStateView::from(&scope);

    // The serde form's non-tag keys and the view's populated fields must be
    // the same set. Either direction failing is data the other face loses.
    let mut serde_keys: Vec<&str> = obj
      .keys()
      .map(String::as_str)
      // `is` is the discriminant, carried by the view as `state` rather than
      // as a field. It used to be spelled `state`; the tag was renamed so the
      // extract reads `"state": {"is": ...}` instead of doubling the word.
      .filter(|k| *k != "is")
      .collect();
    serde_keys.sort_unstable();

    let mut view_keys = Vec::new();
    if view.evidence.is_some() {
      view_keys.push("evidence");
    }
    if view.to.is_some() {
      view_keys.push("to");
    }
    if view.by.is_some() {
      view_keys.push("by");
    }
    if view.reason.is_some() {
      view_keys.push("reason");
    }
    view_keys.sort_unstable();

    assert_eq!(
      serde_keys, view_keys,
      "AcStateView and the serde form disagree about which fields {scope:?} carries"
    );

    // And the values themselves, not just their presence.
    for key in &serde_keys {
      let serde_val = obj.get(*key).and_then(Value::as_str);
      let view_val = match *key {
        "evidence" => view.evidence.as_deref(),
        "to" => view.to.as_deref(),
        "by" => view.by.as_deref(),
        "reason" => view.reason.as_deref(),
        other => panic!("unexpected AcState field {other} -- the projection needs updating"),
      };
      assert_eq!(
        serde_val, view_val,
        "AcState field {key} differs between faces"
      );
    }
  }
}
