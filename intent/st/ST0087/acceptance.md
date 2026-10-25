---
st_id: ST0087
title: Intent conforms to the dev, opt and brew contract
---

# ST0087: Intent conforms to the dev, opt and brew contract -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### WP-01 -- The tree function, the runtime set and the version line (status: WIP)

- AC-01.1 One function in intentsvcs names the running tree in the contract's order, brew when the manifest says `managed_by: brew`, opt when its `tool` is intent, dev when the root is a git work tree with no manifest, unknown otherwise, reading the manifest before the path and answering brew for a manifest-less keg whose root resolves under the opt link; every caller asks it, and doctor fails on unknown and on a dev tree whose git cannot be read. -- satisfied: no (computed)
- AC-01.2 One function lists the runtime set, the support paths git names plus the pair built from the same commit and nothing else, and install and doctor both read it; `intent install --print-runtime-set` prints it from the caller-named pair and is the verb's only mode until WP-02. -- satisfied: no (computed)
- AC-01.3 `intent --version`, `intent version`, intentd's own version print and doctor's first human line come from one formatter and read `intent <version> (<tree>, <sha12>[+dirty]) at <root> [<provenance>]`: the sha is the built commit read from the binary and never HEAD, `+dirty` appears only in dev and only when the support set differs from the built commit or the pair was built dirty, the root is absolute and brew's is the opt link, the provenance word stays last, and a dev tree whose git cannot be read prints `(dev, unknown)`. -- satisfied: no (computed)
- AC-01.4 Every reader of the old line is moved and driven against the new one: `gen_explorer_shots.sh`, `version_spellings_agree.rs`, `arguments_do_not_start_a_daemon.rs`, `macos_release_assets.bats` and `burn_subject_provenance.bats`; the gate's self-provenance arm and the build's pair verification still read the embedded source-commit marker, and readers of the version field alone are unchanged. -- satisfied: no (computed)

### WP-02 -- intent install and the manifest, with doctor's manifest arms (status: Not Started)

- AC-02.1 `intent install [--prefix <dir>] [--force] [--managed-by brew]` runs in dev only and is refused by name from opt and brew; it refuses a dirty runtime set by name with no override while live project records never count; the prefix comes from `--prefix`, else `INTENT_INSTALL_PREFIX`, else it refuses by name; an existing prefix is replaced only with `--force` and only over a manifest saying `tool: intent`; and the layout it writes is the keg's libexec layout exactly. -- satisfied: no (computed)
- AC-02.2 The manifest is `<root>/.intent-install` in T3's exact form: a header of `# key: value` lines carrying at least tool, version, source_commit as the full sha and managed_by, then one `shasum -a 256 -c` line per runtime file, path relative to the root, sorted by `LC_ALL=C sort`, the binaries checksummed as installed. -- satisfied: no (computed)
- AC-02.3 Doctor in opt and brew checks the manifest body and names every file that drifted; a keg with no manifest whose root resolves under the opt link is a note naming `brew upgrade intent`, never a failure; and an opt prefix with no manifest is not Intent's, so `use opt` refuses it by name. -- satisfied: no (computed)
- AC-02.4 A manifest written with `--managed-by brew` makes `intent install` and `intent use` refuse every write into that tree, naming brew as its manager. -- satisfied: no (computed)

### WP-03 -- intent use and the shadow check (status: Not Started)

- AC-03.1 `intent use` with no argument reports the tree each link serves and changes nothing; `use dev` and `use opt` point both links of the set, `intent` and `intentd` in `~/.local/bin`, at that tree's own copies as one set, each refusing a tree that is not there by naming the step that makes it; `use brew` removes the whole set and nothing else, so no link means brew. -- satisfied: no (computed)
- AC-03.2 The shadow check, in doctor and in bare `use`, fails when a link in the set exists but is not the first hit on PATH or when the set is split across trees, names what shadows or splits it, and compares resolved directories rather than path strings. -- satisfied: no (computed)
- AC-03.3 `use dev` and `use opt` end by exec'ing the new link's `intent daemon restart` and print that binary's version line, never restarting in-process, so the old tree's daemon does not keep serving. -- satisfied: no (computed)
- AC-03.4 Intent.app keeps the link path it resolved and follows it live at every spawn rather than a realpath taken at launch, its tests move off the realpath pin, and `VersionService` parses the new line's tree, sha, optional `+dirty`, root and provenance word, with tests over the new line, its `+dirty` form, the brew form and the `(dev, unknown)` form. -- satisfied: no (computed)

### WP-04 -- The formula, the release tooling and the docs (status: Not Started)

- AC-04.1 The formula is rendered, never typed, by `bin/int brew formula` with `render` and `publish` from the release's version and per-asset checksums; it installs the signed release assets, writes `<root>/.intent-install` in T3's form with `managed_by: brew`, links both binaries into Homebrew's bin, runs the version line in `test do`, and its caveats name `intent bootstrap` and `intent use`. -- satisfied: no (computed)
- AC-04.2 (non-test) `intent/docs/releasing.md` no longer unlinks and pins the keg, `bin/int brew smoke --reinstall` leaves the keg linked, and `docs/install.md`'s maintainer section documents the three trees, the per-tree upgrade procedure of T6 and the PATH hazard the shadow check cannot see, while the user's docs name brew alone. -- satisfied: no

### WP-05 -- The proof, per tree, on rhadamanth and kovacs (status: Not Started)

- AC-05.1 (non-test) On rhadamanth, from a login session started after Molt-matts' PATH prepend, `intent use dev`, `intent use opt` and `intent use brew` run in turn, each followed by the version line and `intent doctor` with the shadow check passing, and the transcript is attached to this thread. -- satisfied: no
- AC-05.2 (non-test) On kovacs, with brew alone, the version line and `intent doctor` pass on the release that carries WP-04 once it reaches the tap, and the run's record is attached to this thread. -- satisfied: no
- AC-05.3 (non-test) In this checkout, in each of devbin's trees, `bin/devbin doctor` names the install devbin's switch chose and why, and `bin/devbin upgrade` reads that install; the record is attached to this thread. -- satisfied: no

## Acceptance Tests

### WP-01 -- The tree function, the runtime set and the version line (status: WIP)

_(no tests in this group)_

### WP-02 -- intent install and the manifest, with doctor's manifest arms (status: Not Started)

_(no tests in this group)_

### WP-03 -- intent use and the shadow check (status: Not Started)

_(no tests in this group)_

### WP-04 -- The formula, the release tooling and the docs (status: Not Started)

_(no tests in this group)_

### WP-05 -- The proof, per tree, on rhadamanth and kovacs (status: Not Started)

_(no tests in this group)_

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
