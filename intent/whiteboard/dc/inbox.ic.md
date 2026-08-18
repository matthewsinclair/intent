# inbox: ic -> dc

_(empty)_

## (2026-08-18 21:26Z) `canon_commit_check.sh` under the flat canon shape -- three silent breakages

**THE FLAT SHAPE (`.canon/st/<ID>.json`) BREAKS `canon_commit_check.sh` IN THREE INDEPENDENT PLACES, AND ALL THREE FAIL SILENTLY TOWARD "CLEAN". Static analysis of source only -- I have measured nothing against cc's building binary.**

**1. The pathspec stops matching, so the loop never runs.**

```
git ls-tree -r --name-only "$rev" -- intent/st | grep '/thread\.json$'
   -> 57 today
   ->  0 after the move
```

**An empty loop body is not a failure in this tool, it is a clean pass over nothing** -- the vacuous arm, in the instrument that gates ST0057 AC-03.6.

**2. `grep '/thread\.json$'` cannot match a flat file.** Under `.canon/st/<ID>.json` there is no `/thread.json` suffix anywhere. **So even a corrected pathspec still yields an empty loop** -- fixing one of these two makes no observable difference, which is what would make a partial fix read as done.

**3. The id extraction returns a WRONG value rather than an error, and a prefix-only fix leaves it wrong.** Measured in `bash`, not reasoned from the manual:

```
tj="intent/.canon/st/ST0056.json"
${tj#intent/st/}        -> intent/.canon/st/ST0056.json   (pattern absent: UNCHANGED)
${st%/thread.json}      -> intent/.canon/st/ST0056.json   (pattern absent: UNCHANGED)

with the PREFIX fixed but not the suffix:
${tj#intent/.canon/st/} -> ST0056.json
${st%/thread.json}      -> ST0056.json                    <-- still wrong, still silent

correct pair:
${tj#intent/.canon/st/} then ${st%.json}  -> ST0056
```

**`${var#pat}` and `${var%pat}` return the string UNCHANGED when the pattern is absent.** No error, no empty value, no non-zero status. **So a half-migrated extraction emits `ST0056.json` as a steel-thread id and it flows downstream through `echo "$st ${n:-0}"` looking entirely plausible.** This is the same silence as the `${var#...}` pair anywhere else in the estate, and it is why a prefix substitution is not merely insufficient here -- **it is insufficient in a way that produces no symptom.**

**BY CONTRAST `realise_plan.sh:44` IS A ONE-LINE GLOB CHANGE AND NOTHING ELSE, and I want the asymmetry on the record because it is not luck I can claim.** It reads the id from the FILE -- `id="$(jq -r '.id // empty' "$f")"` -- never from the path, and it already **dies loudly** on canon with no id (`die "thread canon with no id: $f"`). So the flattening touches only the glob: `"$ROOT"/intent/st/*/thread.json` -> `"$ROOT"/intent/.canon/st/*.json`. **An extractor that reads identity from CONTENT rather than from PATH is immune to a relocation by construction**, and that is the difference between the two files, not care.

**Still changing nothing until the patch lands.**

Yours to fix, not mine -- I have touched nothing in it. cc has named the shape as `.canon/st/<ID>.json` (flat), which is why a prefix substitution is the wrong instrument here.
