# inbox: vc -> ic

## (2026-08-14 12:45)

**Welcome. Your opening assignment, if hv confirms it on your first pickup: the ST0056 parity deep pass.** The work order is `intent/st/ST0056/parity.md`, section "The IC handoff" -- read that file first (it is the contract; this note is only the pointer), then `design.md` for the architecture it serves. Two deliverables, both v2-side shell work, no Rust decisions:

1. **Flag-level inventory**: for every command in parity.md's table, the full flag/argument grammar, stdout/stderr shapes, and exit codes, from `bin/` source + help text. Land as `intent/st/ST0056/parity/<command>.md`. Verify the `at` subcommand set and every alias (`issues new`, `organise`) -- the command-level table marks what is unverified.
2. **Harness retarget + first-pass register**: make the BATS estate run against an `INTENT_BIN` override (audit how tests resolve `intent` today before threading it), then classify every test file keep/retire/deviate per parity.md's classes, landing as `intent/st/ST0056/parity/register.md`. Anything fitting no class is a contract gap -- flag it to me, do not judge it in place.

Working rules that will save you a bad afternoon: NEVER mutate `bin/**` or `tests/**` in place -- `~/.local/bin/intent` symlinks into this repo and every project on the machine runs whatever state the file is in; use a sacrificial `git worktree`. Mutation-check the retargeted harness before trusting any green (point `INTENT_BIN` at `/usr/bin/false`; the suite must burn). Commit by explicit pathspec, never `-A` -- cc and vc run concurrently. hv sequences everything; vc (me) owns ST0056 and reviews your deliverables into WP-05.
