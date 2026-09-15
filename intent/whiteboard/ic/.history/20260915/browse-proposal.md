# What `intent browse <kind> <id>` opens

**Status: THE DECISION REQUEST IS WITHDRAWN; THE MEASUREMENTS STAND.** vc commissioned this 2026-09-04 and withdrew it the same afternoon on the grounds that the design already answers it -- correctly, for the parts the design covers. **It is kept because four things in it are NOT in the design and are inputs to the build:** the `AC-11.3` environment allowlist (constraint 11), that no URL opener exists anywhere in the workspace (constraint 10), the route-versus-fragment tension (below), and the first-run token handover. **Written in my own node directory rather than `intent/st/ST0056/` because an unratified proposal does not belong in the design canon.** Options A-D below are no longer a request for a ruling; A remains what I intend to build unless told otherwise.

**Every constraint below was read at source this session and is cited. Nothing here is recalled.**

## What is being decided

`AC-17.6` requires that **`intent edit <kind> <id>` and `intent browse <kind> <id>` reach ONE MODEL through ONE SERVICE.** It is the last open row in WP-17 (17 of 18). The row contains no clause about a daemon-served HTML page -- vc drove that, and it is why this is a proposal rather than an escalation.

The row's own words route the shape question to me: _the shape change was therefore a deliberate surface decision belonging to the interface owner._

## Established constraints

**From `tui-design.md` section 9 (the `intent edit` surface):**

1. **`--browser` uses localhost HTTP, not a custom `intent://` scheme.** Reasoning recorded there: a custom scheme needs per-platform OS registration and buys nothing, while adding a URL-handler surface anything on the machine can invoke.
2. **The URL addresses the ENTITY, not a filename.** The example given is `http://127.0.0.1:<port>/threads/ST0058?mode=edit`.
3. **`--browser` refuses when the daemon is not running**, naming `intent daemon start`, rather than spawning a process the operator did not ask for.
4. **`intent browse <kind> <id>` IS `intent edit <kind> <id> --browser`** -- two spellings, one capability. `ST0058 AC-00.6` refuses DISAGREEMENT between a flag and its subcommand twin, not duplication, so two spellings that agree are what it asks for. The register row is where the agreement is asserted.

**From `D56` (`design.md`) and `tui-design.md` section 10a:**

5. **`intentd` emits JSON only and renders no view**, deliberately, because the same answers must serve a browser, a terminal and a menubar app. The deciding argument is SwiftUI, which cannot consume server-rendered HTML.
6. **The browser face is designed: plain ES modules, served same-origin from the binary. No npm, no bundler, no wasm, nothing in CI.** It renders `{label, value, widget}` triples and never learns the domain.

**From the as-built:**

7. **`intentd` serves exactly three routes** -- `/`, `/intent-logo.svg`, `/op` (`web.rs:77-79`).
8. **`/op` requires `Authorization: Bearer`, and a query parameter is explicitly refused** (`web.rs:185-197`): _a query string is the one place a secret reliably leaks: it lands in browser history, in a `Referer` header on every outbound link, and in any proxy log between here and nowhere._
   9a. **THE FORM DERIVATION EXISTS AND HAS NO DOOR ON THE WIRE.** `intentsvcs::form::triples(form, entity) -> Vec<Triple>` is built, pure and shared, and `surface/forms.json` declares the layout. `web.rs` says the derivations a UI needs are already built and are not re-walked there. **But the op vocabulary is exhaustively `ThreadList`, `Graphql`, `Set`, `Registry`, `Subscribe`, `Shutdown` -- there is NO form op**, and `Graphql` is reads-only under `EmptyMutation` and exposes no form. So section 10a's claim that the daemon resolves the form declaration server-side and emits `{entity, title, fields}` describes something nothing currently does. **A WIRE DOOR HAS TO BE BUILT BEFORE ANY ES RENDERER CAN ASK FOR A FORM.** Declared-but-unwired, one layer below the class this thread keeps finding.

9. **No ES modules exist.** `intentd` embeds exactly two assets: `shell.html` and the logo SVG. The browser renderer is designed and unbuilt.
10. **No URL opener exists anywhere in the workspace.** Measured: nothing shells out to `open` or `xdg-open`, and there is no opener crate.

**And one constraint nobody handed me, which binds any opener:**

11. **`AC-11.3`'s audit is an ALLOWLIST over environment reads, enforced structurally.** `no_intent_home.rs:79` declares `ALLOWED = ["COLUMNS", "EDITOR", "HOME", "USER", "VISUAL"]`, each with a SOLE-FILE grant, and its own text says **a new entry needs an hv ruling and a row, not a quiet addition.** **So honouring `$BROWSER` is not a free convenience -- it is an hv ruling.** Shelling out to a fixed program name reads no variable and is unaffected.

## The one real tension, and it is narrow

**section 9's example URL is path-addressed (`/threads/ST0058?mode=edit`), which implies a fourth route. Constraints 5 and 7 say three routes and no view rendering.** That is the whole of the conflict. It is resolvable at the client, so it does not go to hv.

## Options

**A. Fragment addressing on the existing `/` route.** `http://127.0.0.1:<port>/#/threads/ST0058?mode=edit`

- A fragment is never sent to the server, so **no fourth route and no server change at all**.
- The ES module reads `location.hash`.
- Honours section 9's _URL addresses the entity_ in substance; departs from its literal example path.
- Leak surface: a fragment lands in browser history but not in `Referer`, not in server logs, not in proxy logs. **An entity id is not a secret**, so constraint 8's reasoning does not bite here -- but it is worth saying out loud rather than assuming, because 8 is the codebase's own stated rule about URLs.

**B. Query string on `/`.** `http://127.0.0.1:<port>/?entity=st/ST0058&mode=edit`

- Also needs no new route.
- **Strictly worse than A for no gain**: the query IS sent to the server and DOES travel in `Referer`, which is the exact surface constraint 8 names. Same cost, more leak.
- Listed because it is the obvious first thing to reach for, and the reason to reject it is written down rather than left to be re-derived.

**C. A CATCH-ALL FALLBACK serving the SAME static shell.** Any `nav::View`-parseable path returns the identical bytes `/` returns; the ES module reads `location.pathname`.

- **It renders no view** -- it serves the shell that is already served, at more paths -- so it does not breach D56.
- **MY EARLIER OBJECTION TO THIS OPTION WAS WRONG AND THE MEASUREMENT KILLED IT.** I wrote that C costs a per-kind route table that grows as kinds are added. It does not: `View::parse` is generic over every declared kind, so this is ONE axum `fallback`, not a table. I had not read `nav.rs` when I wrote that.
- **Needs cc's ruling** on whether a route that renders no view clears their stated constraint. I am not deciding that for them.

**D. Print the URL and open nothing.** Rejected. It does not reach the model, and reaching the model through one service is the whole criterion.

## Recommendation

**C. THIS REVERSES MY OWN EARLIER RECOMMENDATION OF A, AND `nav.rs` IS WHAT REVERSED IT.**

I recommended A (fragment addressing) before reading `intentsvcs/src/nav.rs`. That module IS the ratified path contract -- `web.rs` calls it the path contract the TUI and the browser must share, it cites `AC-17.7` and `AC-17.12` and vc's 2026-08-30 ruling, and `View::path` / `View::parse` are held to a round trip over every view the real declaration produces.

**`View::path()` PRODUCES `/`, `/{kind}`, `/{kind}/{id}`, `/{kind}/{id}/{field}`, `/settings`, `/help`, `/help/{name}`.** It is designed to BE the browser's URL path. Putting it behind a `#` would take the shared contract and bury it one level down, in a place the server never sees, for no gain.

**AND `AC-17.12` SAYS THE BROWSER'S URL IS THE SEQUENCE**, with `nav.rs` naming browser history as the web's view stack explicitly -- _which it already has and must not be given a second copy of_. A real path gives that natively.

**THE DESIGN DOCUMENT DISAGREES WITH ITSELF AND THE RATIFIED SIDE IS `nav.rs`, NOT section 9.** Section 9's example URL is `http://127.0.0.1:<port>/threads/ST0058?mode=edit`. That is wrong twice on the contract's own terms: `nav.rs` says **`/thread/ST0056/wps`, never `/threads/ST0056/work-packages`** and _no segment is invented, pluralised or prettified_ -- so `/threads` is exactly the pluralisation the contract forbids -- and `?mode=edit` is not in the contract at all. **Section 9's example is an ILLUSTRATION written before the contract; `nav.rs` is the contract.** This is the disagreement vc asked me to drive first, and driving it cost one file read.

## The auth handover -- the part that is NOT solved

**cc told me to withdraw the auth constraint from my inputs because section 10a answers it. I am accepting half of that and I want the other half visible.**

What section 10a genuinely settles: **where the token lives in the client** (`localStorage`, not `sessionStorage` -- the latter is per-tab and made Safari re-prompt), and **the liveness handling to copy** (Conflab's `daemon_bridge.js`, three consecutive failed probes before declaring the daemon dead).

What it does not settle: **how the token first reaches `localStorage` on a page `browse` has just opened.** It cannot come through the URL -- constraint 8 forbids exactly that, in the strongest terms in the file. So first-run is one of:

- **The operator pastes it once.** `shell.html` already displays the `curl` invocation naming `~/.local/share/intent/intentd.token`, with a copy button. This is the smallest path and requires no new mechanism.
- **A one-shot nonce in the fragment, exchanged at `/op` for the token.** Materially different from leaking a long-lived secret, but it is a new auth mechanism and therefore an architecture commitment. **If this is wanted, it goes to hv, not to me.**

**I am not proposing between these, because the choice is a product-feel call about first-run UX and that is hv's, in the same class as the palette `Home`/`End` flip I am already holding for them.**

## Sizes, with the guesses marked as guesses

- **The `browse` arm + `--browser` success path + calling `running_daemon_pid`: S. THIS IS A GUESS.** I have written none of it. Basis: argument parsing, a URL build, a daemon probe that already exists, and a shell-out. **What I have NOT costed is the shell-out's per-platform surface** -- there is no opener anywhere in the workspace (constraint 10), so this is new ground, and `launch_editor` is the precedent to copy rather than a thing to reuse.
- **The ES renderer: NOT SIZED. I have not costed it at all, and it is the bulk of the work.** section 10a specifies it in detail -- vertical split, widget vocabulary, the bridge precedent -- **and specification is not sizing.** Reading a detailed design and inferring a size from how well specified it is is precisely `W29`, which I banked this morning after doing exactly that to A1. I am not repeating it eight hours later.

## What I have not measured

- **Whether the ES renderer belongs to WP-17 or wants its own WP.** That is vc's sequencing call, and it is the question cc and I converged on.
- **Whether an operator pasting a token once is acceptable to hv.** Product call.
- **Whether `open`/`xdg-open` behave acceptably when no browser is configured**, on either platform. Unmeasured.
- **Whether any AC row other than `AC-17.6` would be touched by building this.** I checked that no other criterion owns the entity page and found `AC-08.9` owns the web face as an intentsvcs surface -- a different thing, now resolved -- but I did not sweep every row for adjacent claims.
