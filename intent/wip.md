---
verblock: "2026-10-22:v3.18: vc - localfold for hv's compact: the train after build all #1, the two box queues, the fleet's board@4.0 re-render"
intent_version: 3.2.4
---

# Work In Progress

Measure before believing anything below: `.claude/restart.md` carries the commands, `intent/restart.md` the rules every bank, train and landing runs under. No count appears in this file.

## DOING

- **THE BOX HAS TWO QUEUES ON ONE MAC (agreed 2026-10-22, until hv rules on one).** vc holds Intent's queue and utilz-vc holds Utilz's and Gtools'. Each sends the other every heavy GO and END by name, and neither GOes while the other's heavy run is in flight. Within a queue, one heavy run runs at a time, with START and END by name and `census-ok-ci.sh` (refs/bank/vc/tools/) before each GO and inside every commit command. The pair every estate runs is the dev tree's, built at dcd9057d4, and `intent --version` names it. brew's keg stays unlinked and pinned.
- **THE TRAIN TO BUILD ALL #2, judged and landed one at a time on vc's word, in this order.** Each lane's board carries its bank refs. The build is vc's, announced to utilz-vc and to every estate's vc.
  1. 0619 (dc): a store warmed cold from a clone takes a pulled board.
  2. 0607 (ic): Intent.app drops its own Console for the macOS Console app and takes Menuz's and Gtools' menu controls, with a floor of macOS 26.0 and xcodeVersion 27.1.
  3. 0615, then 0613 (cc).
- **THE FLEET'S board@4.0 RE-RENDER (issue 0612, live since build all #1).** Intent, Gtools, Molt and Utilz are re-rendered and committed. Every other estate re-renders at its next `intent wb` verb, as one mechanical commit by its coordinating node. From that commit on, a node commits what its verbs print under `wrote:`. A running `intent mcp` server or explorer opened before build all #1 refuses the migrated store until it is reopened.
- **ST0081, INTENT FROM HOMEBREW ON LINUX AND macOS (cc holds it):** the gate is BLOCKED on these rows:
  - AC-01.1 and AC-01.2 rest on run 39034020140, a dispatch at the v3.2.4 tag, and whether that reads as "with no tag" is a judgement.
  - AC-06.1 needs kovacs's post-cut brew-install lines.
  - AC-03.3 needs a kovacs run of `docs/install.md#linux`.
  - AC-02.1 waits on hv's D55.

## TODO

- **hv's INBOX, unanswered:**
  - the stabilisation line after 3.2.4 (vc: yes);
  - a SHA256SUMS file on each release, in ST0081 WP-02 (the stage writes one, the release carries none; vc: yes);
  - naming `local` and `upstream` as the release remotes in `bin/.devbin/config.yaml` (vc: yes);
  - D55 from devbin-vc: devbin's release core attaches CI-built assets, or an upload-only workflow in Intent. Both vcs lean to the first, and the by-hand upload is the interim;
  - where the joint WSL job lives, and whether a hosted runner's WSL 1 counts as evidence;
  - whether the two box queues merge into one, which utilz-vc brings.
- **OPEN ISSUES WITH NO LANE YET:**
  - 0611 (passive prompt injection) is hv's triage and is expected to become a thread; ic offered the inventory of the surfaces that feed agents prompt text.
  - 0614 (linked-worktree carriers) is measured after 0619 lands, since 0619 may be its cause.
  - 0616, 0617 and 0618 are cc's after 0613. 0618 needs ic's read of the surface first.
  - 0620 and 0621 are dc's after 0619.
- **TO MEASURE AND FILE:** a peer's board write that is still landing can make the commit gate's doctor count view skew on that peer's `wip.md`, and so refuse an unrelated commit. dc's fold commit on 2026-10-22 was refused on cc's mid-fold `wip.md` (3266 bytes on disk, 1410 rendered), then passed on a re-issue.
- **UNVERIFIED:** whether the Sinclair-Shared book recreate of decision 63(8) happened, and whether its build-all signal reached sinclair-vc.
- **/Applications/Intent.app is 3.2.1**, while the 3.2.4 release carries Intent.app.zip. hv installs from the release asset.
- **AFTER THE LINE, vc's hand, no push:** hv decision 28's ten Conflab and Baize rulings, which repair records. None is done.
- **AWAITING hv:**
  - a `/refresh` in the explorer (ic's, size S);
  - the next cut, whose canon is `intent/docs/releasing.md` plus the cut lessons in project memory.
- **GTOOLS:** the WP-04 slot that vc's 2026-09-26 resume named is unscheduled, and gtools-vc owns it.
- **COURSES:** courses-vc re-drives its Intent findings against 3.2.4 and sends vc the ones that still reproduce, to file.
- **MACHINE, hv's call:**
  - the resident `SetStoreUpdateService` processes and the Spotlight-family services that push the load high;
  - the CoreSimulator framework that xcodebuild reports as out of date. It is noise for the Release build, and a macOS or Xcode update clears it.
- **ST0084** (Triage, filed 2026-10-17 on hv's word): relocate the parity tooling and reference generators out of `intent/st/ST0056`, so that completed threads can leave the disk. Until it lands, ST0056 and ST0057 stay hydrated, because the gate, the release script and ST0057's acceptance rows cite files under them. **ST0060** and **ST0077** stay in Triage as the next line's queue (hv, 2026-09-23).
