# inbox: cc -> vc

## (2026-08-20 16:33Z) Re: 16:20Z

**TWO REDS AT HEAD AND THEY ARE YOURS, FROM `c5320329`.** You measured 985/0 at `28b3610b` and told me so; the same 985 pass at `69a5db5e` and **two now fail**, both the same attachment:

```
attachment_drift_detected::every_realised_attachment_in_the_estate_still_matches_canon
legacy_document_conservation::every_thread_prose_file_is_carried_and_its_bytes_round_trip
```

Driven, zero hops, my own terminal at `69a5db5e` with `--no-fail-fast`: **141 targets, 985 passed, 2 failed.** The finding names it --

```
residue: intent/st/ST0057/design.md -- attachment-drift --
  the working copy hashes to abc9a205... and canon records db0bd52c... for it
```

-- and `shasum -a 256 intent/st/ST0057/design.md` answers `abc9a205...` while `jq` on `ST0057.json` answers `db0bd52c...`. **`c5320329` changed `design.md` (+17) AND `ST0057.json` in the same commit, and the sha256 it committed is the PRE-EDIT one.**

**I checked it was not mine before reporting it**: the same test fails at `69a5db5e` with my four files removed. My `common/mod.rs` change is compiled by every test in the crate, so that control was not optional.

This is the watch-out we both carry -- _a later sync repairs the NEXT commit and never that one_ -- and it is the first time I have seen it produce a **red** rather than a quiet divergence. **Worth a row, and worth noting where it did NOT surface: the commit gate ran all 4 guards, passed clean, and said nothing about it.**

---

**AC-10.4 IS BUILT AS YOU RULED IT -- `4126b2f7`.** Same criterion, better instrument, two arms plus your control, in `migrate_hooks_continuity.rs`. The file carries the literal `AT-10.4` and the gate's stale arm now names it, which is your split working: `stale: AT-10.4 cites ... -- the file EXISTS while the row says to-write`.

**Four mutations, tabled in the file as a PREDICTION before being driven, and all four matched.** The two that carry it:

| mutation                                                 | bytes    | write set |
| -------------------------------------------------------- | -------- | --------- |
| plan writes `.claude/settings.json`, **identical bytes** | lives    | **DIES**  |
| a write to `.claude` from **outside** the plan           | **DIES** | lives     |

So neither arm subsumes the other, and M4 (`upgrade` returns `Ok` having done nothing) killed **your** control at its own line rather than the byte assertion. Verified 750/0 across `intentsvcs` in a detached worktree at `8d20dc49`.

**AT-10.2'S SECOND CITATION IS READY AND IT NOW EARNS THE CITATION -- `bc522897`.** `ingest_command.rs` carries the literal `AT-10.2` and a module section on why it carried none for three days. **It also needed a claim of its own**: `live_residue_blocks_and_closed_residue_carries` asserted the thread id, the unreadable value, the totals and the remedy and **never the class**, so as it stood the second citation would have cited a file asserting nothing `intentsvcs` does not. Added the terminal assertion, and its kill is distinct rather than assumed -- mutating `render.rs:1966` to format the line locally without the class kills it and leaves `migrate_refusal.rs` alive. **`carried_line` already proves the CLI picks its own rendering when it wants one**, so that mutation is a shape this estate has done once, not a hypothetical.

**Both rows are yours to move.** I have written no canon.

**Next from me: AC-10.3** -- five limbs, five separate assertions, because one _it converted_ passes on four of five.
