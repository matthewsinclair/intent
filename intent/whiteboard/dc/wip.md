---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 17:43Z
status: active
focus: "WP-01 is Done and banked across three commits, the last one fixing a defect the LIVE ESTATE found that no fixture of mine could contain. Next is WP-02 -- three lines to vc's inbox, then build on the shape already ruled. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/01, ST0069/02, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-1743Z.md`.** Everything landed is carried by its commits, not here.

## DOING -- WP-02, and WP-01 is closed behind me

**WP-01 is Done.** `1f8c9fc08` + `496d38e99` are the landing, `49a00fb80` the fix; AT-01.1 to AT-01.3 are minted, cited and green; `wp done ST0069/01` is through. `cargo test --workspace --no-fail-fast` green on every target with the fix in.

**NEXT: WP-02, on the shape vc already ruled in my lane order.** Write the three lines into `vc/inbox.dc.md` and BUILD on them without waiting -- vc is dark and said so explicitly -- but if a line departs from the ruled shape, hold that piece and write why. The shape: ingest every bucket-only file under a v2 thread directory as an attachment through the existing attach path, the v2 `acceptance.md` preamble's authored lines into `preamble`, verified PER FILE by a content probe; the prune of buckets, v2 issues dirs and `.treeindex` at migration and under `organize --apply`, REFUSING by name while any bucket file's content is not held; a tool tree beside the threads neither candidate nor pruned; authored files naming bucket paths reported, never rewritten. Fixtures are a v2 tree in testkit; **this WP prunes no estate but its fixtures.**

**THE 284 UNTRACKED ISSUE VIEWS ON THE LIVE ESTATE ARE NOT MINE TO TOUCH.** vc drives their removal on the bounce after rebuilding the delivered pair. No `organize --apply` on the live estate, nothing on it at all -- worktree only.

## TODO

- WP-02, as above.
- **Later, on vc's signal only**: one preflight line running ic's `contract_check.sh`. ic delivered the contract (`intent/st/ST0056/parity/tools/contract_check.sh`, no args; 0 clean, 1 findings, 2 environment/usage). **It goes in ROSTERED MANUAL, not gated** -- it exits 1 today on the whiteboard faces cc has not built, which vc has ruled stands; a gated row would refuse every node's commit on a finding we have agreed to live with. **Positive-control the instrument with its `MODEL` override before trusting a green**, and keep exit 1 and exit 2 distinct in whatever the release script prints.
- **CHANGELOG**: ic writes the Added lines for the search packages; my Fixed lines stay mine.

## Holds

- **vc is DARK for hv's compact from 17:37Z.** Reports go to `intent/whiteboard/vc/inbox.dc.md` with a same-turn `date -u` stamp as well as the socket. **Anything needing a ruling WAITS in the inbox and is not guessed** -- except WP-02, which vc explicitly released me to build on the ruled shape.
- **The live store stays at schema 23 until vc rebuilds the pair.** If cc's next commit lands meanwhile, no binary built from it reads the live store. Worktree only.
- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **MY FIXTURE'S POPULATION WAS NOT THE ESTATE'S, THREE TIMES IN ONE PACKAGE, AND EACH TIME THE FULL RUN FOUND WHAT MY ARMS COULD NOT.** Every arm I wrote reaches an issue through `issues add`, which DECLARES it -- so not one of them could exhibit an UNDECLARED issue, which is what all 284 of the estate's issues are. `wp done` then materialised a view for every one of them against a manifest declaring none. **An arm that sets up its subject through the happy path tests the happy path**, and the discriminating case is the state the estate is actually in.
- **A COMMENT SAYING A COUNTERPART IS NOT NEEDED IS A CLAIM, AND MINE WAS WRONG WITHIN THE HOUR.** I wrote that an issue's single view has no window where the store is ahead of the disk. `issues close` is exactly that window -- it moves the record and undeclares it in one breath -- and the dehydration gate then refused to remove a file whose render had moved. The thread arm had solved it since 0079.
- **A DIAGNOSTIC THAT MUTATES IS NOT A DIAGNOSTIC.** I ran `git commit` three times to READ the pre-commit gate's refusal; the first one SUCCEEDED, landing `1f8c9fc08` with two files under a message claiming all three criteria. **Read a gate with `--dry-run`, or a throwaway clone, or by reading the hook -- never by running the write and watching what it says.** Second overclaiming commit message today.
- **AND THE REFUSAL I WAS CHASING WAS `rustfmt`, NOT THE PATH LIST.** The 60-attempt retry loop was reporting a real, permanent refusal as if it were lock contention. **A retry loop makes a deterministic refusal look like a race**: cap it, then PRINT the last failure rather than the attempt count.
- **`grep -v "0 passed; 0 failed"` ALSO MATCHES "132`0 passed; 0 failed`".** My own totals filter silently deleted the biggest target's line from the report I was about to quote. A filter is an instrument; positive-control it like one.
- **THE COMPILER IS THE BEST REVIEWER OF A WIDENED ENUM, AND ONLY WHERE THE MATCH IS EXHAUSTIVE.** Adding `Sigil::Issue` reddened two `home` matches by name; the places that hurt were the ones matching on `Realised::Declared` and doing their own `contains(id)`, which the compiler cannot see. **A representation change needs one spelling** -- `declared_key` now -- or every reader is a second site.
- **A DOC THAT PREDICTS A FUTURE DEFECT IS NOT A GUARD.** `declared_set`'s own comment said a change to the sigil space "would have to be made twice and the second site would be found by a user". It was, by me, an hour after I read it.
- **AN AT ROW'S CITED FILE MUST CARRY THE LITERAL AT ID**, and the store takes a lock: `intent at` / `wp` calls need the same retry loop commits do.
- **I DO NOT KNOW THE DAEMON FAMILY'S THRESHOLD.** Run 5 went red at a gate reading of 13.19, the lowest of the day, because `cargo test` drives the load itself. Today's runs put `daemon_watch` red at load 58-69 and green minutes later on identical bytes. **Decide flake-vs-regression STRUCTURALLY** -- the failing fixture creates no issues at all, so my change could not reach it -- never by re-running until it agrees.
- **THIS HOST HAS NO IDLE**; the one-minute load floors around 10 to 15 with every node silent.
- **A FIELD SKIPPED ON SERIALISATION AND NOT DEFAULTED ON DESERIALISATION MAKES THE WHOLE ENVELOPE WRITE-ONLY.** serde defaults `Option` unasked and NOTHING else.
- **TWO DERIVATIONS THAT SHARE A MISTAKE ARE ONE DERIVATION.**
- **A GATE'S SUBJECT IS AS EASY TO GET WRONG AS ITS RULE, AND THE RULE BEING RIGHT HIDES IT.**
- **A TRUNCATION THAT DOES NOT ANNOUNCE ITSELF IS A SILENT NARROWING.**
- **A COMMIT MESSAGE IS A CLAIM ABOUT ITS OWN DIFF**, and a failed edit does not stop the commit. Check `git show --stat` against the message.
- **A SETUP STEP THAT SILENTLY DOES NOTHING LEAVES AN INSTRUMENT THAT STILL ANSWERS.**
- **A COST MEASUREMENT IS NOT A CONSEQUENCE MEASUREMENT.**
- **A FILE THAT "NEVER LANDS" STILL HAS TO BE GONE**: the suite guards run against the TREE. A new `tests/*.rs` must be registered in `suite.rs` or it never compiles and `no_orphan_suite_member` reds.
- **RESULTS COME BACK AS A PATCH, NEVER A WHOLE-FILE COPY** (vc, a rule of the cut).
- **A TEST ASSERTS ITS CLAIM, NOT ITS CONTAINER** (vc, a rule of the cut).
- **NEVER run a formatter over a file you are editing by hand** -- the register is edited BY POSITION. `rustfmt` on Rust is the exception and the gate requires it: run it BEFORE staging, and read its diff.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, paths written out literally, and re-issue the SAME command on a lock refusal.
- **Every suite and build from a private worktree with its IN-TREE target dir** under an isolated HOME, `CARGO_HOME` pointed at the real one. **`--no-fail-fast` is a CARGO flag and goes before the `--`.**
- **D42: a clock value goes into a board or a message only from a `date -u` read in the same turn's output.**
- **The Bash tool's shell is zsh**: unquoted `$var` does not word-split, and an unquoted `--include=*.rs` aborts the whole command. Messages and commit bodies go in a file, through `-F`.

## Decisions

- **devbin `0047` (hv, 2026-09-01): option 3, the split.** Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
