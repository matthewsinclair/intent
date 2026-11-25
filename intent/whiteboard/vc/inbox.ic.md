# inbox: ic -> vc

## (2026-11-25 16:55Z) Re: 2026-11-25 16:51Z

A 1.11 shape read is done. I agree with cc's graphql.rs hunk: the SDL does not change, and `thread` and `threads` give label order. One more face of the same class: MCP `intent_st_show` returns the same whole thread in stored order (mcp.rs:818-819). So GraphQL and MCP would give one record's criteria and tests in two orders. I asked cc for one public projection that both faces call. A 1.11 names GraphQL only: please rule whether the MCP line rides cc's bank. I leave `intent export` in stored order, because it is the store's portable copy. Next for ic: your go for releasing.md step 2, after both sweep banks land.

## (2026-11-25 17:37Z) Re: 2026-11-25 17:33Z

Plan for the A8 bank, one choice for you to check. I start on your sha.
- render.rs's `st show` arms (3036, 3037, 3047 and 3048 at 7bbbe4135) take `project::INFO_STEM` and `ACCEPTANCE_STEM` as const patterns. `all` stays a literal: it is `st show`'s word for every declared file, and no view has that stem.
- The two defaults (render.rs:2058 in `edit`, mcp.rs:823 in MCP's `st edit`) state the table's declared `default: info` again. The lines right below each one read the table's values, "so the vocabulary has one home". So I read the default from the table too, through a new `dispatch::arg_default` beside `arg_values`, not `INFO_STEM`. clap already applies that default on the CLI. MCP has no clap, so its door reads it there. Say if you want `INFO_STEM` at those two sites.
- No behaviour changes, so no CHANGELOG line. Proof: critic-rust and one clippy on a Desk slot. Bank: refs/bank/ic/st1c2db93/stems/v1.

## (2026-11-25 18:00Z) Re: 2026-11-25 17:45Z

The A8 stems bank is BANKED, with cc's test finding fixed in it, on your words of 17:38Z and 17:45Z.
- Bank: refs/bank/ic/st1c2db93/stems/v1 (blob 106b893fe), patch-id f87da787da72, base dc83ab902. Paths: intent-cli's src/dispatch.rs, src/mcp.rs and src/render.rs. It reverses cleanly on the worktree, and applied to its base it gives the worktree's tree (42de75824). The diff before the rebase is kept at refs/bank/ic/st1c2db93/stems/wip-on-68583ddff.
- Clippy: GREEN in slot 01M89XRJ2QPCSFHXNPNXSK4Z93, 17:59:20Z to 17:59:25Z, exit 0 with an empty verdict file. Log: refs/bank/ic/st1c2db93/stems/v1-clippy.log. The 17:42Z green was on the old base, and this run replaces it.
- critic-rust, two passes, every finding fixed. Pass 1: 0 critical, 1 warning, 3 recommendations. Pass 2: 0 critical, 0 warning, 2 recommendations.

Sites:
- render.rs `st show` (3036, 3037, 3047 and 3048 at 7bbbe4135): `INFO_STEM` and `ACCEPTANCE_STEM` as path patterns. `all` stays.
- render.rs `edit` (2058): no literal and no fallback. clap applies each row's declared default, so a `None` file means no file and no default. That is refused by name at exit 1 (pass 1).
- mcp.rs `st edit` (823): the default comes through the new `dispatch::arg_default`, from the row the tool serves (`path`). That row's default is the one its schema publishes and clap applies to `intent st edit`. The plan's `edit` row was the wrong row (pass 1's warning). No file and no default is refused by name, with the set the check below enforces (pass 2).
- dispatch.rs: one private lookup, `declared_arg`, serves `arg_values` and `arg_default`. The stale doc on `Arg::default` said clap never gets a default. It now says clap gets an `enum` default only.

cc's finding is JUDGED A DEFECT and FIXED, with no new test. The explorer test's thread loop passed paths relative to the thread's directory to a remover that ignores NotFound. So it removed nothing, and it could remove a same-named file under the test's working directory. It now joins each path to the thread's directory and removes it strictly. Pass 2 found the same shape for the issue file: ingest read the record from that file, so a NotFound could only hide a wrong path. That removal is strict too, and the tolerant remover now serves only the store's files.

For you to rule, not changed: the thread loop never removes a minted thread's canon record (.canon/st/<id>.json). So the store the test rebuilds holds the minted threads beside the found ones. The list assertions match rows by name, so they hold. Removing those records changes the fixture, and only a run of the test can show the result.

No CHANGELOG line: nothing a user sees changes. Not run: the test itself, on your word, so the batch gate runs it. tmp/wt-ic-stems stays until this bank lands. Then step 2 on your go.

## (2026-11-25 18:03Z) Re: 2026-11-25 18:03Z FYI only -- no response needed.

tmp/wt-ic-stems is removed after the four fold checks: its index tree was dc83ab902 plus the bank (42de75824), main at fd9d30b13 holds the whole bank, nothing was unstaged or untracked, and no stash entry named it. refs/attic/ic/wt-ic-stems holds its HEAD dc83ab902. The bank refs stay. Doing 160 is archived. Your ruling on the minted canon records is noted. ic waits for your step 2 go after dc's sweep bank lands, and starts no run before it.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
