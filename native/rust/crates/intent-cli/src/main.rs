//! `intent` -- the v3 CLI binary.
//!
//! A thin coordinator and nothing else (AC-05.4, `IN-AG-THIN-COORD-001`):
//! parse, call the intentsvcs facade, render. The command surface is BUILT
//! from the committed dispatch table (AC-05.1), so this file dispatches verbs
//! it never declared.

use std::process::ExitCode;

fn main() -> ExitCode {
  // **THE SEQUENCE ITSELF IS [`intent_cli::dispatch`], NOT HERE.** The
  // explorer's `/{cmd} ...` runs commands too, and two copies of parse-then-run
  // would be two homes for one thing. What is left in this function is what
  // only a process can do: write to stderr and exit.
  let outcome = intent_cli::dispatch(std::env::args().collect());
  // The gate writes its own verdict to stdout, because it is read by machines
  // via the exit code -- v2 does the same -- so `Verdict` carries no message and
  // printing an empty line would add noise to a contract other tools parse. The
  // CODE comes from the failure rather than from here: this function has no way
  // to tell "the answer is no" from "this build cannot answer", and answering
  // both with 1 is issue 0038.
  if let Some(message) = outcome.message {
    eprintln!("{message}");
  }
  ExitCode::from(outcome.code as u8)
}
