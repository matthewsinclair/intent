//! `intent use [dev|opt|brew]` and its shadow check (ST0087 WP-03; devbin's
//! `docs/dev-opt-brew.md`, T5).
//!
//! **THE LINKS ARE THE ONLY RECORD OF THE CHOICE.** Nothing else stores which
//! tree a user runs: `use` reads the link set, points it at a tree, or removes
//! it, and no link means brew.
//!
//! Every ambient input arrives as an argument -- the link directory, the
//! working directory, the running root, the PATH lookup -- so each arm is
//! driven against planted trees; [`read`] and [`point`] are the doors that
//! gather them. **PATH IS NEVER READ HERE** (`no_intent_home.rs`): [`on_path`]
//! spawns a shell that inherits it, as `critic.rs` finds its tools.

use std::path::{Path, PathBuf};

use crate::finding::{Finding, FindingClass};
use crate::install::{self, Tree};

/// The link set (T5): Intent's per-user links, moved as one, since the CLI
/// finds the daemon only as `intentd` beside its own executable.
pub const LINKS: [&str; 2] = ["intent", "intentd"];

/// Where a dev checkout's release pair is built, relative to its root:
/// what `use dev` links.
pub const DEV_PAIR: &str = "native/rust/target/release";

/// The tree `use` is asked to point the set at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
  Dev,
  Opt,
  Brew,
}

impl Target {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Dev => "dev",
      Self::Opt => "opt",
      Self::Brew => "brew",
    }
  }

  /// The word as the command line spells it; anything else is not a tree.
  pub fn parse(word: &str) -> Option<Self> {
    match word {
      "dev" => Some(Self::Dev),
      "opt" => Some(Self::Opt),
      "brew" => Some(Self::Brew),
      _ => None,
    }
  }
}

/// One link of the set, as it stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkState {
  Absent,
  /// A symlink into an Intent tree: the root it resolves to and its tree.
  Serves {
    root: PathBuf,
    tree: Tree,
  },
  /// A symlink that resolves into no Intent tree, or resolves nowhere.
  Foreign {
    target: PathBuf,
  },
  /// Something that is not a symlink: never Intent's to touch.
  NotALink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
  pub name: &'static str,
  pub path: PathBuf,
  pub state: LinkState,
}

impl Link {
  /// Whether `use` may replace or remove this link: it is absent, or it is a
  /// link into one of Intent's trees.
  fn is_ours(&self) -> bool {
    matches!(self.state, LinkState::Absent | LinkState::Serves { .. })
  }
}

/// Which tree the set serves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Serving {
  /// No link at all: Homebrew's own links answer.
  Brew,
  /// Both links into one tree.
  Tree { tree: Tree, root: PathBuf },
  /// Anything else: the links serve different trees, one is missing, or one
  /// is not Intent's.
  Split,
}

/// What the shadow check found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
  /// The link exists and is not the first hit on PATH; `by` is what is.
  Shadowed { link: PathBuf, by: Option<PathBuf> },
  /// The set does not serve one tree; `detail` says how each link stands.
  Split { detail: String },
}

/// The link set and the shadow check, as bare `use` and doctor read them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
  pub dir: PathBuf,
  pub links: Vec<Link>,
  pub serving: Serving,
  /// The first `intent` on PATH, which a login shell runs.
  pub first_hit: Option<PathBuf>,
  pub faults: Vec<Fault>,
}

/// The one line bare `use` adds while the set serves dev or opt (hv's ruling
/// of 2026-10-25, decision 80): brew is the resting state.
pub const RESTING: &str = "brew is the resting state; `intent use brew` puts it back";

impl Report {
  /// [`RESTING`] while the set serves dev or opt, and nothing otherwise.
  pub fn resting_note(&self) -> Option<&'static str> {
    match self.serving {
      Serving::Tree {
        tree: Tree::Dev | Tree::Opt,
        ..
      } => Some(RESTING),
      _ => None,
    }
  }
}

/// Why no Homebrew intent would answer once the links are gone, as `use
/// brew`'s refusal names it (A4, issue 0655). Each copy is named with its
/// tree, and `instead` is what would answer in Homebrew's place, if anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoBrewCause {
  /// No `brew` is on PATH, so whether its intent is installed or linked
  /// cannot be read.
  BrewNotOnPath { instead: Option<String> },
  /// The formula has no keg behind its opt link.
  NotInstalled {
    opt: String,
    instead: Option<String>,
  },
  /// The keg is installed and the prefix's `bin/intent` does not lead into
  /// it.
  NotLinked {
    keg: String,
    link: String,
    instead: Option<String>,
  },
  /// Another copy comes first on PATH, ahead of Homebrew's linked one.
  AnotherCopyFirst { by: String, brew: String },
}

impl std::fmt::Display for NoBrewCause {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let answering = |instead: Option<&str>| match instead {
      Some(copy) => format!("{copy} would answer instead"),
      None => {
        "nothing else would answer `intent` on PATH, so no hook, MCP server or shell would find one"
          .to_string()
      }
    };
    match self {
      Self::BrewNotOnPath { instead } => write!(
        f,
        "Homebrew's `brew` is not on PATH, so whether its intent is installed or linked could not be read, and {}",
        answering(instead.as_deref())
      ),
      Self::NotInstalled { opt, instead } => write!(
        f,
        "the intent formula is not installed (there is no keg at {opt}), and {}",
        answering(instead.as_deref())
      ),
      Self::NotLinked { keg, link, instead } => write!(
        f,
        "the intent formula's keg at {keg} is not linked ({link} does not lead into it), and {}",
        answering(instead.as_deref())
      ),
      Self::AnotherCopyFirst { by, brew } => write!(
        f,
        "another copy comes first on PATH: {by}, ahead of Homebrew's {brew}"
      ),
    }
  }
}

#[derive(Debug, thiserror::Error)]
pub enum SwitchError {
  #[error(transparent)]
  Home(#[from] crate::userstate::UserStateError),
  #[error("/bin/sh could not be run to read PATH: {0}")]
  Shell(#[source] std::io::Error),
  #[error(
    "intent use dev needs a dev checkout, and neither {cwd} nor the running binary is in one"
  )]
  NoDevTree { cwd: String },
  #[error("the dev checkout at {root} has no release pair: {missing} is not there")]
  NoDevPair { root: String, missing: String },
  #[error("intent use opt needs a prefix: none was given and INTENT_INSTALL_PREFIX is not set")]
  NoPrefix,
  #[error("{prefix} has no {manifest}, so it is not an opt tree Intent published")]
  NoManifest {
    prefix: String,
    manifest: &'static str,
  },
  #[error("{prefix} is managed by brew, by its manifest, so intent use never points into it")]
  BrewManaged { prefix: String },
  #[error("{prefix} is not an opt tree of Intent's: {why}")]
  NotIntents { prefix: String, why: String },
  #[error("the opt tree at {prefix} has no pair: {missing} is not there")]
  NoOptPair { prefix: String, missing: String },
  #[error("{link} is not a link into an Intent tree ({what}), so intent use leaves it as it is")]
  NotOurs { link: String, what: String },
  #[error(
    "intent use brew removed nothing: no Homebrew intent would answer once the links are gone, because {cause}. The links it would have removed: {links}"
  )]
  NoBrew { links: String, cause: NoBrewCause },
  #[error("{link} could not be written: {source}")]
  Write {
    link: String,
    #[source]
    source: std::io::Error,
  },
  #[error(
    "{link} could not be written ({source}), and {first} could not be put back as it was ({back})"
  )]
  WriteAndBack {
    link: String,
    first: String,
    back: String,
    #[source]
    source: std::io::Error,
  },
  #[error("the daemon restart through {binary} could not be run: {source}")]
  Restart {
    binary: String,
    #[source]
    source: std::io::Error,
  },
}

impl crate::remedy::Remedy for SwitchError {
  fn remedy(&self) -> String {
    match self {
      Self::Home(e) => e.remedy(),
      Self::Shell(_) => {
        "the shadow check reads PATH through /bin/sh, so it needs a system shell; run this from a normal login shell".to_string()
      }
      Self::NoDevTree { .. } => {
        "run `intent use dev` from inside the Intent checkout whose pair you mean; nothing was written".to_string()
      }
      Self::NoDevPair { .. } => {
        "build the pair in that checkout with `bin/devbin build all`, then run this again; nothing was written".to_string()
      }
      Self::NoPrefix => {
        "pass `--prefix <dir>`, or set INTENT_INSTALL_PREFIX in this sleeve's environment; nothing was written".to_string()
      }
      Self::NoManifest { .. } => {
        "publish an opt tree there from the dev checkout with `intent install --prefix <dir>`, then run this again; nothing was written".to_string()
      }
      Self::BrewManaged { .. } => {
        "Homebrew manages that tree: run `intent use brew` to let Homebrew's links answer, and `brew upgrade intent` to move it; nothing was written".to_string()
      }
      Self::NotIntents { .. } | Self::NoOptPair { .. } => {
        "republish the opt tree from the dev checkout with `intent install --prefix <dir> --force`, then run this again; nothing was written".to_string()
      }
      Self::NotOurs { .. } => {
        "move that file aside by hand if it is yours, then run this again; nothing was written".to_string()
      }
      Self::NoBrew {
        cause: NoBrewCause::BrewNotOnPath { .. },
        ..
      } => {
        "put Homebrew's bin on PATH, then run `intent use brew` again; or stay on dev or opt".to_string()
      }
      Self::NoBrew {
        cause: NoBrewCause::NotInstalled { .. },
        ..
      } => {
        "install the formula with `brew install intent`, then run `intent use brew` again; or stay on dev or opt".to_string()
      }
      Self::NoBrew {
        cause: NoBrewCause::NotLinked { .. },
        ..
      } => {
        "link the formula with `brew unpin intent && brew link intent`, then run `intent use brew` again; or stay on dev or opt".to_string()
      }
      Self::NoBrew {
        cause: NoBrewCause::AnotherCopyFirst { .. },
        ..
      } => {
        "take the copy named above off PATH, or put Homebrew's bin ahead of it, then run `intent use brew` again; or stay on dev or opt".to_string()
      }
      Self::Write { .. } => {
        "nothing was changed: check that ~/.local/bin is writable, then run this again".to_string()
      }
      Self::WriteAndBack { .. } => {
        "the link set is split. Run `intent use` to see each link, then `intent use dev`, `opt` or `brew` again to put both on one tree".to_string()
      }
      Self::Restart { .. } => {
        "the links are pointed; run `intent daemon restart` yourself so the new tree's daemon serves".to_string()
      }
    }
  }
}

/// Every hit for `name` on PATH, in PATH's order, as a login shell finds them.
///
/// **THE SURFACE READS NO PATH**: a child inherits it, so the walk runs in a
/// spawned shell and the answer comes back as text (`no_intent_home.rs`).
pub fn on_path(name: &str) -> Result<Vec<PathBuf>, SwitchError> {
  const WALK: &str = r#"set -f; IFS=:; for d in $PATH; do [ -n "$d" ] || d=.; if [ -f "$d/$1" ] && [ -x "$d/$1" ]; then printf '%s\n' "$d/$1"; fi; done"#;
  let out = std::process::Command::new("/bin/sh")
    .args(["-c", WALK, "sh", name])
    .output()
    .map_err(SwitchError::Shell)?;
  Ok(
    String::from_utf8_lossy(&out.stdout)
      .lines()
      .map(PathBuf::from)
      .collect(),
  )
}

/// A PATH lookup, so each arm can be driven against a planted PATH.
pub type Lookup<'a> = &'a dyn Fn(&str) -> Result<Vec<PathBuf>, SwitchError>;

/// The link set in `~/.local/bin` and its shadow check.
pub fn read() -> Result<Report, SwitchError> {
  read_at(&crate::userstate::local_bin()?, &on_path)
}

/// [`read`] over a given link directory and PATH lookup.
pub fn read_at(dir: &Path, lookup: Lookup<'_>) -> Result<Report, SwitchError> {
  let links: Vec<Link> = LINKS
    .iter()
    .map(|name| {
      let path = dir.join(name);
      Link {
        name,
        state: state_of(&path),
        path,
      }
    })
    .collect();
  let serving = serving(&links);
  let first_hit = lookup("intent")?.into_iter().next();
  let mut faults = Vec::new();
  for link in links.iter().filter(|l| l.state != LinkState::Absent) {
    let first = lookup(link.name)?.into_iter().next();
    if first.as_deref().and_then(Path::parent).map(canonical) != Some(canonical(dir)) {
      faults.push(Fault::Shadowed {
        link: link.path.clone(),
        by: first,
      });
    }
  }
  if serving == Serving::Split {
    faults.push(Fault::Split {
      detail: links
        .iter()
        .map(|l| format!("{} {}", l.name, describe(&l.state)))
        .collect::<Vec<_>>()
        .join("; "),
    });
  }
  Ok(Report {
    dir: dir.to_path_buf(),
    links,
    serving,
    first_hit,
    faults,
  })
}

/// What doctor says about the link set: one finding per fault.
pub fn findings(report: &Report) -> Vec<Finding> {
  report
    .faults
    .iter()
    .map(|fault| match fault {
      Fault::Shadowed { link, by } => Finding::new(
        link.display().to_string(),
        FindingClass::InstallLinkShadowed,
        format!(
          "this link is not the first hit on PATH, so a login shell runs {}",
          by.as_ref()
            .map_or("no intent at all".to_string(), |b| shadow_name(b))
        ),
      ),
      Fault::Split { detail } => Finding::new(
        report.dir.display().to_string(),
        FindingClass::InstallLinkSplit,
        format!("the link set serves no single tree: {detail}"),
      ),
    })
    .collect()
}

/// A shadowing binary named with its tree, so a stale link a brew link
/// shadows reads as one.
fn shadow_name(binary: &Path) -> String {
  match install::resolve(binary) {
    Ok(root) => format!(
      "{} (the {} tree at {})",
      binary.display(),
      install::tree_at(&root, &commit_of(binary)).tree,
      root.display()
    ),
    Err(_) => binary.display().to_string(),
  }
}

/// How one link stands, in words.
pub fn describe(state: &LinkState) -> String {
  match state {
    LinkState::Absent => "is absent".to_string(),
    LinkState::Serves { root, tree } => format!("serves the {tree} tree at {}", root.display()),
    LinkState::Foreign { target } => {
      format!("links to {}, which is in no Intent tree", target.display())
    }
    LinkState::NotALink => "is a file Intent did not write".to_string(),
  }
}

/// What `use dev`, `use opt` or `use brew` changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pointed {
  pub target: Target,
  /// The tree the set now serves; for brew, the root Homebrew's intent
  /// resolves to.
  pub root: PathBuf,
  /// Each link written, with what it points at.
  pub linked: Vec<(PathBuf, PathBuf)>,
  /// Each link removed.
  pub unlinked: Vec<PathBuf>,
  /// The binary whose `intent daemon restart` finishes the switch: the new
  /// link, or for brew the first hit Homebrew's link gives.
  pub restart_with: PathBuf,
}

/// `intent use dev|opt|brew` over `~/.local/bin`, the working directory, the
/// running binary's install root and PATH.
pub fn point(target: Target, prefix: Option<&Path>) -> Result<Pointed, SwitchError> {
  let dir = crate::userstate::local_bin()?;
  let cwd = std::env::current_dir().unwrap_or_default();
  let running = install::home().ok();
  point_at(
    target,
    &dir,
    &cwd,
    running.as_deref(),
    install::opt_prefix(prefix).as_deref(),
    &on_path,
  )
}

/// [`point`] over given inputs. Every refusal comes before the first write.
pub fn point_at(
  target: Target,
  dir: &Path,
  cwd: &Path,
  running: Option<&Path>,
  prefix: Option<&Path>,
  lookup: Lookup<'_>,
) -> Result<Pointed, SwitchError> {
  let report = read_at(dir, lookup)?;
  match target {
    Target::Brew => unlink_for_brew(&report, lookup),
    Target::Dev | Target::Opt => {
      if let Some(link) = report.links.iter().find(|l| !l.is_ours()) {
        return Err(SwitchError::NotOurs {
          link: link.path.display().to_string(),
          what: describe(&link.state),
        });
      }
      let (root, pair) = match target {
        Target::Dev => dev_pair(cwd, running)?,
        _ => opt_pair(prefix)?,
      };
      let linked = link_set(dir, &pair)?;
      Ok(Pointed {
        target,
        root,
        linked,
        unlinked: Vec::new(),
        restart_with: dir.join("intent"),
      })
    }
  }
}

/// The dev checkout `use dev` points at: the one holding `cwd`, else the
/// running binary's when it is dev, and its release pair.
fn dev_pair(cwd: &Path, running: Option<&Path>) -> Result<(PathBuf, PathBuf), SwitchError> {
  let is_dev =
    |d: &Path| d.join(install::MARKER).is_dir() && install::tree_at(d, "unknown").tree == Tree::Dev;
  let root = cwd
    .ancestors()
    .find(|d| is_dev(d))
    .map(Path::to_path_buf)
    .or_else(|| running.filter(|r| is_dev(r)).map(Path::to_path_buf))
    .ok_or_else(|| SwitchError::NoDevTree {
      cwd: cwd.display().to_string(),
    })?;
  let pair = root.join(DEV_PAIR);
  for name in LINKS {
    if !pair.join(name).is_file() {
      return Err(SwitchError::NoDevPair {
        root: root.display().to_string(),
        missing: pair.join(name).display().to_string(),
      });
    }
  }
  Ok((root, pair))
}

/// The opt tree `use opt` points at, refused unless its manifest says Intent
/// published it and Intent manages it, and its pair.
fn opt_pair(prefix: Option<&Path>) -> Result<(PathBuf, PathBuf), SwitchError> {
  let prefix = prefix.ok_or(SwitchError::NoPrefix)?;
  let shown = prefix.display().to_string();
  if !prefix.join(install::MANIFEST).is_file() {
    return Err(SwitchError::NoManifest {
      prefix: shown,
      manifest: install::MANIFEST,
    });
  }
  let id = install::tree_at(&canonical(prefix), "unknown");
  match id.tree {
    Tree::Opt => {}
    Tree::Brew => return Err(SwitchError::BrewManaged { prefix: shown }),
    _ => {
      return Err(SwitchError::NotIntents {
        prefix: shown,
        why: id.why.unwrap_or_default(),
      });
    }
  }
  let pair = prefix.join("bin");
  for name in LINKS {
    if !pair.join(name).is_file() {
      return Err(SwitchError::NoOptPair {
        prefix: shown,
        missing: pair.join(name).display().to_string(),
      });
    }
  }
  Ok((prefix.to_path_buf(), pair))
}

/// Point both links at `pair`, as one: each is staged beside itself and
/// renamed in, and when the second rename fails the first is put back as it
/// was, so a failed switch leaves the set where it found it.
fn link_set(dir: &Path, pair: &Path) -> Result<Vec<(PathBuf, PathBuf)>, SwitchError> {
  let write = |link: &Path, source: std::io::Error| SwitchError::Write {
    link: link.display().to_string(),
    source,
  };
  std::fs::create_dir_all(dir).map_err(|e| write(dir, e))?;
  let mut staged = Vec::new();
  for name in LINKS {
    let link = dir.join(name);
    let stage = dir.join(format!(".{name}.intent-use"));
    clear(&stage).map_err(|e| write(&stage, e))?;
    std::os::unix::fs::symlink(pair.join(name), &stage).map_err(|e| write(&link, e))?;
    // The link is absent or a link into an Intent tree (`point_at` refused
    // anything else), so no target is the absent case.
    let was = std::fs::read_link(&link).ok();
    staged.push((stage, link, pair.join(name), was));
  }
  let mut linked: Vec<(PathBuf, PathBuf)> = Vec::new();
  for (stage, link, to, _) in &staged {
    if let Err(source) = std::fs::rename(stage, link) {
      let mut left: Vec<String> = staged
        .iter()
        .filter_map(|(stage, ..)| {
          clear(stage)
            .err()
            .map(|e| format!("{}: {e}", stage.display()))
        })
        .collect();
      for (first, _) in &linked {
        let was = staged
          .iter()
          .find(|(_, l, ..)| l == first)
          .and_then(|(.., was)| was.as_deref());
        if let Err(e) = restore(first, was) {
          left.push(format!("{}: {e}", first.display()));
        }
      }
      return Err(match linked.first() {
        Some((first, _)) if !left.is_empty() => SwitchError::WriteAndBack {
          link: link.display().to_string(),
          first: first.display().to_string(),
          back: left.join("; "),
          source,
        },
        _ => write(link, source),
      });
    }
    linked.push((link.clone(), to.clone()));
  }
  Ok(linked)
}

/// Remove `path` if it is there; its absence is the answer wanted.
fn clear(path: &Path) -> std::io::Result<()> {
  match std::fs::remove_file(path) {
    Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e),
    _ => Ok(()),
  }
}

/// Put one link back as it was: pointing at `was`, or absent.
fn restore(link: &Path, was: Option<&Path>) -> std::io::Result<()> {
  clear(link)?;
  match was {
    Some(target) => std::os::unix::fs::symlink(target, link),
    None => Ok(()),
  }
}

/// The links `use brew` removes: each one that points into an Intent tree.
fn removable(report: &Report) -> impl Iterator<Item = &Link> {
  report
    .links
    .iter()
    .filter(|l| matches!(l.state, LinkState::Serves { .. }))
}

/// The root of the Homebrew tree `binary` resolves into, or none when it is
/// not Homebrew's: a keg by its manifest, or a keg from before the contract by
/// its opt link (`install::tree_at`).
fn brew_root(binary: &Path) -> Option<PathBuf> {
  let id = install::tree_at(&install::resolve(binary).ok()?, &commit_of(binary));
  (id.tree == Tree::Brew).then_some(id.root)
}

/// Why no Homebrew intent would answer (A4), read only once `use brew` has
/// refused, and from the filesystem, so no `brew` runs. `hits` are the
/// `intent`s on PATH outside the link directory, in PATH's order.
fn no_brew_cause(hits: &[PathBuf], lookup: Lookup<'_>) -> Result<NoBrewCause, SwitchError> {
  let behind = hits.split_first().and_then(|(first, rest)| {
    rest
      .iter()
      .find(|hit| brew_root(hit).is_some())
      .map(|brew| (first, brew))
  });
  if let Some((first, brew)) = behind {
    return Ok(NoBrewCause::AnotherCopyFirst {
      by: shadow_name(first),
      brew: brew.display().to_string(),
    });
  }
  let instead = hits.first().map(|hit| shadow_name(hit));
  // Homebrew's prefix as brew takes it: the parent of the directory its
  // `brew` is found in.
  let Some(prefix) = lookup("brew")?
    .into_iter()
    .next()
    .and_then(|brew| brew.parent().map(canonical))
    .and_then(|bin| bin.parent().map(Path::to_path_buf))
  else {
    return Ok(NoBrewCause::BrewNotOnPath { instead });
  };
  let opt = prefix.join("opt").join(install::BREW_FORMULA);
  // A linked keg's `bin/intent` is in the directory `brew` was found in, so it
  // was one of `hits`: a keg found here is not linked.
  Ok(match brew_root(&opt.join("bin/intent")) {
    Some(_) => NoBrewCause::NotLinked {
      keg: canonical(&opt).display().to_string(),
      link: prefix.join("bin/intent").display().to_string(),
      instead,
    },
    None => NoBrewCause::NotInstalled {
      opt: opt.display().to_string(),
      instead,
    },
  })
}

/// `use brew`: remove every link of the set that points into an Intent tree,
/// once a Homebrew intent is known to answer afterwards. Otherwise it removes
/// nothing, and names each of those links with its tree and why no Homebrew
/// intent would answer (A4).
fn unlink_for_brew(report: &Report, lookup: Lookup<'_>) -> Result<Pointed, SwitchError> {
  let hits: Vec<PathBuf> = lookup("intent")?
    .into_iter()
    .filter(|hit| hit.parent().map(canonical) != Some(canonical(&report.dir)))
    .collect();
  let answers = hits
    .first()
    .and_then(|hit| brew_root(hit).map(|root| (hit.clone(), root)));
  let Some((hit, root)) = answers else {
    let links: Vec<String> = removable(report)
      .map(|l| format!("{} {}", l.path.display(), describe(&l.state)))
      .collect();
    return Err(SwitchError::NoBrew {
      links: if links.is_empty() {
        "none, since no link of the set serves a tree".to_string()
      } else {
        links.join("; ")
      },
      cause: no_brew_cause(&hits, lookup)?,
    });
  };
  let mut unlinked = Vec::new();
  for link in removable(report) {
    std::fs::remove_file(&link.path).map_err(|source| SwitchError::Write {
      link: link.path.display().to_string(),
      source,
    })?;
    unlinked.push(link.path.clone());
  }
  Ok(Pointed {
    target: Target::Brew,
    root,
    linked: Vec::new(),
    unlinked,
    restart_with: hit,
  })
}

/// What the restart through the new first hit printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Restarted {
  /// `daemon restart`'s exit code; its output went to the caller's terminal.
  pub code: Option<i32>,
  /// The new binary's `--version` line.
  pub version: String,
}

/// The switch's last step (T5): the NEW binary's `intent daemon restart`, run
/// by its link path so it starts its own sibling daemon, never a restart in
/// this process, whose sibling is the old tree's; then that binary's line.
pub fn restart_through(binary: &Path) -> Result<Restarted, SwitchError> {
  let failed = |source| SwitchError::Restart {
    binary: binary.display().to_string(),
    source,
  };
  let status = std::process::Command::new(binary)
    .args(["daemon", "restart"])
    .status()
    .map_err(failed)?;
  let version = std::process::Command::new(binary)
    .arg("--version")
    .output()
    .map_err(failed)?;
  Ok(Restarted {
    code: status.code(),
    version: String::from_utf8_lossy(&version.stdout).trim().to_string(),
  })
}

/// One link's state: absent, a link into an Intent tree, a link elsewhere, or
/// not a link.
fn state_of(path: &Path) -> LinkState {
  match std::fs::symlink_metadata(path) {
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => LinkState::Absent,
    // Unreadable: not something `use` may replace.
    Err(_) => LinkState::NotALink,
    Ok(meta) if !meta.file_type().is_symlink() => LinkState::NotALink,
    Ok(_) => match install::resolve(path) {
      Ok(root) => LinkState::Serves {
        tree: install::tree_at(&root, &commit_of(path)).tree,
        root,
      },
      Err(_) => LinkState::Foreign {
        target: std::fs::read_link(path).unwrap_or_default(),
      },
    },
  }
}

/// Which tree the set serves: none at all is brew, both into one root is
/// that tree, anything else is split.
fn serving(links: &[Link]) -> Serving {
  if links.iter().all(|l| l.state == LinkState::Absent) {
    return Serving::Brew;
  }
  let served: Vec<(Tree, &Path)> = links
    .iter()
    .filter_map(|l| match &l.state {
      LinkState::Serves { root, tree } => Some((*tree, root.as_path())),
      _ => None,
    })
    .collect();
  match served.as_slice() {
    [(tree, a), (_, b)] if install::same_install(a, b) => Serving::Tree {
      tree: *tree,
      root: a.to_path_buf(),
    },
    _ => Serving::Split,
  }
}

/// The commit a binary names, for the tree function; `unknown` when it names
/// none.
fn commit_of(binary: &Path) -> String {
  install::embedded_marker(&canonical(binary), "commit").unwrap_or_else(|| "unknown".to_string())
}

/// A path as it resolves, or as given when it does not.
fn canonical(p: &Path) -> PathBuf {
  std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}
