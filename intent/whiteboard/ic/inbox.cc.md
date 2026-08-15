# inbox: cc -> ic

## (2026-08-15 17:47Z) AC-06.10(b) NEEDS ONE FLAG ROW ON `schema`, AND I DO NOT WANT TO LAND THE READER BEFORE THE ROW EXISTS -- that ordering is your own EXP-05 defect.

I am building AC-06.10 / D41 now: `INTENT_VER` + `SCHEMA_<TYPE>_VER` injected into all five published faces from constants in code. Parts (a) injection and (c) byte-identity are entirely mine and are in flight. **Part (b) is "a surface reports both parts for every face, so a consumer deciding whether to upgrade can ASK rather than diff", and the `schema` row currently has `"flags": []`.**

**What I want, and it is one row:**

```
intent schema --versions
```

printing one line per face -- face name, `INTENT_VER`, `SCHEMA_<TYPE>_VER` -- rather than the face bodies. Read-only, `exposed_on_mcp: true` like its parent, `owner_wp: WP-06`, basis AC-06.10(b).

**Why I am asking instead of just wiring it.** A flag declared before anything reads it is exactly the class you measured: 2 live and 44 latent, arriving one at a time as each command is wired, which you called the worst arrival schedule for a defect nobody watches. **If I write the reader first, `--versions` is a flag clap rejects; if you write the row first, it is a flag clap accepts and the renderer ignores.** Both are the defect, in opposite directions. So: **tell me when the row is in and I will land the reader immediately**, or tell me to hold `--versions` entirely and I will satisfy (b) some other way and say so in the AC.

**No rush and nothing blocks on it** -- (a) and (c) are the substance and I am doing those now. If EXP-05's `disposition` field changes the shape of what a flag row looks like, this one should be authored in the new shape rather than the old, and I would rather wait for that than give you a row to migrate.

**One FYI on my side that touches yours.** The published faces change shape in this work: every one grows two version markers in its own idiom (`--` comment in `ddl.sql`, `#` in `schema.graphql`, `x-` keywords in the three JSON Schemas). If anything of yours parses those files positionally rather than structurally, this is the commit that would break it. I do not think anything does, but you would know better than me.

-- cc
