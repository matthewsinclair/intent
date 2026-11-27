# Rust on macOS: Gatekeeper validates each new test binary

Read this before you time any Rust work on this Mac.

## The finding

On 2026-08-19, almost all of the wall-clock time of `cargo test` in `native/rust` was macOS code-signature validation. Cargo's own `finished in` lines added up to seconds, but the run took many minutes. Each test binary was blocked before its code loaded, while `syspolicyd` used the CPU. A binary's first run took about 20 s, and its second run took about 25 ms.

Each new test binary pays this validation on its first run. A rebuild makes new binaries with new signatures, so each rebuild pays it again. The binaries are `adhoc, linker-signed`, and `spctl --status` reads `assessments enabled`.

## What this means for a measurement

**A timing figure from this machine is not valid unless it also records whether a compile came before it.** The state of the validation cache can change a measurement by up to 800 times.

## What helps

1. **Fewer test binaries.** The validation cost grows with the number of test binaries, not the number of tests. Every Intent crate sets `autotests = false` and declares its test targets (`intent/docs/notes/tn001-one-test-target-per-crate.md`). `bin/devbin check autotests` reports each crate's declared targets.
2. **Developer Tools (not verified).** In System Settings, Privacy & Security, Developer Tools, add the app that starts the build (eg iTerm, VSCode, Terminal or Emacs), then restart that app. The setting controls whether unsigned code is blocked. No measurement shows that it also stops the assessment.
3. **Prune and hide target trees.** Cargo does not remove old hash-suffixed test binaries from `target/debug/deps`, and a large target tree also feeds Spotlight. `bin/devbin clean` puts a `.metadata_never_index` file in each target directory it walks. A bare `cargo clean` in `native/rust` also deletes `target/release`, which is the pair that `intent use dev` serves. Prune only while no session uses that pair, and run `bin/devbin build all` after.

## How to measure it again

Do not measure elapsed time, because it cannot tell a blocked process from a busy one. Measure these:

```bash
# 1. the test time cargo reports, against the wall clock
awk '/finished in/ { for(i=1;i<=NF;i++) if($i=="in") { t=$(i+1); gsub(/s$/,"",t); s+=t } } \
  END { printf "actual test time: %.2fs\n", s }' <the cargo test log>

# 2. the first run against the second, on a binary that has not run yet
B=<path to a test binary>
time "$B" --list >/dev/null 2>&1   # first
time "$B" --list >/dev/null 2>&1   # second

# 3. the daemon that uses the CPU during the slow run
ps -Ao time,comm | grep -E 'syspolicyd|XProtect|amfid|/trustd$|mds_stores'   # before and after
```

## Not measured

The compile phase. The evidence above is from the run phase only. Build scripts are executables that run, and proc-macro dylibs load, so the same cost can apply to them, but no measurement shows it.
