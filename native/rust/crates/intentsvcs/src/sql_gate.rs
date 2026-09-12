//! **THE ONE-STATEMENT GATE FOR `intent search --sql`, AS A PURE FUNCTION.**
//!
//! The SQL door runs exactly one read statement (AC-17.1). A read-only
//! connection refuses the WRITE half on its own; it does not refuse a second
//! statement, because `sqlite3_prepare_v2` prepares the first and hands back
//! the rest as a tail nobody is obliged to look at. rusqlite's own tail check
//! is `pub(crate)`, so there is nothing public to lean on and this is the check.
//!
//! **IT IS PURE, AND THAT IS THE WHOLE REASON IT IS ITS OWN MODULE** (PFIC):
//! the property is about TEXT, so it can be driven to both verdicts on string
//! literals with no database, no store and no estate anywhere near it.
//!
//! **A NAIVE `contains(';')` FAILS IN THE DIRECTION NOBODY MEASURES.**
//! `SELECT ';'` is one statement and a perfectly ordinary query, and a gate
//! that refuses it is an over-refusal the tests for the UNDER-refusal cannot
//! see -- both directions look like "the gate works" from the side that only
//! sends batches. So the scanner knows what SQLite knows: `'...'` string
//! literals with `''` escapes, `"..."` and `[...]` and backtick identifiers,
//! `--` line comments and `/* ... */` block comments.
//!
//! **A TRAILING SEMICOLON IS NOT A SECOND STATEMENT.** `SELECT 1;` is what a
//! person types out of habit, and refusing it would teach them the door is
//! fussy rather than safe. Only a separator with something after it counts.

/// Why the gate refused. The caller renders it; this module states it.
#[derive(Debug, PartialEq, Eq)]
pub enum GateRefusal {
  /// A statement separator with more statement after it.
  MoreThanOneStatement,
  /// Nothing but whitespace and comments.
  Empty,
  /// A quote or a block comment that never closes. **Refused rather than
  /// passed on to SQLite**, because an unterminated literal is exactly where a
  /// scanner and a parser disagree, and a disagreement here is the gate
  /// reporting one statement over text SQLite reads as two.
  Unterminated,
}

/// Is this text exactly one statement?
pub fn single_statement(sql: &str) -> Result<(), GateRefusal> {
  let mut chars = sql.chars().peekable();
  let mut seen_statement = false;
  let mut ended = false;

  while let Some(c) = chars.next() {
    match c {
      // A separator ends the statement. Anything but whitespace and comments
      // after it is a second one.
      ';' => {
        if ended {
          return Err(GateRefusal::MoreThanOneStatement);
        }
        ended = true;
      }
      '\'' | '"' | '`' => {
        if ended {
          return Err(GateRefusal::MoreThanOneStatement);
        }
        seen_statement = true;
        let mut closed = false;
        while let Some(q) = chars.next() {
          if q == c {
            // A doubled quote is an escaped quote and the literal continues.
            if chars.peek() == Some(&c) {
              chars.next();
              continue;
            }
            closed = true;
            break;
          }
        }
        if !closed {
          return Err(GateRefusal::Unterminated);
        }
      }
      '[' => {
        if ended {
          return Err(GateRefusal::MoreThanOneStatement);
        }
        seen_statement = true;
        if !chars.any(|q| q == ']') {
          return Err(GateRefusal::Unterminated);
        }
      }
      '-' if chars.peek() == Some(&'-') => {
        chars.next();
        for q in chars.by_ref() {
          if q == '\n' {
            break;
          }
        }
      }
      '/' if chars.peek() == Some(&'*') => {
        chars.next();
        let mut closed = false;
        let mut prev = '\0';
        for q in chars.by_ref() {
          if prev == '*' && q == '/' {
            closed = true;
            break;
          }
          prev = q;
        }
        if !closed {
          return Err(GateRefusal::Unterminated);
        }
      }
      c if c.is_whitespace() => {}
      _ => {
        if ended {
          return Err(GateRefusal::MoreThanOneStatement);
        }
        seen_statement = true;
      }
    }
  }

  if seen_statement {
    Ok(())
  } else {
    Err(GateRefusal::Empty)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_plain_statement_passes_with_or_without_its_semicolon() {
    assert_eq!(single_statement("select 1"), Ok(()));
    assert_eq!(single_statement("select 1;"), Ok(()));
    assert_eq!(single_statement("  select 1 ;  \n"), Ok(()));
  }

  /// **THE UNDER-REFUSAL, which is the one that turns a read door into a write
  /// door**: SQLite prepares the first statement and hands back the rest.
  #[test]
  fn a_second_statement_is_refused_however_it_is_spelled() {
    for sql in [
      "select 1; select 2",
      "select 1;select 2",
      "select 1;\n-- a comment\ndelete from thread",
      "select 1; ;",
    ] {
      assert_eq!(
        single_statement(sql),
        Err(GateRefusal::MoreThanOneStatement),
        "{sql:?} was not refused as a batch"
      );
    }
  }

  /// **THE OVER-REFUSAL, and it is the control that makes the test above mean
  /// something.** A gate that refuses everything passes every batch test.
  #[test]
  fn a_semicolon_inside_a_literal_or_a_comment_is_not_a_second_statement() {
    for sql in [
      "select ';'",
      "select 'a;b' as x",
      "select '' ';' ''",
      "select \"a;b\"",
      "select [a;b]",
      "select `a;b`",
      "select 1 -- ; not a statement\n",
      "select 1 /* ; not a statement */",
      "select 'it''s ; fine'",
    ] {
      assert_eq!(single_statement(sql), Ok(()), "{sql:?} was refused");
    }
  }

  #[test]
  fn nothing_at_all_is_refused_as_empty() {
    for sql in [
      "",
      "   ",
      "-- just a comment\n",
      "/* just a comment */",
      ";",
    ] {
      assert_eq!(
        single_statement(sql),
        Err(GateRefusal::Empty),
        "{sql:?} was not refused as empty"
      );
    }
  }

  /// An unterminated literal is where a scanner and SQLite's parser part
  /// company, so it is refused rather than guessed at.
  #[test]
  fn an_unterminated_literal_or_comment_is_refused() {
    for sql in [
      "select 'abc",
      "select \"abc",
      "select [abc",
      "select 1 /* abc",
    ] {
      assert_eq!(
        single_statement(sql),
        Err(GateRefusal::Unterminated),
        "{sql:?} was not refused"
      );
    }
  }
}
