# AC-02.1 -- the drive that found six defects, 2026-08-31 afternoon

Moved off the live board at the 12:59Z aggressive localfold. **The RULES stayed on the board; this is the reasoning and the evidence.** Landed as `e2a7a0e4`, `cafde415`, `baf8cf8e`.

## The result

`docs/getting-started.md` as published **reached ZERO satisfied criteria** and failed **4 of its 20 steps, 3 of them its last 5.** Driven under the contaminant control AC-02.1 itself names: fresh `HOME` (no `~/.intent/`), `PATH` scrubbed to the v3 release dir alone (no installed v2), `INTENT_HOME` set explicitly per `install.md`'s source route (no ambient value), fresh `git init` inheriting no `core.hooksPath`.

## The six, in the order they were found, which is the point

| #   | defect                                                            | how it presents                                                                                                           |
| --- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| 1   | `intent init --lang rust` -- **the page's FIRST command** -- rc=2 | refuses; remedy names a command that has shipped                                                                          |
| 2   | nothing on the page creates the project                           | says `cd your-project`, says everything runs against a real repository, no step makes one                                 |
| 3   | **`ac new --kind` defaults to `non-test`**                        | all three criteria the page creates ignore their tests; section 6 then states the opposite as the model's central promise |
| 4   | the non-test example covers `AC-02.9`                             | section 4 never creates it; `at new` refuses, rc=1                                                                        |
| 5   | a green row's cited file must EXIST                               | page greens `AT-01.1` citing `tests/cache_eviction.rs`, never has the reader write it                                     |
| 6   | that file must carry the row's **literal id**                     | stated nowhere; only reachable after fixing #5                                                                            |

**#3 IS THE DAMAGING ONE.** A reader following the page exactly builds a thread in which the tests are decorative -- and section 4's own opening sentence calls the tests-compute-satisfaction model the thing that distinguishes Intent from a task tracker.

**#6 WAS ONLY REACHABLE BY FIXING #5 AND RUNNING AGAIN.** That is the transferable half and it is now on the board: a drive that stops at its first failure reports one defect and hides every one behind it.

## The proven replacement

30 commands, zero failures, empty directory to `ac gate` PASS and a closed thread, with `ac list` printing the block the page shows. Driven as written AFTER the rewrite, because the page's opening paragraph claims it has been run and that claim was the one the commit was making.

The corrected sequence differs from the published one by: `mkdir` + `git init`; `intent init` then `intent lang init rust` as separate steps; `--kind test` on the two test-backed criteria; writing the two test files WITH their AT ids before citing them; the non-test example covering a criterion that exists; and `ac satisfy` for the non-test criterion, a verb the published page never mentioned.

## The `--lang` finding, which inverted on measurement

First read: a docs defect. Measured: `intent lang init rust` works, rc=0, `declared: rust`. So `render.rs:4461`'s refusal -- _`--lang` cannot be honoured in this build -- `intent lang init` is not implemented yet_ -- has a **false reason** and a **misleading remedy** (it tells the reader to wait for a command that shipped). Its CONCLUSION is right: `init` reads the flag nowhere in its body, bounded at 4442-4591 with 118 `lang` hits elsewhere in the file as a positive control, so removing the guard would restore the silent-ignore the guard exists to prevent. **The fix is `init` calling `lang init`.** vc filed `0187` at high severity -- high because it is the first command on the published page -- and routed it to cc.

## `wp cancel`

`0063` titled _WpStatus has no Cancelled variant_ does **not** reproduce at HEAD: `wp cancel --reason` lands the WP in `Cancelled`. Whatever survives of that issue is the narrower thread-close claim in its body, and `0151` means the title cannot be corrected. The page gained the two facts a reader needs: `wp cancel` requires `--reason`, `wp reopen` is legal only from `done`.

## AC-02.3's instrument, as far as it got

Population derived: **105 open of 166**, moved from 103 of 164 while the work was in progress because vc filed `0186` and `0187` -- the row's own argument for deriving rather than transcribing, demonstrating itself inside one session.

The candidate filter went through two wrong shapes before the useful one. Keyed on all published pages it returned **110 of 111 shipping verbs**, which discriminates nothing, because `docs/reference/` is generated from the register and names every verb by construction. Narrowed to reader-followed pages it returned **64 of 111**, with controls holding (`st new` present, `ext validate` absent).

**Then AC-02.3's own five named members were used as the control and it FAILED 2 of 4 driven** -- `0139` (`at lint --fix`) and `0149` (`intent edit`) were missed, because no reader PAGE names `at lint` or bare `edit`. **The correction is the population: the docs a reader follows are the published pages PLUS the binary's own `--help`**, which AC-02.3 says itself in its first named member (_`intent help` is retired while `--help` answers_).
