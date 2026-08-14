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
    Err(message) => {
      // The gate writes its own verdict to stdout and returns an empty
      // message, because it is read by machines via the exit code -- v2 does
      // the same. Printing an empty line there would add noise to a contract
      // that other tools parse.
      if !message.is_empty() {
        eprintln!("{message}");
      }
      ExitCode::from(spine::EXIT_ERROR as u8)
    }
  }
}
