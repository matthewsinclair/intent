# Design: the Intent.app Console

## What hv asked for

Run Doctor in Intent.app's menu runs `intent doctor` and discards its output when the doctor passes, so a clean pass is indistinguishable from a click that did nothing. hv's ruling, 2026-09-14: Intent.app gets a Console that works like the Gtools CMS Console, copied from ../Gtools, and Run Doctor streams its output into it.

## The shape being copied

Gtools' Console is a dark monospaced window over a CLI verb, `gtools cms logs`, streamed line by line. The verb owns the tailing and the filtering; the app only classifies lines for colour. One-off commands (doctor, the index rebuild) stream into the same view between `»` markers: `» gtools doctor`, then the command's lines, then `» exit 0 · 1.2s`. The runner owns a bounded backlog, so a command that runs while the window is closed is there when it opens. The footer carries Follow, Clear, Copy and the file being tailed. One one-off command runs at a time; a second is refused, not queued.

Intent.app already carries the lineage's `ContinuousObservation`, `RunningProcess` and `IntentCLI.stream`, so the copy is the three console files (`ConsoleLine`, `ConsoleRunner`, `ConsoleWindowController`), their test, the brand palette in `Theme` (whose own comment says the palette lands with the console), and the menu wiring.

## `intent daemon logs`

The verb the Console tails, and a terminal verb in its own right.

- It prints one header line, `tailing <intentd.log> and <intentd.err.log>`, then the last lines of each file (40 by default, `--lines N`), stdout's log first, then stderr's.
- A file the daemon has not written yet is named as absent in the header rather than skipped in silence. With neither file present and no `--follow`, it says so and exits 0.
- **Without `--follow` it exits 0 after printing.** With `--follow` it keeps printing lines as the daemon appends them, from either file, until it is terminated. The app always passes `--follow` and treats the exit a terminate produces as normal.
- **This is one deliberate divergence from Gtools**, whose `cms logs` always follows. An agent or a script running `intent daemon logs` must not hang, so following is asked for rather than assumed.
- Following is done by `tail -F`, not by a loop in Rust, so nothing in the workspace reads a clock.
- **The following tail dies with the verb however the verb dies (issue 0281's ruling, as Gtools built it in `Cms.Logs`).** It runs under a shell that reads its own stdin while the verb holds the write end, so SIGKILL closes the pipe as surely as an exit, and the shell kills the tail by the pid it recorded. The verb itself ends when its own stdin closes. No process group is signalled.
- There is no Verbose filter. intentd writes no request lines today, so there is nothing to filter, and the Console's Verbose box is dropped with it.
- Register row: `read`, not on MCP (a follow never returns, and machine-level daemon verbs are withheld as a family), probeable without `--follow`.

## The Console in Intent.app

- A "Console…" item on ⌘L at the top of the menu opens and closes the window. The window tails `intent daemon logs --follow` while it is visible and stops the tail when it closes.
- The app's main menu, which already carries Edit, gains File → Close (⌘W) and View → Clear Console (⌘K), reaching the window through the responder chain as Gtools does.
- **The app gives the follow child a stdin pipe and holds its write end** for as long as the Console tails. Closing it (the window closes, or the app dies however it dies) ends the verb and its tail; `terminate()` does the same. Without the pipe, a GUI app's child inherits an empty stdin and the verb ends at once.
- Lines are classified for colour: `error:`, `caused by:` and `intentd: could not` as errors, `remedy:` as a warning, `»` lines as markers, the rest as log.
- **Run Doctor** runs `intent doctor` in the Console and brings the window forward, so a clean pass is seen. Gtools makes bringing it forward a preference; Intent.app has no settings window, and the operator who clicks Run Doctor has asked to see the result.
- **Rebuild Search Index** is a new item running `intent index rebuild` the same way. intentd's error log already tells the operator to run it.
- **Start, Stop and Restart intentd** write their command and its result into the Console as a marked block, without bringing it forward. They already run through `DaemonService.lifecycle`, which captures each command's output and today sends it only to the system log; that output goes to the runner's note instead.
- Every one-off still raises an alert when it cannot run at all (the binary is missing, the project root is invalid, another command is running).

## Acceptance criteria

One line per user-facing behaviour, each proved by one test.

1. `intent daemon logs` prints the header and the last lines of both logs, then exits 0. AT: an intent-cli test under a temporary HOME with both logs planted.
2. A log intentd has not written yet is named as absent, not skipped. AT: the same test with one file planted, then none.
3. `intent daemon logs --follow` prints a line appended after it started. AT: an intent-cli test that appends a line, reads it from the child, and terminates the child.
4. `intent daemon logs --follow` leaves no tail running once it ends, whether it is sent SIGTERM, SIGINT or SIGKILL or its stdin closes. AT: an intent-cli test that finds the tail's pid first (the control), then ends the verb each way and waits for that pid to go.
5. Console… (⌘L) opens a window tailing the daemon logs, and its footer names the files being tailed. AT: a Swift test that the tailing header is parsed into the footer path; the window is driven by hand once.
6. Console lines are classified for colour as described above. AT: a Swift test over intentd's own line shapes.
7. The backlog keeps the last lines up to its capacity and says how many it dropped. AT: the copied ring test.
8. Run Doctor streams `intent doctor` into the Console between `» intent doctor` and `» exit N · Ts`, and brings the Console forward. AT: a Swift test of the runner's marker sequence over a stub stream.
9. Rebuild Search Index streams `intent index rebuild` the same way. AT: the same test shape.
10. Start, Stop and Restart intentd write a marked block with their result into the Console. AT: a Swift test of the runner's note over a canned result.
11. A second one-off while one is running is refused with an alert naming the running command. AT: a Swift test that the runner throws busy.

## Work packages

- **WP-01, the verb:** `intent daemon logs` with `--lines` and `--follow`, its register row, and the tests for criteria 1 to 4.
- **WP-02, the window:** the three console files copied and renamed, the palette in `Theme`, the Console… item on ⌘L, File → Close and View → Clear Console, and the tests for criteria 5 to 7, and the stdin pipe the follow child needs. Depends on WP-01.
- **WP-03, the streaming items:** Run Doctor and Rebuild Search Index into the Console, Start, Stop and Restart noted there, and the tests for criteria 8 to 11. Depends on WP-02.

## Copied, not shared

The console code will live in two repos. That is accepted for two apps; if a third app wants it, it becomes a shared Swift package.
