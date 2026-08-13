# cc -- archived 2026-08-13

Six of the seven units of vc's 0009-0017 work order, landed as v2.19.0 work in progress.

## DONE

- **U1 -- 0017 + 0014 + 0015: the AT row grammar** (`f28938c`). Two-arm anchored grammar (test arm + non-test arm), `intent at lint` L1-L5, `--fix` migrator, folded into `ac gate`, `at green|red` refuse a dangling citation, `at_grammar` ledger step. Own estate swept: 116 rows, 103 mechanical + 13 by hand. Found a real 0015 instance in our own repo (ST0052 AT-03.1 green on a deck renamed in ST0053).
- **U2 -- 0013: an AC has four states** (`2d63a5e`). descope/rescope + hv-added withdraw/reinstate; marker-based detection checked BEFORE satisfaction; counts reported separately; emptied contract refused to the `acceptance: exempt` escape.
- **U3 -- 0011: one steel-thread enumerator** (`c6097af`). `list_st_dirs_in` / `list_st_dirs` in helpers, five call sites repointed; organize names a collision, finishes the sweep, exits non-zero; `intent doctor` duplicate-id check.
- **U4 -- 0012: the board header block is NOT YAML** (`72d7b21`). Declared line-oriented `key: value` in SKILL.md + README + pickup wording; hygiene enforces THAT rule; delimiter strip moved into `fm_get`.
- **U5 -- 0016: portable hooks** (`4a0ea96`). `intent claude hook <name>` runner; template needs no substitution and is byte-identical everywhere; `[[INTENT_HOME]]` arm removed; our own public repo's leak fixed.
- **U6 -- 0009: prerequisites from declared languages** (`4b4fd94`). Prereqs from `has_project_language`, commands keep probes; lua/swift lines added; `agents sync --check`; AGENTS.md convergence added to `intent upgrade` AFTER the canon apply.

## Inbox

Nothing cleared. vc's (2026-08-13 20:59) work order stays LIVE in `inbox.vc.md` because U7 and the close-out are still outstanding -- a work order is not handled until it is finished.
