---
description: "TCA init: provision steel thread, rule set, component map, and work packages for a Total Codebase Audit"
chains_to: ["in-tca-audit"]
---

# TCA Init

> **Invariant (load-bearing)**: A TCA is always its own dedicated steel thread. NEVER provision a TCA as a work package inside the audited ST. See `intent/docs/total-codebase-audit.md` section 0.0 for the four failure modes this rule prevents. The `tca-init.sh` script enforces this with a provisioning guard and will refuse to run against a path that looks like it is inside an existing work package.

Provisions a Total Codebase Audit: creates the steel thread and its work packages through the CLI, defines the rule set, maps the codebase into components, and writes each WP's scope, file list and rule focus into the store with `intent set`.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Gather parameters

Ask the user:

- Project name
- Ecosystem(s): Elixir, Rust, Swift, Lua, Shell, or a polyglot combination
- Approximate file count (or let discovery determine it)
- Whether the project uses Ash Framework
- Whether it is an umbrella/monorepo

### 2. Select the rule packs

A TCA enforces Intent's rule library. There is no per-audit invented rule numbering -- every cited rule has a stable `IN-*` ID, served by the installed Intent tool via `intent claude rules show <id>` (`intent claude rules list` to enumerate). The schema lives in `intent/docs/rules.md` at the Intent install.

Default rule packs by ecosystem:

| Ecosystem | Packs to load                                                                                               |
| --------- | ----------------------------------------------------------------------------------------------------------- |
| Elixir    | `agnostic` + `elixir/code` + `elixir/test` (+ `elixir/ash`, `elixir/phoenix`, `elixir/lv` per dependencies) |
| Rust      | `agnostic` + `rust/code` + `rust/test`                                                                      |
| Swift     | `agnostic` + `swift/code` + `swift/test`                                                                    |
| Lua       | `agnostic` + `lua/code` + `lua/test`                                                                        |
| Shell     | `agnostic` + `shell/code`                                                                                   |
| Polyglot  | Union of the above per ecosystem; agnostic loads once                                                       |

Enumerate the actual rule IDs to be enforced for this audit:

```bash
intent claude rules list --lang elixir
intent claude rules list --lang agnostic
```

Customize with the user via `.intent_critic.yml` at the audited project root, **not** by inventing a new rule numbering:

- Disable rules that don't apply: `disabled: [IN-EX-TEST-003]` (with a `# reason:` comment per rule) -- a real id, because an invented one in an example is indistinguishable from a citation and this is the line that says not to invent rule numbering.
- Set the body threshold: `severity_min: warning` (default) or `style` to see everything.
- **Project-specific rules have no v3 home yet, so the audit set is the canon rules minus the `disabled:` list.** The `~/.intent/ext/` extension mechanism this line used to point at is v2's: v3 reads that directory nowhere, and hv ruled `ext` out of the 3.0.0 cut on 2026-08-30 rather than porting it. Naming a layout the tool does not read is the same defect as naming a verb that refuses -- correct-looking prose pointing at nothing.

The rule set for the audit is the canonical IN-\* IDs minus anything in the project's `.intent_critic.yml` `disabled:` list. Critics enforce this set automatically -- no per-audit prompt template required.

### 3. Map codebase into components

Guided discovery:

1. List top-level source directories
2. For each directory, enumerate files by type using glob patterns
3. Calculate **effective file counts** using weight table:

| File Type         | Weight | Rationale                          |
| ----------------- | -----: | ---------------------------------- |
| Standard code     |   1.0x | Normal audit effort                |
| Ash DSL resources |  0.25x | Declarative, limited violations    |
| Emission/struct   |   0.5x | Thin files, few rules apply        |
| Dead stubs        |   0.0x | Exclude entirely                   |
| Rust code         |   1.5x | Ownership + lifetime complexity    |
| Swift AppKit      |   1.3x | Legacy framework, higher density   |
| Test files        |   0.0x | Excluded unless testing rules used |

4. Group files into WPs targeting 12-20 effective files each
5. Identify 3-4 special focus rules per WP
6. Identify cross-WP Highlander dependencies

### 4. Create steel thread

```bash
intent st new "TCA: <project and scope>" --start
```

The title should describe the audited project and scope, not just say "TCA". Examples: `"TCA: Lamplight Gen 3.0"`, `"TCA: Intent v2.8 core"`, `"TCA: Conflab umbrella polyglot"`. Informative titles prevent collisions across multiple audits on the same codebase over time.

Capture the ST ID from the output -- this is the TCA steel thread that all subsequent work lives in. The audited project's steel thread is not involved.

### 5. Run tca-init.sh

The script lives alongside this SKILL.md:

```bash
bash "$(find ~/.claude/skills/in-tca-init -name tca-init.sh 2>/dev/null | head -1)" \
  --tca-dir intent/st/STXXXX \
  --wp-count N \
  --project "ProjectName"
```

The script writes files only. It refuses a `--tca-dir` inside an existing WP and refuses to overwrite populated `socrates.md` files, then creates `WP/01`..`WP/NN` on disk, each with a templated `info.md` and an empty `socrates.md`, titling the last one as the synthesis WP. It skips any WP directory that already exists. **The store does not register these directories**: `intent wp list STXXXX` reports no work packages, `intent set` refuses them, and `intent organize --verbose` lists them as unclaimed. Register each one, in order, synthesis last:

```bash
intent wp new STXXXX "<Component name>"          # once per component, in order
intent wp new STXXXX "Cross-Component Synthesis" # always last
```

On a fresh thread `intent wp new` numbers from 01, so each registration lands on the directory the script made. It re-renders that WP's `info.md` from the store, replacing the script's template; the empty `socrates.md` is kept. Step 6 writes the template's sections back through the store.

### 6. Write each WP's objective and body

A WP's `info.md` is a generated view: `intent doctor` reports a hand edit as view skew, and `intent sync --to-disk` discards it. Write through the store, addressing the WP as `intent:///threads/STXXXX/wp/NN`:

```bash
intent set intent:///threads/STXXXX/wp/NN objective "<1-2 sentence scope>"
intent set intent:///threads/STXXXX/wp/NN body --from wp-NN-body.md
```

`wp-NN-body.md` carries: the complete file list, one `` - `path` `` line per file (step 10 greps the rendered `info.md` for these); the applicable rule packs (per §2) and the critic to dispatch (`critic-elixir`, `critic-rust`, etc.); the special-focus IN-\* IDs (eg `IN-EX-CODE-006` for a known-Highlander-prone subsystem); and the cross-WP Highlander dependencies (2-4 other WPs that might overlap). Set the size with `intent wp rescope STXXXX/NN <size>`.

### Mint the acceptance contract

`intent st done` refuses a thread with no criteria, and `/in-tca-finish` closes against them:

```bash
intent ac new STXXXX AC-01 --text "Every component WP has its critic report captured in socrates.md"
intent ac new STXXXX AC-02 --text "The synthesis WP holds the prioritized remediation backlog"
intent ac new STXXXX AC-03 --text "feedback-report.md exists with no [Fill in placeholders"
```

### 7. Write design.md

Head the rule-pack section `## The rule set` -- `tca-report.sh` refuses a design.md without the literal lowercase `rule set` (or `Rule <N>` / `R<N>`), and `## Rule packs loaded` does not match. Once design.md and tasks.md are written, record them in the store with `intent st attach STXXXX design.md --from intent/st/STXXXX/design.md` (and the same for `tasks.md`); until then `intent organize --verbose` lists them as unclaimed.

The steel thread's design.md should contain:

- Rule packs loaded for this audit (per §2), with the actual IN-\* IDs enumerated
- `.intent_critic.yml` content if any rules are disabled or thresholds adjusted
- Component map with effective file counts (per §3 file weights)
- Batch ordering for parallelization (dependency-ordered)
- Pre-filter results (Phase 0.5)
- **False Positive Guidance (REQUIRED -- not optional)**: for each IN-\* rule with known non-violations in this codebase, list the acceptable patterns BEFORE Phase 1 starts. Without this section, mechanical rules generate high FP rates at synthesis time. In Lamplight ST0121, pre-classifying the R7-equivalent rule removed its false positives. If this section is missing or contains placeholder text, do NOT proceed to Phase 1 -- go back and author it.

#### False Positive Guidance format

For each IN-\* rule that has known non-violations in this codebase, add a subsection like this:

```markdown
### IN-EX-CODE-002 (tagged-tuple-returns) False Positive Guidance

`Map.get/2` returning a value that may be `nil` is CORRECT on:

- Plain map types (config.properties, counters, LLM response maps)
- Ash metadata maps
- Any `%{}` not defined with `defstruct`

`Map.get/2` returning `nil` is a VIOLATION on:

- Any module defined with `defstruct` where a missing key indicates an error
- Known typed state containers (Pctx, Pctx.Mechanic, PhaseState, etc.)

### IN-EX-CODE-003 (impl-true-on-callbacks) False Positive Guidance

A `def` without `@impl true` is CORRECT on:

- Plain functions in a module that does not `use` or `@behaviour` anything
- `defp` helpers
```

Rules without known non-violations can be omitted. Rules that do have non-violations MUST be documented -- an unsure auditor is a noisy auditor. Where a project-wide disable is the right answer, lift the rule into `.intent_critic.yml` instead.

### 8. Write tasks.md

Phase checklist:

- Phase 0: Provisioning (this step)
- Phase 0.5: Pre-filtering
- Phase 1: Component audit (list all WPs)
- Phase 2: Synthesis
- Phase 3: Review
- Phase 4: Remediation

### 9. Pre-filter mechanical rules (Phase 0.5)

Mechanical rules with stable grep signals can be pre-filtered before launching critics. The critics will catch the same violations, but pre-filtering gives Phase 0 ground truth that disagreements between mechanical hits and critic findings are signal worth investigating.

```bash
# Debug artifacts (Elixir): IO.inspect / dbg() in production paths
grep -rn "IO\.inspect\|dbg()" lib/ --include="*.ex"

# Bare Map.get on struct candidates (Elixir): IN-EX-CODE-002 territory
grep -rn "Map\.get(" lib/ --include="*.ex"

# Missing @impl on behaviour callbacks (Elixir): IN-EX-CODE-003
grep -rL "@impl" lib/ --include="*.ex" | xargs grep -l "def mount\|def handle_"
```

Adapt patterns for the project's ecosystem. Record results in design.md so synthesis can cross-check critic output against the mechanical baseline.

### 10. Verify file manifests

For every WP, confirm all listed files exist:

```bash
# Quick verification
for f in $(grep "^- \`" intent/st/STXXXX/WP/01/info.md | sed 's/.*`\(.*\)`.*/\1/'); do
  [ -f "$f" ] || echo "MISSING: $f"
done
```

Fix any stale references before proceeding to Phase 1.

## Important Notes

- The sweet spot is 12-20 effective files per WP
- Always create a synthesis WP as the last WP
- Encode cross-WP Highlander dependencies at provisioning time, not synthesis time
- Pre-filter results are ground truth for validating sub-agent findings
- **WP layout is FLAT**. Component audits are top-level `WP/NN` under the TCA ST. Never create `WP/NN/WP/MM` sub-WPs -- the `intent wp` CLI rejects nested specifiers and Intent's WP model does not support them. See Invariant 2 in `intent/docs/total-codebase-audit.md` section 0.0.
- **False Positive Guidance is load-bearing**. Skipping the FP Guidance section produces noisy audits with low signal-to-noise ratios. Budget time at Phase 0 for pre-classification; that time pays for itself by eliminating synthesis-time triage.
