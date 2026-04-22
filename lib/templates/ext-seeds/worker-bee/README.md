# worker-bee (ext seed)

This directory is a **seed**, not a runtime extension. It is the source-of-truth copy of the `worker-bee` subagent that ships with Intent v2.9.0 and later.

## What happens at upgrade

`intent upgrade` (v2.8.2 -> v2.9.0 migration) copies this directory to `~/.intent/ext/worker-bee/` on first run. After that, the seed is untouched: further development of worker-bee happens at the user-local path, not here.

- Seed path (this directory): `lib/templates/ext-seeds/worker-bee/`
- Runtime path (per-user, copied by migration): `~/.intent/ext/worker-bee/`
- Discovery: once copied, `intent claude subagents list` shows worker-bee with a `[ext:worker-bee]` tag.

## Why worker-bee is an ext, not canon

Worker-Bee Driven Design is domain-specific scaffolding for Elixir applications. It is not part of Intent's core machinery. ST0034 extracted it from canon to prove that the `~/.intent/ext/` extension mechanism can host a real, non-trivial subagent end-to-end. If worker-bee can live here, any user-authored extension can.

## Structure

```
lib/templates/ext-seeds/worker-bee/
  extension.json                 # manifest, conforms to intent-extension/v1
  README.md                      # this file
  subagents/
    worker-bee/
      agent.md                   # the subagent itself
      metadata.json              # tool permissions + tags
      resources/                 # templates, validation rules, mix tasks
        README.md
        USER_GUIDE.md
        config/
        lib/
        templates/
        validation/
```

## Further reading

- `intent/docs/writing-extensions.md` uses this seed as the worked example for authoring extensions.
- `subagents/worker-bee/resources/USER_GUIDE.md` is worker-bee's own documentation.
- `subagents/worker-bee/resources/README.md` describes the WDD methodology.
