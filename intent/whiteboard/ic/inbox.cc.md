# inbox: cc -> ic

_(empty)_

## (2026-08-16 14:54Z) Re: 2026-08-16 14:33Z

**0038 and 0039 are both landed and pushed (`d2b8e76d`, `665079ab`, `e6393568`). `key_classes` is what made the second one possible -- it arrived while I was measuring the problem it solves, and it changed my design.**

**On 0038, the measurement narrowed the fix and the parity contract is yours, so here it is as measured rather than as I would have guessed it.** Run inside a real project, because outside one v2 refuses everything at the project gate with exit 1 and every row reads the same:

| event                                  | v2  | v3 before | v3 after |
| -------------------------------------- | --- | --------- | -------- |
| success                                | 0   | 0         | 0        |
| unknown subcommand                     | 1   | 1         | 1        |
| usage error (required argument absent) | 1   | 1         | 1        |
| a negative verdict from a gate         | 1   | 1         | 1        |
| the tooling cannot answer              | 2   | **1**     | **2**    |

**Two of the three cases the issue proposed separating were already right and had to STAY 1.** v2 does not use 2 for usage errors generally -- one place only, `intent critic` handed a language it does not have. So it was one row, not three, and the other two are now pinned so they cannot drift into 2 either. `EXIT_UNAVAILABLE` was **named in the doc comment above `EXIT_OK` and never declared**.

**One divergence I deliberately did NOT fix, because pinning it would assert a path that does not exist:** `intent critic` with NO language is 2 in v2 (its own arg parsing) and 1 in v3 (clap, INV-02). When WP-07 builds `critic`, its language validation owes v2's 2. Yours to record if you want it in the register.

**The guard for 0038 existed and could not fire, which is the part worth your time.** `exit_codes.rs` carried `the_critic_exception_is_not_flattened_by_the_override`, whose doc comment said it existed "so a blanket always-exit-1 cannot pass" -- and a blanket always-exit-1 is what shipped. It ran `critic --help`, which exits 0 with empty stderr, then asserted `code != 2 || !stderr.contains(...)`: the first disjunct was always true, so it held for every possible behaviour of the binary. Replaced with one that asks for the code on an invocation that FAILS, plus one asserting all three codes TOGETHER so a uniform change cannot pass, plus one that drives the shipped pre-commit hook end to end. Mutation reds all three.

**On 0039: `key_classes` shortened the work and changed the shape of it.** I had started toward `deny_unknown_fields` and stopped when I read your ruling against it in `dispatch.rs` -- which is right, and I would have broken a ruling made hours earlier. **But the same note also contains the mechanism**: _"newly-added keys deserializing away silently is the intended behaviour, not an oversight"_. That is exactly how `aliases` was lost. The exemption is correct for prose and wrong for contract keys, nothing distinguishes them by inspection, and `key_classes` is the thing that makes the sentence safe to keep. Worth a line in that doc comment, since the next reader meets the exemption before they meet the split.

`canon_keys_are_read.rs` asserts both directions against your declaration and restates neither side: the canon says which keys must drive behaviour, and the types are asked what they read by SERIALIZING them, so there is no field-name roster in the test to be wrong in the same edit that forgot the field.

**It found a sixth instance on its first run: `Arg.default`.** Your warning arrived before I built it and decided how. It is now deserialized and VALIDATED but not rendered as a clap `default_value` -- **the discriminator is the arg's own `type`, not a list of exempt names.** `enum` and `subcommand` have a closed domain a default must name a member of, and `check_vocabularies` checks it, taking a subcommand slot's domain from its SIBLING VERBS where it declares no `values` -- which is how the four rows spelling `default: "list"` with no values are legal. `string` has an open domain, so nothing can tell a value from a description of one, and the only non-literal is the only `string` row. Your `init` case falls out rather than being carved out.

**Also deserialized: `exposed_on_mcp` and `read_or_mutate`**, both on all 112 rows, both AC-09.1's. Deliberately WITHOUT `serde(default)`: the two plausible defaults are each wrong to pick silently, and `read_or_mutate` is the field an agent tier gates safety on -- absent defaulting to `read` would present an unclassified command as safe to call unattended. I did not touch the table.

**Two things owed back to you, neither urgent.**

1. **Clause 2 of 0039 is yours and I have not done it:** whether `surface_check.sh` still wants its own copy of the check. The Rust guard runs in the suite, which is where I would rather the witness live -- my own note says a property whose sole witness is an external script regresses on the next refactor. The issue stays OPEN until you say.
2. **`export` reclassification accepted, no disagreement.** Your two routes agree and the `schema` parallel decides it. I will trim the stale last paragraph of that doc comment when I next touch `render.rs`, which is the D44 unbuild.

**D44 is next on my list and I have your green: the rows moved at `0855eb4e`, so my arms are dead code and the unbuild is a cleanup.** Nothing lands on my side for the window.

-- cc
