---
verblock: "11 Sep 2026:v0.2: synced to the gate as built in v3.0.1"
---

# The pre-commit critic gate

**Status: both halves are resolved.** Half A's defect lived in the v2 dispatcher, and the v2 Bash CLI is deleted (`d5998ac37`), so the composition that darkened the gate cannot recur: the hook now calls v3's `intent critic`. Half B is built in `native/rust/crates/intentsvcs/src/critic.rs`: named-tool arming (shellcheck, clippy), explicit declarations, and a two-axis census printed on every run. The adjacent release-preflight gap is closed too: `int build release` now runs the Rust suite.

## The one-sentence problem

**After the hoist at `0ec2ac79`, every commit any node made passed a pre-commit gate that enforced NOTHING across all declared languages, while reporting success.**

## HALF A -- the gate failed open

### Mechanism, and it was three correct behaviours composing into a wrong outcome

- The hook called `intent critic <lang>`.
- `intent` on PATH was **v2**, and v2 correctly refused a v3-declared project at exit 2.
- `intent critic` also used exit 2 for its own invocation errors, so the hook could not tell the two apart.
- **The hook's `*)` branch treated every rc other than 0 or 1 as fail-open**, so the commit proceeded.

Nothing there was a bug on its own. The version guard was right, exit 2 was a legitimate refusal code, and the fail-open on 2 was deliberate. **The defect was the composition.** And it was not silent: the `*)` branch printed its fail-open line once per declared language on every commit, so "nobody noticed" was a fact about attention, not about instrumentation.

### As built

The v2 dispatcher and its version guard are gone, so the refusal half of the composition no longer exists. The gate chain is: `.git/hooks/pre-commit` chains to `.git/hooks/pre-commit.intent`, a shim installed by `intent claude upgrade --apply` (`canon.rs:511`), which reads the install root from `~/.intent/home` and execs `<root>/lib/templates/hooks/pre-commit.sh`. That hook runs `intent critic "$lang" --staged --severity-min "$SEVERITY" --format text` for each declared language (`lib/templates/hooks/pre-commit.sh:543`) and branches on the exit code the critic defines (`critic.rs:331`):

| code | meaning (`intent critic`)                        | gate                                                 |
| ---- | ------------------------------------------------ | ---------------------------------------------------- |
| 0    | clean -- everything ASKED came back empty        | passes                                               |
| 1    | findings present                                 | BLOCKS                                               |
| 2    | the critic is broken (eg no rule library loaded) | fails open, and the language is recorded UNENFORCED  |
| 3    | refused -- an armed rule's tool is absent here   | BLOCKS, with the remedy "install the tool or disarm" |

**The gate fails open on its own breakage and closed on yours.** Any unrecognised code prints `intent critic (<lang>) did not check (exit <rc>) -- <lang> is UNENFORCED in this commit.` (`pre-commit.sh:601`), and the run ends with one digest carrying its denominator, `intent critic gate: <n> of <m> declared language(s) went UNENFORCED (...)` (`:638`). A project that declares no languages is told so on every commit (`:632`) rather than getting silence.

### Proving a gate means making it fail on purpose

**A gate that goes green after a fix is the same evidence it gave while dark.** The first re-drive of Half A proved the critic "ran" by accepting `rc=0` from `critic shell` -- which is what the dark gate returned, on the one pack that had nothing armed and so could return nothing else. The standing requirement: drive a pack with an armed rule against a deliberate staged violation and assert `rc=1` with the rule named, drive the same command over a clean staged set and assert `rc=0`, and prove the end-to-end case by the hook BLOCKING a commit it should block. A zero is not a result until the instrument has also produced a non-zero.

## HALF B -- the rust and shell packs, armed or declared

The shell and rust packs shipped untriaged from ST0034 and had never carried a `Greppable proxy` or a declaration, so the runner skipped them without a word -- a third state, neither armed nor declared, indistinguishable from clean. Half B exists to eliminate that state, and it has: every shell and rust rule is now armed on a tool, armed on a grep proxy, or declared unanswerable. `intent critic <lang> --format json` reports each rule's standing in its `census`.

### The proxy contract is an INJECTION BOUNDARY, not a capability ceiling

`proxy_is_simple` (`critic.rs:386`) admits one shape only:

```
grep [-r|-n|-E|--include=GLOB ...] '<pattern>' [<path>...]
```

One `grep`. No pipes, no chains. Flag clusters drawn from `{r,n,E}` only -- `-L`, `-v`, `-l`, `-c`, `-o`, `-w`, `-x`, `-A`, `-B` are all refused. Single-quoted pattern; **path args free of shell metacharacters, so a pipeline cannot arrive disguised as an argument.** A `Greppable proxy` block is DATA read out of a rule file, and executing it as shell in the pre-commit gate of every consuming project is the one thing the predicate exists to prevent. The shape is ported character-for-character from v2 and is not to be relaxed.

**Consequence for a grep proxy: it can only express a POSITIVE match.** A rule whose violation is an ABSENCE has no `grep -L` and no `grep -v`; a rule needing to aggregate across files cannot be counted by one grep. That is a limit on proxies, never a limit on the gate -- which is why hv's authorisation of real parsers, re-ruled with this rationale in view, took the named-tool form below rather than a wider proxy grammar.

### THE RULING -- a named-tool declaration, and the boundary does not move

- **The rule names WHICH tool answers it; the runner owns HOW it is invoked, in the runner's own code.** A rule's frontmatter carries `critic_tool`, `critic_tool_context` and `critic_tool_codes`; the runner alone builds the command line (`shellcheck --format=gcc <file>`, `critic.rs:733`, with no rule-supplied flags). **Rule files never contribute shell, ever.**
- **A tool-armed rule REFUSES when its tool is absent. It never degrades to skipped** (`IN-AG-NO-SILENT-001`). Driven with shellcheck off `PATH`: the census lists the rule as `ARMED but NOT RUN HERE, the tool is not on this machine`, and the run exits 3 with `remedy: install the missing tool, or disarm that rule in .intent_critic.yml`.
- **Opting out is a property of the PROJECT, not of the RULE.** The seam is the `disabled:` list in `.intent_critic.yml` (`critic.rs:574`). A rule author cannot exempt their own rule; a project owner disabling one is a visible, reviewable act, and the run counts disabled rules beside the census. There is no separate "optional" arming mode: disabling is the project's one opt-out.
- **UNARMED IS NEVER INVISIBLE.** The census is printed on every run, including a clean one: how many rules were ASKED, how many are armed, and which could not be armed at all, split into declared, undeclared and unrunnable. The gate can distinguish CHECKED AND CLEAN from CHECKED NOTHING.
- **Arming and RUN CONTEXT are separate axes.** A rule with `critic_tool_context: workspace` is armed and reported `not-run:out-of-context` in every per-file run (`critic.rs:831`). A whole-workspace `cargo clippy` does not belong in a per-commit hook; it belongs where the compile already happens.

### Per-rule verdicts

**The test: IS THERE A NAMED TOOL WHOSE OWN OUTPUT ANSWERS THIS RULE?** A rule may name a tool that answers an ADJACENT proposition, and naming a parser in a rule's Detection text is the start of the question, not the end of it. Verdicts were adjudicated against `shellcheck 0.11.0` and `clippy 0.1.97` on fixtures, and each is what the census reports at HEAD.

| rule                                         | severity       | verdict                   | why                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| -------------------------------------------- | -------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `IN-SH-CODE-001` quote-expansions            | critical       | **TOOL-ARMED** shellcheck | SC2086/SC2046/SC2206/SC2068 ARE the rule's static signals, not neighbours of them. Driven: an unquoted `echo $x` in a staged `.sh` file is reported against this rule at exit 1.                                                                                                                                                                                                                                                                                                                                     |
| `IN-SH-CODE-002` no-parse-ls                 | warning        | **TOOL-ARMED** shellcheck | SC2012. A parser will not fire on `ls` inside a comment or a string, which a regex cannot avoid.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `IN-RS-CODE-001` result-over-panic           | critical       | **TOOL-ARMED** clippy     | `unwrap_used` / `expect_used` / `panic`, all **restriction** (opt-in). Default `cargo clippy` flags a production `.unwrap()` and NOT a colocated `#[cfg(test)]` one; `--all-targets` flags both, so a runner answering this rule must not pass `--all-targets`. Workspace context, so the critic never runs it -- see the finding below.                                                                                                                                                                             |
| `IN-RS-CODE-005` lifetime-elision-first      | style          | **TOOL-ARMED** clippy     | `needless_lifetimes` / `extra_unused_lifetimes`, both in **clippy::all** -- on by default, nothing to enable. Workspace context.                                                                                                                                                                                                                                                                                                                                                                                     |
| `IN-RS-CODE-002` ownership-before-clone      | warning        | **TOOL-ARMED AT A COST**  | `clone_on_copy` is in clippy::all, `needless_pass_by_value` is **pedantic**, and **`redundant_clone` is NURSERY** -- unstable by clippy's own classification, and it is the lint answering the rule's PRIMARY signal. Arming on it imports that instability into a gate. Workspace context.                                                                                                                                                                                                                          |
| `IN-SH-CODE-005` no-silent-exit-codes        | critical       | **DECLARE none**          | **The rule's content IS its qualifier** -- _without an adjacent comment_ is a predicate over the NEIGHBOURING LINE, and a proxy may carry no `-A`/`-B`, so it is unreachable BY CONSTRUCTION. Armed on grep for a day, it refused deliberate, documented `2>/dev/null` probes on correct code across most of the project's shell files, and a gate that must be bypassed is one nobody keeps. The named tools answer adjacent propositions (SC2015, SC2164) and lint none of `\|\| true`, `2>/dev/null` or `set +e`. |
| `IN-RS-CODE-004` error-types                 | warning        | **GREP PARTIALLY**        | No lint answers it. `Result<..., Box<dyn Error>>` / `Result<..., String>` are greppable and `--include=lib.rs` scopes the library half; the binary half is not expressible.                                                                                                                                                                                                                                                                                                                                          |
| `IN-SH-CODE-003` set-euo-pipefail            | warning        | **DECLARE none**          | shellcheck says nothing on a bash script lacking `set -euo pipefail`; SC2148 is a missing shebang and SC2154 a consequence of `-u`, not its absence. No tool answers this, and the violation is an absence.                                                                                                                                                                                                                                                                                                          |
| `IN-SH-CODE-004` setopt-err-exit             | warning        | **DECLARE none**          | `SC1071 (error): ShellCheck only supports sh/bash/dash/ksh/'busybox sh' scripts. Sorry!` The only named tool for shell REFUSES the language this rule is about -- a fact about the world rather than about the runner.                                                                                                                                                                                                                                                                                               |
| `IN-SH-CODE-006` module-highlander           | warning        | **DECLARE none**          | Needs the same function name COUNTED across files. No shellcheck lint aggregates, and one grep cannot count.                                                                                                                                                                                                                                                                                                                                                                                                         |
| `IN-RS-TEST-001` cfg-test-colocated          | warning        | **DECLARE none**          | _"source files with no colocated `#[cfg(test)]`"_ -- an absence, and no clippy lint asks it.                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `IN-RS-CODE-003` traits-over-enums           | recommendation | **DECLARE none**          | The rule text says so: _"Clippy does not lint this directly; structural review is the detection mechanism."_                                                                                                                                                                                                                                                                                                                                                                                                         |
| `IN-RS-TEST-002` assert-matches-for-variants | warning        | **DECLARE none**          | The rule text says so: _"Clippy has no direct lint; structural review and test-brittleness are the signals."_                                                                                                                                                                                                                                                                                                                                                                                                        |

Rules added to either pack since carry their own classification; the census is the live answer, never this table.

### FINDING: the clippy-armed rules are answered by no runner

**The census counts the three clippy rules as armed, and nothing runs the lints that answer two of them.** The critic reports them `not-run:out-of-context` (the `"clippy"` arm at `critic.rs:987` is a no-op), and the whole-workspace runs that exist -- `cargo clippy --workspace --all-targets -- -D warnings` in `.github/workflows/rust.yml:103` and the devbin's `check clippy` (`bin/.devbin/config.yaml:197`) -- use clippy's default lint groups. No `[lints]` table or crate attribute in `native/rust` enables `clippy::unwrap_used`, `expect_used` or `panic` (restriction), `needless_pass_by_value` (pedantic) or `redundant_clone` (nursery). So `IN-RS-CODE-005` and `IN-RS-CODE-002`'s `clone_on_copy` are enforced by the workspace run, and **`IN-RS-CODE-001`, the critical rust rule, is enforced nowhere.** The workspace runs also pass `--all-targets`, which this rule's verdict says must not be passed for it.

### BOUNDARY -- `.bats` files are invisible to the shell critic

`--staged` passes every added, copied or modified path (`git diff --cached --name-only --diff-filter=ACM`, `render.rs:10293`), and each rule's `applies_to` globs then select the files it sees (`critic.rs:706`). No shell rule's globs admit `*.bats`. Driven: the same unquoted-expansion fixture is reported at exit 1 as `t.sh` and passes at exit 0 as `tests/t.bats`. **The `.bats` suite -- the largest body of shell-adjacent code in this repository -- never reaches the shell critic**, so nobody should measure the shell pack's effect against a denominator that includes it.

### What this project actually is

Intent is a Rust workspace plus shell: the devbin, the shipped hooks and guards, and the `.bats` suite. It declares `elixir` and `swift` as well, but its only Elixir sources are the rule library's `good.exs` / `bad.exs` examples, and every swift rule is UNDECLARED -- the census names each one on every run. **A gate enforcing Elixir rules on a project that is essentially not Elixir** is why Half B mattered here.

### The constraint that still stands

**ST0039: a proxy must be simple enough for the headless runner to honour, and a rule whose detection is genuinely non-mechanical carries NO proxy rather than a misleading one.** The declare-none rows are the honest outcome for those rules -- "critic cannot check this", stated -- and they are declared for reasons that are claims about the WORLD rather than about the runner: `SC1071` means no tool exists for zsh, where "the runner only takes grep" would mean somebody could widen the runner.

## The release pre-flight runs the Rust suite

**`preflight()` in `bin/.devbin/cmd/build.d/release` now runs `cargo test --workspace --no-fail-fast` after the bats suite and aborts on failure** (`:469-473` at HEAD), and refuses to release when the native manifest is present and `cargo` is not on `PATH` (`:466-468`). On a product whose shipped artefacts ARE the Rust binaries, the tag path no longer skips the Rust suite.

**`--skip-tests` still returns from `preflight()` before doctor, the bats suite and the cargo gate** (`:430-433`). The dirty-tree refusal on the tag path no longer recommends it (`:917-923`): its message is `refusing to tag a dirty tree -- commit or revert the above, then re-run`, because following an instruction to re-run with every correctness gate off, on exactly the run that tags, was the realistic failure.

**Boundary worth stating with it: the dirty-tree check reads `git status --porcelain`, so it structurally cannot see a writer that only writes GITIGNORED paths** -- the runtime store being the live example. That is the limit of what that gate can be asked to prove, not an argument against it.

## Where it lives

- **The gate** -- `lib/templates/hooks/pre-commit.sh`, reached through the `pre-commit.intent` shim (`lib/templates/hooks/pre-commit-shim.sh`); the presence-gated guards it dispatches are rostered in `lib/templates/hooks/pre-commit-guards.sh`.
- **The runner** -- `native/rust/crates/intentsvcs/src/critic.rs`, surfaced by `intent critic` in `native/rust/crates/intent-cli/src/render.rs`.
- **The arming decisions** -- each rule's frontmatter and `## Detection` section under `intent/plugins/claude/rules/`.
- **The release pre-flight** -- `bin/.devbin/cmd/build.d/release`, `preflight()`.
