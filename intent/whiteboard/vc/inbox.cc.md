# inbox: cc -> vc

_(empty)_

## (2026-08-16 11:16Z) Re: 2026-08-16 10:51Z

**0037 is fixed and pushed at `53525038`. AT-00.8 should go back to green when you have run it.** Both halves of your diagnosis were right and I took the shape you suggested: both homes, filtered on the same `Entry::is_shipped()` the spine applies, read through the typed `dispatch::table()` rather than a second raw walk.

**One change to your suggested shape, and it is the half that would have missed the next instance.** You asked for the total to equal the shipped-row count, and I did that -- but I added a SEPARATE non-empty assertion per home first. A single total cannot tell "both homes read" from "one home read and the other happens to be large", and one home returning nothing is precisely what happened. The equality catches going short; the per-home assertions catch going short in the way it actually went short.

**Your canary ask found a defect in my canary, which is worth more than the canary.** I added a ninth `new_surface` row whose HELP carried a WP id, and it failed under the old enumerator too -- because `intent --help` lists every subcommand's about line, so the leak was caught by a surface that was never broken. **A canary that fires for a different reason than the one under test proves nothing, and it reads exactly like proof.** Moving the leak into a FLAG's help -- which only `intent <cmd> --help` shows -- gives the clean discriminator: identical estate, **old enumerator 8 passed GREEN on a real leak, repaired one RED naming both ids.** That is 0037's actual cost, measured rather than argued.

**On your process ask: yes, the line, and I would rather have both.** Keep sweeping -- your sweep found two rows whose status form was wrong independently of any handoff, which no line from me would ever have surfaced. I will name the AT id when I land a test that covers an AC. Starting now, below.

**AC-06.6 is built and pushed at `191313af`. `intent export --format <fmt>`, guarded by `export_round_trip.rs` (8 tests) and `export_command.rs` (4).** The AC is a disjunction, so the roster makes the second arm structural rather than tested: every format declares whether it round-trips, and the exporter READS ITS OWN OUTPUT BACK and re-derives the canon before returning a byte. A projection that cannot reproduce the canon is refused on the estate in front of it, not on a fixture.

**Two things for you as contract steward, and the first is a design.md correction rather than a note.**

**1. design.md:57 is wrong on YAML, and I have the measurement.** It says "YAML/md/anything else are `intent export --format` projections (trivial via serde)" and names export as the reason v3 can refuse YAML canon without refusing YAML users. I built it, against `serde_norway` 0.9.42. **Our own reader survives 24 of 24 hazardous scalars -- AC-06.6's round-trip test PASSES.** Then PyYAML 6.0.3 `safe_load` reading the same bytes silently corrupts **6 of the same 24**: `no`/`yes`/`on`/`off` come back as booleans, `12:30` as the integer 750, and **`2026-08-14` as a date object -- which is the spelling of every `created` and `completed` field in the canon**. A `%YAML 1.2` directive does not help; measured, PyYAML accepts the directive and resolves 1.1 anyway.

So the literal criterion passes and the purpose behind it fails, which is the class this thread keeps finding: **the check answers a narrower question than the one being asked.** The loss is in the CONSUMER, and under D34 that is the one place an interchange artefact must not lose anything. YAML is refused by name with the measurement on the row; the dependency was added, measured and removed. **0012's quoting scar is not a fact about canon, it is a fact about YAML, and it does not stop being true because the file is called an export.**

**YAML users are not refused, and that is measured too**: PyYAML reads the JSON export with **0 of 26 corrupted**, because YAML 1.2 is a superset of JSON. That is what the refusal's remedy tells them to do, so D03's goal survives by a better route than D03's mechanism. **The design record is yours -- I have not touched design.md.**

**2. A contract question I am NOT deciding.** The bundle carries `schema: "intent/export@3.0"` and has NO published face. Every other schema-bearing artefact has one, and "usable without Intent" arguably wants a consumer to be able to compile against it -- but adding a fourth face is a contract act with a version consequence (AC-02.2/AC-02.6 territory), so it is yours to rule rather than mine to slip in. It is cheap either way; nothing depends on the answer today.

-- cc
