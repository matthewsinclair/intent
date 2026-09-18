# Clean-machine install on gyges: ST0056 AC-00.5, AC-11.1, and issue 0344's never-tapped leg

Written by dc and judged by vc, 2026-09-17. hv runs this on gyges, hv's second Mac, when hv says Intent installs and works on the main machine. A Claude on gyges may drive it. It must not have a checkout of the Intent repo.

## What this proves

- **AC-00.5:** `brew install` on a clean macOS machine yields a working `intent` and a working daemon lifecycle.
- **AC-11.1:** a tagged version's published artefacts and the tap formula install a runnable `intent` and `intentd` on a machine that has never seen this repo.
- **0344:** a stranger's Homebrew, with the tap never cloned and an empty trust store, accepts the documented install command without `brew trust`. On the main machine with the tap already present, stable Homebrew 7.0.3 auto-trusts the fully-qualified name (`install.rb:197`, `Trust.trust_fully_qualified_items!`, prints `==> Trusted formula matthewsinclair/intent/intent`), and a short name is refused. That leaves the never-tapped path, which only a clean machine can show.

## Rules for the run

- Run every step inside one `script` capture so the transcript is the evidence: `script -q ~/intent-clean-install.log`, then the steps, then `exit`.
- Do not run `brew trust` and do not `brew tap` first. The point is the documented command alone.
- Do not fix anything that fails. Stop and hand the log back; a failure is the finding.
- No pushes, no releases, no issue or ST writes on gyges.

## Steps

### 1. The machine has never seen Intent

```bash
sw_vers; uname -m; brew --version
echo "XDG_CONFIG_HOME=${XDG_CONFIG_HOME:-unset}"
brew tap | grep -i intent; echo "tap grep rc=$?"
brew trust --json=v1; echo "trust store rc=$?"
command -v intent intentd; echo "which rc=$?"
ls -d ~/.config/intent ~/.local/share/intent 2>&1
```

Expected: arm64, tap grep rc 1, no `intent` or `intentd` on PATH, no Intent config, and `brew trust --json=v1` names nothing under `matthewsinclair`. Read the store only through `brew trust`, because it lives under `$XDG_CONFIG_HOME` when that is set. A trust store that already names `matthewsinclair/intent` voids the 0344 leg; say so and continue.

### 2. The documented install (0344, AC-11.1)

```bash
brew install matthewsinclair/intent/intent; echo "install rc=$?"
brew trust --json=v1
brew info matthewsinclair/intent/intent
```

Expected: rc 0, a `Trusted formula matthewsinclair/intent/intent` line, no `Refusing to load` line, the store now names that formula, and the version is the latest published tag.

Then the short name, against an empty trust store in a scratch directory so the real store keeps its entry:

```bash
T=$(mktemp -d /tmp/brew-trust.XXXX); chmod 700 "$T"
XDG_CONFIG_HOME="$T" HOMEBREW_NO_AUTO_UPDATE=1 brew info intent; echo "short name, empty store rc=$?"
HOMEBREW_NO_AUTO_UPDATE=1 brew info intent >/dev/null; echo "short name, real store rc=$?"
```

Expected: the empty store refuses with rc 1, and the output reads `Refusing to load formula matthewsinclair/intent/intent from untrusted tap matthewsinclair/intent` and names `brew trust`. The real store, which holds the formula entry from the install, loads it with rc 0, so `brew upgrade intent` in docs/install.md works.

### 3. Both binaries run and are the shipped build (AC-11.1)

```bash
intent --version; intentd --version
for b in "$(command -v intent)" "$(command -v intentd)"; do
  codesign --verify --strict --verbose=2 "$b"; echo "codesign rc=$?"
  codesign -dvv "$b" 2>&1 | grep -E "Authority=Developer ID Application|TeamIdentifier|flags"
  spctl --assess --type open --context context:primary-signature --verbose=2 "$b"; echo "spctl rc=$?"
done
intent info
```

Expected: both report the tag's version. For each binary: codesign rc 0; `Authority=Developer ID Application: Geodica Pty Ltd (76BQL8L47U)`, `TeamIdentifier=76BQL8L47U` and `flags=0x10000(runtime)` (the hardened runtime); and spctl prints `accepted` with `source=Notarized Developer ID`, rc 0. This is where the log shows D38's condition, signed AND notarised. Do not use `spctl --type execute`: it rejects every bare CLI binary as "does not seem to be an app" (rc 3) whatever its signature, as vc measured on the 3.0.3 keg. Finally, `intent info` names the keg as its install root.

### 4. First-time setup and a project (AC-00.5)

```bash
intent bootstrap; echo "bootstrap rc=$?"
P=$(mktemp -d /tmp/intent-clean.XXXX); cd "$P" && git init -q
intent init clean-check; echo "init rc=$?"
intent st new "Clean machine check"; echo "st new rc=$?"
intent st list --status all
intent doctor; echo "doctor rc=$?"
```

Expected: every rc 0. `bootstrap` prints `created: install root recorded -- /opt/homebrew/Cellar/intent/<version>/libexec` and ends `done: this machine is set up` (a machine that has run it before prints `ok: install root already recorded -- ...` instead); `init` prints `created: clean-check at <dir>` and the files it wrote; `st new` prints `created: ST0001`; `st list --status all` lists ST0001 `Clean machine check` as `Triage`, which is where a new thread starts (a bare `st list` shows WIP threads only and would report no match); `doctor` reports `0 finding(s)`.

### 5. The daemon lifecycle (AC-00.5)

```bash
intent daemon status; echo "status rc=$?"
intent daemon start; echo "start rc=$?"
intent daemon status; echo "status rc=$?"
intent st list --daemon; echo "st list via daemon rc=$?"
intent daemon restart; echo "restart rc=$?"
intent daemon status; echo "status rc=$?"
intent daemon logs | tail -40
intent daemon logs | grep -inE "error|panic|fatal"; echo "log error grep rc=$?"
intent daemon stop; echo "stop rc=$?"
intent daemon status; echo "status rc=$?"
pgrep -lf intentd; echo "pgrep rc=$?"
```

Expected: `ok: no intentd is answering; commands run in-process`, then `ok: intentd is answering at <endpoint>`, the daemon answers `st list`, restart comes back answering, the logs (after their `tailing ...` header line) show the start and the restart with no error, panic or fatal line (grep rc 1), and after stop the status line reads not answering and nothing is running (pgrep rc 1).

### 6. Hand back

`exit` the `script` session and return `~/intent-clean-install.log` whole. Report each step's rc, and name any step whose output differs from its Expected line, quoting that output verbatim.

## What the log satisfies, on vc's judgement, recorded with `intent ac satisfy`

- AC-00.5 by steps 2, 4 and 5.
- AC-11.1 by steps 1, 2 and 3.
- 0344 is already closed on the main machine's evidence (655bec8a6). Steps 1 and 2 add the never-tapped leg to its record. If the documented install refuses there, reopen 0344 as a doc change: add `brew trust matthewsinclair/intent` before the install line in README.md, docs/index.md and docs/install.md.

## The AT rows this brief backs

Each row names what it proves once. Signing and notarisation belong to AT-11.1, and AT-00.18 carries only the install and the lifecycle. Both are `(non-test)` rows, so both stay `n/a`: a non-test row carries no test result (`AtStatus::permitted_for`), and the satisfaction is recorded on each criterion's own line with `intent ac satisfy`, citing the returned log.

**AT-00.18, covers AC-00.5 (re-kinded from test to non-test; n/a):**

`(non-test) clean-machine transcript on gyges, ~/intent-clean-install.log from intent/st/ST0056/gyges-brief.md, steps 2, 4 and 5: the documented brew install matthewsinclair/intent/intent exits 0; bootstrap, init, st new and doctor succeed in a fresh git project; and intent daemon status, start, status, st list --daemon, restart, stop and status run in that order, reporting not running, running with an address, answered, running, and not running; the daemon logs show the start and the restart with no error, panic or fatal line; and no intentd process is left after stop (pgrep rc 1).`

**AT-11.1, covers AC-11.1 (re-cited; n/a):**

`(non-test) clean-machine transcript on gyges, ~/intent-clean-install.log from intent/st/ST0056/gyges-brief.md, steps 1 to 3: on a machine with no Intent tap, trust entry, binary or config, the documented brew install matthewsinclair/intent/intent of the published tag exits 0 without brew trust or brew tap; intent --version and intentd --version both report that tag's version; and both binaries pass codesign --verify --strict, carry Authority=Developer ID Application: Geodica Pty Ltd (76BQL8L47U) with flags=0x10000(runtime), and are accepted by spctl --assess --type open --context context:primary-signature as source=Notarized Developer ID.`
