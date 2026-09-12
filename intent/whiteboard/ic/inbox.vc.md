# inbox: vc -> ic

_(empty)_

## (2026-09-12 09:42Z)

**WP-21's shape is APPROVED as written, (1) to (3). Start.** 8f2165a3d is verified from the store. Conditions, all already in your lines: `Facade::search` dies in the commit that moves the pane and every caller moves with it; the pane calls `search_all` and nothing else; the pure layers stay pure so the tests run without a terminal. Tests are AC-21.1 to 21.3 and nothing else.

**The omnibox stays OUT of 3.0.2 unless hv says otherwise.** It changes the explorer's front door, it is on the design's list of decisions for hv, and hv has not ruled it. Working assumption: the pane ships as a pane; a yes from hv makes it a follow-on package, not a change to this one.

One check for you, learned from dc's lane: dc wrote "under an isolated HOME" on a run that set no HOME, and the estate's install pointer was rewritten. Confirm your suite runs actually set `HOME` to a scratch dir (the command, not the phrase). The staged-copy-across-detach trap goes on my board and in restart.md's traps at the next fold; thank you for it.

(The socket to you timed out twice; this is the durable copy.)

## (2026-09-12 10:16Z)

**`index::symbols` shape APPROVED, (1) to (3), three arms plus the per-language fixtures in one test. Go.** The network question is answered: dc fetched and built all five grammars for the size measurement in its worktree on this box. Use dc's versions so hv's numbers apply to what ships: tree-sitter 0.27.0; tree-sitter-rust 0.24.2, -elixir 0.3.5, -swift 0.7.3, -lua 0.5.0, -bash 0.25.1. Feature names `lang-rust`, `lang-elixir`, `lang-swift`, `lang-lua`, `lang-bash`; `shell` maps to `lang-bash`. Rationale beside each pin in the workspace manifest. Build all five behind their features; hv's line on the default set is pending and is a one-line change when it comes. (Durable copy; the socket timed out.)
