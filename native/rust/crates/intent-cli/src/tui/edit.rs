//! The prose handoff: `AT-17.10` covering `AC-17.10`.
//!
//! **THE SEQUENCE IS THE SUBJECT, AND IT IS PROVABLE WITH NO STORE, NO
//! FILESYSTEM AND NO CHILD PROCESS.** Same discipline as [`super::terminal`]
//! and [`super::mode`]: the two outside things a handoff touches are declared
//! as traits, so the properties below are statements about an ORDER and about
//! WHEN A WRITE HAPPENS, which is what they actually are.
//!
//! # The four things that must not happen
//!
//! 1. **THE EDITOR MUST NOT BE HANDED THE PAINTED ROW.** [`intentsvcs::form::Triple`]
//!    collapses whitespace so a value cannot become two screen rows -- correct
//!    on screen, and the destruction of every paragraph break in the operator's
//!    prose the moment those bytes are what gets saved. The scratch file is
//!    written from [`intentsvcs::form::raw`], which is the model's own bytes.
//!    **`AC-17.10` names the RETURN as the dangerous half; this is the same
//!    destruction at the DEPARTURE, and no test of the return path can see it.**
//! 2. **AN EDITOR THAT FAILED MUST NOT WRITE.** `:cq` is how the vi family says
//!    *abort this*, and it is the contract `git commit` has honoured for twenty
//!    years. Proceeding on a non-zero exit turns the operator's explicit
//!    cancellation into a save.
//! 3. **AN UNCHANGED FILE MUST NOT WRITE.** Opening a field and quitting
//!    without saving is a decision. Writing anyway stamps the store on the
//!    operator's behalf at the one moment they said not to.
//! 4. **THE SCRATCH FILE MUST SURVIVE EVERY FAILURE, AND BE NAMED.** On any
//!    error path the only copy of what the operator just typed is in that file;
//!    deleting it in the name of tidiness destroys the work the error was
//!    reporting a failure to save. It is removed only where the bytes are
//!    already somewhere else.
//!
//! # What this is NOT, stated rather than discovered
//!
//! **FULL-PANE, NOT EMBED.** `tui-design.md` §7 spikes both and records hv
//! preferring EMBED; EMBED needs a pty and is its own build. `AC-17.10` asks
//! for the handoff, the reuse of the one launcher, the re-read on return, the
//! terminal restored on every path, and the fate of an unsaved form stated --
//! all five of which full-pane satisfies. **The design's "starting..." line is
//! an EMBED concern**: it exists because an embedded pane is blank for the 2723
//! ms `emacs -nw` takes to paint, and in full-pane the editor owns the screen
//! and paints its own.
//!
//! **THE SOFT-WRAP FLAGS ARE OWED AND ARE NOT HERE.** §7 asks the editor to
//! soft wrap (`-c 'setlocal wrap linebreak breakindent'`, `--eval '(visual-line-mode 1)'`).
//! That is comfort; the CORRECTNESS half of the same clause -- *never hard-wrap
//! the model* -- is satisfied by this module transforming nothing in either
//! direction. Passing per-family flags means widening the one shared launcher
//! for one caller, which is a decision about `render.rs` rather than about this
//! module, so it is recorded rather than taken quietly.
//!
//! # The fate of an unsaved form is STRUCTURAL
//!
//! `AC-17.10` asks that it be stated rather than discovered. It is stated in
//! the machine: [`super::mode::EDGES`] carries no edge from `Field` to `Embed`,
//! so **a handoff cannot be started from inside an in-place edit at all**.
//! `Field` leaves by `Enter` (commit) or `Esc` (discard) and the operator is in
//! `Normal` before any editor can be reached. There is no interleaving to
//! define because there is no state in which it could occur.

use std::path::{Path, PathBuf};

use thiserror::Error;

/// Why one STEP of a handoff did not work.
///
/// **A NEWTYPE RATHER THAN A BARE `String`, AND `IN-RS-CODE-004` IS RIGHT ABOUT
/// WHY.** Two of these steps RETURN a string on success -- the field's bytes,
/// and the scratch file read back -- so a result whose success value and whose
/// failure value are the same primitive is a signature in which getting the two
/// the wrong way round type-checks. What a step can honestly say IS a sentence,
/// so the payload stays one; what it must not be is the same type as the answer.
///
/// (The type this replaced is deliberately not spelled here: the rule's
/// mechanical proxy is a grep and cannot tell a doc comment from a signature,
/// so naming the defect in prose is itself a finding. Filed rather than worked
/// around silently.)
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{0}")]
pub struct Refused(pub String);

impl Refused {
  pub fn new(why: impl Into<String>) -> Self {
    Self(why.into())
  }
}

/// Why a whole handoff did not land.
///
/// **THE VARIANT IS THE ONE QUESTION A CALLER ACTUALLY HAS TO ASK: IS THE
/// OPERATOR'S TEXT STILL ANYWHERE?** In the first version that fact was a
/// sentence appended to a message, which is exactly the "the information is
/// there but it is not queryable" defect `IN-RS-CODE-004` names -- a recovery
/// path, a crash reporter or a second face would have had to parse prose to
/// find out whether there was a file to offer back.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Failed {
  /// Nothing to recover: this failed before the editor ever ran.
  #[error("{0}")]
  Before(Refused),
  /// **THE EDITOR RAN AND WHAT CAME BACK COULD NOT BE STORED.** On the refused
  /// write this file is the only copy of the operator's work that exists.
  #[error("{why}\n  your text is kept at {}", .kept.display())]
  TextIsKept { why: Refused, kept: PathBuf },
}

/// One field of one entity, addressed the way the view stack addresses it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handoff {
  pub kind: String,
  pub id: String,
  pub field: String,
}

/// The model, as a handoff needs it: the raw bytes in, the raw bytes out.
///
/// **`read` IS NOT THE ROW THE OPERATOR IS LOOKING AT.** It is a fresh read of
/// the field, uncollapsed. See the module note.
///
/// **TWO TRAITS RATHER THAN ONE, BECAUSE THE TWO SIDES HAVE DIFFERENT OWNERS.**
/// The store side is held by the loop for its whole run; the session side
/// borrows the terminal guard the loop itself created, and can only exist for
/// the length of one handoff. Folding them together would mean one object
/// holding a borrow of something constructed after it.
pub trait Model {
  fn read(&mut self, h: &Handoff) -> Result<String, Refused>;
  fn write(&mut self, h: &Handoff, value: &str) -> Result<(), Refused>;
}

/// The outside world: a scratch file, and a child that owns the terminal while
/// it runs.
pub trait Session {
  fn scratch(&mut self, h: &Handoff, value: &str) -> Result<PathBuf, Refused>;
  fn launch(&mut self, path: &Path) -> Result<(), Refused>;
  fn read_back(&mut self, path: &Path) -> Result<String, Refused>;
  /// Remove a scratch file whose bytes are already safe somewhere else.
  fn discard(&mut self, path: &Path);
}

/// What the handoff did. **The caller re-reads either way** -- see
/// [`super::run`] -- because this is news for the operator, never a licence to
/// skip the re-read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Landed {
  Written,
  Unchanged,
}

/// Run one prose handoff.
///
/// The order is the property: **read the model, write the scratch, launch,
/// read back, and only then write** -- with every failure between the launch
/// and the write leaving the operator's bytes on disk under a name the error
/// message carries.
pub fn hand_off(
  model: &mut impl Model,
  session: &mut impl Session,
  h: &Handoff,
) -> Result<Landed, Failed> {
  let before = model.read(h).map_err(Failed::Before)?;
  let path = session.scratch(h, &before).map_err(Failed::Before)?;

  // **FROM HERE THE SCRATCH FILE IS THE ONLY COPY OF ANYTHING THE OPERATOR
  // TYPES**, so every exit below carries its path and none of them removes it.
  let kept = |why: Refused| Failed::TextIsKept {
    why,
    kept: path.clone(),
  };

  session.launch(&path).map_err(&kept)?;
  let after = session.read_back(&path).map_err(&kept)?;

  if after == before {
    session.discard(&path);
    return Ok(Landed::Unchanged);
  }

  model.write(h, &after).map_err(&kept)?;
  session.discard(&path);
  Ok(Landed::Written)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;
  use std::rc::Rc;

  type Log = Rc<RefCell<Vec<String>>>;

  /// The store side. **Two fakes sharing one log rather than one fake behind
  /// two references**, which is what lets the ORDER across both sides be
  /// asserted as a single sequence.
  struct FakeModel {
    log: Log,
    stored: String,
    fail_write: bool,
  }

  impl Model for FakeModel {
    fn read(&mut self, _h: &Handoff) -> Result<String, Refused> {
      self.log.borrow_mut().push("read".to_string());
      Ok(self.stored.clone())
    }
    fn write(&mut self, _h: &Handoff, value: &str) -> Result<(), Refused> {
      self.log.borrow_mut().push(format!("write<{value}>"));
      if self.fail_write {
        return Err(Refused::new("the store refused"));
      }
      self.stored = value.to_string();
      Ok(())
    }
  }

  /// The outside side.
  struct FakeSession {
    log: Log,
    on_disk: String,
    /// What the editor leaves behind. `None` leaves the file untouched, which
    /// is an operator quitting without saving.
    edited_to: Option<String>,
    fail_launch: bool,
    fail_read_back: bool,
  }

  impl Session for FakeSession {
    fn scratch(&mut self, _h: &Handoff, value: &str) -> Result<PathBuf, Refused> {
      self.log.borrow_mut().push(format!("scratch<{value}>"));
      self.on_disk = value.to_string();
      Ok(PathBuf::from("/tmp/intent-scratch.md"))
    }
    fn launch(&mut self, _path: &Path) -> Result<(), Refused> {
      self.log.borrow_mut().push("launch".to_string());
      if self.fail_launch {
        return Err(Refused::new("the editor exited 1"));
      }
      if let Some(to) = self.edited_to.clone() {
        self.on_disk = to;
      }
      Ok(())
    }
    fn read_back(&mut self, _path: &Path) -> Result<String, Refused> {
      self.log.borrow_mut().push("read_back".to_string());
      if self.fail_read_back {
        return Err(Refused::new("the scratch file would not read"));
      }
      Ok(self.on_disk.clone())
    }
    fn discard(&mut self, _path: &Path) {
      self.log.borrow_mut().push("discard".to_string());
    }
  }

  struct Rig {
    log: Log,
    model: FakeModel,
    session: FakeSession,
  }

  impl Rig {
    fn new(stored: &str) -> Self {
      let log: Log = Rc::new(RefCell::new(Vec::new()));
      Self {
        log: Rc::clone(&log),
        model: FakeModel {
          log: Rc::clone(&log),
          stored: stored.to_string(),
          fail_write: false,
        },
        session: FakeSession {
          log,
          on_disk: String::new(),
          edited_to: None,
          fail_launch: false,
          fail_read_back: false,
        },
      }
    }

    fn edited_to(mut self, to: &str) -> Self {
      self.session.edited_to = Some(to.to_string());
      self
    }

    fn run(&mut self) -> Result<Landed, Failed> {
      hand_off(&mut self.model, &mut self.session, &handoff())
    }

    fn log(&self) -> Vec<String> {
      self.log.borrow().clone()
    }
  }

  fn handoff() -> Handoff {
    Handoff {
      kind: "thread".to_string(),
      id: "ST0056".to_string(),
      field: "objective".to_string(),
    }
  }

  /// Prose a one-line render would visibly destroy. **The fixture is the
  /// control**: a value with no whitespace runs to collapse could not tell a
  /// correct handoff from one wired to the painted row.
  const PROSE: &str = "First paragraph.\n\nSecond one, which\nwraps across lines.\n";

  #[test]
  fn the_fixture_can_tell_the_two_wirings_apart() {
    let collapsed = PROSE.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_ne!(
      collapsed, PROSE,
      "the fixture survives a one-line render unchanged, so no test below could catch a handoff \
       wired to the painted row"
    );
  }

  /// **THE EDITOR IS HANDED THE MODEL BYTES, WITH THEIR LINE BREAKS INTACT.**
  /// The whole point of [`intentsvcs::form::raw`] existing beside
  /// [`intentsvcs::form::triples`].
  #[test]
  fn the_editor_is_handed_the_model_bytes_and_not_a_rendering_of_them() {
    let mut rig = Rig::new(PROSE).edited_to("edited\n\nstill two paragraphs\n");
    rig.run().expect("the handoff must succeed");
    assert!(
      rig.log().contains(&format!("scratch<{PROSE}>")),
      "the scratch file was not written from the model bytes; log was {:?}",
      rig.log()
    );
  }

  /// The order, BY EQUALITY. A handoff that wrote before it read back, or read
  /// the model after launching, passes any set-membership check.
  #[test]
  fn the_sequence_is_read_scratch_launch_read_back_write_discard() {
    let mut rig = Rig::new("before").edited_to("after");
    assert_eq!(rig.run(), Ok(Landed::Written));
    assert_eq!(
      rig.log(),
      vec![
        "read".to_string(),
        "scratch<before>".to_string(),
        "launch".to_string(),
        "read_back".to_string(),
        "write<after>".to_string(),
        "discard".to_string(),
      ]
    );
  }

  /// **AN EDITOR THAT FAILED WRITES NOTHING, AND THE OPERATOR IS TOLD WHERE
  /// THEIR TEXT IS.** `:cq` is how the vi family says abort.
  #[test]
  fn a_failing_editor_writes_nothing_and_keeps_the_file() {
    let mut rig = Rig::new(PROSE).edited_to("half-typed replacement");
    rig.session.fail_launch = true;
    let err = rig
      .run()
      .expect_err("a failing editor must not report success");
    // **THE VARIANT IS THE ASSERTION, NOT THE PROSE.** A caller offering the
    // operator their work back has to ASK whether there is a file, and this is
    // that question having a type rather than a substring.
    let Failed::TextIsKept { kept, .. } = &err else {
      panic!("a failure after the editor ran must carry the file: {err:?}");
    };
    assert_eq!(kept, &PathBuf::from("/tmp/intent-scratch.md"));
    assert!(
      err.to_string().contains("/tmp/intent-scratch.md"),
      "and the operator has to be told where it is: {err}"
    );
    assert!(
      !rig.log().iter().any(|l| l.starts_with("write<")),
      "an aborted edit reached the store: {:?}",
      rig.log()
    );
    assert!(
      !rig.log().contains(&"discard".to_string()),
      "the only copy of the operator text was deleted on the error path"
    );
    assert_eq!(rig.model.stored, PROSE, "the stored value changed");
  }

  /// A scratch file that will not read back is the same class: nothing is
  /// known, so nothing is written and nothing is deleted.
  #[test]
  fn a_scratch_file_that_will_not_read_back_writes_nothing_and_keeps_the_file() {
    let mut rig = Rig::new(PROSE).edited_to("edited");
    rig.session.fail_read_back = true;
    let err = rig
      .run()
      .expect_err("an unreadable scratch file must not report success");
    assert!(
      matches!(err, Failed::TextIsKept { .. }),
      "the editor ran, so the file is the only copy: {err:?}"
    );
    assert!(!rig.log().iter().any(|l| l.starts_with("write<")));
    assert!(!rig.log().contains(&"discard".to_string()));
  }

  /// **A STORE THAT REFUSES THE WRITE MUST NOT TAKE THE OPERATOR TEXT WITH
  /// IT.** This is the one path where the scratch file holds work that exists
  /// NOWHERE else -- the editor has exited and the store said no.
  #[test]
  fn a_refused_write_keeps_the_file_because_it_is_the_only_copy_left() {
    let mut rig = Rig::new(PROSE).edited_to("the operator new text");
    rig.model.fail_write = true;
    let err = rig
      .run()
      .expect_err("a refused write must not report success");
    assert!(
      matches!(err, Failed::TextIsKept { .. }),
      "a refused write is the ONE path where the file exists nowhere else: {err:?}"
    );
    assert!(
      !rig.log().contains(&"discard".to_string()),
      "the only surviving copy of the operator work was deleted"
    );
  }

  /// **QUITTING WITHOUT SAVING WRITES NOTHING.** Opening a field and changing
  /// nothing is a decision; a write here stamps the store at the one moment the
  /// operator said not to.
  #[test]
  fn an_unchanged_file_writes_nothing_at_all() {
    let mut rig = Rig::new(PROSE);
    assert_eq!(rig.run(), Ok(Landed::Unchanged));
    assert_eq!(
      rig.log(),
      vec![
        "read".to_string(),
        format!("scratch<{PROSE}>"),
        "launch".to_string(),
        "read_back".to_string(),
        "discard".to_string(),
      ],
      "an unchanged handoff must not reach the store"
    );
  }

  /// Whitespace-only edits ARE edits. A comparison that trimmed, or that
  /// compared through the same collapse the row uses, would drop exactly the
  /// change this module exists to protect.
  #[test]
  fn a_change_that_is_only_whitespace_is_still_a_change() {
    let mut rig = Rig::new("one\n\ntwo\n").edited_to("one\ntwo\n");
    assert_eq!(rig.run(), Ok(Landed::Written));
    assert_eq!(rig.model.stored, "one\ntwo\n");
  }
}

/// **A `&mut Session` IS A `Session`.** Without this a decorator has to own its
/// inner session, which would mean the loop handing its session away on the
/// first handoff and having none for the second.
impl<S: Session + ?Sized> Session for &mut S {
  fn scratch(&mut self, h: &Handoff, value: &str) -> Result<PathBuf, Refused> {
    (**self).scratch(h, value)
  }
  fn launch(&mut self, path: &Path) -> Result<(), Refused> {
    (**self).launch(path)
  }
  fn read_back(&mut self, path: &Path) -> Result<String, Refused> {
    (**self).read_back(path)
  }
  fn discard(&mut self, path: &Path) {
    (**self).discard(path);
  }
}

/// The shipped [`Session`]: a scratch file on disk and an INJECTED launcher.
///
/// **THE LAUNCHER IS A PARAMETER AND THAT IS `AC-17.10`'s FIRST CLAUSE.** The
/// criterion names `launch_editor` by symbol and calls a second resolver *the
/// Highlander defect in the one place this estate can least afford it*. Taking
/// the launcher in means this module has nowhere for one to grow: it cannot
/// read `$VISUAL`, it cannot fall back, and it cannot decide that `vi` will do.
pub struct Files<L> {
  dir: PathBuf,
  launch: L,
}

impl<L> Files<L> {
  /// A session whose scratch files live under `dir`.
  pub fn under(dir: PathBuf, launch: L) -> Self {
    Self { dir, launch }
  }

  /// The default scratch directory: one per process, so two TUIs open on the
  /// same field do not write each other's file.
  pub fn scratch_dir() -> PathBuf {
    std::env::temp_dir().join(format!("intent-edit-{}", std::process::id()))
  }
}

/// The scratch file's name.
///
/// **IT NAMES WHAT IS IN IT, BECAUSE IT IS WHAT SURVIVES A FAILURE.** An
/// operator recovering work from `/tmp` after a refused write needs to know
/// which field of which thread they are looking at; `edit-1.tmp` tells them
/// nothing at the one moment it is the only copy. The `.md` suffix is what
/// makes an editor open it in the mode the content is actually in.
pub fn scratch_name(h: &Handoff) -> String {
  let safe = |s: &str| {
    s.chars()
      .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
      .collect::<String>()
  };
  format!("{}-{}-{}.md", safe(&h.kind), safe(&h.id), safe(&h.field))
}

impl<L: FnMut(&Path) -> Result<(), Refused>> Session for Files<L> {
  fn scratch(&mut self, h: &Handoff, value: &str) -> Result<PathBuf, Refused> {
    std::fs::create_dir_all(&self.dir).map_err(|e| {
      Refused::new(format!(
        "error: cannot make a scratch directory at {} -- {e}",
        self.dir.display()
      ))
    })?;
    let path = self.dir.join(scratch_name(h));
    std::fs::write(&path, value)
      .map_err(|e| Refused::new(format!("error: cannot write {} -- {e}", path.display())))?;
    Ok(path)
  }

  fn launch(&mut self, path: &Path) -> Result<(), Refused> {
    (self.launch)(path)
  }

  fn read_back(&mut self, path: &Path) -> Result<String, Refused> {
    std::fs::read_to_string(path)
      .map_err(|e| Refused::new(format!("error: cannot read {} back -- {e}", path.display())))
  }

  fn discard(&mut self, path: &Path) {
    // **A FAILED REMOVAL IS LITTER, NEVER AN ERROR.** This is only ever called
    // where the bytes are already somewhere else, so there is nothing to
    // report and nothing the operator would do about it.
    let _ = std::fs::remove_file(path);
  }
}
