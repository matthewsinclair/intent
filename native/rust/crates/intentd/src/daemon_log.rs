//! The daemon log's one writer: every line `intentd` writes to `intentd.log`
//! (its stdout) and to `intentd.err.log` (its stderr) goes through here.
//!
//! **EVERY LINE OPENS WITH A UTC TIME READ FROM THE CLOCK AT THE WRITE, THEN ONE
//! SPACE, THEN THE LINE AS IT WAS** (issue `0321`). The log used to alternate
//! `intentd listening on ...` and `intentd stopping: ...` with no time on any
//! line, so which stop a restart followed, and whether a sweep was looking at a
//! deliberate restart or a crash and respawn, could be told from the wording of
//! one line and nothing else. **The text after the stamp is unchanged because a
//! reader classifies a line by it**: the Console skips the stamp and colours
//! what follows.
//!
//! **THIS IS THE ONE FILE IN THE WORKSPACE THAT ASKS WHAT TIME IT IS, AND IT DOES
//! SO BY RULING** (hv, 2026-09-15). The reason is recorded once, at ST0056's
//! D42: the daemon log is an operational stream that must write when the store
//! cannot, so its lines cannot wait for a record to be stamped.
//! `intentsvcs/tests/one_clock.rs` exempts this file and no other, so a clock
//! read anywhere else still fails the build.
//!
//! **A NOTICE THAT SPANS LINES IS ONE RECORD WITH ONE READING OF THE CLOCK**, and
//! every line of it carries that stamp, a `remedy:` line included, so a line
//! grepped out of the log still says when it was written.
//!
//! `intentd --version` and `intentd --help` are answers to the person who typed
//! them rather than log lines, and `main.rs` prints them unstamped.

use std::fmt;

/// One record to stdout, which `intent daemon start` and the LaunchAgent both
/// send to `intentd.log`. Takes what `println!` takes.
macro_rules! logln {
  ($($arg:tt)*) => {
    $crate::daemon_log::out(format_args!($($arg)*))
  };
}

/// One record to stderr, which goes to `intentd.err.log`. Takes what
/// `eprintln!` takes.
macro_rules! elogln {
  ($($arg:tt)*) => {
    $crate::daemon_log::err(format_args!($($arg)*))
  };
}

pub(crate) use {elogln, logln};

/// Write one record to stdout.
///
/// **`print!` RATHER THAN A WRITE OF ITS OWN, SO NOTHING ABOUT HOW A LINE REACHES
/// THE FILE CHANGES BUT THE STAMP IN FRONT OF IT**: one lock for the whole
/// record, the same line buffering, and the same panic on a failed write that
/// `println!` had.
pub(crate) fn out(text: fmt::Arguments<'_>) {
  print!("{}", stamped(&text.to_string()));
}

/// Write one record to stderr, with `eprint!`'s behaviour for the same reason.
pub(crate) fn err(text: fmt::Arguments<'_>) {
  eprint!("{}", stamped(&text.to_string()));
}

/// Every line of `text` behind one reading of the clock and one space, each
/// ending in a newline.
///
/// **THE CLOCK IS READ HERE AND NEVER HANDED IN.** D42's signature form holds in
/// the one file allowed a clock as it does everywhere else: no function takes a
/// time, so no caller can supply one (`intentsvcs/tests/no_function_takes_a_time.rs`).
fn stamped(text: &str) -> String {
  let now = utc_now();
  let mut written = String::with_capacity(text.len() + now.len() + 2);
  for line in text.split('\n') {
    written.push_str(&now);
    written.push(' ');
    written.push_str(line);
    written.push('\n');
  }
  written
}

/// Now, in UTC, as `YYYY-MM-DDTHH:MM:SS.mmmZ`.
///
/// RFC 3339, in the shape the store gives its record timestamps
/// (`strftime('%Y-%m-%dT%H:%M:%fZ')`), so a log line and a row written in the
/// same second read alike. Built from the parts rather than through a format
/// description, so there is no formatting error to handle on the way to the log.
fn utc_now() -> String {
  let now = time::OffsetDateTime::now_utc();
  format!(
    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
    now.year(),
    u8::from(now.month()),
    now.day(),
    now.hour(),
    now.minute(),
    now.second(),
    now.millisecond()
  )
}
