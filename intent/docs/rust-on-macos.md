# Rust on this machine: the test run is not slow, Gatekeeper is

**Copy/paste this into any Claude session doing Rust work on matts's Mac.**

Measured 2026-08-19 on Intent (`native/rust`, 2 crates, 88 test suites), twice, while the symptom was live.

## The finding

**`cargo test` wall-clock here is ~99% macOS code-signature validation and ~1% testing.**

| observable                             | value                                                              |
| -------------------------------------- | ------------------------------------------------------------------ |
| wall clock at sampling                 | 11 min 20 s, then 22 min 40 s                                      |
| cargo's own summed `finished in` lines | **8.51 s**, then **11.87 s**                                       |
| `rustc` running during that window     | none -- compilation was already finished                           |
| test-binary process state              | `S` (blocked), RSS **32 KB** -- blocked BEFORE the test code loads |
| `syspolicyd` CPU                       | 21-22% sustained                                                   |

Same binary, first execution vs second:

```
intent                  20,633 ms  ->  26 ms     (794x)
acceptance_surface      19,459 ms  ->  24 ms     (811x)
session_hook_lockout    10,949 ms  ->  23 ms     (476x)
```

**Causally confirmed, not inferred:** across one 17.5 s first-exec, `syspolicyd` cumulative CPU went 11:01.84 -> 11:03.84 -- **exactly 2.00 s consumed**. Nothing else moved: `amfid` 0, `XProtect` 0, `trustd` +0.04 s, `mds_stores` +0.09 s.

`spctl --status` = `assessments enabled`. The binaries are `adhoc, linker-signed`.

## Why it costs so much here specifically

**Cargo builds ONE TEST BINARY PER `.rs` FILE directly under `tests/`.** That is Cargo's rule, not a project decision.

```
intent-cli     25 files  ->  25 binaries
intentsvcs     56 files  ->  56 binaries
               81 binaries per build
```

81 binaries x ~17 s of first-exec validation is **~23 minutes**, wrapped around under 30 seconds of actual testing. Every rebuild produces new binaries with new signatures, so the validation cache resets every time.

Side effects of the same cause: `target/debug/deps` holds **291 executables** (81 current plus up to 5 stale hash-suffixed generations each, which cargo never garbage-collects) and **778,425 files**; `target/debug` is 15 GB, ~25 GB across the per-node target dirs.

## What this invalidates

**Any timing figure recorded on this machine is untrustworthy unless it also records whether a compile preceded it.** Subject and revision do not determine the number; validation-cache state does, and it moves a measurement by up to 800x.

Two concrete casualties, both mine:

- A "6x timing noise floor" (identical warm suite: 16.12 s then 100.10 s) was not noise. It was warm-after-compile versus warm-after-run.
- A Lamplight-vs-Intent control group (19 binaries vs 80) that concluded "we are the faster project" was comparing two cache states, not two binary counts. It killed a correct piece of work for a full day.

## What actually helps

1. **Reduce the binary count.** Consolidating the 81 `tests/*.rs` into 2-3 binaries cuts validation from ~23 min to under a minute and is correct regardless of any OS setting.
2. **System Settings -> Privacy & Security -> Developer Tools**, add the app that spawns the build (iTerm, VSCode, Terminal, Emacs). **UNVERIFIED AS OF THIS NOTE.** The exemption is evaluated against the responsible process, so an already-running app will not pick it up -- the app must be restarted before any measurement means anything. It also governs whether unsigned code is BLOCKED, which is not obviously the same as whether it is ASSESSED, and only a measurement settles that.
3. Prune stale generations (`cargo clean`) and mark target dirs `.noindex` -- 778k files also feeds Spotlight.

## The instrument, if you need to re-measure

Do not measure elapsed time. **Elapsed time cannot distinguish blocked from busy**, which is why this survived four wrong explanations over two days. Measure these instead:

```bash
# 1. actual test time vs wall clock -- the whole finding in one line
awk '/finished in/ { for(i=1;i<=NF;i++) if($i=="in") { t=$(i+1); gsub(/s$/,"",t); s+=t } } \
  END { printf "actual test time: %.2fs\n", s }' <the cargo test log>

# 2. first-exec vs second-exec on a binary that has not been run
B=<path to a test binary>
time "$B" --list >/dev/null 2>&1   # first
time "$B" --list >/dev/null 2>&1   # second

# 3. which daemon is actually consuming CPU during the slow exec
ps -Ao time,comm | grep -E 'syspolicyd|XProtect|amfid|trustd'   # before and after
```

## Not measured, not claimed

**The COMPILE phase.** All evidence above is from the run phase. `build.rs` outputs are executables that get run, and proc-macro dylibs get loaded, so the same tax plausibly applies -- but there is no measurement, and two probes during this investigation returned exactly what was expected while being structurally incapable of returning anything else.
