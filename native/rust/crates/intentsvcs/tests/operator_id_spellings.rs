//! An operator's spelling of an id resolves, and a spelling that names nothing
//! is refused as a spelling rather than as a missing thing.
//!
//! **THE ACCEPTED SET IS v2's, AND IT WAS DRIVEN RATHER THAN READ OFF THE
//! SOURCE.** `bin/intent_helpers:688 normalise_st_id` was run in the v2
//! checkout across all five forms; each resolved to `ST0046`, with the error
//! echoing the NORMALISED id, which is what makes the observation a measurement
//! rather than a reading. That is why `046` is in the table below: it is in the
//! contract and a four-form port would have silently dropped it.
//!
//! **TWO ARMS PIN BEHAVIOUR v2 HAS AND THIS MUST NOT.** `foo` must not become
//! `STfoo`, and `99999` must not become `ST99999`. Both are live in v2 today,
//! and both end with the operator being told a thread they never named is
//! missing -- so the normaliser would be MANUFACTURING the defect it exists to
//! prevent. They are tested here rather than trusted to the port's good sense
//! because the shape of a port is to reproduce, and reproducing was wrong twice.

use intentsvcs::model::{self, IdError, IdKind};

/// The five forms v2 accepts, driven in the v2 checkout before being pinned.
#[test]
fn the_v2_parity_contract_is_five_forms() {
  for raw in ["46", "ST46", "0046", "046", "ST0046"] {
    assert_eq!(
      model::normalise_thread_id(raw),
      Ok("ST0046".to_string()),
      "`{raw}` is one of v2's five accepted spellings"
    );
  }
}

/// **THE OCTAL PROPERTY, WHICH IS THE ONE PART OF v2's `10#` WORTH PORTING.**
/// Without it, /bin/bash 3.2 reads `0044` as octal and produces ST0036 -- a
/// different, real thread. Rust cannot express that defect, so the MECHANISM is
/// not ported; this pins the property so a future rewrite through a base-aware
/// path cannot reintroduce it silently.
#[test]
fn a_leading_zero_is_decimal_and_not_octal() {
  assert_eq!(
    model::normalise_thread_id("0044"),
    Ok("ST0044".to_string()),
    "0044 is thread 44 -- read as octal it would be ST0036, which also exists"
  );
  assert_eq!(model::normalise_thread_id("046"), Ok("ST0046".to_string()));
  assert_eq!(model::normalise_issue_id("0044"), Ok(44));
}

/// The explicit tags, which are what give the ambiguous door something to
/// recommend. Case-insensitive: a hand-typed argument is not a committed
/// manifest, so there is nothing behind refusing `st0059` for its case.
#[test]
fn an_explicit_tag_names_the_collection() {
  for raw in ["s59", "S59", "st59", "ST59", "ST0059", "sT0059"] {
    assert_eq!(model::normalise_thread_id(raw), Ok("ST0059".to_string()));
  }
  for raw in ["i59", "I59", "0059"] {
    assert_eq!(model::normalise_issue_id(raw), Ok(59));
  }
}

/// **THE ARM THIS WHOLE CHANGE IS FOR.** A category error must not be reported
/// as a not-found. Before this, `st show i59` answered *no steel thread i59 in
/// this project* -- sending an operator to look for something never addressed.
#[test]
fn a_wrong_collection_tag_is_refused_by_name_and_not_as_a_not_found() {
  assert_eq!(
    model::normalise_thread_id("i59"),
    Err(IdError::WrongCollection {
      named: IdKind::Issue
    })
  );
  assert_eq!(
    model::normalise_issue_id("s59"),
    Err(IdError::WrongCollection {
      named: IdKind::Thread
    })
  );
  assert_eq!(
    model::normalise_issue_id("ST0059"),
    Err(IdError::WrongCollection {
      named: IdKind::Thread
    })
  );
}

/// **v2 HOLE ONE, NOT PORTED.** `normalise_st_id foo` is `STfoo` in v2, and
/// `46abc` is `ST46abc`; v2 then reports a missing thread by that fabricated
/// name. A normaliser that invents a plausible id is worse than one that
/// refuses, because the refusal it produces points at the estate.
#[test]
fn a_non_id_is_refused_rather_than_having_a_prefix_glued_to_it() {
  for raw in ["foo", "46abc", "---", "", "  ", "ST", "s", "i", "ST-46"] {
    assert_eq!(
      model::normalise_thread_id(raw),
      Err(IdError::NotAnId),
      "`{raw}` names nothing, so it must not become an id"
    );
  }
}

/// **v2 HOLE TWO, NOT PORTED.** `printf "ST%04d"` and `format!("{:04}")` are
/// both MINIMUM widths, so `99999` survives as `ST99999` unless the range is
/// checked. `"99999".parse::<u32>()` is `Ok(99999)`, so Rust reproduces this
/// one exactly -- being immune to the octal hazard bought nothing here.
#[test]
fn a_sequence_wider_than_the_fixed_id_is_out_of_range() {
  for raw in ["99999", "ST99999", "s99999", "00099999"] {
    assert_eq!(
      model::normalise_thread_id(raw),
      Err(IdError::OutOfRange),
      "`{raw}` does not fit the fixed four-digit form"
    );
  }
  assert_eq!(
    model::normalise_issue_id("i99999"),
    Err(IdError::OutOfRange)
  );
  // A 32-bit overflow is still out of range and not a parse failure: the
  // spelling IS digits, so `NotAnId` would blame the wrong thing.
  assert_eq!(
    model::normalise_thread_id("99999999999999"),
    Err(IdError::OutOfRange)
  );
}

/// Padding is a spelling, not a value: `0046` and `046` and `46` are one id, so
/// the zero-run must not change the answer or the width verdict.
#[test]
fn leading_zeros_are_spelling_and_not_width() {
  assert_eq!(model::normalise_thread_id("0000"), Ok("ST0000".to_string()));
  assert_eq!(model::normalise_issue_id("0000"), Ok(0));
  assert_eq!(
    model::normalise_thread_id("000000046"),
    Ok("ST0046".to_string()),
    "nine characters, but the sequence is 46 and fits"
  );
}

/// A padded id copied out of a filename is the same issue as the bare number.
#[test]
fn a_filename_suffix_is_stripped() {
  assert_eq!(model::normalise_issue_id("0021.json"), Ok(21));
  assert_eq!(model::normalise_issue_id("0021.md"), Ok(21));
  assert_eq!(model::normalise_issue_id(" 0021.json "), Ok(21));
}

/// **THE AGNOSTIC DOOR ONLY GAINS.** Both canonical forms still resolve there;
/// untagged short digits are refused as AMBIGUOUS rather than guessed, and the
/// refusal carries the sequence so the caller can name `s59` and `i59` back.
#[test]
fn the_collection_agnostic_door_refuses_only_what_is_genuinely_ambiguous() {
  assert_eq!(model::normalise_id("ST0059"), Ok((IdKind::Thread, 59)));
  assert_eq!(model::normalise_id("0059"), Ok((IdKind::Issue, 59)));
  assert_eq!(model::normalise_id("s59"), Ok((IdKind::Thread, 59)));
  assert_eq!(model::normalise_id("i59"), Ok((IdKind::Issue, 59)));

  // `59` and `046` name both collections and neither door is named. In THIS
  // estate both ST0059 and issue 0059 exist, so this is not hypothetical.
  assert_eq!(
    model::normalise_id("59"),
    Err(IdError::Ambiguous { seq: 59 })
  );
  assert_eq!(
    model::normalise_id("046"),
    Err(IdError::Ambiguous { seq: 46 })
  );
  assert_eq!(model::normalise_id("foo"), Err(IdError::NotAnId));
}

/// **THE NORMALISER AGREES WITH THE PREDICATES THAT ALREADY OWN IDENTITY.**
/// Two declarations of one fact agree until they do not, so this pins the join
/// rather than asserting the widths a second time.
#[test]
fn every_canonical_output_satisfies_the_existing_predicate() {
  for raw in ["0", "7", "46", "s999", "ST9999", "9999"] {
    let id = model::normalise_thread_id(raw).expect("in range");
    assert!(
      model::is_thread_id(&id),
      "`{raw}` normalised to `{id}`, which is not a thread id"
    );
    assert_eq!(
      model::thread_seq(&id),
      model::normalise_thread_id(raw)
        .ok()
        .and_then(|i| model::thread_seq(&i))
    );
  }
  for raw in ["0", "7", "46", "i999", "9999"] {
    let n = model::normalise_issue_id(raw).expect("in range");
    assert!(model::is_issue_id(&format!("{n:04}")));
  }
}
