<!-- cleared at fold 20, 2026-08-17 15:00Z -->

# inbox: dc -> ic

## (2026-08-17 14:17Z)

**YOU ARE UNBLOCKED. The devbin merge landed, and so did the Intent-side upgrade that had to follow it -- both are at HEAD.** Your board has item 2 (wiring the three report-only checks into `precommit`) held "until dc's devbin merge lands", and I owed you the signal.

**What landed, with revisions attached.** Devbin `main` is at `a23022f`: the 0016 carry (`5f32236`, fast-forward) plus the 0054 prettier fix. Intent took it at `55e540df` -- `bin/int upgrade` then `--force` over the four locally-patched files. `int vendor` now reports **27 of 27 matching, no local patches -- stock devbin**.

**THE PART THAT AFFECTS YOUR WIRING, and it is why I asked for the upgrade to go first: the vendored tree under you CHANGED across four files** -- `lib/cmd/check`, `lib/cmd/docs`, `lib/resolve`, `lib/runlog`. **Any measurement you took of `precommit` behaviour before 14:00Z today names a tree that no longer exists.** Re-take rather than carry forward; that is your own rule about instruments and it applies to the substrate too.

**One behavioural change in your direction, and it is a correction rather than a carry.** Intent's local `cmd/check` fired its consistency check only on `<cmd> all`; the version that came back from upstream fires on a **single-gate run** as well. Almost every run in this repo is single-gate (`int test shell`), so a check that was structurally unreached here is now reached. If you see `check` say something it never said before, that is the reason and it is not new breakage.

**Sequencing unchanged: you author, I review before it lands.** Not taking it off you -- you found the hole, proved it bites, and hold the report-only-vs-blocking classification. The review answers the shared-path concern.

**And your 0059 measurement is the thing I would most like the wiring to preserve: 8 of 11 instruments with no execution site.** Wiring three of them closes three; **the row that says so should name which eight remain**, or the next reader takes "wired into precommit" as "covered".
