//! **THE SHAPE OF A COMMAND'S OUTPUT, ORTHOGONAL TO WHAT THE COMMAND COMPUTES.**
//!
//! Before this module the shape of output was decided four different ways, per
//! verb, by whichever flag that verb happened to declare: `--width` on two rows,
//! `--markdown` on one, `--json` on three, `--format` on two. **Eleven flags,
//! four spellings, one concern** -- so an operator could not learn the surface
//! once, and `issues list` was width-aware while `wp list` and `todo` were not,
//! for no reason anybody had decided.
//!
//! # The v2 spellings are kept, and NOT as a compatibility shim
//!
//! `--json`, `--markdown`, `--width` and `--format` all exist in v2 and every
//! row declaring one cites a v2 source. **They are parity obligations, which is
//! a different thing from backwards compatibility** -- Intent's fail-forward
//! rule prunes shims, and a parity contract is not a shim. So they survive as
//! aliases INTO this vocabulary rather than beside it, and there is still one
//! place that decides what the output looks like.
//!
//! # Conflicts refuse rather than resolve
//!
//! `--json --format=md` is not a precedence question, it is a caller who
//! believes two things. **A silent winner would make the surface unlearnable in
//! the one case where the operator has already shown they are confused**, so it
//! is refused by name (`IN-AG-NO-SILENT-001`).

use crate::views::{self, TableMode};

/// What form the output takes. **Not the same axis as how MUCH output there
/// is** -- `--quiet` and `--verbose` are volume, and folding them in here would
/// put two questions behind one flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
  /// On-screen, width-aware, clipped to fit.
  Terminal,
  /// A persisted file: canonical GFM, always content-fit.
  Markdown,
  /// Machine-readable. Width has no meaning and is not applied.
  Json,
}

impl Format {
  /// The canonical spelling, which is also what `--format` prints back.
  pub fn as_str(&self) -> &'static str {
    match self {
      Format::Terminal => "terminal",
      Format::Markdown => "md",
      Format::Json => "json",
    }
  }

  /// Every spelling this accepts, for a refusal that can name the set.
  pub const SPELLINGS: &'static [&'static str] = &["terminal", "text", "md", "markdown", "json"];

  /// Parse a `--format` value. Case-insensitive: a hand-typed flag is not a
  /// committed manifest, so refusing `JSON` for its case refuses nothing real.
  pub fn parse(s: &str) -> Option<Self> {
    match s.trim().to_ascii_lowercase().as_str() {
      "terminal" | "text" => Some(Format::Terminal),
      "md" | "markdown" => Some(Format::Markdown),
      "json" => Some(Format::Json),
      _ => None,
    }
  }
}

/// Why an output request cannot be honoured.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OutputError {
  #[error("`{found}` is not a format -- `--format` accepts {}", Format::SPELLINGS.join(", "))]
  UnknownFormat { found: String },
  #[error("`{found}` is not a column count -- `--width` takes a positive number")]
  BadWidth { found: String },
  /// **TWO FLAGS, TWO ANSWERS, NO WINNER.**
  #[error("`{first}` and `{second}` ask for different output formats")]
  ConflictingFormat { first: String, second: String },
}

impl crate::remedy::Remedy for OutputError {
  fn remedy(&self) -> String {
    match self {
      OutputError::UnknownFormat { .. } => {
        format!("name one of {}", Format::SPELLINGS.join(", "))
      }
      OutputError::BadWidth { .. } => {
        "give a column count, eg `--width 80`; omit it to use the terminal's".to_string()
      }
      OutputError::ConflictingFormat { .. } => {
        "drop one of them -- `--json` and `--markdown` are spellings of `--format`".to_string()
      }
    }
  }
}

/// A resolved request for output in a particular shape.
///
/// **`width` IS RESOLVED BY THE CALLER, NOT DISCOVERED HERE.** Reading `COLUMNS`
/// is a CLI concern and AC-11.3 permits the shipped surface exactly one
/// environment variable; a services module that reached for the environment
/// would put that permission somewhere nothing is checking it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Output {
  format: Format,
  width: usize,
}

impl Output {
  pub fn new(format: Format, width: usize) -> Self {
    Self { format, width }
  }

  pub fn format(&self) -> Format {
    self.format
  }

  /// The target width. **Zero in every non-terminal format**, because a width
  /// in JSON is meaningless and a width in markdown is actively harmful: a
  /// persisted file whose columns depended on the window that generated it
  /// would change bytes at every regeneration and the skew check would report
  /// files nobody had touched.
  pub fn width(&self) -> usize {
    match self.format {
      Format::Terminal => self.width,
      Format::Markdown | Format::Json => 0,
    }
  }

  /// Resolve a request from the raw flag values, refusing rather than choosing.
  ///
  /// `json` and `markdown` are the v2 alias flags. `default_width` is what the
  /// caller discovered from the terminal.
  pub fn resolve(
    format: Option<&str>,
    width: Option<&str>,
    json: bool,
    markdown: bool,
    default_width: usize,
  ) -> Result<Self, OutputError> {
    let mut chosen: Option<(Format, &'static str)> = None;
    let mut claim = |f: Format, spelling: &'static str| -> Result<(), OutputError> {
      match chosen {
        Some((already, first)) if already != f => Err(OutputError::ConflictingFormat {
          first: first.to_string(),
          second: spelling.to_string(),
        }),
        _ => {
          chosen = Some((f, spelling));
          Ok(())
        }
      }
    };

    if let Some(raw) = format {
      let f = Format::parse(raw).ok_or_else(|| OutputError::UnknownFormat {
        found: raw.to_string(),
      })?;
      claim(f, "--format")?;
    }
    if json {
      claim(Format::Json, "--json")?;
    }
    if markdown {
      claim(Format::Markdown, "--markdown")?;
    }

    let width = match width {
      Some(raw) => match raw.trim().parse::<usize>() {
        Ok(n) if n > 0 => n,
        _ => {
          return Err(OutputError::BadWidth {
            found: raw.to_string(),
          });
        }
      },
      None => default_width,
    };

    Ok(Self {
      format: chosen.map_or(Format::Terminal, |(f, _)| f),
      width,
    })
  }

  /// Render a table in whatever shape was asked for.
  ///
  /// **JSON is refused HERE rather than rendered badly.** A table is columns and
  /// rows; the JSON a caller wants is the domain object, which only the caller
  /// has. Emitting a list-of-lists would be a shape nobody asked for wearing the
  /// name of one they did, so callers that support JSON branch before this and
  /// callers that do not say so by name.
  pub fn table(&self, headers: &[&str], rows: &[Vec<String>]) -> Option<String> {
    match self.format {
      Format::Terminal => Some(views::table(
        headers,
        rows,
        TableMode::Terminal { fill: self.width },
      )),
      Format::Markdown => Some(views::table(headers, rows, TableMode::Markdown)),
      Format::Json => None,
    }
  }
}
