# The gate shim -- resolution contract

**Status:** contract, written before any code, on hv's plan (R1+R2). Implementation is cc's.

## The defect this closes

`lib/templates/hooks/` holds TWO propagation classes and nothing on the page says which is which:

- **`pre-commit.sh` is THE GATE, and it is COPIED** into each estate as `pre-commit.intent` at install. It is stale in all 17: three generations in the field (G1 2, G2 14, G3 1), and the copy in Intent's own `.githooks` was six days and 59 lines behind its own template.
- **The guard bodies and the runner are READ LIVE** at run time out of `$GUARD_HOME/lib/templates/hooks/`. They have never drifted, because they cannot.

So a guard fix reaches the whole fleet the moment it lands, and a gate fix reaches nobody -- which is why `4d9e70c2` was installed in zero estates including the tree that wrote it. **The gate does not obey its own architecture. That is the whole defect.**

## R1 -- the gate becomes a shim

What is installed per-estate stops being 25KB of logic and becomes a generated shim whose only job is to locate the real gate and exec it. **One gate body, one version, no generations, ever.** Baize and Conflab -- currently running no guards at all and saying nothing -- are fixed by the same act rather than as a separate task.

## R2 -- the shim does not ask the binary where the gate lives

Every absence branch in today's gate exists because it runs `intent info` to resolve `INTENT_HOME`. Shadow the binary or catch it mid-rebuild and the gate goes blind, which is why three instruments grew three different opinions about what blindness should do.

**`$INTENT_HOME` MUST NOT BE READ FROM THE ENVIRONMENT.** `install.rs:20-30` refuses it deliberately -- a machine mid-rollout carries a stale v2 value -- and a shim that honoured an env var the binary refuses would put two answers to one question back into the estate, which is the class this whole contract exists to remove.

### The resolution, in full

1. The shim reads **`~/.intent/home`**: one line, the absolute path of the install root. Nothing else, no parsing, no fallback chain.
2. It asserts the path carries the install marker -- `lib/templates` is a directory under it. This is the same predicate as `install::is_install()`, and it is what makes a STALE pointer fail loudly instead of resolving to a plausible wrong tree.
3. It execs `$root/lib/templates/hooks/pre-commit.sh`, passing argv through.

### Where `~/.intent/home` comes from

**The installer writes it, from `install::home()`.** That function stays the single computation of where Intent is installed; the file is its PUBLISHED OUTPUT, refreshed on every install and every `local build`. **One source, one cache, and the source writes the cache** -- so the two cannot disagree without the writer being wrong, which is a different and much smaller surface than two independent resolvers.

This is also what makes R3 -- moving `intent` out of the build tree -- **a one-line change to one file rather than a second fleet sweep.** That property is the reason for the indirection and should not be traded away for the marginally simpler stamped-path variant.

### What happens when it cannot resolve

Two failures, and BOTH REFUSE, because both are actionable by the person being blocked right now -- dc's rule, and here it is uncontested because there is no self-resolving case left:

- **`~/.intent/home` absent or empty** -- the install never completed. Refuse, naming the file and the command that writes it.
- **the path does not carry `lib/templates`** -- the install moved or was deleted. Refuse, quoting the path read so the operator sees WHAT it pointed at rather than being told it was wrong.

**Never skip. Never fall through.** A shim that cannot find the gate has not run the gate, and the one thing this whole family taught us is that a skip is indistinguishable from a pass to everything downstream.

### What this DISSOLVES rather than answers

The ordering question hv is holding -- three instruments, three answers -- **exists only because the guard arm and the critic arm shared one resolution.** Under this contract:

- **the guards run with no binary at all**, so ABSENCE 1, its fall-through, and the inversion have no subject;
- **the critic arm still needs the binary and refuses when it is missing** (`4d9e70c2`), now obviously correct because it is the ONLY arm that depends on it.

**The asymmetry was never a policy disagreement. It was one dependency shared by two arms that need different things.** Split the dependency and each answer is self-evident.

### The rebuild window, which stops mattering

`~/.local/bin/intent` is a symlink into `native/rust/target/release/`, so a release build genuinely removes the binary for the length of the build (met for real: a probe returned `no such file or directory` mid-build). Under this contract that window no longer blinds the guards, because the guards never needed the binary. It still stops the critic arm, which refuses -- correctly, and for a reason the operator can act on.

## Sequencing

R1 and R2 land as ONE change. R2 is what makes R1 safe to install everywhere: shipping the shim without fixing resolution would put a binary dependency in front of every commit in 17 estates.

Then **one reinstall sweep, and it is the last one anyone runs**, because after it the gate body is live like the guards.

## What must not be built

- No `$INTENT_HOME` env override, in the shim or anywhere else.
- No second resolver, no fallback chain, no "try the binary if the file is missing". A fallback is how the two propagation classes got here.
- No auto-repair of a stale `~/.intent/home`. Refuse and name it; a shim that silently rewrites its own pointer hides the day the install moved.
