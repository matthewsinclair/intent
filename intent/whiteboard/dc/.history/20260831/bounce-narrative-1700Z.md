# The post-bounce session -- discovery narrative, 2026-08-31 16:17Z to 17:00Z

Moved off the live board at the 18:12Z localfold. **The RULES stayed on the board; this is the reasoning that produced them.** Nothing here is needed to act; everything here is needed to understand why the board says what it says.

Landed as `d7b644d8`, `a239bdf9`, `9adff0c1`, `e935a294`, `16cdd671`, `74c39226`, `90fb0c52`, `1c3c17cb`, `fea027ba`.

## Four claims died today and only ONE was a reasoning error

That ratio is the finding. Each of the other three was a correct procedure pointed at a place that could not answer.

| claim                                       | killed by                         | why the instrument, not the reasoning                                                              |
| ------------------------------------------- | --------------------------------- | -------------------------------------------------------------------------------------------------- |
| `at lint` passes a bad citation             | re-driving before writing         | true only for a `to-write` row; the draft stated it unqualified                                    |
| no verb connects the read-back to the store | cc's construction, my third drive | a FRESH project cannot exhibit the carry, and `st show` prints neither field                       |
| `--note` is declared on neither build       | dumping one arm raw               | zsh passed `"at green"` as ONE argv token, so every arm errored and the symmetry read as a control |
| `0078` is unreachable                       | cc constructing the condition     | correct fact, wrong question -- `uninstall` acts on the TARGET, not on the source payload          |

**CONSTRUCTING THE CONDITION BEAT READING THE CODE THREE TIMES OUT OF THREE.** cc settled `0192` by building an indexed estate where two of us had read our way to the wrong answer; they settled `0078` by hand-placing a skill where I had read a listing. **A listing answers what is there. Only a constructed case answers what happens.**

## AC-02.1 -- the re-drive that confirmed, and why it was still necessary

`gate: ST0001 PASS -- 3/3` on the tree, `BLOCKED -- 1/3 satisfied; unsatisfied: AC-01.1 AC-01.2` on the keg. Unchanged from the morning -- **and the morning's log is stamped 16:31 local against a binary built at 16:49.** The run predated its own subject by eighteen minutes. W30 with a receipt rather than a caution.

**The discriminator, which the first drive never isolated:** the keg's `at green --help` declares only `-h`; the tree's declares `--daemon` and `--note <text>`. Three of the page's lines use `--note` and all three are refused with `error: unexpected argument '--note' found`. **One flag, two verbs, and it is the whole difference between the flagship onboarding page terminating red and green.**

A second, unlooked-for find in the same output: the keg's summary line reads `Set an AT green (reachable only from red)` and the tree's reads `Set an AT green`. So `criteria-and-tests.md:80`'s claim that the help "still says" it was true of the tag and false of main, and would have gone silently false at 3.0.1. Version-marked in both pages. **The durable half was re-driven and holds on both: `to-write` -> `green` direct is rc=0 everywhere, so no v3 build enforces it.**

## The at.recast draft, and the finding that did not survive contact

vc had RULED the `at lint` finding be filed. Re-driving it before writing killed it:

    $ intent at lint ST0001
    tests/unrelated.rs does not carry the literal id AT-03.1
    lint: ST0001 FAILED -- 1 finding(s) over 1 AT row(s)     [rc=1]

Identical on `553ac304` and on the keg, reached by two independent routes -- via `at edit` on the tree, and via a direct `at new` citation on both, since **the keg has no `at edit` to reach it with**.

**The narrower version is true and is not a defect.** lint passes the same bad citation while the row is `to-write` (`ok -- 1 AT row(s) conform`, rc=0) and raises it once the row claims a verdict. An unverdicted row is an intention, not a claim, and the gate still blocks it as unsatisfied. **So there is no state where an operator is told they are finished while carrying a bad citation -- which was the entire ground for filing.** vc re-measured and withdrew their own ruling.

**What that did to the prose.** Most of the draft described `at edit`, which does not exist in 3.0.0 (`error: unrecognized subcommand 'edit'`), and the `at new` duplicate-refusal block was already owned by `known-defects.md:35` + `install.md:77`. The landed edit is shorter than the draft: the version marker the estate already uses at four other sites, the `at new` block dropped as a second home, and the lint paragraph reframed as the true statement.

## The register already owned the finding I was told to file

`0151` owns the row and `0154` owns the class, and `0154`'s table names issue `title` with door _none after creation -- 0151_. **I came within one command of filing a third record**, under an explicit instruction to file. What stopped it was reading the sibling rows my own board already NAMED.

**And `0154` had PREDICTED the half-fix that then shipped.** Its closing paragraph asks for `issues edit <id>` covering title AND body plus a `wp` verb, and warns that _a fix reaching only one field or only one entity closes the instance already filed and leaves the case that prompted the filing exactly where it is_. `issues edit` then shipped with `--body`/`--from` and no `--title`. **No new row could have said that.** Driven in a scratch tree: body door rc=0, `--title` refused rc=1, `wp`'s ten verbs all status or size.

## AC-02.3 -- the denominator, and what the number is FOR

138 -> 143 as I sat down, 143 -> 144 when cc filed `0196` mid-task. **Three of the five new members were `0191`/`0192`/`0193`, filed by me that morning.**

**It converges because filing stops at the tag**, not because anyone catches up. Any criterion derived from a register anyone can add to has this shape.

`0081` took a sixth probe and **the five failures are the finding**: on v3.0.0 `st hydrate 0001` resolves as ST0001, `st dehydrate` is rc=2 unimplemented, `edit --path` does not exist. Every route that works on main is closed on the tag. The one that opens it is `intent edit intent:///issues/0001` -- **the usage the keg's own `--help` prescribes.** It carries a second, unfiled defect: the remedy offers _a thread or an issue_ when an issue is what was just refused.

## The two audits, in both directions

**vc audited my manifest and found four divergences; one was real.** `0016`, `0067` and `0079` sit under _Recorded against v3.0.0 and NOT present in it_, where a bolded entry means the OPPOSITE of reachable -- the page and the manifest agree. **`0191` was real and was mine**: the row cited a page where the id appeared nowhere, and my verified quote was the defect's MITIGATION rather than a statement of it. **The instrument could not catch it -- it checks that the quote is in the bytes, and my quote was. A row can cite a real page and a real string and still be about nothing the page says.**

**I audited vc's and found the mirror.** Their `0162` and `0194` were driven, written into `known-defects.md`, and committed -- and both still counted UNDISPOSITIONED, because the tool reads a TSV manifest and neither commit touched it. **The hard half done, the cheap half invisible, nothing failing and nothing warning.** That is W34.

## The rebuild window, from both ends

The gate's currency arm flipped to REFUSING mid-session when `abe69906` and `5f58265b` landed under `native/rust`. cc owned the commits, announced the rebuild, and verified it: `abe69906`, sha `66d9d77b`, both symlinks resolving, `verify_pair` green. **`Removed 49 files, 53.0MiB total` before a single line compiled; 1m18s to come back; benign only because the build succeeded.**

My own commit walked straight into it and **the gate refused correctly**, diagnosing the dangling symlink, naming the cause, and saying DO NOT reinstall because that races the build. It offered `--no-verify` and that was declined.

**cc's mirror-image observation is the better half:** the running `intentd` still holds the DELETED INODE, serving code that no longer exists on disk while `daemon status` answers live. **Absence is invisible from both ends** -- W28 forwards is an absent instrument reading as an empty answer; W28 backwards is an absent artefact with its dependant reporting healthy.

## Routing, and the instrument nobody designed

The register has no field for who filed a row -- every issue reads `reporter: Matthew Sinclair`. cc's path method (the filing commit's whiteboard directory) answered nothing for `0078`, `0162` or `0196`. **vc found `0162`'s author in the issue PROSE** -- _"Found 2026-08-30 by vc, by running it"_ -- a convention neither of them designed as an instrument, and the only one that answered. `0078` had neither, so its disposition is recorded as dc's judgement rather than the author's, under vc's rule that an unclaimed row is taken rather than left.

**cc refused to let commit adjacency stand in for provenance**, correctly: it is the same species of inference as reading a git author field, and the path method's whole value is that it does not rely on proximity.
