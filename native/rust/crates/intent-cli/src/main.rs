//! `intent` -- the v3 CLI binary.
//!
//! A thin coordinator and nothing else (AC-05.4, `IN-AG-THIN-COORD-001`):
//! parse, call the intentsvcs facade, render. The command surface is BUILT
//! from the committed dispatch table (AC-05.1), so this file dispatches verbs
//! it never declared.

use std::process::ExitCode;

use intent_cli::{render, spine};

fn main() -> ExitCode {
  let argv: Vec<String> = std::env::args().collect();
  let matches = match spine::parse(argv) {
    Ok(matches) => matches,
    Err(code) => return ExitCode::from(code as u8),
  };
  match render::run(&matches) {
    Ok(()) => ExitCode::from(spine::EXIT_OK as u8),
    Err(failure) => {
      // The gate writes its own verdict to stdout, because it is read by
      // machines via the exit code -- v2 does the same -- so `Verdict` carries
      // no message and printing an empty line would add noise to a contract
      // other tools parse. The CODE comes from the failure rather than from
      // here: this function has no way to tell "the answer is no" from "this
      // build cannot answer", and answering both with 1 is issue 0038.
      if let Some(message) = failure.message() {
        eprintln!("{message}");
      }
      ExitCode::from(failure.code() as u8)
    }
  }
}
