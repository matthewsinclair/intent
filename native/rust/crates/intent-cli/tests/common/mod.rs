//! Helpers shared by the integration tests in this crate.
//!
//! **A DIRECTORY, NOT `tests/common.rs`.** Cargo compiles every `.rs` FILE
//! directly under `tests/` as its own test binary; a directory module is not
//! one, so this is the spelling that shares code without inventing an empty
//! test target.
//!
//! **AND NOT `testkit`, WHICH WOULD HAVE BEEN THE OBVIOUS HOME.** That crate
//! declares zero dependencies on purpose -- its own manifest says a testkit
//! with a dependency graph becomes a thing to reason about rather than a thing
//! to reach for -- and `dep_graph_guard.rs` walks every manifest under
//! `crates/`. `openpty` needs `libc`, which is already a dev-dependency HERE
//! and would be a new one THERE. Both callers of the harness below live in
//! this crate, so the narrower home is also the correct one.

#![allow(dead_code)]

/// A connected pseudo-terminal pair, as owned files.
///
/// **THE MASTER MUST OUTLIVE THE CHILD.** Dropping it closes the terminal's
/// other end, and the child then reads EOF -- or takes a hangup -- in the
/// middle of whatever it was doing. Every caller keeps it in scope for the
/// whole run, which is why this returns it rather than using it itself.
///
/// **IT IS THE ONLY WAY TO REACH A TERMINAL-GATED ARM FROM A TEST, AND THE
/// ALTERNATIVE WAS TRIED AND FAILED.** `script -q /dev/null` allocates a pty
/// from a shell, but it calls `tcgetattr` on ITS OWN stdin -- so under a
/// harness whose stdin is a socket or a pipe it exits 1 with `Operation not
/// supported on socket` and produces nothing. Two nodes measured that
/// independently on 2026-08-29, one of them after reporting the opposite from a
/// run that did not reproduce. `openpty` asks the kernel directly and does not
/// care what the ambient stdin is, so it behaves the same under a terminal,
/// under a test harness and under CI.
pub fn pty_pair() -> (std::fs::File, std::fs::File) {
  use std::os::fd::FromRawFd;
  let mut master: libc::c_int = 0;
  let mut slave: libc::c_int = 0;
  // SAFETY: both out-parameters are valid for writes, and the three null
  // pointers are documented as "use the defaults" for termios, winsize and the
  // returned slave name.
  let rc = unsafe {
    libc::openpty(
      &mut master,
      &mut slave,
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
    )
  };
  assert_eq!(
    rc,
    0,
    "openpty failed, so this test can say nothing about the arm it exists for: {}",
    std::io::Error::last_os_error()
  );
  // SAFETY: openpty returned 0, so both descriptors are open and owned by us.
  unsafe {
    (
      std::fs::File::from_raw_fd(master),
      std::fs::File::from_raw_fd(slave),
    )
  }
}

/// Everything the child wrote to the terminal, read after it has exited.
///
/// **`EIO` IS THE END OF THE STREAM HERE, NOT A FAILURE.** A pty master whose
/// every slave descriptor has closed returns `EIO` rather than a clean zero on
/// Darwin, so a reader that treats an error as a fault reports one on every
/// successful run. Anything else is propagated by panicking, because a test
/// that cannot read the terminal must not quietly return "nothing was written"
/// -- which is exactly the assertion its caller is about to make.
pub fn drain(mut master: std::fs::File) -> String {
  use std::io::Read;
  let mut out = Vec::new();
  let mut buf = [0u8; 4096];
  loop {
    match master.read(&mut buf) {
      Ok(0) => break,
      Ok(n) => out.extend_from_slice(&buf[..n]),
      Err(e) if e.raw_os_error() == Some(libc::EIO) => break,
      Err(e) => panic!(
        "reading the terminal failed, so this test can assert nothing about what was written to it: {e}"
      ),
    }
  }
  // The line discipline turns a bare newline into CRLF on the way out. Callers
  // compare against paths they built themselves, which carry neither.
  String::from_utf8_lossy(&out).replace('\r', "")
}
