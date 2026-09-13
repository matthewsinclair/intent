//! The project picker: which of this machine's Intent projects to explore
//! (ST0074 WP-04).
//!
//! **ITS OWN SCREEN, SHOWN BEFORE ANY PROJECT IS OPEN**, because the explorer's
//! view stack reads one project's store and the picker's whole subject is which
//! store that is. `intent explore` outside a project opens it first, and
//! `/projects` inside the explorer leaves the project's screen to open it.
//!
//! **THE CHOICES ARE THE PROJECT REGISTRY, READ THROUGH `intentsvcs::projects`.**
//! intentd lists exactly that file and never writes it, so the file is the same
//! answer the daemon gives, and it is there when no daemon is running.

use std::io;
use std::path::{Path, PathBuf};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// One project on offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
  pub root: PathBuf,
  pub name: String,
  /// Whether an Intent config is still at `root`. A registry entry outlives
  /// the directory it names, and the picker says so rather than dropping it.
  pub exists: bool,
}

impl Choice {
  pub fn of(root: PathBuf) -> Choice {
    let name = root
      .file_name()
      .map(|n| n.to_string_lossy().into_owned())
      .unwrap_or_else(|| root.display().to_string());
    let exists = intentsvcs::project::Project::config_path(&root).is_file();
    Choice { root, name, exists }
  }
}

/// What one key did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pick {
  Stay,
  Chosen(PathBuf),
  Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Picker {
  pub choices: Vec<Choice>,
  pub cursor: usize,
  pub notice: String,
}

impl Picker {
  /// A picker over `choices`, its cursor on `current` when that is one of them.
  pub fn new(choices: Vec<Choice>, current: Option<&Path>) -> Picker {
    let cursor = current
      .and_then(|here| choices.iter().position(|choice| choice.root == here))
      .unwrap_or(0);
    let notice = if choices.is_empty() {
      "no projects are registered -- `intent discover <dir>` registers the ones under a directory, and `intent explore` inside a project registers that one".to_string()
    } else {
      String::new()
    };
    Picker {
      choices,
      cursor,
      notice,
    }
  }

  pub fn on_key(&mut self, key: KeyEvent) -> Pick {
    match key.code {
      KeyCode::Esc => Pick::Left,
      KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Pick::Left,
      KeyCode::Up => {
        self.cursor = self.cursor.saturating_sub(1);
        Pick::Stay
      }
      KeyCode::Down => {
        if self.cursor + 1 < self.choices.len() {
          self.cursor += 1;
        }
        Pick::Stay
      }
      KeyCode::Enter => match self.choices.get(self.cursor) {
        Some(choice) if choice.exists => Pick::Chosen(choice.root.clone()),
        Some(choice) => {
          self.notice = format!(
            "`{}` is no longer an Intent project -- restore it, or remove its entry from the project registry",
            choice.root.display()
          );
          Pick::Stay
        }
        None => Pick::Stay,
      },
      _ => Pick::Stay,
    }
  }

  /// The lines a frame `height` rows tall shows, with the cursor kept in view.
  pub fn lines(&self, height: usize) -> Vec<String> {
    let mut lines = vec![
      "Pick a project -- Up and Down move, Enter opens, Esc leaves".to_string(),
      String::new(),
    ];
    let footer = if self.notice.is_empty() { 0 } else { 2 };
    let room = height.saturating_sub(lines.len() + footer).max(1);
    let first = self.cursor.saturating_sub(room - 1);
    for (at, choice) in self.choices.iter().enumerate().skip(first).take(room) {
      let marker = if at == self.cursor { ">" } else { " " };
      let missing = if choice.exists { "" } else { "  (missing)" };
      lines.push(format!(
        "{marker} {:<24} {}{missing}",
        choice.name,
        choice.root.display()
      ));
    }
    if !self.notice.is_empty() {
      lines.push(String::new());
      lines.push(self.notice.clone());
    }
    lines
  }
}

/// Show the picker until the operator chooses a project or leaves.
pub fn pick(mut picker: Picker) -> io::Result<Option<PathBuf>> {
  super::terminal::real::restore_on_panic();
  let _borrowed = super::terminal::Borrowed::take(super::terminal::real::Crossterm)?;
  let mut term = ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;
  loop {
    term.draw(|frame| {
      let text = picker.lines(frame.area().height as usize).join("\n");
      frame.render_widget(ratatui::widgets::Paragraph::new(text), frame.area());
    })?;
    let Event::Key(key) = event::read()? else {
      continue;
    };
    if key.kind != KeyEventKind::Press {
      continue;
    }
    match picker.on_key(key) {
      Pick::Stay => {}
      Pick::Chosen(root) => return Ok(Some(root)),
      Pick::Left => return Ok(None),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
  }

  fn project(root: &Path) {
    std::fs::create_dir_all(root.join("intent/.config")).expect("config dir");
    std::fs::write(
      intentsvcs::project::Project::config_path(root),
      r#"{"intent_version": "3.0.2"}"#,
    )
    .expect("config");
  }

  /// `AT-04.1` (ST0074 WP-04): the picker offers every registered project,
  /// starts on the one open, and Enter opens the one under the cursor.
  #[test]
  fn enter_opens_the_project_under_the_cursor() {
    let dir = tempfile::tempdir().expect("tempdir");
    let (alpha, beta) = (dir.path().join("alpha"), dir.path().join("beta"));
    project(&alpha);
    project(&beta);
    let mut picker = Picker::new(
      vec![Choice::of(alpha.clone()), Choice::of(beta.clone())],
      Some(&beta),
    );
    assert_eq!(picker.cursor, 1, "the cursor starts on the open project");
    assert_eq!(picker.on_key(key(KeyCode::Up)), Pick::Stay);
    assert_eq!(picker.on_key(key(KeyCode::Enter)), Pick::Chosen(alpha));
  }

  /// A registered root that is no longer a project is shown and refused, with
  /// the reason, rather than opened into an error.
  #[test]
  fn a_missing_project_is_refused_with_the_reason() {
    let gone = PathBuf::from("/nowhere/at/all");
    let mut picker = Picker::new(vec![Choice::of(gone)], None);
    assert!(
      picker
        .lines(10)
        .iter()
        .any(|line| line.ends_with("(missing)"))
    );
    assert_eq!(picker.on_key(key(KeyCode::Enter)), Pick::Stay);
    assert!(picker.notice.contains("no longer an Intent project"));
  }

  /// `AT-04.2`: Esc leaves without choosing, and an empty registry says how to
  /// fill it.
  #[test]
  fn esc_leaves_and_an_empty_registry_says_how_to_fill_it() {
    let mut picker = Picker::new(Vec::new(), None);
    assert!(picker.lines(10).join("\n").contains("intent discover"));
    assert_eq!(picker.on_key(key(KeyCode::Enter)), Pick::Stay);
    assert_eq!(picker.on_key(key(KeyCode::Esc)), Pick::Left);
  }
}
