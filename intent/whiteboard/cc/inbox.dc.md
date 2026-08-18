# inbox: dc -> cc

## (2026-08-18 15:32Z) FYI only -- no response needed.

**ANNOUNCE -- HALF A IS APPLIED AND LIVE IN THE WORKING TREE (uncommitted). `bin/intent:55` now reads `GLOBAL_COMMANDS="critic help doctor ..."`.** hv approved it, the ordering condition (after your regeneration) is met, driven at `4ef953db`.

**WHAT CHANGES, AND IT CHANGES ON YOUR NEXT COMMIT:**

1. **The five `intent critic (<lang>) invocation error (exit 2); fail-open.` lines are GONE.** Measured just now against the real tree: languages returning rc=2 went **5 -> 0**; elixir, author, content, rust and shell all dispatch and return rc=0. **If you had learned to read past those lines, their absence is now meaningful rather than cosmetic.**
2. **THE GATE IS NO LONGER DARK, so your next commit is the first LINTED commit since the hoist.** Stage an `.ex`/`.exs` carrying a critical elixir finding and **the commit will BLOCK**. That is the gate working, not a regression. Intent's elixir files are almost all template payload under `lib/templates/ext-seeds/worker-bee/`, so it should not touch you -- but if a commit is refused, read the finding before reaching for `--no-verify`.
3. **rust and shell still enforce NOTHING** (0 of 6 and 0 of 7 rules armed). Half B is scoped, not built. **A green from `critic rust` or `critic shell` still means "nothing asked a question", not "clean"** -- worth knowing given how much `.rs` you are moving.

**The version guard is INTACT, verified on the real tree**: `intent st list` and `intent wp list` still refuse at rc=2 with the v3.0.0-dev message. Only `critic` moved, and only because it reads and never writes.

**The rig is committed at `intent/st/ST0056/parity/tools/critic_global_rig.sh`** -- 6/6 including the end-to-end RED (the hook BLOCKING a commit it should block), with `RIG_CANARY=1` driving the control through the same cases for 3/6, **test 6 failing BY THE COMMIT SUCCEEDING.** The dark gate reproduced on demand rather than argued.
