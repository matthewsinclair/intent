---
id: "0012"
title: ws hygiene passes a board whose frontmatter is not valid YAML, and ws list mis-renders one that is
date: 2026-08-02
reporter: matts
status: CLOSED
severity: medium
---

# 0012: ws hygiene passes a board whose frontmatter is not valid YAML, and ws list mis-renders one that is

## Tags

whiteboard, ws-hygiene, frontmatter, false-green

## Summary

`intent claude ws hygiene` is the whiteboard's lint gate, and it does not check that a board's frontmatter parses. It reads keys with `fm_get`, a line-scanning `awk` that splits on the first `:` and never parses YAML, so a `wip.md` whose frontmatter no YAML parser can read reports `hygiene: ok` and exits 0.

**The second half is what turns this from a missing check into a design fork: the tooling does not merely tolerate the invalid form, it prefers it.** `cmd_ws_list` renders `focus:` by stripping the surrounding quotes with `sed 's/^"//; s/"$//'` and never unescaping the body. So a board with unescaped `"` inside its quoted scalar -- invalid YAML -- displays correctly, and a board corrected to valid YAML displays its escapes literally as `\"`. Both files pass hygiene. The format is declared YAML in the protocol and consumed as line-oriented text by every reader in the tool, and where the two disagree the tool rewards the file that is wrong.

Nothing has broken from this, and the reason is the finding rather than a mitigation: the protocol says peers read each other's frontmatter at pickup, but every actual reader today is a human, an LLM, or `fm_get`. **The one channel specified to be machine-read is the one nothing machine-reads.**

## Reproduction

Observed in the Lamplight project, 2026-08-02, on a five-node whiteboard.

```
# 1. A focus: scalar containing an unescaped quote is invalid YAML.
#    Both of the project's most active boards had one.
ruby -ryaml -e '
  Encoding.default_external = Encoding::UTF_8
  %w[hv cc vc ic ac].each do |n|
    t = File.read("intent/whiteboard/#{n}/wip.md", encoding: "UTF-8")
    begin YAML.safe_load(t.split(/^---$/)[1]); puts "#{n}: OK"
    rescue => e; puts "#{n}: FAIL #{e.message[0,60]}" end
  end'
# hv: OK
# cc: FAIL (<unknown>): did not find expected key while parsing a block mapping
# vc: OK
# ic: FAIL (<unknown>): did not find expected key while parsing a block mapping
# ac: OK

# 2. The lint gate passes the same tree.
intent claude ws hygiene; echo "exit=$?"
# WARN      ac/wip.md is large (48577b) -- archive DONE content
# ... (size warnings only) ...
# hygiene: ok
# exit=0

# 3. Correct one board to valid YAML (escape the internal quotes as \"),
#    then look at how the tool renders it.
intent claude ws list | grep -o '\\"' | wc -l
# 8      <- the escapes are displayed literally, mid-prose
```

The offending scalars in the two failing boards were ordinary prose: `"2x per-model tokenizer gap"`, `"."`, `"the counted body is the SENT body"`, `"probably next"`, `"an open question for hv"`, `"cc holds the build lock"`. Quoting a phrase inside a `focus:` line is the natural thing for a node to write, and nothing at any point in the loop says not to.

## Root Cause

`intent/plugins/claude/bin/intent_claude_cwi`:

- `cmd_ws_hygiene` (`:191`) checks six things: `wip.md` exists; `fm_get node` is non-empty; `fm_get status` is non-empty; `.history/.gitkeep` exists; each `inbox.*.md` opens with a `# inbox: ` header; and a size warning above 16000 bytes. **None of them is "the frontmatter parses."**
- `fm_get` (`:63-72`) is `awk` that walks lines between the `---` fences, splits each on the first `:`, and prints the remainder of the matching line. It returns a value for `node:` and `status:` regardless of whether the document as a whole is well-formed, so the two frontmatter checks that do exist cannot fail for this reason.
- `cmd_ws_list` (`:167`) reads the same way and post-processes `focus:` with `sed 's/^"//; s/"$//'` -- a hand-rolled approximation of one YAML quoting rule that implements the delimiters and not the escaping.

This is a gap rather than a defect in what was built: hygiene was scoped to mechanical structure -- files present, keys present, headers present -- and does that correctly. Validity of the document was never in scope. Same family as 0006/0007: a gate reports on what it was pointed at and is silent about everything else.

## Impact

Low today, and the shape of the risk is worth stating precisely rather than inflated.

- **No live breakage.** Every consumer in the tool is line-based, so an unparseable board is read successfully by `ws list`, by `ws hygiene`, and by the humans and LLMs that read it at pickup.
- **The exposure is any future strict reader.** The protocol (`intent/whiteboard/README.md`, and the `pickup` procedure) specifies that peers read peer frontmatter, and specifies it as YAML. The first tool that honours that literally -- a status dashboard, a `ws list --json`, a hygiene check added later -- fails on real boards, and fails on the busiest ones, because a long hand-written `focus:` is exactly where quoted prose accumulates.
- **The defect self-heals, which is why it has never been caught and why the rate is easy to under-estimate.** Sweeping the last 25 revisions of one node's `wip.md` in the reporting project: **4 were invalid, in two separate episodes on different days, and both repaired themselves without anyone acting** -- the next fold rewrites the `focus:` line and the offending quote pair leaves with it. A defect whose lifetime is shorter than the interval between observations leaves no corpse to autopsy. The point-in-time measurement (2 of 5 boards invalid) is therefore a floor, not an estimate.
- **A node that discovers the rule and complies is punished for it.** Escaping correctly makes `ws list` -- the tool's own display of the board -- render `\"` in the middle of prose. That is an active disincentive to fix, and it means the correct state is currently the ugly one.

## Proposed Fix

**The fork is the decision, and it belongs to the maintainer rather than to this report: either the frontmatter is YAML or it is not.** The two halves above are one inconsistency seen from opposite ends, and fixing only the missing check would make `ws list` mis-render every board that then complies.

- **If YAML:** add a parse check to `cmd_ws_hygiene` beside the existing key checks, and fix `cmd_ws_list` to unescape rather than `sed` off the delimiters. The parse needs a dependency the tool does not currently carry -- there is no YAML parser anywhere in `bin/`, `lib/`, or `intent/plugins/` -- so this is a real decision about the tool's floor, not a one-line addition. A shell-only approximation (counting unescaped quotes inside a double-quoted scalar) would catch this specific class without a dependency, and should be labelled as the approximation it is rather than as a validity check.
- **If not YAML:** say so in `intent/whiteboard/README.md` and in the `pickup` procedure, and stop describing the block as YAML frontmatter. A documented line-oriented key/value block with a "quotes are literal, do not escape" rule would make every board in the reporting project correct as-is, make `ws list` correct as-is, and reduce this issue to a documentation change. **This is the cheaper answer and may well be the right one** -- the block is hand-written by LLM nodes in prose-heavy fields, which is close to the worst case for a quoting-sensitive format.

Either way, whichever is chosen should be enforced at hygiene, because the current state is that neither is.

Not proposed, deliberately: a fix that silently rewrites a node's `wip.md`. Single-writer ownership is the whiteboard's core invariant, and hygiene edits another node's board only at its owner's request.

## Related

- 0006, 0007 -- the acceptance-parser fixes: same family, and the same lesson that a gate reports on what it was pointed at
- 0010 -- `st done` close-gate scoped narrower than the record it appears to certify

## Resolutions

FIXED + CLOSED (2026-08-14), shipped in v2.19.0. Both halves reproduced: an invalid-YAML board passed `ws hygiene` with exit 0, and the *escaped* (valid-YAML) board rendered a literal `\"` mid-prose in `ws list`.

### The ruling: the board's header block is NOT YAML (hv-directed, vc-ratified)

The block was **documented** as YAML frontmatter and **consumed** as line-oriented text by every reader in the tool. That is the defect -- not the invalid boards, which were a symptom. Where the two disagreed, the tooling rewarded the file that was wrong: `ws list` stripped the surrounding quotes without unescaping, so a board with unescaped quotes inside a `focus:` scalar displayed correctly while a board *corrected* to valid YAML displayed the escape. And hygiene never checked that the block parsed at all -- the one channel the protocol specifies as machine-read was the one nothing machine-checked.

The fork could have gone either way, and it went to **line-oriented `key: value`**: one line per key, a single pair of surrounding quotes as a display delimiter, quotes inside a value literal and never escaped. The reason is the writer, not the format: the block is hand-authored by LLM nodes in prose-heavy fields, which is close to the worst case for a quoting-sensitive format. On the reporting board two of five nodes were unparseable at a point in time, and a sweep of one node's last 25 revisions found four invalid across two separate episodes -- **every one of which repaired itself at the next fold before anyone noticed**. A format that is silently violated and silently repaired is not being enforced by anything; declaring the simpler contract makes the implemented behaviour correct rather than accidental.

### What shipped

- The rule is stated in the `/in-whiteboard` skill, the whiteboard `README.md`, and `intent/docs/working-with-llms.md` -- which had described the block as "frontmatter" and never described its format at all.
- `ws hygiene` rejects any line in the block that is not a single-line `key: value` (the shape that genuinely breaks a line-oriented reader), reporting the offending line number. It **warns** rather than fails on a missing recommended key, so boards predating the rule still pass. It says nothing about YAML validity, because validity is not the contract.
- The display-delimiter strip moved into `fm_get`, so `ws list` and `ws hygiene` read one value through one reader. `ws list` no longer hand-strips quotes.
- Canonical skill source is `intent/plugins/claude/skills/in-whiteboard/`, and `SKILL.md` itself was changed so `intent claude skills sync` propagates it (the sync checksums `SKILL.md` only -- a script-only edit would not have shipped).
