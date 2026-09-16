//! `intentd`'s two logs read as one list (issue 0425).
//!
//! **ONE LIST, MERGED BY EACH LINE'S STAMP** (hv, 2026-09-16, "ok: a"). `intent
//! daemon logs` printed the last lines of `intentd.log` and then the last lines
//! of `intentd.err.log`, so an error older than the newest start line was
//! printed below it and read as if it came after. Every line `intentd` writes
//! opens with a UTC stamp (issue 0321, `intentd::daemon_log`), and that stamp
//! orders the two files into one backlog.
//!
//! **THE MERGE IS PURE AND THE READ IS NOT, AND THEY ARE SPLIT** (PFIC): [`merged`]
//! takes the two files' lines and a count and does no I/O, so the ordering is
//! driven without a file; [`backlog`] reads the files and hands their lines in.

use std::path::Path;

/// Where following resumes in one log: the byte after the last whole line the
/// backlog read, one-based as `tail -c +N` counts. A log with no whole line
/// resumes at its start.
pub type Resume = u64;

/// The merged backlog of both logs, and where following resumes in each.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backlog {
  pub lines: Vec<String>,
  pub resume: [Resume; 2],
}

/// The stamp a line opens with: `YYYY-MM-DDTHH:MM:SS.mmmZ`, the one shape
/// `intentd::daemon_log` writes, followed by a space or the end of the line.
///
/// **FIXED WIDTH, SO STAMPS ORDER AS STRINGS.** A line that only resembles a
/// stamp is not one, which is what keeps a message that happens to start with a
/// date from being moved.
fn stamp(line: &str) -> Option<&str> {
  const SHAPE: &[u8] = b"dddd-dd-ddTdd:dd:dd.dddZ";
  let bytes = line.as_bytes();
  if bytes.len() < SHAPE.len() {
    return None;
  }
  let fits = SHAPE.iter().zip(bytes).all(|(want, got)| match want {
    b'd' => got.is_ascii_digit(),
    other => got == other,
  });
  let ends = bytes.get(SHAPE.len()).is_none_or(|b| *b == b' ');
  (fits && ends).then(|| &line[..SHAPE.len()])
}

/// The last `n` lines of the two logs as one list, ordered by stamp.
///
/// **A LINE WITH NO STAMP STAYS UNDER THE LINE ABOVE IT IN ITS OWN FILE**: it
/// takes that line's stamp, so a record written before stamps began, or a
/// continuation, is never split from its head. Lines above a file's first stamp
/// sort before every stamped line, and equal stamps keep file order, stdout's
/// log first, because the sort is stable.
pub fn merged(out: &[String], err: &[String], n: usize) -> Vec<String> {
  let mut keyed: Vec<(Option<&str>, &String)> = Vec::new();
  for file in [out, err] {
    let mut last = None;
    let keys: Vec<(Option<&str>, &String)> = file
      .iter()
      .map(|line| {
        last = stamp(line).or(last);
        (last, line)
      })
      .collect();
    let from = keys.len().saturating_sub(n);
    keyed.extend_from_slice(&keys[from..]);
  }
  keyed.sort_by(|a, b| a.0.cmp(&b.0));
  let from = keyed.len().saturating_sub(n);
  keyed[from..]
    .iter()
    .map(|(_, line)| (*line).clone())
    .collect()
}

/// A log's whole lines, and where following resumes after them. An absent log
/// has none and resumes at its start, so a follow prints it whole once it is
/// written.
///
/// **A LAST LINE WITH NO NEWLINE IS LEFT FOR THE FOLLOW**, which prints it whole
/// once it is finished rather than the backlog printing half of it.
fn whole_lines(path: &Path) -> std::io::Result<(Vec<String>, Resume)> {
  let bytes = match std::fs::read(path) {
    Ok(bytes) => bytes,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), 1)),
    Err(e) => return Err(e),
  };
  let whole = bytes
    .iter()
    .rposition(|b| *b == b'\n')
    .map_or(0, |at| at + 1);
  let lines = String::from_utf8_lossy(&bytes[..whole])
    .lines()
    .map(str::to_string)
    .collect();
  Ok((lines, whole as u64 + 1))
}

/// Read both logs and merge their last `n` lines: `logs` is stdout's log, then
/// stderr's.
pub fn backlog(logs: &[std::path::PathBuf; 2], n: usize) -> std::io::Result<Backlog> {
  let (out, out_resume) = whole_lines(&logs[0])?;
  let (err, err_resume) = whole_lines(&logs[1])?;
  Ok(Backlog {
    lines: merged(&out, &err, n),
    resume: [out_resume, err_resume],
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  fn lines(text: &[&str]) -> Vec<String> {
    text.iter().map(|s| s.to_string()).collect()
  }

  /// Issue 0425: an error older than a newer start line is printed above
  /// it, where it happened, not below it because it is in the other file.
  #[test]
  fn an_older_error_sorts_above_a_newer_log_line() {
    let out = lines(&[
      "2026-09-16T18:21:50.659Z intentd stopping: asked over the wire",
      "2026-09-16T19:12:42.909Z intentd listening on intentd.sock",
    ]);
    let err = lines(&["2026-09-16T16:55:02.160Z warning: intentd: ingesting failed"]);
    assert_eq!(
      merged(&out, &err, 40),
      lines(&[
        "2026-09-16T16:55:02.160Z warning: intentd: ingesting failed",
        "2026-09-16T18:21:50.659Z intentd stopping: asked over the wire",
        "2026-09-16T19:12:42.909Z intentd listening on intentd.sock",
      ])
    );
  }

  /// Issue 0425: a line with no stamp stays under the stamped line above
  /// it in its own file, and a newer line from the other file does not come
  /// between them.
  #[test]
  fn an_unstamped_line_keeps_its_place_under_its_stamped_predecessor() {
    let out = lines(&[
      "2026-09-16T10:00:00.000Z intentd listening",
      "2026-09-16T12:00:00.000Z intentd stopping",
    ]);
    let err = lines(&[
      "2026-09-16T11:00:00.000Z error: could not read the committed canon",
      "  caused by: database is locked",
      "  remedy: retry",
    ]);
    assert_eq!(
      merged(&out, &err, 40),
      lines(&[
        "2026-09-16T10:00:00.000Z intentd listening",
        "2026-09-16T11:00:00.000Z error: could not read the committed canon",
        "  caused by: database is locked",
        "  remedy: retry",
        "2026-09-16T12:00:00.000Z intentd stopping",
      ])
    );
    let before = lines(&[
      "written before stamps began",
      "2026-09-16T09:00:00.000Z first stamped",
    ]);
    assert_eq!(
      merged(&out, &before, 40)[..2],
      lines(&[
        "written before stamps began",
        "2026-09-16T09:00:00.000Z first stamped"
      ])[..],
      "lines above a file's first stamp did not sort first"
    );
  }

  /// Issue 0425: `n` counts lines of the merged list, not of each file.
  #[test]
  fn n_counts_merged_lines() {
    let out = lines(&[
      "2026-09-16T10:00:00.000Z out 1",
      "2026-09-16T10:00:02.000Z out 2",
      "2026-09-16T10:00:04.000Z out 3",
    ]);
    let err = lines(&[
      "2026-09-16T10:00:01.000Z err 1",
      "2026-09-16T10:00:03.000Z err 2",
      "2026-09-16T10:00:05.000Z err 3",
    ]);
    assert_eq!(
      merged(&out, &err, 3),
      lines(&[
        "2026-09-16T10:00:03.000Z err 2",
        "2026-09-16T10:00:04.000Z out 3",
        "2026-09-16T10:00:05.000Z err 3",
      ])
    );
    assert!(merged(&out, &err, 0).is_empty());
  }

  #[test]
  fn only_the_written_shape_is_a_stamp() {
    assert_eq!(
      stamp("2026-09-16T10:00:00.000Z x"),
      Some("2026-09-16T10:00:00.000Z")
    );
    assert_eq!(
      stamp("2026-09-16T10:00:00.000Z"),
      Some("2026-09-16T10:00:00.000Z")
    );
    for not in [
      "2026-09-16 was a Wednesday",
      "2026-09-16T10:00:00Z x",
      "  caused by: x",
      "",
    ] {
      assert_eq!(stamp(not), None, "{not:?}");
    }
  }

  #[test]
  fn a_log_resumes_after_its_last_whole_line() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("intentd.log");
    std::fs::write(&path, "a\nb\npart").expect("write");
    assert_eq!(
      whole_lines(&path).expect("read"),
      (lines(&["a", "b"]), 5),
      "the unfinished line was read, or the resume is not the byte after `b\\n`"
    );
    assert_eq!(
      whole_lines(&dir.path().join("absent.log")).expect("absent is not an error"),
      (Vec::new(), 1)
    );
  }
}
