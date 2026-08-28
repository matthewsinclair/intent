//! AC-10.7, at the surface: NO shipped command answers a question about an
//! unmigrated project.
//!
//! The facade-level test (`intentsvcs/tests/unmigrated_project.rs`) proves the
//! gate refuses. It cannot prove that no command gets past the gate -- that is
//! a claim about the shape of the CLI, and the only honest way to make it is
//! to drive every command in the dispatch table and see.
//!
//! **Derived from the table, never hand-listed.** A hand-list certifies the
//! status quo: it is complete on the day it is written and silently incomplete
//! from the next verb onward, which is precisely how WP-06's long tail would
//! reintroduce this defect one command at a time. Reading the table means a
//! newly-wired verb is REQUIRED to be guarded the moment it is wired, and the
//! failure names it.
//!
//! **The property is the exit code, not the text**, and that is a correction
//! to this test's first draft. Classifying by what a command PRINTED meant
//! deciding whether "`ST0001` is not a work package" counted as guarded, and
//! every such judgement is a place to accidentally widen the pass condition
//! until the test agrees with whatever the code does. The defect being closed
//! was `intent st list` exiting **0** over an estate it could not see, so the
//! invariant is exactly that: on an unmigrated project, no command SUCCEEDS.
//! A refusal exits 1, an unwired verb exits 1, a usage error exits 1 -- only a
//! command that answered exits 0, and answering is the bug.
//!
//! The message-level counts below are reported rather than asserted, except
//! for one: at least one command must have been observed naming the migration.
//! Without that, a classification bug could make every command look fine by
//! making none of them run.

use std::path::Path;
use std::process::Command;

use intent_cli::dispatch;

/// Commands exempt from the unmigrated refusal. Each is exempt for a stated
/// reason -- an allow-list without reasons is a place to hide a bug.
///
/// # It was called `reads_no_model` and the name lost a member
///
/// **Every entry below except one is exempt because it never opens a project,
/// and `critic` is exempt for a different reason entirely: it DOES read the
/// project, and must still not refuse.** Under the old name there was no
/// honest way to add it -- it would have been either a false claim about
/// `critic` or an exemption invented somewhere else, and the second is worse
/// because the reason would then live nowhere.
///
/// **This estate has the precedent written down already.**
/// `gen_dispatch_table.sh` records `SURFACE_NONRETURNING` losing `claude
/// upgrade`, which returns perfectly well and writes into the operator's real
/// `~/.claude` -- two reasons, one name, and the member that did not fit the
/// name fell out. A `why` per member was the remedy there and it is the remedy
/// here; the name now describes the DECISION and each reason states its own
/// ground.
///
/// So there are two grounds, and a new entry must say which:
///
/// 1. **it never reads a project** -- the original list, unchanged.
/// 2. **it reads the project and must not refuse on this state** -- because
///    its consumer fails CLOSED on the refusal, so refusing does more damage
///    than answering. `critic` alone, and see its entry for why.
fn exempt_from_the_migration_refusal(path: &str) -> Option<&'static str> {
  match path {
    // Prints the generated schema faces, which are compiled in via
    // `include_str!` and are the same bytes in every project.
    "schema" => Some("faces are compile-time constants; it never reads a project"),
    // Creates a project. Refusing before one exists would be a bootstrap
    // deadlock, and it is WP-06's remaining work in any case.
    "init" | "bootstrap" => Some("creates a project rather than reading one"),
    // The migrator itself, and the thing whose whole job is this state.
    "upgrade" => Some("it is the remedy; refusing it would leave no way out"),
    "doctor" => Some("reports the migration as a finding rather than refusing"),
    // Stdio and socket servers: wiring these later must not hang this test.
    "mcp" | "daemon" => Some("long-running servers -- excluded by construction"),
    "version" | "info" | "help" => Some("tool-level, not project-level"),
    // The agent guide is generated from the dispatch table compiled into this
    // binary -- the same category as `schema`, and verified rather than
    // assumed: `guide.rs` touches no facade and opens no project. **It is the
    // one command an agent in an unmigrated project needs MOST**, because the
    // guide is where `intent upgrade` is written down; refusing it would
    // withhold the instructions for the state the reader is stuck in.
    // (Exemption added by cc with the wiring that made it reachable; ic owns
    // the renderer and this list, and should reword if the framing is wrong.)
    // **AND BARE `intent llm` IS THE SAME COMMAND.** ST0067 AC-00.1 put both
    // doors on one match arm, so bare `llm` emits the byte-identical document
    // -- asserted, not assumed, by `bare_llm_and_llm_guide_serve_the_same_document`.
    // The reason above therefore applies to it verbatim; exempting the bytes
    // under one spelling and refusing them under the other would be a
    // distinction the surface does not make.
    //
    // **This row is EXACT-PATH and must not become family-wide** -- see the
    // skip at the sweep below. `llm usage_rules` reads the project and keeps
    // refusing here.
    "llm" | "llm guide" => Some("generated from the compiled-in table; it never reads a project"),
    // The rule library belongs to the INSTALL, not to any project -- the same
    // category as `schema` and `llm guide`, and verified rather than assumed:
    // `rules.rs` resolves its roots from `install::home()` and `ext::base()`,
    // touches no facade and opens no project.
    //
    // **AND IT IS THE SECOND COMMAND AN AGENT IN AN UNMIGRATED PROJECT NEEDS
    // MOST, for the same reason `llm guide` is the first.** `CLAUDE.md` and
    // `AGENTS.md` tell every agent to read the four rules of the road with
    // `intent claude rules show <id>`; those files are laid down by the
    // migration and are the FIRST thing an agent meets in a project that has
    // not been migrated yet. Refusing here would hand an agent a contract and
    // withhold the contract's text, in exactly the state most likely to
    // violate it.
    //
    // **The gating question is `does it read the project`, never `does it look
    // project-scoped`** -- `intent claude` reads as project-scoped and none of
    // this family's shipped verbs are. `claude hook` execs an install script;
    // this serves install assets. A sibling verb that DOES open a project must
    // not inherit this exemption, which is why the entry is the full path and
    // not the family.
    "claude rules" => Some("serves install assets; it never reads a project"),
    // Same ground as `claude rules`, verified the same way rather than assumed
    // by family: `skills.rs` takes its roots from `install::home()` and its
    // per-user paths from `userstate`, touches no facade and opens no project.
    //
    // **AND THE REASON IT MUST NOT REFUSE HERE IS SHARPER THAN CONVENIENCE.**
    // Skills are per-INSTALL and per-USER. An operator on a fresh machine, or
    // standing in a project they have not migrated, is exactly who needs to
    // install the skills that tell an agent how to migrate it. Refusing would
    // make the tool that lays down the contract unreachable from the state the
    // contract is for -- the same argument `llm guide` and `claude rules`
    // already carry, and the third member of one class rather than a new one.
    // (Exemption added by cc with the wiring that made it reachable; ic owns
    // this list and should reword if the framing is wrong.)
    "claude skills" => Some("serves install assets into per-user state; it never reads a project"),
    // Plugin manifests belong to the INSTALL -- `<install>/intent/plugins/*/
    // plugin.json` -- so this is the same class as `schema`, `llm guide`,
    // `claude rules` and `claude skills`, and it is verified rather than
    // assumed by family: `plugins.rs` takes its root from the install path the
    // renderer resolved via `install::home()`, and neither `plugin` arm calls
    // `open()` or touches a facade.
    //
    // **THIS ONE IS DELIBERATELY THE FAMILY AND NOT THE PATH**, against the
    // rule three entries above, and the difference is checkable rather than a
    // matter of taste: `claude` was pathed because it HAS project-scoped
    // siblings to protect against, and `plugin` has three shipped verbs --
    // bare, `list`, `show` -- of which all three read the install and none
    // reads a project. A sibling that ever DOES open a project breaks that,
    // and the entry must be split the moment one is added.
    //
    // v2 answers `plugin` outside a project at exit 0, so this is the observed
    // contract rather than a v3 concession.
    // (Exemption added by cc with the wiring that made it reachable; ic owns
    // this list and should reword if the framing is wrong.)
    "plugin" => Some("reads the install's plugin manifests; it never reads a project"),
    // **`lang` IS THE FAMILY THAT PROVES THE ENTRY ABOVE HAD TO EARN ITS FORM.**
    // `plugin` is exempt as a FAMILY because all three of its shipped verbs read
    // the install. `lang` ships four and they SPLIT: `list` and `show` answer
    // from `rules::declarable()`, a compile-time registry; `init` and `remove`
    // write `intent/.config/config.json`. A family-level entry here would exempt
    // precisely the two verbs that mutate.
    //
    // **AND IT WOULD HAVE EXEMPTED A LIVE DEFECT.** Wired first without the
    // migration gate, `intent lang init rust` in an unmigrated v2 project exited
    // 0 and rewrote its config into v3 shape -- `author`, `intent_dir` and the
    // `todo` block added, `intent_version: 2.19.0` left in place. This test
    // caught it. A family-level exemption would have silenced the report and the
    // half-migration would have shipped.
    //
    // `show` is listed even though it currently refuses the placeholder argument
    // on its own merits: it refuses because `ST0001` is not a language, not
    // because the project is unmigrated, so it passes this sweep for a reason
    // unrelated to what the sweep asks. A placeholder that ever became a real
    // language would turn that accident into a red with no defect behind it.
    // (Exemptions added by cc with the wiring that made them reachable; ic owns
    // this list and should reword if the framing is wrong.)
    "lang" => Some("prints its own usage; it reads neither the install nor a project"),
    // **`modules` IS THE FAMILY HEAD ALONE, AND THAT IS THE POINT OF CONTRAST.**
    // `plugin` is exempt as a family (all three verbs read the install); `lang`
    // takes three of four (two verbs read a compile-time registry); `modules`
    // takes ONE OF THREE. Both of its verbs read `intent/llm/MODULES.md`, which
    // is a PROJECT file, so neither ground applies to them and neither is
    // listed. Only the bare head is exempt, and only because it prints usage.
    //
    // **The fallback is written down, which is what makes refusing safe here.**
    // `CLAUDE.md` tells an agent to drive `intent modules find` and to fall back
    // to a grep of `MODULES.md` if it does not answer -- so a refusal in an
    // unmigrated project costs a reader one documented step, where `llm guide`
    // and `claude rules` would have withheld the instructions for the very state
    // the reader is stuck in.
    // (Exemption added by cc with the wiring that made it reachable; ic owns
    // this list and should reword if the framing is wrong.)
    "modules" => Some("prints its own usage; it reads neither the install nor a project"),
    "lang list" => Some("lists a compile-time language registry; it never reads a project"),
    "lang show" => Some("describes a compile-time language registry; it never reads a project"),
    // **GROUND 2, AND THE ONLY MEMBER OF IT.** `critic` READS the project --
    // `languages` out of `intent/.config/config.json`, the threshold out of
    // `.intent_critic.yml` -- so it fails every test the entries above pass.
    // It is exempt because of what happens DOWNSTREAM of a refusal.
    //
    // `Facade::open` calls `readable()` before anything else, so a `critic`
    // built the obvious way returns `Unmigrated -> Failure::Error -> 1` in
    // every unmigrated project. **The shipped pre-commit gate reads 1 as
    // FINDINGS and BLOCKS**, printing a remedy about findings that do not
    // exist while the true remedy -- run `intent upgrade` -- sits on screen
    // above it, overridden. That is issue 0038 rebuilt on the git side, and
    // every project is unmigrated until WP-10 runs on it.
    //
    // **So this is not a claim that `critic` sees the estate correctly here.**
    // It answers over a project whose model it has not migrated, and the
    // grounds are that the alternative is worse, not that the answer is
    // authoritative. `exit_codes.rs`'s `an_unmigrated_project_can_still_commit`
    // is the other end of this and drives the shipped hook rather than the
    // number; issue 0045 (vc) is the record.
    "critic" => Some(
      "its consumer FAILS CLOSED on a refusal -- the gate reads 1 as findings and blocks every        commit in every unmigrated project (issue 0038 / 0045)",
    ),
    _ => None,
  }
}

fn legacy_project() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let config = dir.path().join("intent").join(".config");
  std::fs::create_dir_all(&config).expect("mkdir .config");
  std::fs::write(
    config.join("config.json"),
    "{\n  \"intent_version\": \"2.19.0\",\n  \"project_name\": \"Legacy\",\n  \"author\": \"matts\",\n  \"intent_dir\": \"intent\",\n  \"languages\": [\"shell\"]\n}\n",
  )
  .expect("write config");
  let thread = dir.path().join("intent").join("st").join("ST0001");
  std::fs::create_dir_all(&thread).expect("mkdir thread");
  std::fs::write(
    thread.join("info.md"),
    "---\nstatus: In Progress\n---\n\n# ST0001: a real thread\n",
  )
  .expect("write v2 info.md");
  dir
}

/// Well-formed placeholder arguments, derived from each argument's declared
/// TYPE in the table.
///
/// By type rather than by position, because a malformed argument is rejected
/// before the project is ever opened -- so a lazy placeholder does not make
/// the test lenient, it makes it SHALLOW: the command never reaches the gate,
/// and the run proves nothing about it while looking like it did. The ids need
/// not exist; the gate sits at `Facade::open`, ahead of any lookup.
fn placeholder_args(entry: &dispatch::Entry) -> Vec<String> {
  let mut argv = Vec::new();
  for arg in &entry.args {
    // An optional slot with nothing to put in it -- a family header's verb.
    if arg.kind == "subcommand" && arg.values.is_empty() {
      continue;
    }
    if let Some(first) = arg.values.first() {
      argv.push(first.clone());
      continue;
    }
    argv.push(
      match arg.kind.as_str() {
        "st-id/NN" => "ST0001/01",
        "st-id" => "ST0001",
        "ac-id" => "AC-01.1",
        "at-id" => "AT-01.1",
        _ => "ST0001",
      }
      .to_string(),
    );
    // One value satisfies a variadic slot.
    if arg.arity.contains('n') || arg.arity.contains('+') {
      break;
    }
  }
  argv
}

fn run(root: &Path, argv: &[String]) -> (i32, String) {
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(argv)
    .current_dir(root)
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run the v3 binary");
  let merged = format!(
    "{}{}",
    String::from_utf8_lossy(&out.stdout),
    String::from_utf8_lossy(&out.stderr)
  );
  (out.status.code().unwrap_or(-1), merged)
}

#[test]
fn no_shipped_command_answers_from_an_unmigrated_project() {
  let dir = legacy_project();
  let table = dispatch::table();

  let mut answered: Vec<String> = Vec::new();
  let mut guarded = 0usize;
  let mut unwired = 0usize;
  let mut exempt = 0usize;
  let mut rejected = 0usize;

  for entry in dispatch::shipped_entries(&table) {
    let family = entry.path.split(' ').next().unwrap_or_default();
    // **A FAMILY EXEMPTION COVERS EVERY VERB UNDER IT, AND `llm`'s MUST NOT.**
    // Bare `intent llm` is exempt because it serves the compiled-in guide; its
    // sibling `llm usage_rules` opens the project and has to keep refusing
    // here. Letting the family fallback answer for `llm` would retire a live
    // check as a side effect of exempting a different command -- the quiet way
    // a guard's population shrinks while it still reports a pass.
    let by_family = (family != "llm")
      .then(|| exempt_from_the_migration_refusal(family))
      .flatten();
    if exempt_from_the_migration_refusal(&entry.path).is_some() || by_family.is_some() {
      exempt += 1;
      // THE SKIP MUST STAY AHEAD OF `run()`, and it is load-bearing rather than
      // an efficiency. Every entry in this loop shares ONE `legacy_project()`
      // fixture, so an exempt command that actually executed could CHANGE it
      // mid-sweep -- `upgrade` migrates it, and every verb ordered after would
      // then answer legitimately instead of refusing. The guard would go red
      // for a reason that is not the guard, on a command nobody had touched.
      //
      // Recorded because the property is currently true by accident of
      // ordering and nothing else in the file says so. Checked when `upgrade`
      // was wired (cc, 2026-08-17): it was exempt, the `continue` fired first,
      // and no change was needed here -- safe by this loop's construction, not
      // by anyone having thought about it at the time.
      continue;
    }

    let mut argv: Vec<String> = entry.path.split(' ').map(str::to_string).collect();
    argv.extend(placeholder_args(entry));
    let (code, output) = run(dir.path(), &argv);

    // THE assertion, collected rather than tripped one at a time so a failure
    // names every offender in one run instead of one per fix-and-rerun.
    if code == 0 {
      answered.push(format!(
        "`intent {}` SUCCEEDED (exit 0) over an estate it cannot see -- {}",
        argv.join(" "),
        if output.trim().is_empty() {
          "and printed nothing at all".to_string()
        } else {
          output
            .trim()
            .lines()
            .take(2)
            .collect::<Vec<_>>()
            .join(" / ")
        }
      ));
      continue;
    }
    if output.contains("has not been migrated") {
      guarded += 1;
    } else if output.contains("not yet wired to the facade") {
      unwired += 1;
    } else {
      rejected += 1;
    }
  }

  assert!(
    answered.is_empty(),
    "on an unmigrated project no command may succeed:\n{}",
    answered.join("\n")
  );
  assert!(
    guarded > 0,
    "no command was observed naming the migration, so this run proved nothing \
     -- suspect the sweep, not the code"
  );
  eprintln!("guarded {guarded}, unwired {unwired}, rejected-on-args {rejected}, exempt {exempt}");
}

/// The refusal goes to STDERR and the exit code is 1.
///
/// Worth pinning separately: the defect this closes was a command exiting 0
/// with nothing on either stream, so "it printed something somewhere" is not
/// the property. A script doing `intent st list > threads.txt` must get a
/// non-zero status and an empty file, not a file full of error text.
#[test]
fn the_refusal_is_a_failure_on_stderr_not_output_on_stdout() {
  let dir = legacy_project();
  let out = Command::new(env!("CARGO_BIN_EXE_intent"))
    .args(["st", "list"])
    .current_dir(dir.path())
    .env("HOME", testkit::fixture_home())
    .output()
    .expect("run");

  assert_eq!(out.status.code(), Some(1), "INV-02: refusals exit 1");
  assert!(
    String::from_utf8_lossy(&out.stdout).is_empty(),
    "a refusal writes nothing to stdout"
  );
  let stderr = String::from_utf8_lossy(&out.stderr);
  assert!(stderr.starts_with("error: "), "v2's voice: {stderr}");
  assert!(
    stderr.contains("ST0001"),
    "and it names the work it cannot see, which is what distinguishes this \
     from an empty project: {stderr}"
  );
}
