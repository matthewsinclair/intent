---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 12:35Z
status: active
focus: "On the bounce, reopened from the eleventh fold. hv put ST0064 (macOS menubar app) into 3.0.1 as an M PORT, and it is my primary work. Plan built and criterion-bound this turn: most of the nine ACs are buildable now; two seams are gated (cc's daemon predicate for AC-01.2/06, hv's URI ruling for AC-01.5). Pickup finding: the GraphQL executor LANDED at dbfc1eb1 (34 commits after the 62d2d633 read every -not-shipping- note rests on), so AC-09.2 / AC-00.4 clause / AC-00.3 GraphQL are re-measure-not-descope. Awaiting hv's go per the boot ritual, then work with vc on the return."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold verbatim at `.history/20260831/wip-fold-1213Z.md` (eleventh fold). Cold-session minimum: state, not story. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree. Tree clean of me at reopen; peers moved four wb/fleet-verdict commits since 2410b89c, none in my lane.**

## DOING

**ST0064 -- THE MENUBAR APP, MY PRIMARY WORK (hv reversed it into 3.0.1). Plan built this turn, criterion-bound, HELD FOR hv's GO (boot ritual: show plan, stop).** M PORT of Gtools/Geodica (~2470 LOC, size+shape ref); the recorded design spin is `intent/st/ST0064/design-menubar-app.md`. All nine ACs are non-test (evidence-satisfied) and bind to WP-01. `native/macos/` is greenfield.

**BUILDABLE NOW (no cross-node/hv dependency):**

- **AC-01.9 + AC-01.1 -- the clean first target.** `LoginShell.swift` (96 lines, priced from Geodica) captures the login-shell PATH; ONE CLI wrapper owns binary resolution + env + error mapping; nothing else in Swift spawns. AC-01.1 is the no-product-logic constraint I build TO (verified by absence at the end: no log parser, no pidfile path, no config key in Swift).
- **AC-01.3 -- status is a GraphQL query shelled through `intent graphql`** (executor landed dbfc1eb1), `stopped` = the query failing. NO `GET /_status` (superseded per D56, and the AC says so explicitly). Buildable NOW precisely because the hatch landed.
- **AC-01.7 (verb half) -- EXTEND the existing `int macos`** (`bin/.devbin/cmd/macos`, already has stage/sign/notarize/checksum/verify/formula/publish per AC-11.1) with app `build/run/test/install`, reusing its sign/notarize machinery for the `.app`. REFUSAL ARMS FIRST: `verify` (read-only prove) + a doctor check refuse with no creds -- an unsigned local build is the most anyone drives. Notarise arm = AC-01.7's hv-ADC.
- **AC-01.8 (asset half) -- the turtle** at `docs/design/intent-logo.{svg,png,pdf}`; rasterise with `rsvg-convert` not qlmanage (handoff trap). Menubar glyph wired; the state-derivation half is gated (below).

**GATED SEAMS (build the wire-in point, complete when the dependency lands):**

- **AC-01.2 / AC-01.6 / AC-01.8-derivation** -- health predicate LIVE/STALE/ABSENT, icon derived at paint time. Gated on **cc's daemon seam** (cc widening `daemon::route` 2->3 + machine-readable `daemon status`; cc's #1 on the bounce). The app calls the CLI verb, never a Swift probe (AC-01.2). cc brings vc the shape; I consume it.
- **AC-01.5 -- the `intent://` handler -> the ONE resolver** (ST0057 WP-07), never a second resolver; opens via `intent edit`. Gated on hv's two URI rulings; my half of the ic/cc split.

## TODO

1. **THE URI-UNIFORMITY WORK -- release-blocking for TWO threads** (hv's bug + ST0064 AC-01.5), co-designed with cc, HELD FOR hv. Two rulings: (a) nav SINGULAR grammar user-facing or internal; (b) is strict the only form -- and strict needs an EMIT partner because NOTHING emits a canonical address (cc measured zero across every read surface incl `--json`). **SPLIT (cc's board confirms): cc takes ~40 narrow-door sites + promote-then-narrow-then-RE-RENDER helper + property test + ratchet; I take explore + address->view resolution + grammar surface + the EMIT side.** Ready to execute on hv's ruling.
2. **SCOPE Q for vc -- the console has NO log source.** `intent daemon` is start/stop/status/run only; no `logs`/tail verb. AC-01.4's orphan-trap gate ("verified before any console is built") and the console feature both need a log source -- a `daemon logs` verb (whose lane?) or a daemon-owned logfile tailed by path-from-verb. Is the console in 3.0.1, or a follow-on? Surface on the return.
3. **SCOPE Q for vc -- GraphQL executor landed (dbfc1eb1, in HEAD, 34 commits after 62d2d633).** Both faces (CLI `graphql`, MCP `intent_graphql`, served) route the one hatch; reads-only. AC-09.2 / AC-00.4's clause / AC-00.3's GraphQL are re-measure, not descope. vc's own TODO already says "GraphQL now ships."
4. **AC-09.6** -- the 21-row exposed=>servable flip. BLOCKED on dc's `dispatch-table.json` (dc on ST0068 this bounce). 15-min interrupt when `git status` clean on it; build from `git show HEAD:`.
5. **AC-06.8** -- instrument fixed, row RED by vc's note (0180 withdrawn by dc; 0181 mine, same question). Not mine to close.
6. **Parity-tool hardening, mine, unblocked, harmless-today-wrong (not tag-blocking):** conservation_check.sh `refiled` partition + `UNACCOUNTED` 5-way split (cc's `intentsvcs::sync::NOT_CARRIED`/`NOT_YET_BUILT`); gen_dispatch_table.sh rotted mutation-proof (doctor gained 3 keep flags at cb78080d).
7. **Standing:** 0142 guide.rs write (mine); TUI remainder (AC-17.1/17.6, hv drives); ST0065 (hv takes with me, out of cut).

## Watch-outs -- mechanisms only

1. **A CROSS-CRATE GUARD IS INVISIBLE TO `cargo test -p <the-crate-you-changed>`.** Run `cargo test -p intentsvcs` before any address-touching landing (the URI work will touch it).
2. **`git add $P` WITH A SPACE-JOINED VAR DOES NOT WORD-SPLIT UNDER zsh** -- one pathspec, matches nothing, commits NOTHING while a peer's commit sits at HEAD. List paths explicitly.
3. **cc's POST-VERIFY IS STANDING: after every commit, `git show --name-only HEAD` must EQUAL your intended path set** -- never your staged list.
4. **THE `intent://` SCHEME IS `address.rs`'s -- PLURAL (`intent:///threads/ST0056`, 4-digit issue ids), through `address::parse`/`Entity`. The SINGULAR `/thread/` is `nav.rs`'s TUI/web grammar.** Do not conflate; the guard reds it.
5. **THE FIGURES ROT AND THE PROPERTY DOES NOT** (vc, 08:15Z). Release binaries on PATH are ~14 commits blind; drive `native/rust/target/debug/intent` for recently-landed behaviour, or `instrument_currency_check.sh`. Re-run tools; never quote a peer's count.
6. **A DELETE/CREATE HAS THREE POPULATIONS; the second is skipped: what EXECUTES, what CITES, what CHECKS.** ST0064 criteria bind to WP-01 by AC-01.N prefix -- WP-01 owns the whole boundary.
7. **RealDaemon REFUSES A STALE SIBLING intentd; `cargo test -p intent-cli` does NOT build intentd.** `cargo build -p intentd` before any daemon-backed run.
8. **AN ATTACHMENT RE-ATTACH MUST CARRY THE FINAL BYTES: prettier the .md BEFORE the last `st attach`.** A `sync --to-disk` realises a peer's pending attachment into your extract -- splice their leaf to HEAD with jq.
9. **THE STORE IS SHARED BEFORE THE TREE IS.** Land canon writes in the same breath; a path-scoped `--only` excludes peers' dirty files but the STORE already has your writes.
10. **MARKDOWN PRETTIER-FORMATTED BEFORE `git add`.** A WORKED EXAMPLE IN A REMEDY IS SHIPPED SURFACE (`ST0000`). A FLAG NOT IN THE USAGE LINE READS AS A FINDING.

## Decisions

- **2026-08-31 GraphQL executor is BUILT and in HEAD (`dbfc1eb1`), 34 commits after the `62d2d633` read.** The daemon executes reads-only GraphQL (`Op::Graphql` -> `store.rs:634` -> `Facade::graphql`); `SERVED_BY_DAEMON=[("st list",...)]` is a DIFFERENT roster (dual-path identity); `graphql` is `DAEMON_ONLY`, no in-process twin (CLI links no async runtime). My AC-09.3 bridge-restart test is SOUND. The peers' "GraphQL not shipping / AC-09.2 unowned" notes are stale.
- **2026-08-31 `int macos` ALREADY EXISTS with the sign/notarize/verify pipeline** (AC-11.1). AC-01.7 EXTENDS it for the `.app`, not a wholesale Conflab port.
- **2026-08-31 hv REVERSED ST0064 INTO 3.0.1 as an M PORT, not L/XL** (via vc: _"The intentd and the menubar app that controls it are linked and need to both land for 3.0.1... We need both."_). Signing path IN. Criteria bind to WP-01 by AC-01.N prefix.
- **2026-08-31 the URI-uniformity fix (co-designed cc+ic, hv to rule):** `intent://` plural is the one user-facing grammar via `promote`; narrow doors delegate promote-THEN-NARROW; nav singular is TUI-internal. Nothing emits a canonical address, so strict needs an emit partner.
- **2026-08-31 vc bounded AC-09.5: resources serve entity docs through EXISTING facade doors, ZERO new facade surface;** the one `crate::show` seam makes contents==CLI read by construction.
