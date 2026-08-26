# Design handoff: a macOS menubar app with a terminal-styled console (from Geodica)

Received by vc from the Geodica `_tools` session (`geodica-7f`) at 2026-08-26 14:51Z, at hv's request: hv wants Intent to have the equivalent and will liaise on what to do with it and where it should live. Parked here verbatim so it survives the session; NOT a work item until hv says so. Questions go back to `geodica-7f`.

---

WHAT WE BUILT

Geodica has a local CMS daemon (`geodica cms serve`, Elixir/Bandit on :4360). It now also has `~/Applications/Geodica.app`: a Swift menubar app that shows the daemon's state, starts/stops it, tails its log in a console window, holds preferences, and handles a custom `geodica://` URL scheme. ~1,900 lines of Swift. The design doc is `Dropbox/Geodica/_tools/design/closed/008-menubar-app.md` and the user-facing guide is `_tools/docs/app.md`; the code is `_tools/native/macos/Geodica/`.

THE LOAD-BEARING PRINCIPLE

The app holds NO product logic. Every control is a CLI verb it shells out to; every fact it displays comes from the daemon or the CLI. Start/Stop/Restart run `geodica cms serve` / `--stop`; "Rebuild index" runs a verb; the console is literally a child `geodica cms logs` streamed into a text view. Nothing in Swift knows the log format, the pidfile layout or the config schema. That is what keeps a GUI from becoming a second implementation that drifts. For Intent the same rule would be: the app runs `intent` subcommands and renders their output, nothing more.

The one thing it reads directly is a status endpoint we added to the daemon: `GET /_status` returns JSON -- site, port, pid, version, started_at, uptime_s, requests answered, mounts, search-index state. The app polls it every 5s with a 1.5s timeout and derives running / degraded (answers but not with /_status -- an old daemon) / stopped (connection refused). The daemon is the authority on the daemon. If Intent has no daemon, the equivalent is a `--json` status verb.

THE CONSOLE (the bit Matthew called "excellent")

An NSWindow holding an NSTextView in an NSScrollView, styled from our brand tokens: ink ground (#0E141F), bone text (#F5F2E7) at 85%, request lines at 55%, errors and markers in the accent (#CD285E), IBM Plex Mono 12 with a system-monospace fallback. Footer bar: **Verbose** checkbox, **Follow** checkbox, **Clear** (also Cmd-K), **Copy**, and the path being tailed. Window title carries the log file name. Cmd-L toggles it; closing it ends the tail.

Details that made it feel right:

- Verbose toggles by killing the child and re-running the verb with `--verbose`. The filter lives in the CLI, never duplicated in Swift.
- Follow auto-scrolls, clears itself when the user scrolls up (observe `NSScrollView.didLiveScrollNotification`, compare visible maxY to frame maxY), and re-arms at the bottom.
- One-off commands (doctor, index rebuild, a URL click) stream into the SAME view between accent-coloured markers: `>> geodica doctor` ... output ... `>> exit 0 - 3.4s`. One place to look for everything the app did. One command at a time; a second request is refused with an alert, not queued.
- A 5,000-line ring buffer. When it trims N lines the view deletes the same N leading lines from its text storage, so the two never diverge.
- Line classification is a pure function over the text (marker / error / warning / request / log) -- trivially unit-testable, and the only thing Swift assumes about the log.

THE TRAP YOU WILL HIT -- this cost us the most time

Killing the tail child does not work the way you expect. Our `cms logs` is `tail -f | grep` under `sh`, under the BEAM, under a bash dispatcher. `Process.terminate()` from Swift reaches only the top of that chain and leaves orphaned tails behind. Worse, we discovered the CLI verb had been leaking a tail on EVERY exit, including plain ctrl-c at a terminal, because the BEAM starts port programs in their own process group so no signal ever reached them.

The fix belongs in the CLI, not the app: the verb runs its pipeline under a shell that reads its own stdin and takes the process group down when that pipe closes -- the runtime's death closes it, however it dies. Verified against SIGTERM, SIGINT and SIGKILL: zero orphans, where every previous run leaked one. Then the app needs no special handling at all -- plain terminate() works. If Intent's log tailing is a pipeline behind a runtime, check for this before you build any UI on top of it; `ps -A | grep tail` after a few ctrl-c's will tell you in seconds.

THE BUILD PIPELINE

No hand-edited Xcode project: `project.yml` + xcodegen generates the `.xcodeproj` (which we track so Xcode opens it directly). One CLI verb, `geodica app`, owns everything -- `build` (regenerates the project only when the spec or the SET OF SOURCE FILES changed, since xcodegen scans sources by presence and a new file otherwise silently isn't in the project), `run`, `test`, `install`, `uninstall`, `start`, `stop`, `status`, `icons`.

Four things worth stealing verbatim:

1. Version/commit/source-tree are passed to xcodebuild as BUILD SETTINGS and substituted into Info.plist -- not written into a generated Swift file. The generated-file approach has a well-known failure where the build phase is skipped by dependency analysis and the app ships stamped with a commit it was not built from.
2. `status` compares the installed bundle's source-tree hash against the working tree and says "stale -- reinstall". That line is repeated in `doctor`.
3. `run` execs the built binary directly rather than `open`ing the bundle: LaunchServices resolves `open` by bundle id and will cheerfully launch an INSTALLED copy instead of the one you just built. Similarly `status`/`stop` use `lsappinfo info -only pid <bundle-id>`, not `pgrep` -- children of a hardened-runtime app cannot see it in the process table, so a pgrep shelled from inside the app reports it dead.
4. Pin `SYMROOT` and `OBJROOT` on the command line, not just `-derivedDataPath`. An Xcode "Locations -> Custom, relative to workspace" preference otherwise puts products next to the project; ours landed on a Dropbox tree and codesign died on synced extended attributes ("resource fork, Finder information, or similar detritus not allowed").

A GUI app also launches with a bare PATH and none of your environment. We capture the login shell's PATH once at launch (`/bin/zsh -ilc 'printf %s "$PATH"; command -v <cli>'`, ~0.15s) and hand it to every child, and resolve the CLI binary through it with a symlink fallback and a Locate... override in settings. Nothing is baked into the bundle.

SETTINGS + THE REST

Tabbed NSWindow of NSGridView panels in the System Settings register (right-aligned labels, left-aligned controls, hint rows under each). Four tabs: General (launch at login via SMAppService, registered on first run; start-the-daemon-at-launch; console-on-run), Daemon (the status rows plus lifecycle buttons; config file shown as a path with Edit.../Reveal -- the app never WRITES what the CLI reads), Estate/Paths (everything as resolved, with Reveal buttons), About (the build stamps).

Menubar icon: we tint it with the brand accent when the daemon is REACHABLE and leave it template-rendered (adapts black/white to the bar) when it is not, with orange for the transient. The obvious alternative -- tint the failure red -- makes an idle daemon look like an alarm, and leaves "running", the state you are in all day, looking like every other template glyph. Also: an icon that is a set of OVERLAPPING shapes flattens to a silhouette blob at 18px; we redrew the glyph with gaps. And use `rsvg-convert`, not `qlmanage`, to rasterise -- QuickLook flattens transparency to white, which a template image renders as a solid square.

One last thing worth copying: we put a small on/off switch in the WEB app's header that starts and stops the menubar app (state from `/_status`, click POSTs a whitelisted action). The two surfaces can each recover the other.

Files to read if useful: `_tools/design/closed/008-menubar-app.md` (design + a closing note with measurements and everything that went wrong), `_tools/docs/app.md`, `_tools/bin/geodica_app`, `_tools/native/macos/Geodica/Geodica/Views/Console/ConsoleWindowController.swift` and `Services/ConsoleRunner.swift`.
