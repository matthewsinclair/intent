# inbox: cc -> hv

## (2026-09-03 17:21Z) FOLDED ON YOUR INSTRUCTION -- EVERYTHING BELOW IS A LIVE DECISION AND NOTHING ELSE

**19,134 bytes of handled entries archived VERBATIM and cmp-verified to `cc/.history/20260903/hv-inbox-cc-prefold-1718Z.md` (sha `3f307d11`).** Nothing is lost; what is gone from here is discharged, and it went because you should not have to read nine dead items to find seven live ones.

**DISCHARGED AND REMOVED, so you do not go looking:** `sync` skipping untracked bytes (the code took a third option I told you did not exist); the `~/.intent/home` dead-worktree blocker (resolves, `state: OK`); `autotests = false` (all three crates carry it); the test-target consolidation (landed); `Node`'s model and the two FYI entries. **And four items I escalated as YOURS have since been ruled by vc under the pen, so they are my work now rather than your decisions:** the migrator-commit contradiction, the `0086`/`0063`/`0095`/`0096` disposition, `0192`'s in-or-out, and `0205`'s vendored block. **`0214` is CLOSED**, which kills the cut decision I asked you for on it.

**SEVEN THINGS NEED A WORD FROM YOU. Each is a question you can answer with one.**

**[1] Should `at green` run the L3 arm?** (i) warn, do not refuse (ii) refuse (iii) leave as is. **REC (i).** Refusing breaks the legitimate write-then-cite order -- greening a row before the citing test exists -- **which is the order that produced this morning's gate outage**, so refusing would have converted a self-inflicted stoppage into a permanent one.

**[2] `INTENT_BIN` flip and the re-baseline -- which order?** (i) flip then re-baseline (ii) re-baseline then flip (iii) neither this cut. **REC (i).** The estate's default `INTENT_BIN` is `bin/intent`, **the v2 SHELL SCRIPT**, and it is THREE binaries not two (`test_helper.bash:21` v2 shell, `run_v2_suite.bash:55` v3 debug, `~/.local/bin/intent` v3 release). Re-baselining first pays the wall time twice.

**[3] `burn.sh` re-run, or accept `AC-06.1`'s coverage half red into 3.0.1?** (i) run it (ii) accept red and say so on the row (iii) descope the half. **REC (i), and it is yours because full-suite runs are yours** -- a double full-estate sweep that hung 3.5h once. `coverage_map.sh` refuses to publish and is right to: the burn TSV no longer covers the estate.

**[4] `WP-08` -- endorse vc's hold or override it?** vc ruled HELD under the pen, on a falsifiable condition: **conformance coverage exists for the daemon, or the gap is explicitly accepted on the record by you.** `wp done` is an XS away and I am not taking it. **REC endorse.** I built one piece of that coverage today and I am explicitly not arguing it discharges my own blocker.

**[5] `config` bare resolves to `target: undefined`. What should it do?** (i) print the resolved config (ii) print help (iii) refuse with a remedy. **REC (i) -- FLAGGED: this rests on my reading of the surface, NOT on a census of what other bare noun verbs do.** If that consistency claim is load-bearing for you, it wants driving first.

**[6] `agents` bare is recorded `pending-hv`. Same question, same options.** **REC: whatever you rule for [5], for the same reason** -- two bare nouns answering differently is the defect, not either answer.

**[7] Flip `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` to a gate?** (i) clear the class then gate (ii) stay report-only (iii) gate now. **REC (i)** -- the class is one-token path repairs, since five of six targets I once recorded as absent exist under another name. **PREMISE MOVED AND YOU SHOULD HAVE IT BEFORE RULING: the account lived in `0214`, and `0214` is now CLOSED.** The lint half is dc's.

**AND ONE THING THAT IS NOT A DECISION, RECORDED BECAUSE IT WOULD OTHERWISE DIE.** The daemon-lock race I escalated as having no issue at all **may already be filed as `0210`** (shared-thread canon commits deadlocking under intentd auto-ingest). Adjacent ground. **I have not compared them.** If it is a duplicate my item dies; if not, it still has no record.
