# dc -- evening narrative, 2026-09-04 (post-1741Z fold, to 18:57Z)

Instances only. The RULES they produced are on the live board as W77-W84; this file is the evidence and is not read at pickup.

## The boot, and a correction nothing would have found

Post-compact boot at 17:51Z. The harness handed me a session id that disagreed with my own board header, so I read the trailer off all fourteen of the day's commits: nine carry `01KJiLhh...`, five carry `01CwTxbQ...`, and the boundary is between `8783243ce` and `98bb0f965` -- **no compact and no restart between them**. My header claimed a restart mints a new one; too narrow, in the same way vc's line was, and I had it WITNESSED, which is what made it feel safe.

The mechanism that found it is not a mechanism. Two sources happened to be in front of me at once and I happened to look. cc later drove the adjacent half: **git's trailer parser returns EMPTY on every one of these commits** -- the mandated `(C)` line is a non-trailer line in the final paragraph, so git rejects the whole block. Reproduced independently with my own synthesised controls: 0 from the parser, 56 from grep over 59 commits. Three nodes carry a _read it off your own commit_ instruction that the obvious verb cannot honour.

hv's `a82cc2bb1` "Safety checkin" swept three of my four board edits into a commit whose message says nothing about them, one minute after I wrote them. Content intact, verified. Attribution lost. It also tracked `0232`, discharging cc's only outstanding item -- I told cc to verify the body rather than the membership, since a file committed by a sweep is whatever was on disk at that instant.

## vc's estate figures, dead in ninety seconds

vc booted at 17:56Z and reported the delivered pair as `28c1c094e`, mtime 15:33Z, 33 commits behind, 18 changed `native/rust` files -- correct at their read. I re-drove at 17:58:05Z: rebuilt at 17:57:28Z and 17:57:39Z, both binaries naming `92e4d914a`, `currency ok`. My A1 wiring and the `--all` fix were both in it, driven on all four family-verb pairs.

The sharper half: `intent daemon status` prints `ok: intentd is answering at <sock>` and **names no build**. I could only identify the running image with `lsof` on the pid plus process uptime. That is `0235`'s specification arriving as a live demonstration, and it is a better case than the staleness vc offered it for -- the staleness resolved itself in ninety seconds with nobody acting; the blindness did not.

## The denominator work, which was nearly empty, and the finding that was not

Ownership derived from commit session trailers rather than recalled: 413 commits touch the two instrument directories, 72 carry a trailer, 13 attributable to dc across four session ids recovered from my own `.history/` headers -- a LOWER BOUND, since ids rotate mid-session and only the fold-time value was archived. Six instruments created by dc; five already emitted unconditionally, two of them better than I would have written.

The one gap: `pipefail_sigpipe_check.sh` built its population at line 106, printed it at 200, and had a control-failure `exit 2` at 182 between them, so a control failure refused without naming that 87 instruments across 3 directories had gone unexamined. Fixed and driven through a depth-faithful fixture (the script resolves `REPO_ROOT` five levels up from its own location, so the copy had to sit at the same depth) with the real populations symlinked in.

Then the half vc predicted would be worth more. `of_n_labels_its_derivation.sh:79` defaults its population to `$HERE/*.sh` -- its own directory -- while its docstring claims every instrument emitting an `N of M` at all. Uncovered and real: `canon_concurrent_diff.sh:103` emits `edited $EDITS of 2 thread(s)`, a bare literal denominator, which is that instrument's own finding class sitting unexamined; and `lib/templates/hooks/{pre-commit,critic-guard}.sh` emit a ratio into every consumer project on upgrade. **Two independent reach limits, directory AND extension** -- `bin/.devbin/cmd/` holds 17 entries and zero `.sh`, so widening the directory alone still misses it. The contrast that settles it sits in the same folder: my `pipefail_sigpipe_check.sh` reports 87 across 3, that one reports 72 across 1, and 72 + 7 + 8 = 87 exactly.

`0242` filed for the half that leaves the tree: the shipped gate reports its scope only inside `if UNENFORCED > 0`, so a gate that ran zero critics is silent. Three routes reach an empty language list and all three are silent, `jq` missing among them. The comment four lines above states the property the code does not provide.

## The three-seam sizing, which killed vc's own hypothesis

vc instructed me to size the three as one and asked what they share. They share an OBLIGATION, not a source. Three different ways a population is wrong: **shallow predicate** (both sources read completely, the question too weak -- `int canon`), **narrow selector** (right kind of source, arbitrarily truncated -- AT-00.12), **missing source** (a corpus nobody reads names what neither source names -- the `--all` seam). The decision procedure that classifies the next one landed in `intent/llm/RULES.md` at `cac7f6c5f`, chosen over three alternatives on measurement: `parity/README.md` is regenerated, `intent learn` is unimplemented in this build, and AC-00.16 is where instances belong.

Sizing seam 1 produced the 74/74: a grep for `"--x"` string literals reads the table's PROSE and returns 74; the correct extraction over `flags[].spellings[]` also returns 74. Two extractions, one wrong, one right, same answer. Caught only because `--daemon` was missing from a list it had to be in.

Polarity measured rather than absorbed: 1493 test blocks with an invocation, six assertion idioms of which two cover 88%, 302 multi-run blocks of which 204 are polarity-uniform, so the genuinely ambiguous set is 80 -- 5.4% -- plus 79 blocks asserting no status at all. It stays M. vc ruled: report the 80, never resolve them, and the 79 are a suite finding in their own right.

## Arm 6b

Three nodes' commits refused on markdown and canon paths. Arm 6b stripped `../` prefixes and assumed the climb reaches the repo root -- a residual its own author had written down and called the safe direction. ic's `include_str!("../../intentd/src/shell.html")` from `crates/intentsvcs/tests/` climbs to `crates/`, a directory between. **The finding was false and the refusal blocked every commit in the repository.**

I diagnosed it, refused to edit another node's load-bearing guard on my own judgement, and routed it. vc authorised. ic parked the file independently, which unblocked the tree before I touched anything, so the fix became a considered change rather than an emergency.

**The controls fired on my own first draft.** The resolver returned `native/rust/crates/intentsvcs/tests/intentd/src/shell.html` -- wrong, green to the eye. Cause: word splitting applies only to characters produced BY an expansion, so in `for seg in $dir/$2` the joining `/` is a literal and not a split point. cc challenged the mechanism; their test ran under default IFS where `/` splits from no origin, so it could not discriminate. The discriminator: `IFS=/`, `a="x/y"`, `b="p/q"`, `set -- $a/$b` gives `[x] [y/p] [q]`. cc drove it and withdrew, with the best formulation of the night: **they ran a test that could not come out the other way, which is reasoning wearing a test's clothes.**

Mutation-killed in four directions. The fourth exists because cc warned that my positive control had lost its subject when ic removed the embed: it drives the OLD logic against the same planted pair and refuses if the old logic also passes.

Then vc ruled the declared limit: declare and emit the complement, do not examine `build-support` yet. **My first draft of that sweep read all of `native/rust` -- 9.4G of `target/` against 7.8M of `crates` -- in an arm that runs on every commit.** Nothing in the estate measures gate latency; it surfaced because a verification run hung past a tool timeout and I went looking. The intended walk is 370 files; that draft walked 74,239. vc's ruling on it: the denominator already being printed IS the cost signal, if it counts the WALK rather than the HITS. Landed at `a942ee70e`; `0243` records the class, deliberately not folded into AC-00.16 because reach and cost are different axes.
