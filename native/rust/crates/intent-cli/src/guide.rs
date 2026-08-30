//! The agent guide -- `intent llm guide` (AC-09.4).
//!
//! **The command reference is GENERATED from the dispatch table at render
//! time.** There is no committed guide file and no hand-maintained command
//! list, which is the whole of the criterion: completeness comes from
//! iterating [`dispatch::shipped_entries`], so a command cannot be omitted by
//! forgetting that the guide exists.
//!
//! That is the failure mode this replaces, and it was measured rather than
//! assumed (`surface/agent-guide.spec.md`, 2026-08-15). v2's `usage-rules.md`
//! named 54 of the surface's 111 commands and invented none -- so the defect
//! of a hand-written list is not drift into falsehood, it is **silent
//! omission**, and omission is the worse of the two for an agent. A wrong
//! command earns an error the agent can react to; a missing one reads as a
//! capability the tool does not have, and the agent quietly builds a
//! workaround. Three whole families (`issues`, `modules`, `lang`) were absent
//! as commands, and the guide never named `intent llm` -- the command that
//! prints it.
//!
//! **Shipped, not declared, and the difference is five commands.** The table
//! is a PARITY REGISTER before it is a command list: a row records that the
//! question was asked about v2, never that the answer was yes. Rendering every
//! DECLARED row would tell an agent to call `intent st_zero`, which hv
//! explicitly ruled dead.
//!
//! **Why this is not in `render.rs`.** That module is parse -> facade ->
//! render and nearly every function in it opens a project; this one touches no
//! facade, needs no project, and reads only the table compiled into the
//! binary. It is also the hottest file in the tree.
//!
//! ## The projection, and why it is in this order
//!
//! Not the projection a human help screen wants. Ruled by D45 (hv,
//! 2026-08-16): _"the CLI is the precise surface and the MCP layer is the
//! imprecise one. A skill drives `intent` directly."_
//!
//! 1. `read_or_mutate` -- the safety constraint, before the description.
//! 2. the call, then what it does, then its flags.
//! 3. `exposed_on_mcp` -- a ROUTING note, and nothing more.
//! 4. surface-wide facts, stated ONCE and never per row.
//!
//! **`exposed_on_mcp` was first in that list until D45**, glossed "may an
//! agent call this at all". Under the ruling that gloss is not merely
//! mis-emphasised, it is false: the agent's default route is the CLI, all 107
//! shipped rows are on it, and the flag withholds a row from the imprecise
//! alternative rather than from the agent. Leading with a false gate would
//! have taught its reader that 26 of 107 rows were closed -- a larger silent
//! omission than v2's, arrived at by generating rather than by forgetting.
//! **Completeness of the ROW SET comes for free; the truth of each rendered
//! field does not, and no generator will ever check it.**

use crate::dispatch::{self, Arg, Entry, Flag, Table};
// **THE PHRASE IS IMPORTED, NEVER RE-TYPED, AND THE DISCRIMINATOR IS WHAT THIS
// FILE IS ABOUT.** `exit_codes.rs` holds its own literal because the phrase is
// that file's SUBJECT -- a witness importing the value it asserts states that
// the emitter equals itself. Here the phrase is INCIDENTAL: this file is about
// the guide, and it cites what the emitter says. So a copy here is a second
// home that drifts silently, and the import makes drift impossible rather than
// catchable. (dc's `pub const` landed at `ca744f1d`; its doc comment already
// said this import existed, and it did not -- so `render.rs` read finished
// while this file was still typing the literal out.)
use crate::render::UNWIRED_PHRASE;
use crate::spine::Failure;

/// Render the guide.
///
/// `Result` rather than `String` because the surface-wide section reads its
/// facts out of the table BY ID, and a lookup that finds nothing must refuse
/// rather than render a guide with a hole in it. Same posture as the argument
/// lookups in `render.rs`: a build defect says so, loudly, in the terminal.
/// **THE CRATE'S OWN ERROR TYPE, NOT A `String` AND NOT A NEW ONE**
/// (IN-RS-CODE-004). These three signatures used to carry a bare `String` as the
/// error, which the rule flags because such an error cannot be matched on -- a
/// caller wanting to treat one cause differently has to parse prose.
///
/// **THE PRIOR SPELLING IS DESCRIBED RATHER THAN QUOTED, AND THAT IS NOT
/// SQUEAMISHNESS.** IN-RS-CODE-004's greppable proxy matches the literal type
/// expression, and it reads comments -- so writing the old signature out here
/// tripped the rule inside the doc explaining its own remedy, and the pre-commit
/// gate blocked the fix. A checker that cannot separate code from prose makes
/// DOCUMENTING a defect an instance of it, which is the same shape as a guard
/// that would refuse a whiteboard message reporting a bad timestamp.
///
/// `Failure` rather than a `thiserror` enum of this module's own, for two
/// reasons that point the same way. `intent-cli` is a BINARY and carries no
/// `thiserror` dependency; adding one to give a single-variant error its own type
/// is ceremony the rule's own `does_not_apply_when` disclaims. And `Failure` is
/// already this crate's one error vocabulary, carrying the exit code with it --
/// a second type beside it would be the Highlander violation, and the call site
/// in `render.rs` was already converting into it by hand.
pub fn render(table: &Table) -> Result<String, Failure> {
  let mut out = String::new();

  out.push_str(&format!(
    "# intent -- the agent guide\n\n{}\n\n",
    table.root_help
  ));
  out.push_str(HOW_TO_READ);
  out.push_str(&surface_wide(table)?);
  out.push_str(&command_reference(table));
  out.push_str(AUTHORED_HALF);

  Ok(out)
}

const HOW_TO_READ: &str = "\
The command reference below is generated from the dispatch table compiled into \
this binary, so it lists exactly what this build ships -- no more, and nothing \
missing. Each command carries, in this order:

- **safety** -- `mutate` if invoking it can change durable state, `read` if not. A mutation also says whether this surface can put the estate back: **reversible** (another command undoes it), **idempotent** (running it again is the same state), or **ONE-WAY** (nothing here undoes it). Treat one-way as needing a human.
- **call** -- the path and its positional arguments. `<x>` is required, `[x]` optional, `...` repeatable.
- **does** -- what the command is for.
- **flags** -- omitted when the command takes none.
- **mcp** -- whether the MCP tool surface also carries it. This is a note about ROUTING, not permission: the CLI is the precise surface and every command below is on it.

";

/// The facts that are true of the whole surface, stated once.
///
/// **Selected by ID, with the text read from the table.** The selection is
/// authored -- it has to be, because `invariants` conflates two kinds of claim
/// and nothing in the schema separates them: INV-01..04 are v3's contract,
/// INV-05..08 are measurements of v2 defects being CORRECTED (`error ...;
/// usage` unreachable, a fifth of failures on the wrong stream). Rendering all
/// eight would tell an agent that v3 writes failures to stdout.
///
/// So the four IDs are named here and their titles are not. A retitled
/// invariant reaches this section; a deleted or renamed one refuses the render
/// by name instead of quietly dropping a fact an agent parses exit codes on.
fn surface_wide(table: &Table) -> Result<String, Failure> {
  let inv = |id: &str| -> Result<&str, Failure> {
    table
      .invariants
      .iter()
      .find(|i| i.id == id)
      .map(|i| i.title.as_str())
      .ok_or_else(|| {
        // Byte-identical to the string this replaced, leading `error: ` included:
        // the prefix belongs to the output contract, and moving it in the same
        // change as the type would make a parity failure impossible to attribute.
        Failure::Error(format!(
          "error: the agent guide cites invariant `{id}`, which the dispatch table does not declare\n  remedy: this is a build defect -- the renderer and surface/dispatch-table.json disagree"
        ))
      })
  };

  Ok(format!(
    "\
## Facts about the whole surface

- **{}** ({}). `0` is success. `1` means the command RAN and the answer is no -- a refused verb, a blocked gate, a usage error. **`2` means this build cannot answer the question at all, and it NEVER carries a verdict about your work** -- which is exactly why the shipped pre-commit gate fails open on it. The two you will meet most often are a command that is declared but not implemented yet, which says `{UNWIRED_PHRASE}` on stderr, and `intent critic` rejecting an invocation it cannot act on, such as a language it does not know. **That is not a closed list, and this sentence deliberately no longer offers one**: what makes a `2` a `2` is the tool being unable to answer, not membership of an enumeration -- an OS call failing, a flag whose feature is unbuilt, and a `--daemon` that cannot be reached are all the same fact about the tool. **Never read `2` as a verdict about your code, and never read `1` as a broken run.**
- **{}** ({}). Results go to stdout; failures go to stderr with a lowercase `error: ` prefix. Nothing is banner-wrapped.
- **{}** ({}). A usage error -- an unknown flag, a missing argument -- exits `1`, not clap's default of 2.
- **{}** ({}). A command that needs to be inside an Intent project says so plainly when it is not, rather than half-working.
- **`--help` works on every command**, at every level, and is not listed per row below. clap supplies it to all of them, while only ten rows declare it -- so a per-row rendering would under-report it.

",
    inv("INV-04")?,
    "INV-04",
    inv("INV-01")?,
    "INV-01",
    inv("INV-02")?,
    "INV-02",
    inv("INV-03")?,
    "INV-03",
  ))
}

/// Every shipped command, grouped by family in table order.
///
/// Grouping is by first appearance rather than by the `families` array,
/// because `new_surface` rows reach the surface through the same
/// [`dispatch::shipped_entries`] and belong beside their siblings: `intent llm
/// guide` is a `new_surface` row and an agent looking under `llm` must find
/// it. From the operator's side there is no difference between a ported
/// command and an added one.
fn command_reference(table: &Table) -> String {
  let mut out = String::from("## Commands\n\n");
  let entries = dispatch::shipped_entries(table);

  let mut families: Vec<&str> = Vec::new();
  for e in &entries {
    if !families.contains(&e.family()) {
      families.push(e.family());
    }
  }

  for family in families {
    out.push_str(&format!("### {family}\n\n"));
    for entry in entries.iter().filter(|e| e.family() == family) {
      out.push_str(&entry_block(entry));
    }
  }
  out
}

fn entry_block(entry: &Entry) -> String {
  let mut out = format!("#### intent {}\n\n", entry.path);

  out.push_str(&format!("- **safety:** {}\n", safety(entry)));
  out.push_str(&format!("- **call:** `{}`\n", call(entry)));
  out.push_str(&format!("- **does:** {}\n", entry.help));

  let flags: Vec<&Flag> = entry.flags.iter().filter(|f| f.ships()).collect();
  if !flags.is_empty() {
    out.push_str("- **flags:**\n");
    for flag in flags {
      out.push_str(&format!("  - {}\n", flag_line(flag)));
    }
  }

  if !entry.aliases.is_empty() {
    out.push_str(&format!(
      "- **also spelled:** {}\n",
      entry
        .aliases
        .iter()
        .map(|a| format!("`intent {a}`"))
        .collect::<Vec<_>>()
        .join(", ")
    ));
  }

  out.push_str(&format!(
    "- **mcp:** {}\n\n",
    if entry.exposed_on_mcp {
      "also a tool on the MCP surface"
    } else {
      "CLI only -- call it directly"
    }
  ));
  out
}

/// `read` and `mutate` are the two declared values, and an unrecognised third
/// is rendered AS ITSELF rather than folded into either.
///
/// **Defaulting an unknown value to `read` would present an unclassified
/// command as safe to call unattended**, which is the exact reason the field
/// carries no `serde(default)` one layer down. Defaulting it to `mutate` would
/// be safe and would also hide the defect. Printing the raw value makes a
/// typo visible to the one reader who can act on it.
fn safety(entry: &Entry) -> String {
  match entry.read_or_mutate.as_str() {
    "read" => "`read` -- cannot change durable state".to_string(),
    "mutate" => format!(
      "`mutate` -- can change durable state; {}",
      recoverability(entry)
    ),
    other => format!("`{other}` -- UNDECLARED VALUE; treat as a mutation until it is classified"),
  }
}

/// What the surface can do about a mutation after the fact.
///
/// **On the SAFETY line rather than in a field of its own, because it is the
/// second half of one question.** "Can this change durable state" and "can it
/// be undone" are read together or not at all -- an agent deciding whether to
/// call `lang remove` unattended needs both, and a guide that separates them by
/// four bullets has put the mitigating half where the alarming half is already
/// read.
///
/// **A mutation with NO declared recoverability says so.** It cannot happen
/// through the committed table -- `check_vocabularies` refuses the load, and
/// `gen_dispatch_table.sh` refuses the generate -- so this arm exists for the
/// hand-built tables the tests drive, and because a renderer whose fallback is
/// silence is the failure this whole file was written against.
fn recoverability(entry: &Entry) -> String {
  match entry.recoverability.as_deref() {
    Some("reversible") => "**reversible** -- another command on this surface undoes it".to_string(),
    Some("idempotent") => "**idempotent** -- running it again produces the same state".to_string(),
    Some("one-way") => {
      "**ONE-WAY** -- nothing on this surface puts back what it changes".to_string()
    }
    Some(other) => {
      format!("**`{other}`** -- UNDECLARED VALUE; treat as one-way until it is classified")
    }
    None => "recoverability UNDECLARED -- treat as one-way until it is classified".to_string(),
  }
}

/// The call line: the path, then its positionals.
///
/// Flags are deliberately absent here even though three of them are
/// `required`. A usage line carrying every keep-flag runs past 200 characters
/// on the widest rows and stops being read; the flag list below marks the
/// required ones, in the place a reader is already looking for the spelling.
fn call(entry: &Entry) -> String {
  let mut parts = vec![format!("intent {}", entry.path)];
  for arg in &entry.args {
    if let Some(slot) = verb_slot(arg) {
      parts.push(slot);
    } else if arg.kind != "subcommand" {
      parts.push(delimit(&placeholder(arg), arg));
    }
  }
  parts.join(" ")
}

/// A `subcommand` pseudo-arg carrying VALUES is the surface's third level --
/// `intent claude skills install` is `claude skills` with `install` in its
/// verb slot, not a row of its own. Rendered as the alternation, because those
/// verbs appear nowhere else in the guide: they have no rows to be listed
/// under, so dropping them here drops them entirely.
///
/// A valueless slot is a family's verb slot, filled by SIBLING ROWS which are
/// each rendered in full below it, so rendering anything for it would
/// duplicate the section it heads.
fn verb_slot(arg: &Arg) -> Option<String> {
  if arg.kind != "subcommand" || arg.values.is_empty() {
    return None;
  }
  Some(delimit(&arg.values.join("|"), arg))
}

/// An `enum` positional's declared values are the useful name; anything else
/// is named by its own `name`, which is what the surface's help already shows.
fn placeholder(arg: &Arg) -> String {
  if arg.kind == "enum" && !arg.values.is_empty() {
    arg.values.join("|")
  } else {
    arg.name.clone()
  }
}

/// `<x>` required, `[x]` optional, `...` repeatable -- read from
/// [`Arg::required`] / [`Arg::repeated`] rather than from the arity string, so
/// this and the clap spine cannot part company on what `0..n` means.
fn delimit(inner: &str, arg: &Arg) -> String {
  let ellipsis = if arg.repeated() { "..." } else { "" };
  if arg.required() {
    format!("<{inner}>{ellipsis}")
  } else {
    format!("[{inner}]{ellipsis}")
  }
}

/// One flag: every spelling it answers to, its value placeholder, and the two
/// facts that change how it is called.
///
/// The placeholder is the table's own (`<ref>`, `[dir]`), delimiters included
/// -- they are the author showing the reader what they will see. Only 31 of
/// the 64 shipped flags carry one, and a `bool` flag correctly has none.
///
/// **THE `...` IS DERIVED FROM `arity`, NEVER AUTHORED BESIDE IT** (vc, ruling
/// on ic's finding). Three rows used to write it into `value` while `arity`
/// said the same thing, and `spine.rs` read BOTH into `num_args` thirty lines
/// apart -- benign only while the two carriers had disjoint populations. The
/// case that was not benign: `arity: "1"` on a row whose value ends `...` sets
/// `num_args(1..)` and is then overwritten with `num_args(1..=1)`, while the
/// surface goes on rendering the ellipsis. The register says list, the parser
/// says one, and nothing compares them.
///
/// **A test that the two agree was the other option and it loses**: it can only
/// fire after somebody has written the second value, so it catches the
/// divergence rather than preventing it. Deriving makes the second value
/// impossible to write. Same reasoning as [`delimit`] above, which has read the
/// ellipsis off `Arg::repeated` since it was written.
fn flag_line(flag: &Flag) -> String {
  let spellings = flag
    .spellings
    .iter()
    .map(|s| format!("`{s}`"))
    .collect::<Vec<_>>()
    .join(", ");
  let ellipsis = match crate::dispatch::arity_bounds(&flag.arity) {
    Some((_, None)) => "...",
    _ => "",
  };
  let value = flag
    .value
    .as_deref()
    .map(|v| format!(" `{v}{ellipsis}`"))
    .unwrap_or_default();
  let required = if flag.required { " **(required)**" } else { "" };
  let default = flag
    .default
    .as_deref()
    .map(|d| format!(" (default: `{d}`)"))
    .unwrap_or_default();
  format!("{spellings}{value}{required} -- {}{default}", flag.help)
}

/// The other half of the guide: the part no table can generate.
///
/// **A guide with no workflow section reads as a tool with no workflow
/// conventions**, which is the same silent-omission failure this module opens
/// by measuring, one level up.
///
/// It stood as a named placeholder while the v3 workflow surface settled --
/// prose describing a workflow that changes next week is worse than prose that
/// says it is coming. That condition ended when the port closed, and this is
/// what replaced it.
///
/// **Compiled in rather than read from a file, so it versions with the tool.**
/// A guide shipped alongside the binary can describe a build the reader is not
/// standing on; this one cannot, because there is only ever one copy and it is
/// the one that was built.
///
/// **Every `intent <cmd>` written here must be a command that exists.** The
/// generated half above guarantees the LIST is complete; nothing guarantees
/// prose, which is where a reference actually rots -- and the measured failure
/// mode is silent OMISSION rather than drift into falsehood, because a wrong
/// command earns an error the reader can react to while a missing one reads as
/// a capability the tool does not have. `guide_refs_check.sh` is the pair to
/// the generator and this text is checked by it.
const AUTHORED_HALF: &str = "\
## Workflows, methodology and conventions

The reference above says what every command IS. It cannot say which you run in which order, or why a command that would obviously work is the wrong one to reach for. That is what follows.

### The unit of work is a steel thread, and it is written before it is built

**A steel thread is one end-to-end slice of intent: something the project means to become true.** `intent st new` opens one, `intent st start` puts it in flight, `intent st done` closes it. **`intent st list` shows the threads IN PROGRESS, not all of them** -- reach for `--status all` when a thread you know exists does not appear.

**Describe the thread before writing code for it.** A thread documented afterwards records what was built; the model exists to record what was INTENDED, so that what was built has something to be judged against. Those are only the same document when nothing was learned on the way.

Three kinds of child hang off a thread, and they answer three different questions:

- **Work packages** -- `intent wp new`, then `wp start` and `wp done`. How the work is divided. A work package is a unit of delivery, sized so that finishing one means something.
- **Acceptance criteria** -- `intent ac new`. What must be TRUE for the thread to be finished. A criterion is a claim about the world, not a task; if it reads like something to do, it is a work package wearing the wrong hat.
- **Acceptance tests** -- `intent at new`, then `at green`, `at red` or `at na`. What EXECUTES to show a criterion holds. **A criterion with no test is an opinion**, which is why these are two objects and not one field.

### Closing is gated on the criteria, never on the work

**`intent st done` refuses a thread whose criteria are not satisfied**, and the refusal names criteria and nothing else. Finishing every work package does not close a thread; satisfying every criterion is what does:

    error: ST0000 is not ready to close -- gate: ST0000 BLOCKED -- 0/4 satisfied
      remedy: satisfy or formally descope the remaining criteria, then close again

`intent ac gate` asks that question directly, and it is what to run before believing a thread is nearly done. `intent ac satisfy` records that a criterion holds. `intent ac withdraw` and `intent ac descope` are how a criterion leaves the count ON THE RECORD rather than by being quietly deleted -- **the criterion you would delete because it turned out to be hard is the one that mattered.**

### Canon is the source of truth and the markdown is a view of it

**Intent keeps its state in a store, and the files under `intent/` are generated FROM that store.** A thread's `info.md`, the thread index and `todo.md` are views. **Never hand-edit a view.** Nothing merges the edit, the next render overwrites it, and in between there are two answers to one question with nothing comparing them.

Change state through the verbs -- `st`, `wp`, `ac`, `at`, `issues` -- and let the views be produced. When a view looks wrong the fix is to correct the state and regenerate, never to correct the file.

**The committed extract is not the store.** `intent/.canon/` holds a JSON extract that goes into git so a clone can rebuild the state; the store itself is per-machine and is deliberately never committed. That split is exactly why the two can disagree, and why reconciling them is a verb rather than an assumption.

### The sync discipline: the flag names the DESTINATION

`intent sync` moves state between the store and the committed extract, and the flag says where the data ENDS UP:

- **`intent sync --to-disk`** writes the extract from the store. This is the routine one: you changed state through the verbs, and the change now belongs in git.
- **`intent sync --to-store`** replaces the store from the extract. This is the recovery one, and it is **destructive** -- anything the store holds that has not been written to disk is gone.

**Naming both flags chooses neither, and is refused.** They are opposite directions over the same two endpoints, so running both just makes whichever went last authoritative by accident of ordering.

**Bare `--to-store`, with no thread named, replaces the WHOLE store**, including durable history that cannot be rebuilt from the files. **Naming threads scopes it**: only those are taken from the extract and every other thread keeps the store's value. Repairing one object with the bare form is how **a remedy whose blast radius exceeds the fault it repairs** gets reached for.

**Authored markdown has NO path into the store in this build**, and that is worth knowing BEFORE planning a repair around one. `intent sync` reads the JSON extract and opens no markdown at all. `intent ingest --from-md` does read markdown, including a v2 estate's, and reports what it found -- it tells you an estate PARSES, and it writes nothing.

### Gate semantics: an exit code is a verdict about a particular thing

Read the codes as verdicts about DIFFERENT SUBJECTS rather than as degrees of bad:

- **`1` -- the tool ran and the answer is no.** Your code, your criteria, or your arguments. Act on it.
- **`2` -- this build could not answer at all.** The tool is unavailable; nothing has been said about your work. **The shipped pre-commit gate fails OPEN on `2`**, deliberately, because a check that could not run must not block a commit it never examined.
- **`3` -- a rule this project armed could not be enforced.** Not a flavour of `1`: `1` says the answer is no, `3` says part of the question went unanswered. **The gate BLOCKS on `3`**, because a rule that silently goes unenforced is the failure a gate exists to prevent.

**So a passing gate is a claim about what RAN, and no more than that.** A skipped check says so in its own line, and that line is not decoration: an instrument that cannot report its own blindness is worse than no instrument, because it converts an unknown into a green.
";

#[cfg(test)]
mod tests {
  use super::*;

  fn guide() -> String {
    render(&dispatch::table()).expect("the committed table renders")
  }

  /// AC-09.4 in the direction that matters most: **nothing shipped is
  /// missing**, asserted over the enumerated population rather than sampled.
  ///
  /// The population is `shipped_entries`, which is deliberately NOT
  /// `families[].entries[]`: the top-level `new_surface` array carries rows
  /// too, and a check written against the families alone would pass while the
  /// guide omitted `search` and `schema`.
  ///
  /// **THE COUNTS THAT USED TO BE HERE ARE GONE ON PURPOSE, AND SO IS ONE OF
  /// THE EXAMPLES.** This paragraph read `104 of 112 rows, and the other 8`
  /// against a table that is now 121 and 13, and it cited `intent llm guide`
  /// as a row a families-only check would miss when `llm guide` is IN a
  /// family. **Because all of it was prose, nothing could fire** -- vc found
  /// the stale figure in four separate homes on 2026-08-30, and the wrong
  /// example turned up only when the claim was finally asserted.
  ///
  /// **A correct warning carrying a stale figure is worse than no warning**,
  /// because the figure is the part an author acts on: follow the old numbers
  /// and you take the families plus eight, miss five, and the check still
  /// passes. `the_families_alone_are_not_the_population` below now asserts
  /// the STRUCTURE this paragraph describes, so the claim has a witness
  /// instead of a number.
  /// The witness for the warning above. **A population claim written only in
  /// prose has no way to fail**, which is how four homes went on saying 112
  /// while the table grew to 134, and how an example stayed in the text after
  /// the row it named moved into a family.
  ///
  /// Asserted as a RELATIONSHIP and by NAME rather than as counts, so it
  /// survives the table growing. The disjointness check is the general half:
  /// it cannot rot by example the way the cited paths can.
  #[test]
  fn the_families_alone_are_not_the_population() {
    let table = dispatch::table();
    assert!(
      !table.new_surface.is_empty(),
      "`new_surface` is empty, so the warning above describes a distinction that no longer exists and the population is just the families"
    );

    let in_families: Vec<&str> = table
      .families
      .iter()
      .flat_map(|f| f.entries.iter())
      .map(|e| e.path.as_str())
      .collect();
    let in_new: Vec<&str> = table.new_surface.iter().map(|e| e.path.as_str()).collect();

    // **DECLARED, NOT FILTERED**, for the reason the mode machine's Esc
    // exemption is declared: a predicate excluding `organize` would also pass
    // for a path that started overlapping by accident, because the known
    // defect and the new one look identical to a filter. Asserted by EQUALITY,
    // so a SECOND overlap fails here even while this one stands.
    const KNOWN_OVERLAP: &[(&str, &str)] = &[
      (
        "organize",
        "ONE WORD, TWO DIFFERENT PROGRAMS -- NOT ONE PROGRAM DECLARED TWICE, AND NOT A DEFECT. hv \
       ruled the reclamation on 2026-08-19 and the ruling is recorded in the `new_surface` row's \
       own `name_reclaimed` field: the v2 face stays retired and the two share nothing but the \
       word. The family row is `bin/intent_organize`, which moved ST directories into status \
       folders (`--dry-run`), retired 2026-08-14. The `new_surface` row is a different program \
       that hydrates and dehydrates artefacts against `.intentfiles` (`--apply`, `--default`, \
       `--force`). Both declarations are correct as written. **NOTHING IS LATENT: `is_shipped()` \
       is `disposition != retire && target.state != retire`, the family row is `retire` on BOTH, \
       so `shipped_entries` filters it out and `entry(table, \"organize\")` has exactly one \
       candidate to find.** Table order decides nothing here. The entry stays because this check \
       counts PATHS and a path-keyed census still double-counts -- so it is a permanent declared \
       fact, not a parked defect. RECORDED BY ic 2026-08-30, after escalating it to hv as a \
       contradiction that did not exist: an intersection on `path` can only find a shared path, so \
       it could not have told these two apart, and the field that settled it was sitting in the \
       row.",
      ),
      (
        "help",
        "ONE WORD, TWO DIFFERENT PROGRAMS, AND THE SECOND INSTANCE OF THE EXACT SHAPE `organize` \
       RECORDS ABOVE. hv ruled `help` INTO the 3.0.0 cut at the ROOT ONLY on 2026-08-30, and the \
       family row's own `target.spelling_note` had SCHEDULED this reclamation since 2026-08-26 -- \
       _this row is scheduled to follow the `organize` pattern_ -- so the overlap is a ruling \
       landing rather than a drift. The family row is `bin/intent_help` plus the whole `lib/help/` \
       mechanism, retired for its drift: 11 help files for 27 commands, each grammar used once, \
       and `upgrade` still described as an STP migration at v2.19.0. The `new_surface` row is a \
       different program that ships NO help text of its own -- it rebuilds the clap `Command` from \
       the compiled-in table and asks IT to render, so `intent help` and `intent --help` are two \
       spellings of ONE renderer and cannot drift apart. **NOTHING IS LATENT, for the same reason \
       as `organize`: the family row is `retire` on BOTH `disposition` and `target.state`, so \
       `shipped_entries` filters it out and `entry(table, \"help\")` has exactly one candidate. \
       Table order decides nothing here either.** RECORDED BY vc 2026-08-30 while closing ST0058 \
       AC-00.6, whose falsifier is a capability present by one spelling and refused by its twin; \
       `intent --help` rc=0 against `intent help` rc=2 was its last live instance (issue 0086).",
      ),
    ];

    let overlap: Vec<&str> = in_new
      .iter()
      .filter(|p| in_families.contains(p))
      .copied()
      .collect();
    let declared: Vec<&str> = KNOWN_OVERLAP.iter().map(|(p, _)| *p).collect();
    assert_eq!(
      overlap, declared,
      "the set of paths in BOTH `new_surface` and a family changed. An overlap double-counts every \
       census over the table, and where BOTH rows are shipped it also lets TABLE ORDER decide \
       which one `entry()` finds, because that is a `.find()` over `shipped_entries` and it takes \
       the first silently. Add it to `KNOWN_OVERLAP` with its reason -- stating whether the two \
       rows are one program or two, which is the distinction a path intersection cannot make -- \
       or, better, resolve it"
    );
    assert!(
      KNOWN_OVERLAP.iter().all(|(_, why)| !why.trim().is_empty()),
      "a declared overlap with no stated reason is indistinguishable from one nobody could fix"
    );

    for path in ["search", "schema"] {
      assert!(
        in_new.contains(&path),
        "the warning above cites `{path}` as a row a families-only check would miss, and it is not in `new_surface`. Either it moved into a family, in which case the warning needs rewording, or the path changed"
      );
    }
  }

  #[test]
  fn every_shipped_command_appears() {
    let table = dispatch::table();
    let text = guide();
    let missing: Vec<&str> = dispatch::shipped_entries(&table)
      .iter()
      .map(|e| e.path.as_str())
      .filter(|p| !text.contains(&format!("#### intent {p}\n")))
      .collect();
    assert!(
      missing.is_empty(),
      "the guide is generated from the table and still omits {} command(s): {missing:?}",
      missing.len()
    );
  }

  /// **The same defect wearing the opposite sign.** A retired row rendered
  /// into the guide tells an agent to call a command hv killed, and it would
  /// pass the completeness test above untouched.
  #[test]
  fn no_retired_command_appears() {
    let table = dispatch::table();
    let text = guide();
    // **`path` IS NO LONGER UNIQUE ACROSS THE TABLE, AND THAT IS RULED RATHER
    // THAN ACCIDENTAL** (hv, 2026-08-19). The table is two registers in one: a
    // parity record of what became of each v2 command, and a declaration of what
    // v3 ships. When hv reclaimed `organize` for v3 those two registers came to
    // hold the same word -- a retired v2 face and a shipped v3 verb -- so a
    // reclaimed name is excluded here by asking whether a SHIPPED row also
    // carries it.
    //
    // **THE RESIDUAL BLIND SPOT IS NAMED RATHER THAN LEFT TO BE FOUND.** For a
    // reclaimed path this test can no longer tell the retired row's heading from
    // the shipped one's, because they are the same string. That is tolerable only
    // because the guide is GENERATED from shipped entries and a retired row has
    // no path into it -- if the generator ever walked the full table, this check
    // would go quiet on exactly the rows it exists for.
    let shipped_paths: std::collections::BTreeSet<&str> = dispatch::shipped_entries(&table)
      .iter()
      .map(|e| e.path.as_str())
      .collect();
    let retired: Vec<String> = table
      .families
      .iter()
      .flat_map(|f| f.entries.iter())
      .chain(table.new_surface.iter())
      .filter(|e| !e.is_shipped() && !shipped_paths.contains(e.path.as_str()))
      .map(|e| e.path.clone())
      .collect();
    assert!(
      !retired.is_empty(),
      "the table declares no retired rows, so this test proves nothing -- it asserted a vacuous green"
    );
    for path in retired {
      assert!(
        !text.contains(&format!("#### intent {path}\n")),
        "`{path}` is retired by ratification and the guide offers it"
      );
    }
  }

  /// v2's guide never named `intent llm` -- the command that prints it. The
  /// generated one cannot make that mistake, and this is the regression test
  /// for the specific measurement that motivated the AC.
  #[test]
  fn the_families_v2_never_documented_are_present() {
    let text = guide();
    for path in ["llm", "llm guide", "issues", "modules", "lang"] {
      assert!(
        text.contains(&format!("#### intent {path}\n")),
        "`{path}` was absent from v2's hand-maintained guide; a generated one has no way to lose it"
      );
    }
  }

  /// **A blank `does:` line reads as a styling choice, not as a missing
  /// measurement.** `help` carries `serde(default)`, so a row without one
  /// deserializes to an empty string and renders a bullet with nothing after
  /// the colon -- indistinguishable from a command whose purpose is obvious.
  #[test]
  fn no_shipped_command_renders_an_empty_description() {
    let table = dispatch::table();
    let blank: Vec<&str> = dispatch::shipped_entries(&table)
      .iter()
      .filter(|e| e.help.trim().is_empty())
      .map(|e| e.path.as_str())
      .collect();
    assert!(
      blank.is_empty(),
      "these rows would render `- **does:** ` with nothing after it: {blank:?}"
    );
  }

  /// The safety field is what an agent gates on, so an unclassified value must
  /// reach the page as itself. Driven with a value no vocabulary declares --
  /// the same `banana` probe that found three unchecked fields in this table.
  #[test]
  fn an_unclassified_safety_value_is_rendered_not_swallowed() {
    let table = dispatch::table();
    let mut entry = dispatch::shipped_entries(&table)[0].clone();
    entry.read_or_mutate = "banana".to_string();
    let rendered = safety(&entry);
    assert!(
      rendered.contains("banana") && rendered.contains("UNDECLARED"),
      "an unrecognised safety value must be visible to the reader who can fix it, got: {rendered}"
    );
    assert!(
      !rendered.contains("cannot change durable state"),
      "an unknown value defaulting to `read` presents an unclassified command as safe to call"
    );
  }

  /// The four cited invariants are read from the table by ID, so a rename must
  /// refuse the render rather than drop a fact an agent parses exit codes on.
  #[test]
  fn a_missing_invariant_refuses_the_render() {
    let mut table = dispatch::table();
    assert!(
      render(&table).is_ok(),
      "the committed table renders, or every case below passes for the wrong reason"
    );
    table.invariants.retain(|i| i.id != "INV-04");
    // `Failure` carries the message rather than BEING it, since the error type
    // stopped being a `String` (IN-RS-CODE-004). Asserted on the rendered text
    // rather than on the variant, deliberately: what this test is about is what
    // an operator READS, and a variant match would keep passing while the
    // sentence rotted into something that names neither the id nor the cause.
    // Matched rather than `to_string()`d: `Failure` carries its message and does
    // not implement `Display`, and adding one to a shared type to satisfy a test
    // in another module is a wider change than this needed.
    let err =
      match render(&table).expect_err("a cited invariant vanished and the guide rendered anyway") {
        Failure::Error(msg) | Failure::Unavailable(msg) => msg,
        Failure::Verdict => panic!("a build defect must carry a message, not a bare verdict"),
        // Added 2026-08-20 when `critic` introduced `Refused` (`5043d0c4`) and
        // this match stopped compiling. **The absence of a `_` arm is why
        // anyone was told**: a wildcard would have swallowed a new variant here
        // in silence, and the type system asking the question is the whole
        // reason the arms are enumerated.
        Failure::Refused(msg) => panic!(
          "a missing invariant is a BUILD DEFECT, not a refusal -- `Refused` says a rule this \
           project armed could not be enforced, and the guide arms nothing: {msg}"
        ),
      };
    assert!(
      err.contains("INV-04") && err.contains("build defect"),
      "the refusal must name the missing id and say it is a build defect, got: {err}"
    );
  }

  /// `0..n` carries neither `+` nor `*`, which is the trap
  /// [`dispatch::Arg::repeated`] exists to hold -- asserted here too because
  /// this renderer is the second reader, and the point of the extraction was
  /// that the two cannot part company.
  #[test]
  fn the_call_line_delimits_by_arity() {
    let arg = |arity: &str| Arg {
      name: "x".to_string(),
      kind: "string".to_string(),
      arity: arity.to_string(),
      values: vec![],
      default: None,
      required_unless: None,
    };
    assert_eq!(delimit("x", &arg("1")), "<x>");
    assert_eq!(delimit("x", &arg("0..1")), "[x]");
    assert_eq!(delimit("x", &arg("1..n")), "<x>...");
    assert_eq!(delimit("x", &arg("0..n")), "[x]...");
  }

  /// **The 0033 case, rendered.** `at green` is `one-way` because the verb
  /// destroys the row's authored note, so the documented round trip through
  /// `at red` moves the status back and does not restore it. An agent reading
  /// `reversible` there would call it believing the change is undoable.
  ///
  /// Pinned against the COMMITTED table rather than a fixture, because the
  /// value is a claim about the shipped surface and a fixture would keep
  /// passing after somebody changed the row.
  #[test]
  fn a_one_way_mutation_says_so_on_its_safety_line() {
    let text = guide();
    let i = text
      .find("#### intent at green\n")
      .expect("`at green` is shipped and must appear");
    let block = &text[i..i + 400];
    assert!(
      block.contains("ONE-WAY") && block.contains("nothing on this surface puts back"),
      "a one-way mutation must be marked where the mutation is marked, got: {block}"
    );
  }

  /// The two halves of one question are read together or not at all, so the
  /// recoverability sits ON the safety line rather than four bullets below it.
  #[test]
  fn recoverability_rides_the_safety_line_and_reads_are_left_alone() {
    let text = guide();
    let mutation = text
      .find("#### intent st done\n")
      .map(|i| &text[i..i + 300])
      .unwrap();
    assert!(
      mutation.contains("- **safety:** `mutate`") && mutation.contains("reversible"),
      "a reversible mutation carries both halves on one line, got: {mutation}"
    );

    let read = text
      .find("#### intent config\n")
      .map(|i| &text[i..i + 300])
      .unwrap();
    assert!(
      !read.contains("reversible") && !read.contains("recoverability"),
      "a read changes nothing, so the question is vacuous and must not be answered: {read}"
    );
  }

  /// **A renderer whose fallback is silence is the failure this file was
  /// written against.** The committed table cannot reach this state -- the
  /// loader refuses it -- so the arm is driven directly, which is the only way
  /// to prove it is not silence.
  #[test]
  fn a_mutation_with_no_declared_recoverability_says_undeclared() {
    let table = dispatch::table();
    let mut entry = dispatch::shipped_entries(&table)
      .into_iter()
      .find(|e| e.read_or_mutate == "mutate")
      .expect("the surface has mutations")
      .clone();

    entry.recoverability = None;
    let blank = safety(&entry);
    assert!(
      blank.contains("UNDECLARED") && blank.contains("treat as one-way"),
      "an absent classification must present as unknown-and-assume-the-worst, got: {blank}"
    );

    entry.recoverability = Some("banana".to_string());
    let bogus = safety(&entry);
    assert!(
      bogus.contains("banana") && bogus.contains("UNDECLARED"),
      "an unrecognised value must reach the reader who can fix it, got: {bogus}"
    );
  }

  /// The third level of the surface has no rows of its own, so the alternation
  /// in the call line is the only place those verbs appear at all.
  #[test]
  fn a_verb_slot_renders_its_values() {
    let text = guide();
    assert!(
      text.contains("intent claude skills <install|list|uninstall|sync>")
        || text.contains("intent claude skills <"),
      "a subcommand slot's declared values are the only rendering those verbs get"
    );
  }
}
