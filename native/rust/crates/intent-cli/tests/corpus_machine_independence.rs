//! AT-03.9 / AC-03.7, the clause the first cut missed: **the corpus is a
//! property of the repository, not of the machine the tool runs on.**
//!
//! Found by vc. With the walker's defaults, `intent/probe.sql` was silently
//! out of corpus on their machine -- their `~/.gitignore_global` carries
//! `*.sql` -- and in corpus on a machine without that rule. Same commit, same
//! command, different answer to "what does this project contain". AC-10.2
//! makes residue a migration BLOCK, so the same fleet member migrates cleanly
//! for one operator and blocks for another, with nothing in the repository to
//! explain the difference.
//!
//! It also fails D29's derivation on its own terms: the rule is "a path git
//! can NEVER commit can never be canon", and a path excluded only by someone's
//! personal config is one `git add` away from being committed by anybody else.
//! This repository already collides with it -- its `.gitignore` carries
//! `!schema/ddl.sql` for no reason other than to defeat that global rule, so a
//! committed, generated, load-bearing schema face was invisible to the corpus
//! on precisely the machines that have it.
//!
//! **This test lives here rather than beside the rest of AT-03.7 because it
//! needs a controlled environment.** The global excludes path comes from git
//! config, which the walker reads from the process environment, and setting a
//! process-wide variable from inside a threaded test binary is both unsafe and
//! racy. Driving the real CLI as a subprocess is the honest way to do it, and
//! it exercises the shipped path rather than a library call.
//!
//! **The fixture proves itself live**, via `git check-ignore` as an oracle: if
//! git itself does not report the probe as ignored under this configuration,
//! the environment plumbing did not take and the test says so instead of
//! passing vacuously. That check is the difference between this test and one
//! that would go green with the global config never loaded at all.

use std::path::Path;
use std::process::Command;

/// Not valid UTF-8, so strict ingest reports it as residue if -- and only if
/// -- it is in corpus.
const NOT_UTF8: &[u8] = b"\xff\xfe\x00Bud1\xff\xfe";

/// Deliberately an extension nobody's real global gitignore carries. `*.sql`
/// would have been truer to vc's report and useless as a fixture: it is in the
/// author's real global, so the test would pass identically whether or not the
/// temporary config was ever read.
const PROBE: &str = "probe.zzz";

struct Fixture {
  dir: tempfile::TempDir,
  home: tempfile::TempDir,
}

impl Fixture {
  fn new() -> Self {
    let dir = tempfile::tempdir().expect("tempdir");
    let home = tempfile::tempdir().expect("home");

    let config = dir.path().join("intent").join(".config");
    std::fs::create_dir_all(&config).expect("mkdir .config");
    std::fs::write(
      config.join("config.json"),
      "{\n  \"intent_version\": \"3.0.0\",\n  \"project_name\": \"Corpus\",\n  \"author\": \"cc\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"rust\"]\n}\n",
    )
    .expect("write config");

    // A global excludes file, reachable only through the temporary git config.
    std::fs::write(home.path().join("excludes"), format!("*{}\n", ".zzz")).expect("excludes");
    std::fs::write(
      home.path().join("gitconfig"),
      format!(
        "[core]\n\texcludesFile = {}\n",
        home.path().join("excludes").display()
      ),
    )
    .expect("gitconfig");

    let ok = Command::new("git")
      .args(["init", "-q"])
      .current_dir(dir.path())
      .status()
      .expect("run git")
      .success();
    assert!(ok, "git init failed");
    // A committed rule, so the run also demonstrates that ignore machinery is
    // switched on at all -- otherwise "nothing was excluded" would be the
    // expected result for the wrong reason.
    std::fs::write(dir.path().join(".gitignore"), "*.bin\n").expect("gitignore");

    Self { dir, home }
  }

  fn root(&self) -> &Path {
    self.dir.path()
  }

  fn write(&self, rel: &str, bytes: &[u8]) {
    let path = self.root().join(rel);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(path, bytes).expect("write");
  }

  fn thread(&self, id: &str) {
    self.write(
      &format!("intent/st/{id}/thread.json"),
      format!(
        "{{\n  \"schema\": \"intent/thread@3.0\",\n  \"id\": \"{id}\",\n  \"title\": \"A thread\",\n  \"status\": \"wip\",\n  \"created\": \"2026-08-14\",\n  \"objective\": \"\",\n  \"context\": \"\"\n}}\n"
      )
      .as_bytes(),
    );
  }

  /// Run a command with the temporary git config in force.
  fn run(&self, program: &str, args: &[&str]) -> (bool, String) {
    let out = Command::new(program)
      .args(args)
      .current_dir(self.root())
      .env("GIT_CONFIG_GLOBAL", self.home.path().join("gitconfig"))
      .env("HOME", self.home.path())
      .output()
      .unwrap_or_else(|e| panic!("run {program}: {e}"));
    (
      out.status.success(),
      format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
      ),
    )
  }
}

#[test]
fn a_global_gitignore_rule_does_not_shrink_the_corpus() {
  let fx = Fixture::new();
  fx.thread("ST0001");
  fx.write(&format!("intent/st/ST0001/{PROBE}"), NOT_UTF8);
  fx.write("intent/st/ST0001/shared.bin", NOT_UTF8);

  // THE ORACLE. git's own answer under this exact configuration. If git does
  // not consider the probe ignored, the temporary global config never took
  // effect and everything below would pass for the wrong reason.
  let (ignored_by_git, _) = fx.run(
    "git",
    &["check-ignore", "-q", &format!("intent/st/ST0001/{PROBE}")],
  );
  assert!(
    ignored_by_git,
    "the fixture is not live: git does not see {PROBE} as ignored under the \
     temporary global config, so this test could not have failed"
  );

  let (_, output) = fx.run(env!("CARGO_BIN_EXE_intent"), &["doctor"]);

  assert!(
    output.contains(PROBE),
    "a file excluded ONLY by the operator's personal global config is freely \
     committable by everyone else, so it is in corpus and strict ingest must \
     account for it -- git ignores it, Intent must not:\n{output}"
  );
  assert!(
    !output.contains("shared.bin"),
    "the control: a rule in the repository's own committed `.gitignore` DOES \
     take a file out of corpus, so this run had ignore machinery switched on \
     and the assertion above is not passing because nothing was ignored at \
     all:\n{output}"
  );
}
