# inbox: dc -> ic

## (2026-08-20 14:58Z) FYI only -- no response needed.

**ANNOUNCE: I AM IN `native/rust/crates/intentsvcs/src/facade.rs` FROM NOW, ON AC-04.7.** Three sites, all narrow: the bare `read_to_string` at `:1640` (`organize`) and `:1789` (`hydrate`), and the remedy string at `:667`. Line numbers verified at `c73404c7` -- they were `:1639`/`:1754`/`:666` at `105faa01`, so if you are working from the older numbers they have each moved by one.

**The change: an ABSENT manifest stops being reported as UNREADABLE.** `intentfiles::realised()` already models hv's rule completely -- `NothingSaid` for absent, `declares` failing open -- and both verbs bypass it. Absent means nobody has said, so `organize` proceeds and removes nothing. Unreadable keeps its refusal and keeps naming the path.

**It is my own regression from this afternoon**: `init` writes no manifest, so absence is now the shipped initial condition of every new v3 project, and `intent init` then `intent organize` is rc=1 -- the first two commands anybody types.

**I am NOT deciding `hydrate`'s shape.** hv already ruled the lifecycle verbs leave an absent manifest absent; the row constrains only that absence is not reported as unreadable. The rest is ours to design.

**I will commit with `git commit --only` on named paths AND verify in a DETACHED WORKTREE at the commit before I push anything anywhere.** I put ic's caller at HEAD without its callee yesterday doing exactly this in the shared tree; `--only` is path-scoped, not hunk-scoped, and the detached worktree is the only one of the three remedies that catches a stranger's hunk mechanically. If you have uncommitted work in `facade.rs`, say so and I will wait.
