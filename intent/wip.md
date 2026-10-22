---
verblock: "2026-10-22:v3.19: vc - globalfold after the 3.2.5 cut: the two box queues, the fleet's board@4.0 re-render, ST0081's last rows, the cut's tail and the lanes after it"
intent_version: 3.2.5
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

- **THE BOX HAS TWO QUEUES ON ONE MAC (agreed 2026-10-22, until hv rules on one; restart.md's one-queue rule is the standing text).** vc holds Intent's queue and utilz-vc holds Utilz's and Gtools'. Each sends the other every heavy GO and END by name, and neither GOes while the other's heavy run is in flight. At a gap, the queue whose run ENDed names the gap and the census, and the other queue's holder GOes its own runs. Within a queue, one heavy run runs at a time, with START and END by name and `census-ok-ci.sh` (refs/bank/vc/tools/) before each GO and inside every commit command. The pair every estate runs is the dev tree's, built at the v3.2.5 tag (2e0e9215d), and `intent --version` names it. brew's 3.2.5 keg is installed, unlinked and pinned, and intentd runs on the pair.
- **THE FLEET'S board@4.0 RE-RENDER (issue 0612, in 3.2.5).** Intent, Gtools, Molt and Utilz are re-rendered and committed. Every other estate re-renders at its next `intent wb` verb, as one mechanical commit by its coordinating node. From that commit on, a node commits what its verbs print under `wrote:`. A running `intent mcp` server or explorer opened before the migration refuses the migrated store until it is reopened.
- **ST0081, INTENT FROM HOMEBREW ON LINUX AND macOS (cc holds it):** the gate is BLOCKED on AC-02.1 alone, which waits on hv's D55, and WP-02 is not started.

## TODO

- **hv's INBOX, unanswered:**
  - the stabilisation line after 3.2.4 (vc: yes);
  - a SHA256SUMS file on each release, in ST0081 WP-02 (the stage writes one, the release carries none; vc: yes);
  - naming `local` and `upstream` as the release remotes in `bin/.devbin/config.yaml` (vc: yes);
  - D55 from devbin-vc: devbin's release core attaches CI-built assets, or an upload-only workflow in Intent. Both vcs lean to the first, and the by-hand upload is the interim;
  - where the joint WSL job lives, and whether a hosted runner's WSL 1 counts as evidence;
  - whether the two box queues merge into one, which utilz-vc brings.
- **OPEN ISSUES AND THEIR LANES:**
  - 0611 (passive prompt injection) is hv's triage and is expected to become a thread; ic offered the inventory of the surfaces that feed agents prompt text.
  - 0614 (linked-worktree carriers) carries dc's lead from its e_worktree_carriers control (refs/bank/dc/325kd/), and the issue is measured against it.
  - 0616, 0617 and 0618 are cc's. 0618 needs ic's read of the surface first.
  - 0622 and 0623 (low, filed at the 3.2.5 reference pass) wait for a lane; ic filed both and is the candidate.
  - 0624 (low, dc's finding after the cut): `intent issues edit` and `close` write tracked files and print no `wrote:` lines, so a node commits what they leave dirty after reading each path. It waits for a lane.
  - 0626 (low, cc's finding after the cut): `macos stage` tars the support paths from the release machine's working copy, so v3.2.5's `intent-support.tar.gz` carries three empty directories git does not track. It waits for a lane.
- **A CLEAN-MAC RE-PROOF AT 3.2.5, hv's loop-back on gyges:** ST0081 WP-06's objective names it and no criterion carries it. It was last proven at 3.2.0 on 2026-09-21, which satisfied ST0056's AC-00.5 and AC-11.1 (`intent/st/ST0056/gyges-install.md`).
- **CLEANUP, on vc's word:** the spent bank refs under `refs/bank/`, which every lane keeps until then.
- **TO MEASURE AND FILE:** a peer's board write that is still landing can make the commit gate's doctor count view skew on that peer's `wip.md`, and so refuse an unrelated commit. dc's fold commit on 2026-10-22 was refused on cc's mid-fold `wip.md` (3266 bytes on disk, 1410 rendered), then passed on a re-issue.
- **UNVERIFIED:** whether the Sinclair-Shared book recreate of decision 63(8) happened, and whether its build-all signal reached sinclair-vc.
- **AFTER THE LINE, vc's hand, no push:** hv decision 28's ten Conflab and Baize rulings, which repair records. None is done.
- **AWAITING hv:** a `/refresh` in the explorer (ic's, size S).
- **GTOOLS:** the WP-04 slot that vc's 2026-09-26 resume named is unscheduled, and gtools-vc owns it.
- **COURSES:** courses-vc re-drives its Intent findings against the installed release (3.2.5 from 2026-10-22) and sends vc the ones that still reproduce, to file.
- **MACHINE, hv's call:**
  - the resident `SetStoreUpdateService` processes and the Spotlight-family services that push the load high;
  - the CoreSimulator framework that xcodebuild reports as out of date. It is noise for the Release build, and a macOS or Xcode update clears it.
- **ST0084** (Triage, filed 2026-10-17 on hv's word): relocate the parity tooling and reference generators out of `intent/st/ST0056`, so that completed threads can leave the disk. Until it lands, ST0056 and ST0057 stay hydrated, because the gate, the release script and ST0057's acceptance rows cite files under them. **ST0060** and **ST0077** stay in Triage as the next line's queue (hv, 2026-09-23).
