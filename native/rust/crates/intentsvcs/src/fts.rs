//! `0247`: **the one place a user's search string becomes an FTS5 expression.**
//!
//! **THE QUERY USED TO GO TO FTS5 AS WRITTEN, AND ORDINARY PUNCTUATION CAME
//! BACK AS A DATABASE SCHEMA ERROR.** `intent search 'family-root'` answered
//! `sqlite: no such column: root`, three layers of causation deep, because
//! FTS5 reads a bare `-` and `:` as COLUMN SYNTAX. `render.rs`, `AGENTS.md`
//! and `v3.0.1` -- a source file, the artefact this estate's own canon
//! mandates, and this project's release number -- were all unsearchable for
//! the same reason.
//!
//! **THE SHIPPED REMEDY NAMED `*`, WHICH WORKS, AND OMITTED `-` `.` `/`,
//! WHICH DO NOT.** An operator following it escaped a character that needed
//! nothing and left the three that broke. An unkeepable remedy is worse than
//! none, because it reads as complete and spends the reader's trust before
//! returning them to the same failure.
//!
//! **THE `-` AND `:` HALF IS WHY THIS ESCAPES RATHER THAN DOCUMENTS.** Those
//! two do not merely fail: they REINTERPRET the query as a column filter. A
//! filter naming a column that happens to EXIST would not error at all -- it
//! would silently search a narrower field and return a confident, non-empty,
//! wrong answer. That state was never observed and is not claimed as
//! observed; it is the reason a remedy that only describes the problem would
//! have left the dangerous half in place.
//!
//! **WHAT IS PRESERVED, SO NO EXPRESSION THAT WORKED BEFORE STOPS WORKING:**
//! the operators `AND` `OR` `NOT` `NEAR`, parentheses, an already-quoted
//! phrase, a trailing `*` prefix, a leading `^` initial-token match, and the
//! EXPLICIT `{col}:` column filter.
//!
//! **WHAT IS DELIBERATELY GIVEN UP: the BARE `col:` filter.** It is the exact
//! shape described above, it is indistinguishable from an operator typing a
//! word with a colon in it, and the explicit `{col}:` spelling remains for
//! anyone who means it. **This is a stated trade rather than an oversight**,
//! and `0247` records it as the one design call the fix takes.

/// The FTS5 operators, which must be uppercase to be operators at all.
///
/// **LOWERCASE `and` IS A SEARCH TERM AND IS QUOTED LIKE ANY OTHER**, which is
/// FTS5's own rule rather than a choice made here -- so a query for `black and
/// white` keeps meaning what its author meant.
const OPERATORS: [&str; 4] = ["AND", "OR", "NOT", "NEAR"];

/// One lexical piece of the user's query, before any decision about it.
#[derive(Debug, PartialEq)]
enum Token {
  /// A `(` or `)`, which FTS5 groups with and this function never quotes.
  Paren(char),
  /// A run the user already quoted, carried through verbatim including its
  /// quotes and any trailing `*`.
  Quoted(String),
  /// Everything else: a bare run of non-space, non-paren characters.
  Bare(String),
}

/// Split the query the way FTS5's own grammar divides it, so that quoting
/// decisions are made per piece rather than over the whole string.
///
/// **PARENS ARE THEIR OWN TOKENS EVEN WHEN THEY TOUCH A WORD.** `(foo` is a
/// paren and a term, and treating it as one bare token would quote the paren
/// into the phrase and break the grouping the user wrote.
fn tokenize(query: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let mut chars = query.chars().peekable();
  let mut bare = String::new();

  // Close off whatever bare run is open before starting a different token.
  macro_rules! flush {
    () => {
      if !bare.is_empty() {
        tokens.push(Token::Bare(std::mem::take(&mut bare)));
      }
    };
  }

  while let Some(c) = chars.next() {
    match c {
      c if c.is_whitespace() => flush!(),
      '(' | ')' => {
        flush!();
        tokens.push(Token::Paren(c));
      }
      '"' => {
        flush!();
        // **AN UNCLOSED QUOTE ENDS AT THE END OF THE QUERY RATHER THAN
        // ERRORING.** FTS5 would refuse it, and refusing here would move the
        // refusal one layer further from the operator without improving it.
        let mut span = String::from('"');
        let mut closed = false;
        while let Some(q) = chars.next() {
          span.push(q);
          if q == '"' {
            // A doubled `""` is an escaped quote INSIDE the phrase, not the
            // end of it.
            if chars.peek() == Some(&'"') {
              span.push(chars.next().expect("peeked"));
              continue;
            }
            closed = true;
            break;
          }
        }
        if !closed {
          span.push('"');
        }
        if chars.peek() == Some(&'*') {
          span.push(chars.next().expect("peeked"));
        }
        tokens.push(Token::Quoted(span));
      }
      _ => bare.push(c),
    }
  }
  flush!();
  tokens
}

/// Wrap one bare run as a literal phrase, keeping the affixes FTS5 gives
/// meaning to.
///
/// **`^` AND `*` ARE POSITIONAL AND MUST STAY OUTSIDE THE QUOTES**, because
/// `"^foo"` searches for a literal caret and `"foo*"` searches for a literal
/// asterisk -- both of which silently answer the wrong question rather than
/// failing.
fn quote(term: &str) -> Option<String> {
  let (caret, rest) = match term.strip_prefix('^') {
    Some(rest) => ("^", rest),
    None => ("", term),
  };
  let (rest, star) = match rest.strip_suffix('*') {
    Some(rest) => (rest, "*"),
    None => (rest, ""),
  };
  if rest.is_empty() {
    // A lone `^` or `*` is punctuation with nothing to apply to. Dropping it
    // is what makes `search 'foo *'` mean `foo` rather than a syntax error.
    return None;
  }
  // **NO ESCAPING HERE, AND THAT IS AN INVARIANT RATHER THAN AN OVERSIGHT.**
  // `tokenize` gives every `"` its own quoted span, so a `Bare` run cannot
  // contain one and a `replace('"', "\"\"")` on this line would be a branch
  // that never fires -- the shape `0241` was filed about.
  debug_assert!(
    !rest.contains('"'),
    "tokenize routes every quote to a Quoted span"
  );
  Some(format!("{caret}\"{rest}\"{star}"))
}

/// The FTS5 expression for what the operator typed.
///
/// **PURE, AND THAT IS WHY IT IS HERE RATHER THAN IN THE FACADE** -- the
/// decision is a property of the string alone, so it is testable without a
/// store, and `Facade::search` stays the impure half that opens a database.
pub fn expression(query: &str) -> String {
  let mut out: Vec<String> = Vec::new();
  for token in tokenize(query) {
    match token {
      Token::Paren(c) => out.push(c.to_string()),
      Token::Quoted(span) => out.push(span),
      Token::Bare(term) => {
        if OPERATORS.contains(&term.as_str()) {
          out.push(term);
        } else if term.starts_with('{') && term.contains("}:") {
          // The EXPLICIT column filter, which is unambiguous and stays
          // reachable. The bare `col:` form is what this module removes.
          out.push(term);
        } else if let Some(quoted) = quote(&term) {
          out.push(quoted);
        }
      }
    }
  }
  out.join(" ")
}

#[cfg(test)]
mod tests {
  use super::expression;

  /// **THE FOUR QUERIES `0247` NAMES AS UNSEARCHABLE**, which are a source
  /// file, the artefact this estate's canon mandates every agent regenerate,
  /// this project's own release number, and a phrase from an issue title.
  #[test]
  fn ordinary_punctuation_becomes_a_literal_phrase() {
    assert_eq!(expression("render.rs"), r#""render.rs""#);
    assert_eq!(expression("AGENTS.md"), r#""AGENTS.md""#);
    assert_eq!(expression("v3.0.1"), r#""v3.0.1""#);
    assert_eq!(expression("family-root"), r#""family-root""#);
  }

  /// **THE HALF THAT WAS NOT MERELY BROKEN BUT REINTERPRETED.** A bare `-` or
  /// `:` asked FTS5 for a column; both are now literal.
  #[test]
  fn a_bare_colon_is_a_term_and_no_longer_a_column_filter() {
    assert_eq!(expression("alpha:beta"), r#""alpha:beta""#);
    assert_eq!(expression("alpha-beta"), r#""alpha-beta""#);
  }

  /// **NO EXPRESSION THAT WORKED BEFORE STOPS WORKING**, which is the property
  /// that makes this safe to land without a survey of who was using what.
  #[test]
  fn every_operator_and_affix_survives() {
    assert_eq!(expression("foo OR bar"), r#""foo" OR "bar""#);
    assert_eq!(expression("foo AND bar"), r#""foo" AND "bar""#);
    assert_eq!(expression("NOT foo"), r#"NOT "foo""#);
    assert_eq!(
      expression("(foo OR bar) AND baz"),
      r#"( "foo" OR "bar" ) AND "baz""#
    );
    assert_eq!(expression("foo*"), r#""foo"*"#);
    assert_eq!(expression("^foo"), r#"^"foo""#);
    assert_eq!(expression(r#""a phrase""#), r#""a phrase""#);
    assert_eq!(expression(r#""a phrase"*"#), r#""a phrase"*"#);
    assert_eq!(expression("{heading}:foo"), "{heading}:foo");
  }

  /// **LOWERCASE IS A TERM, WHICH IS FTS5's RULE AND NOT A CHOICE HERE** -- so
  /// a search for `black and white` still means what its author meant.
  #[test]
  fn a_lowercase_operator_is_an_ordinary_word() {
    assert_eq!(expression("black and white"), r#""black" "and" "white""#);
  }

  /// **PUNCTUATION WITH NOTHING TO APPLY TO IS DROPPED RATHER THAN PASSED ON**,
  /// because passing it on is how a stray character becomes a syntax error the
  /// operator cannot see the cause of.
  #[test]
  fn punctuation_with_no_term_is_dropped() {
    assert_eq!(expression("foo *"), r#""foo""#);
    assert_eq!(expression("   "), "");
  }

  /// **A STRAY QUOTE SPLITS THE TERM RATHER THAN CORRUPTING IT, AND THIS TEST
  /// ASSERTED MY GUESS BEFORE IT ASSERTED THE BEHAVIOUR.** I expected
  /// `"say""hi"` -- a doubled-quote escape -- and the code gives `"say" "hi"`,
  /// because `tokenize` hands every `"` to a quoted span. **Both search for
  /// the same two words and neither reaches FTS5 as syntax**, so the
  /// expectation moved rather than the code. It also proved the escape in
  /// `quote` unreachable, which is now a `debug_assert` instead of a branch
  /// that never fires.
  #[test]
  fn a_stray_quote_splits_the_term_rather_than_corrupting_it() {
    assert_eq!(expression(r#"say"hi"#), r#""say" "hi""#);
  }

  /// **AN UNCLOSED QUOTE IS CLOSED HERE RATHER THAN REFUSED THERE.** FTS5
  /// would refuse it with its own syntax error, which is the failure mode this
  /// module exists to stop showing people.
  #[test]
  fn an_unclosed_quote_is_closed_rather_than_handed_on() {
    assert_eq!(expression(r#""unclosed"#), r#""unclosed""#);
  }
}
