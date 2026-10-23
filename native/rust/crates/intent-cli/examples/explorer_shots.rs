//! explorer_shots -- the screenshots on docs/explorer.md, taken from the
//! explorer itself.
//!
//! `intent explore` runs in a pty, vt100 reads the screen it draws, and each
//! screen is written out as an SVG for rsvg-convert to rasterise. It is driven
//! by `intent/st/ST0056/parity/tools/gen_explorer_shots.sh`, which owns the
//! scratch HOME and seeds the demo projects this reads; run that rather than
//! this.
//!
//! WHY A PTY AND NOT A SCREEN GRAB. A grab needs a window on somebody's desktop,
//! the Screen Recording permission and a person to arrange it, so it is taken
//! once and never again, and the page drifts from the explorer it shows. A pty
//! needs none of them: the size is fixed, the input is scripted, and the same
//! command re-takes every shot at the next cut.
//!
//! WHY AN EXAMPLE OF THIS CRATE AND NOT A CRATE OF ITS OWN. It replaced
//! `explorer_shots.py`, which read the screen with pyte, on hv's rule of
//! 2026-10-23 that no repository commits Python. `bin/devbin build all` builds
//! every binary in the workspace, so a crate of its own would be compiled into
//! every build of the pair and its dependencies' features would join the
//! pair's. An example is compiled only by the test and lint runs that keep it
//! building.
//!
//! THE VERSION AND THE COMMIT THE EXPLORER DRAWS ARE ELIDED AS `...`, the way a
//! transcript in these docs elides them: a shot is published with a release it
//! was not taken on. A shot that still shows either after elision is refused
//! rather than written, so a new place the explorer draws them cannot slip into
//! the docs unnoticed.
//!
//! THE SEED'S DATE IS PINNED (issue 0623). The demo projects are seeded on the
//! day of the run, so every date they carry is that day's, and two runs on
//! different days differed by it alone. Every date passed with `--seed-date` is
//! drawn as [`PINNED_DATE`], so two runs over an unchanged explorer write
//! byte-identical files on any day.
//!
//! Exit codes: 0 every shot written, 1 a shot could not be taken, 2 usage.

use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::num::NonZeroU16;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::time::{Duration, Instant};

const COLS: u16 = 100;
const ROWS: u16 = 30;

/// The date every seed date is drawn as: the date the committed shots carried
/// when the pin arrived, so pinning it moved no published pixel.
const PINNED_DATE: &str = "2026-10-16";

const ENTER: &[u8] = b"\r";
const DOWN: &[u8] = b"\x1b[B";
const SLASH: &[u8] = b"/";
const CTRL_C: &[u8] = b"\x03";

const EXPLORE: &[&str] = &["intent", "explore"];
const EXPLORE_THREAD: &[&str] = &["intent", "explore", "ST0001"];

/// One thing done to the explorer before its screen is read. The steps are
/// keys a reader can press, and nothing else, except `Run`, which is a command
/// run beside the explorer, as a second terminal would.
enum Step {
  Key(&'static [u8]),
  Text(&'static str),
  Wait(Duration),
  Run(&'static [&'static str]),
}

struct Shot {
  name: &'static str,
  argv: &'static [&'static str],
  steps: Vec<Step>,
}

/// A command typed into the omnibox: the slash, the words, the return.
fn command(words: &'static str) -> Vec<Step> {
  vec![Step::Key(SLASH), Step::Text(words), Step::Key(ENTER)]
}

fn shots() -> Vec<Shot> {
  let down = |n: usize| std::iter::repeat_with(|| Step::Key(DOWN)).take(n);
  vec![
    Shot {
      name: "landing",
      argv: EXPLORE,
      steps: vec![],
    },
    Shot {
      name: "find",
      argv: EXPLORE,
      steps: vec![Step::Text("pay")],
    },
    Shot {
      name: "thread",
      argv: EXPLORE_THREAD,
      steps: down(5).collect(),
    },
    Shot {
      name: "criteria",
      argv: EXPLORE_THREAD,
      steps: down(11).chain([Step::Key(ENTER)]).chain(down(3)).collect(),
    },
    Shot {
      name: "edit",
      argv: EXPLORE_THREAD,
      steps: vec![Step::Key(ENTER), Step::Text(" online")],
    },
    Shot {
      name: "menu",
      argv: EXPLORE,
      steps: vec![Step::Key(SLASH)],
    },
    Shot {
      name: "outstanding",
      argv: EXPLORE,
      steps: command("outstanding"),
    },
    Shot {
      name: "search",
      argv: EXPLORE,
      steps: command("search card")
        .into_iter()
        .chain([Step::Wait(Duration::from_millis(1500))])
        .collect(),
    },
    Shot {
      name: "projects",
      argv: EXPLORE,
      steps: command("projects"),
    },
    Shot {
      name: "help",
      argv: EXPLORE,
      steps: command("help"),
    },
    // LAST, because it writes: an issue added from outside while the list is on
    // screen, and no key pressed after it.
    Shot {
      name: "live",
      argv: EXPLORE,
      steps: command("issues")
        .into_iter()
        .chain([
          Step::Run(&[
            "intent",
            "issues",
            "add",
            "The order page times out on a large basket",
            "--severity",
            "high",
          ]),
          Step::Wait(Duration::from_millis(1500)),
        ])
        .collect(),
    },
  ]
}

// -- the terminal ------------------------------------------------------------

/// Answers the two questions the explorer asks as it starts -- where the cursor
/// is, and what kind of terminal this is -- at the point in the stream it asks
/// them, the way a terminal does. The explorer refuses a terminal that never
/// answers. The answers are the ones pyte gave, so the explorer sees the same
/// terminal it was photographed in before.
#[derive(Default)]
struct Answers(Vec<u8>);

impl vt100::Callbacks for Answers {
  fn unhandled_csi(
    &mut self,
    screen: &mut vt100::Screen,
    i1: Option<u8>,
    _i2: Option<u8>,
    params: &[&[u16]],
    c: char,
  ) {
    let first = params.first().and_then(|p| p.first()).copied();
    match (i1, c, first) {
      (None, 'n', Some(6)) => {
        let (row, col) = screen.cursor_position();
        self
          .0
          .extend_from_slice(format!("\x1b[{};{}R", row + 1, col + 1).as_bytes());
      }
      (None, 'c', None | Some(0)) => self.0.extend_from_slice(b"\x1b[?6c"),
      _ => {}
    }
  }
}

/// One explorer, running in a pty of a fixed size.
struct Session {
  child: Child,
  master: std::fs::File,
  parser: vt100::Parser<Answers>,
}

impl Session {
  fn start(argv: &[&str], env: &[(OsString, OsString)], cwd: &Path) -> io::Result<Self> {
    let (master, slave) = open_pty()?;
    let mut cmd = Command::new(argv[0]);
    cmd
      .args(&argv[1..])
      .current_dir(cwd)
      .env_clear()
      .envs(env.iter().map(|(k, v)| (k, v)))
      .stdin(Stdio::from(slave.try_clone()?))
      .stdout(Stdio::from(slave.try_clone()?))
      .stderr(Stdio::from(slave));
    // SAFETY: the closure runs in the child between fork and exec, and calls
    // only setsid(2) and ioctl(2), which are async-signal-safe. Together they
    // make the pty the child's controlling terminal, as a terminal emulator
    // does, so a program that opens /dev/tty reaches this pty.
    unsafe {
      cmd.pre_exec(|| {
        if libc::setsid() == -1 {
          return Err(io::Error::last_os_error());
        }
        if libc::ioctl(0, libc::TIOCSCTTY as _, 0) == -1 {
          return Err(io::Error::last_os_error());
        }
        Ok(())
      });
    }
    let child = cmd.spawn()?;
    // The command holds the parent's copies of the far end; dropping it closes
    // them, so the explorer's exit reaches this end as an end of input.
    drop(cmd);
    let parser =
      vt100::Parser::new_with_callbacks(nonzero(ROWS), nonzero(COLS), 0, Answers::default());
    Ok(Self {
      child,
      master: std::fs::File::from(master),
      parser,
    })
  }

  /// Read what the explorer draws until it has been quiet for `quiet`, or for
  /// `timeout` at most. Each read follows a poll that saw input, so it returns
  /// what is there rather than waiting for more.
  fn pump(&mut self, timeout: Duration, quiet: Duration) -> io::Result<()> {
    let end = Instant::now() + timeout;
    let mut last = Instant::now();
    let mut buf = vec![0u8; 65536];
    while Instant::now() < end {
      if readable(&self.master, Duration::from_millis(50))? {
        match self.master.read(&mut buf) {
          Ok(0) => return Ok(()),
          Ok(n) => {
            self.parser.process(&buf[..n]);
            let answers = std::mem::take(&mut self.parser.callbacks_mut().0);
            if !answers.is_empty() {
              self.master.write_all(&answers)?;
            }
            last = Instant::now();
          }
          Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
          // The explorer has gone and taken the pty's far end with it, which
          // Linux reports as EIO rather than as an end of input.
          Err(e) if e.raw_os_error() == Some(libc::EIO) => return Ok(()),
          Err(e) => return Err(e),
        }
      } else if last.elapsed() >= quiet {
        return Ok(());
      }
    }
    Ok(())
  }

  fn send(&mut self, bytes: &[u8]) -> io::Result<()> {
    self.master.write_all(bytes)
  }

  fn alive(&mut self) -> io::Result<bool> {
    Ok(self.child.try_wait()?.is_none())
  }

  fn step(&mut self, step: &Step, ctx: &Context) -> Result<(), String> {
    match step {
      Step::Key(bytes) => self
        .send(bytes)
        .map_err(|e| format!("could not press a key: {e}"))?,
      Step::Text(text) => self
        .send(text.as_bytes())
        .map_err(|e| format!("could not type `{text}`: {e}"))?,
      Step::Wait(pause) => std::thread::sleep(*pause),
      Step::Run(argv) => {
        let shown = argv.join(" ");
        let output = Command::new(argv[0])
          .args(&argv[1..])
          .current_dir(&ctx.cwd)
          .env_clear()
          .envs(ctx.env.iter().map(|(k, v)| (k, v)))
          .output()
          .map_err(|e| format!("could not run `{shown}`: {e}"))?;
        if !output.status.success() {
          return Err(format!(
            "`{shown}` failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
          ));
        }
      }
    }
    self
      .pump(Duration::from_secs(4), Duration::from_millis(400))
      .map_err(|e| format!("could not read the explorer: {e}"))
  }

  fn shoot(&mut self, shot: &Shot, ctx: &Context) -> Result<(), String> {
    self
      .pump(Duration::from_secs(8), Duration::from_secs(1))
      .map_err(|e| format!("could not read the explorer: {e}"))?;
    for step in &shot.steps {
      self.step(step, ctx)?;
    }
    if !self
      .alive()
      .map_err(|e| format!("could not ask whether the explorer is running: {e}"))?
    {
      return Err(format!(
        "the explorer exited before `{}` could be read",
        shot.name
      ));
    }
    let mut grid = cells(self.parser.screen());
    elide(&mut grid);
    pin_dates(&mut grid, &ctx.seed_dates);
    let shown = grid
      .iter()
      .map(|line| text_of(line))
      .collect::<Vec<_>>()
      .join("\n");
    if shown.trim().is_empty() {
      return Err(format!("`{}` read a blank screen", shot.name));
    }
    let commit: String = ctx.commit.chars().take(7).collect();
    for (label, needle) in [
      ("version", ctx.version.as_str()),
      ("commit", commit.as_str()),
    ] {
      if !needle.is_empty() && shown.contains(needle) {
        return Err(format!(
          "`{}` still shows the {label} `{needle}` after elision -- add a pattern to ELIDE for where the explorer draws it",
          shot.name
        ));
      }
    }
    let path = ctx.out.join(format!("{}.svg", shot.name));
    std::fs::write(&path, svg(&grid))
      .map_err(|e| format!("could not write {}: {e}", path.display()))
  }

  /// End the explorer: CTRL-C, as the operator would, then a kill for one that
  /// ignores it. One that has already gone is the expected case, not an error.
  fn close(mut self) -> io::Result<()> {
    if self.alive()? {
      match self.send(CTRL_C) {
        Ok(()) => self.pump(Duration::from_secs(1), Duration::from_millis(400))?,
        // It went between the question and the key, taking the far end with it.
        Err(e) if e.raw_os_error() == Some(libc::EIO) => {}
        Err(e) => return Err(e),
      }
    }
    if self.alive()? {
      self.child.kill()?;
    }
    self.child.wait()?;
    Ok(())
  }
}

fn nonzero(n: u16) -> NonZeroU16 {
  NonZeroU16::new(n).unwrap_or(NonZeroU16::MIN)
}

fn open_pty() -> io::Result<(OwnedFd, OwnedFd)> {
  let mut master: libc::c_int = -1;
  let mut slave: libc::c_int = -1;
  let mut size = libc::winsize {
    ws_row: ROWS,
    ws_col: COLS,
    ws_xpixel: 0,
    ws_ypixel: 0,
  };
  // SAFETY: openpty(3) writes two descriptors through the two pointers, which
  // point at live locals, and reads the window size; the name and the termios
  // are null, which it accepts. The termios is typed because Linux declares it
  // `*const` and macOS `*mut`, and only a typed null coerces to both.
  let rc = unsafe {
    libc::openpty(
      &mut master,
      &mut slave,
      std::ptr::null_mut(),
      std::ptr::null_mut::<libc::termios>(),
      &mut size,
    )
  };
  if rc != 0 {
    return Err(io::Error::last_os_error());
  }
  // SAFETY: openpty succeeded, so both are open descriptors this process owns
  // and nothing else holds.
  let (master, slave) = unsafe { (OwnedFd::from_raw_fd(master), OwnedFd::from_raw_fd(slave)) };
  // Neither end may outlive the exec. The explorer gets the far end as its
  // stdin, stdout and stderr; a copy it, or a daemon it starts, inherited
  // under another number would hold the pty open after it exits, which
  // pty.fork never let the Python taker's children do.
  set_cloexec(&master)?;
  set_cloexec(&slave)?;
  Ok((master, slave))
}

fn set_cloexec(fd: &OwnedFd) -> io::Result<()> {
  // SAFETY: fcntl(2) on a descriptor this process owns, with integer arguments.
  let flags = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_GETFD) };
  // SAFETY: as above.
  if flags == -1
    || unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_SETFD, flags | libc::FD_CLOEXEC) } == -1
  {
    return Err(io::Error::last_os_error());
  }
  Ok(())
}

fn readable(file: &std::fs::File, wait: Duration) -> io::Result<bool> {
  let mut fds = libc::pollfd {
    fd: file.as_raw_fd(),
    events: libc::POLLIN,
    revents: 0,
  };
  let ms = libc::c_int::try_from(wait.as_millis()).unwrap_or(libc::c_int::MAX);
  // SAFETY: poll(2) reads and writes one pollfd, which is a live local.
  let rc = unsafe { libc::poll(&mut fds, 1, ms) };
  if rc < 0 {
    let e = io::Error::last_os_error();
    if e.kind() == io::ErrorKind::Interrupted {
      return Ok(false);
    }
    return Err(e);
  }
  Ok(rc > 0)
}

// -- the screen as cells -----------------------------------------------------

/// The page's own colours: the text, and the background the SVG is drawn on.
const FG: &str = "#d4d4d4";
const BG: &str = "#1e1e1e";

/// The sixteen ANSI colours, keyed by xterm's value for each, as each is drawn
/// on a dark page. An xterm value for an index past these, or an RGB colour,
/// is drawn as itself unless it equals one of these keys.
const PALETTE: [(&str, &str); 16] = [
  ("000000", "#000000"),
  ("cd0000", "#cd3131"),
  ("00cd00", "#0dbc79"),
  ("cdcd00", "#e5e510"),
  ("0000ee", "#2472c8"),
  ("cd00cd", "#bc3fbc"),
  ("00cdcd", "#11a8cd"),
  ("e5e5e5", "#e5e5e5"),
  ("7f7f7f", "#808080"),
  ("ff0000", "#f14c4c"),
  ("00ff00", "#23d18b"),
  ("ffff00", "#f5f543"),
  ("5c5cff", "#3b8eea"),
  ("ff00ff", "#d670d6"),
  ("00ffff", "#29b8db"),
  ("ffffff", "#ffffff"),
];

/// xterm's value for a 256-colour index: the sixteen, the 6x6x6 cube, then the
/// greys.
fn xterm(index: u8) -> String {
  match index {
    0..=15 => PALETTE[usize::from(index)].0.to_string(),
    16..=231 => {
      let n = index - 16;
      let level = |v: u8| if v == 0 { 0 } else { 55 + 40 * v };
      format!(
        "{:02x}{:02x}{:02x}",
        level(n / 36),
        level(n / 6 % 6),
        level(n % 6)
      )
    }
    _ => {
      let grey = 8 + 10 * (index - 232);
      format!("{grey:02x}{grey:02x}{grey:02x}")
    }
  }
}

fn colour(c: vt100::Color, default: &str) -> String {
  let hex = match c {
    vt100::Color::Default => return default.to_string(),
    vt100::Color::Idx(index) => xterm(index),
    vt100::Color::Rgb(r, g, b) => format!("{r:02x}{g:02x}{b:02x}"),
  };
  PALETTE
    .iter()
    .find(|(key, _)| *key == hex)
    .map_or_else(|| format!("#{hex}"), |(_, drawn)| (*drawn).to_string())
}

#[derive(Clone, PartialEq)]
struct Style {
  fg: String,
  bg: String,
  bold: bool,
  italic: bool,
  underline: bool,
}

/// One drawn character and the columns it takes: a wide glyph takes two.
#[derive(Clone)]
struct Glyph {
  text: String,
  style: Style,
  width: u16,
}

fn cells(screen: &vt100::Screen) -> Vec<Vec<Glyph>> {
  (0..ROWS)
    .map(|y| {
      let mut line = Vec::new();
      let mut x = 0;
      while x < COLS {
        let Some(cell) = screen.cell(y, x) else { break };
        let width = if cell.is_wide() { 2 } else { 1 };
        let (mut fg, mut bg) = (colour(cell.fgcolor(), FG), colour(cell.bgcolor(), BG));
        if cell.inverse() {
          std::mem::swap(&mut fg, &mut bg);
        }
        let text = if cell.contents().is_empty() {
          " "
        } else {
          cell.contents()
        };
        let style = Style {
          fg,
          bg,
          bold: cell.bold(),
          italic: cell.italic(),
          underline: cell.underline(),
        };
        line.push(Glyph {
          text: text.to_string(),
          style,
          width,
        });
        x += width;
      }
      line
    })
    .collect()
}

fn text_of(line: &[Glyph]) -> String {
  line.iter().map(|g| g.text.as_str()).collect()
}

/// Where a line names the build -- `Intent <version> (<commit>)`, with the
/// turtle and its space before it when `turtle` -- as the glyphs it spans.
fn find_build(line: &[Glyph], turtle: bool) -> Option<(usize, usize)> {
  let text: Vec<&str> = line.iter().map(|g| g.text.as_str()).collect();
  let is = |i: usize, s: &str| text.get(i) == Some(&s);
  let run = |i: usize, ok: fn(&str) -> bool| {
    text[i.min(text.len())..]
      .iter()
      .take_while(|s| ok(s))
      .count()
  };
  (0..text.len()).find_map(|start| {
    let mut i = start;
    if turtle {
      if !(is(i, "🐢") && is(i + 1, " ")) {
        return None;
      }
      i += 2;
    }
    for ch in ["I", "n", "t", "e", "n", "t", " "] {
      if !is(i, ch) {
        return None;
      }
      i += 1;
    }
    for part in 0..3 {
      let digits = run(i, |s| s.len() == 1 && s.as_bytes()[0].is_ascii_digit());
      if digits == 0 {
        return None;
      }
      i += digits;
      if part < 2 {
        if !is(i, ".") {
          return None;
        }
        i += 1;
      }
    }
    i += run(i, |s| s != " ");
    if !(is(i, " ") && is(i + 1, "(")) {
      return None;
    }
    i += 2;
    let hex = run(i, |s| {
      s.len() == 1 && matches!(s.as_bytes()[0], b'0'..=b'9' | b'a'..=b'f')
    });
    if !(7..=40).contains(&hex) || !is(i + hex, ")") {
      return None;
    }
    Some((start, i + hex + 1))
  })
}

/// Rewrite where the explorer names the build as `...`, right-aligned inside
/// the columns the name took, so nothing after it moves. The banner with its
/// turtle is tried first, then the name alone, once each per line.
fn elide(grid: &mut [Vec<Glyph>]) {
  for line in grid.iter_mut() {
    for turtle in [true, false] {
      let Some((start, end)) = find_build(line, turtle) else {
        continue;
      };
      let taken: u16 = line[start..end].iter().map(|g| g.width).sum();
      let style = line[end - 1].style.clone();
      let mut new: Vec<Glyph> = Vec::new();
      if turtle {
        new.push(Glyph {
          style: style.clone(),
          ..line[start].clone()
        });
      }
      let words = if turtle {
        " Intent ... (...)"
      } else {
        "Intent ... (...)"
      };
      new.extend(words.chars().map(|ch| Glyph {
        text: ch.to_string(),
        style: style.clone(),
        width: 1,
      }));
      let pad = taken.saturating_sub(new.iter().map(|g| g.width).sum());
      let filler = Glyph {
        text: " ".to_string(),
        style: line[start].style.clone(),
        width: 1,
      };
      let rebuilt: Vec<Glyph> = line[..start]
        .iter()
        .cloned()
        .chain(std::iter::repeat_n(filler, usize::from(pad)))
        .chain(new)
        .chain(line[end..].iter().cloned())
        .collect();
      *line = rebuilt;
    }
  }
}

/// Draw each seed date wherever it appears as [`PINNED_DATE`]. A date and its
/// replacement are the same width, so nothing moves.
fn pin_dates(grid: &mut [Vec<Glyph>], seed_dates: &[String]) {
  for line in grid.iter_mut() {
    for date in seed_dates {
      let want: Vec<String> = date.chars().map(String::from).collect();
      let mut i = 0;
      while i + want.len() <= line.len() {
        if line[i..i + want.len()]
          .iter()
          .zip(&want)
          .all(|(g, w)| g.text == *w)
        {
          for (glyph, ch) in line[i..i + want.len()].iter_mut().zip(PINNED_DATE.chars()) {
            glyph.text = ch.to_string();
          }
          i += want.len();
        } else {
          i += 1;
        }
      }
    }
  }
}

// -- the SVG -----------------------------------------------------------------

const FONT: &str = "Menlo, Monaco, 'DejaVu Sans Mono', monospace";
const SIZE: f64 = 14.0;
const CW: f64 = 8.43;
const CH: f64 = 17.0;
const PAD: f64 = 18.0;

fn escape(text: &str) -> String {
  text
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
}

/// Consecutive glyphs of one style, drawn as one text element: each glyph's
/// column, text and width.
struct Run<'a> {
  chars: Vec<(u16, &'a str, u16)>,
  style: &'a Style,
}

fn svg(grid: &[Vec<Glyph>]) -> String {
  let width = PAD * 2.0 + f64::from(COLS) * CW;
  let height = PAD * 2.0 + f64::from(ROWS) * CH;
  let mut out = vec![
    format!(
      r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width:.0}" height="{height:.0}" viewBox="0 0 {width:.2} {height:.2}">"#
    ),
    format!(r#"<rect width="100%" height="100%" rx="10" fill="{BG}"/>"#),
    format!(r#"<g font-family="{FONT}" font-size="{SIZE:.1}">"#),
  ];
  for (line, y) in grid.iter().zip(0u16..) {
    let top = PAD + f64::from(y) * CH;
    let mut runs: Vec<Run> = Vec::new();
    let mut x = 0;
    for glyph in line {
      match runs.last_mut() {
        Some(run) if *run.style == glyph.style => {
          run.chars.push((x, glyph.text.as_str(), glyph.width))
        }
        _ => runs.push(Run {
          chars: vec![(x, glyph.text.as_str(), glyph.width)],
          style: &glyph.style,
        }),
      }
      x += glyph.width;
    }
    for Run { chars, style } in &runs {
      let x0 = chars.first().map_or(0, |c| c.0);
      let span: u16 = chars.iter().map(|c| c.2).sum();
      if style.bg != BG {
        out.push(format!(
          r#"<rect x="{:.2}" y="{top:.2}" width="{:.2}" height="{CH:.2}" fill="{}"/>"#,
          PAD + f64::from(x0) * CW,
          f64::from(span) * CW,
          style.bg
        ));
      }
      if chars.iter().all(|c| c.1 == " ") {
        continue;
      }
      let xs = chars
        .iter()
        .map(|c| format!("{:.2}", PAD + f64::from(c.0) * CW))
        .collect::<Vec<_>>()
        .join(" ");
      let body = escape(&chars.iter().map(|c| c.1).collect::<String>());
      let mut attrs = format!(r#"fill="{}""#, style.fg);
      if style.bold {
        attrs.push_str(r#" font-weight="bold""#);
      }
      if style.italic {
        attrs.push_str(r#" font-style="italic""#);
      }
      if style.underline {
        attrs.push_str(r#" text-decoration="underline""#);
      }
      out.push(format!(
        r#"<text x="{xs}" y="{:.2}" {attrs} xml:space="preserve">{body}</text>"#,
        top + CH * 0.76
      ));
    }
  }
  out.push("</g>".to_string());
  out.push("</svg>".to_string());
  out.join("\n")
}

// -- the run -----------------------------------------------------------------

struct Context {
  cwd: PathBuf,
  out: PathBuf,
  version: String,
  commit: String,
  only: Vec<String>,
  seed_dates: Vec<String>,
  env: Vec<(OsString, OsString)>,
}

const USAGE: &str = "usage: explorer_shots --cwd <dir> --out <dir> --version <v> --commit <sha> [--only <a,b>] [--seed-date <YYYY-MM-DD>]...";

fn is_date(s: &str) -> bool {
  let b = s.as_bytes();
  b.len() == 10
    && b.iter().enumerate().all(|(i, c)| {
      if i == 4 || i == 7 {
        *c == b'-'
      } else {
        c.is_ascii_digit()
      }
    })
}

fn required<T>(value: Option<T>, flag: &str) -> Result<T, String> {
  value.ok_or_else(|| format!("`{flag}` is required\n{USAGE}"))
}

fn parse(mut args: impl Iterator<Item = String>) -> Result<Context, String> {
  let (mut cwd, mut out, mut version, mut commit) = (None, None, None, None);
  let (mut only, mut seed_dates) = (Vec::new(), Vec::new());
  while let Some(flag) = args.next() {
    let mut value = || {
      args
        .next()
        .ok_or_else(|| format!("`{flag}` needs a value\n{USAGE}"))
    };
    match flag.as_str() {
      "--cwd" => cwd = Some(PathBuf::from(value()?)),
      "--out" => out = Some(PathBuf::from(value()?)),
      "--version" => version = Some(value()?),
      "--commit" => commit = Some(value()?),
      "--only" => {
        only = value()?
          .split(',')
          .filter(|s| !s.is_empty())
          .map(String::from)
          .collect()
      }
      "--seed-date" => {
        let date = value()?;
        if !is_date(&date) {
          return Err(format!(
            "`--seed-date {date}` is not a YYYY-MM-DD date\n{USAGE}"
          ));
        }
        seed_dates.push(date);
      }
      "-h" | "--help" => return Err(USAGE.to_string()),
      other => return Err(format!("unknown argument `{other}`\n{USAGE}")),
    }
  }
  // The terminal the explorer is told it is in, over whatever the caller's is.
  const TERMINAL: [(&str, &str); 3] = [
    ("TERM", "xterm-256color"),
    ("COLORTERM", "truecolor"),
    ("LANG", "en_US.UTF-8"),
  ];
  let mut env: Vec<(OsString, OsString)> = std::env::vars_os()
    .filter(|(k, _)| {
      !TERMINAL
        .iter()
        .any(|(name, _)| k.to_string_lossy() == *name)
    })
    .collect();
  env.extend(TERMINAL.map(|(k, v)| (k.into(), v.into())));
  Ok(Context {
    cwd: required(cwd, "--cwd")?,
    out: required(out, "--out")?,
    version: required(version, "--version")?,
    commit: required(commit, "--commit")?,
    only,
    seed_dates,
    env,
  })
}

fn take(shot: &Shot, ctx: &Context) -> Result<(), String> {
  let mut session = Session::start(shot.argv, &ctx.env, &ctx.cwd)
    .map_err(|e| format!("could not start the explorer for `{}`: {e}", shot.name))?;
  let taken = session.shoot(shot, ctx);
  let closed = session
    .close()
    .map_err(|e| format!("could not end the explorer after `{}`: {e}", shot.name));
  taken.and(closed)
}

fn main() -> ExitCode {
  let ctx = match parse(std::env::args().skip(1)) {
    Ok(ctx) => ctx,
    Err(why) => {
      eprintln!("error: {why}");
      return ExitCode::from(2);
    }
  };
  let shots = shots();
  let unknown: Vec<&str> = ctx
    .only
    .iter()
    .filter(|name| !shots.iter().any(|s| s.name == *name))
    .map(String::as_str)
    .collect();
  if !unknown.is_empty() {
    eprintln!("error: no shot named {}", unknown.join(", "));
    return ExitCode::from(2);
  }
  for shot in shots
    .iter()
    .filter(|s| ctx.only.is_empty() || ctx.only.iter().any(|name| name == s.name))
  {
    if let Err(why) = take(shot, &ctx) {
      eprintln!("error: {why}");
      return ExitCode::from(1);
    }
    println!("shot: {}", shot.name);
  }
  ExitCode::SUCCESS
}
