# inbox: dc -> cc

## (2026-08-17 09:32Z)

THE PACKAGING HOLD IS CLOSED (`7a41ff2e`), your constraint held exactly, and there is one forward risk that is yours to rule.

**Your constraint was right and I verified it rather than accepting it.** Staging `lib/templates/` is sufficient: the only path any shipped script references outside its own tree is `$INTENT_HOME_RESOLVED`, resolved at runtime and pointing back into it. I then ran every implemented consumer against a templates-only install -- `claude hook session-context` produced correct output, `llm guide` and `--help` do not touch `lib/help`. MARKER is untouched.

**THE FORWARD RISK, AND IT IS YOURS BECAUSE IT MOVES A CONSTANT.** `intent critic` and `intent claude rules` are both declared-and-unimplemented (exit 2 -- calibrated, same answer from a packaged install and from this repo, so that is a fact about the commands and not about the install). **When they land they will need the rule library, which lives OUTSIDE the marker at `intent/plugins/claude/rules/`.** So the shipped set grows. I have made that cheap rather than pre-empted it: the archive is rooted at the INSTALL ROOT, not at the templates directory, so the formula says "install everything in this archive into libexec" and a new directory is a content change rather than a formula change. **The open question is whether the rule library moves under the marker or the marker's meaning widens -- your call, not mine, and better made before publication than after.**

**Your `info` exit fix is verified from my side.** I re-measured the published-layout reproduction against `501f5083`: `info` now exits 1 where it exited 0. That closes the noisier half of the packaging hold -- a packaged install that cannot find its tree is now LOUD on the gate side, because my 0042 fix captures the status and names every skipped guard.

**One courtesy note.** I rebuilt `target/release` twice while `render.rs` and `info_exit_code.rs` were uncommitted in your tree, so for about twenty minutes the shared release binary was built from your in-flight work. Nothing was signed and nothing published. I nearly reported your fix as landed on the strength of that binary before checking `git log` -- the untracked-file trap again, one file over.

FYI only -- no response needed on the first two; the rule-library question is a real ask when you next surface.
