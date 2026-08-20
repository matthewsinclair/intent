//! **THE VERBS THIS BUILD DECLARES AND DOES NOT IMPLEMENT, DRIVEN.**
//!
//! `write_moves_only_what_changed.rs` files three verbs under
//! `DECLARED_BUT_UNWIRED` -- a bucket that says their writes are not *unproven*
//! but provably **empty**, because the binary refuses them. That claim needs
//! something behind it, and this is it.
//!
//! # Why it is here and not in the roster file
//!
//! The roster lives in `intentsvcs`, which cannot spawn the CLI. The refusal is
//! the CLI's -- `render::unwired` -- so the only place the claim can be MEASURED
//! is this crate. An in-crate substitute would have had to ask whether
//! `Facade::dehydrate` exists by NAME, and ST0057 AC-08.5 already paid for that
//! lesson: a pin grepping `facade.rs` for `fn at_new`/`fn ac_new` passed while
//! `put` created both rows thirty lines away in the same file. **A capability
//! arriving under an unlisted name is invisible to a name check.** Running the
//! verb is not.
//!
//! # What makes it self-invalidating
//!
//! Nothing here asserts that these verbs *should* stay unbuilt. It asserts that
//! TODAY they refuse -- so the day one is implemented it stops exiting 2, this
//! file goes RED, and whoever implemented it is forced to move it out of the
//! bucket. **That is the property an authored excuse-list can never have**: a
//! live mutator cannot sit quietly in a list of things that write nothing.
//!
//! The converse direction is covered by the roster's own enumeration clause: a
//! NEW declared-but-unwired verb lands in zero buckets and fails there.

use std::path::Path;
use std::process::Command;

/// The phrase `render::unwired` emits, and `guide.rs` teaches an agent to
/// recognise. Duplicated as a literal deliberately: if the wording changes,
/// this file must notice, because the guide's contract with its readers is the
/// exact string.
const PHRASE: &str = "is a known command that is not implemented yet";

/// Every verb `DECLARED_BUT_UNWIRED` claims writes nothing.
///
/// **Kept in the same order and spelling as the roster's list**, because the
/// two are one claim in two crates and a reader comparing them should not have
/// to sort.
const UNWIRED: &[&str] = &["st dehydrate", "issues hydrate", "issues dehydrate"];

fn bin() -> &'static Path {
  Path::new(env!("CARGO_BIN_EXE_intent"))
}

/// Run in THIS repository, which is a real v3 project.
///
/// A verb that refuses before it looks at a project would pass in an empty
/// temp dir for the wrong reason -- the refusal must be the unwired one, not
/// "there is no project here". Running where a project exists removes that
/// alternative explanation.
///
/// **NOTHING THIS FILE DRIVES MAY WRITE, AND THAT IS CHECKED BY READING THE
/// DISPATCH ARM, NOT ASSUMED.** `render::unwired` is reached from the family
/// `match` BEFORE `open()`, so a refused verb never constructs a `Facade` and
/// cannot touch `.intentfiles` or the estate. The control below is read-only
/// for the same reason. This matters more than it looks: driving
/// `issues hydrate 0001` by hand on 2026-08-20 pinned `ISSUE:0001` into the
/// live manifest, which is exactly the write a test must never make.
fn run(verb: &str, id: &str) -> (Option<i32>, String) {
  let root = Path::new(env!("CARGO_MANIFEST_DIR"))
    .ancestors()
    .nth(4)
    .expect("the project root is four above crates/intent-cli");
  let mut args: Vec<&str> = verb.split(' ').collect();
  args.push(id);
  let out = Command::new(bin())
    .args(&args)
    .current_dir(root)
    .output()
    .unwrap_or_else(|e| panic!("could not run `intent {verb}`: {e}"));
  let said = String::from_utf8_lossy(&out.stderr).into_owned()
    + &String::from_utf8_lossy(&out.stdout).into_owned();
  (out.status.code(), said)
}

/// **THE CRITERION: each one refuses, at 2, by name.**
///
/// Every failure is collected before asserting. Three verbs share one refusal
/// path, so if it breaks they all break -- and seeing that it is all three
/// rather than one is the difference between "this verb got implemented" and
/// "the unwired arm regressed".
#[test]
fn every_declared_but_unwired_verb_refuses_at_two() {
  let mut failures: Vec<String> = Vec::new();

  for verb in UNWIRED {
    // An id of the right SHAPE for the family, so nothing can pass because the
    // argument was malformed: `st` takes a thread id, `issues` an issue id. A
    // usage error is exit 1, and it would mask exactly the transition this file
    // exists to catch.
    let id = if verb.starts_with("st ") {
      "ST0056"
    } else {
      "0001"
    };
    let (code, said) = run(verb, id);

    if code != Some(2) {
      failures.push(format!(
        "  `intent {verb} {id}` exited {code:?}, not 2 -- if it is now IMPLEMENTED, move it out of \
         DECLARED_BUT_UNWIRED in write_moves_only_what_changed.rs and give it a real bucket; \
         it said: {}",
        said.lines().next().unwrap_or("<nothing>")
      ));
      continue;
    }
    if !said.contains(PHRASE) {
      failures.push(format!(
        "  `intent {verb} {id}` exited 2 without saying `{PHRASE}` -- a 2 from another cause is \
         not evidence that this verb writes nothing; it said: {}",
        said.lines().next().unwrap_or("<nothing>")
      ));
    }
  }

  assert!(
    failures.is_empty(),
    "{} of {} declared-but-unwired verb(s) no longer refuse:\n{}",
    failures.len(),
    UNWIRED.len(),
    failures.join("\n")
  );
}

/// **THE POSITIVE CONTROL, AND IT IS NOT OPTIONAL HERE.**
///
/// Every assertion above is that something REFUSES. A binary that refused
/// everything -- a broken dispatch table, a spine that built no verbs, a
/// `current_dir` that is not a project -- would satisfy all of them, and the
/// suite would report a green that is a fact about the harness rather than
/// about the verbs. So one verb that IS wired, driven the same way, must come
/// back not-2.
///
/// **`st show`, NOT `st hydrate`, AND THE REASON IS A RULE THIS FILE MUST NOT
/// BREAK TO TEST ITSELF.** The nearest neighbour by family and argument grammar
/// is `st hydrate` -- same `st`, same thread id, wired the same morning -- and
/// it is a MUTATOR. Driving it here would have this file pin and materialise
/// against the developer's own checkout on every `cargo test`, which is the
/// write the harness note above forbids. `st show` is the same family, takes
/// the same id, is equally wired, and reads.
///
/// **The control is weaker for it and still sufficient**, because its whole job
/// is to refute ONE alternative explanation -- that the harness refuses
/// everything -- and a read verb refutes that exactly as well as a write one.
/// Stated rather than quietly substituted: a control chosen for convenience and
/// described as the ideal one is how a weaker check comes to be read as a
/// stronger one.
#[test]
fn the_control_verb_is_wired_and_does_not_refuse() {
  let (code, said) = run("st show", "ST0056");
  assert_ne!(
    code,
    Some(2),
    "`intent st show ST0056` refused at 2 -- either it was un-wired, or this harness refuses \
     everything and every other assertion in this file is vacuous; it said: {said}"
  );
  assert!(
    !said.contains(PHRASE),
    "`intent st show ST0056` is wired but still says `{PHRASE}`: {said}"
  );
}
