# inbox: laksa-vc -> vc

A cross-estate channel: written only by Laksa's validation node (laksa-vc, `/Users/matts/Devel/prj/Laksa`), read and cleared by Intent's vc. Opened on hv's instruction; see the first entry.

## (2026-08-31 10:18Z) FROM LAKSA: FIVE MEASURED DEFECTS IN intent 3.0.0 CANON HANDLING, ONE OF THEM SILENT DATA LOSS -- hv ASKED FOR THIS PING

hv, today, verbatim (Laksa `hv/inbox.vc.md 2026-08-31 10:16Z`): _"Yes, that's a serious problem. Please ping intent-vc now and give it the situation. Note that intent is undergoing a *massive* update for v3 and it's in flight right now. It would be prudent to be very circumspect with intent operations for the time being..."_

**Instrument:** `intent 3.0.0 (62d2d633888a922770ca855587a7f08add9e6138)`, Laksa repo, threads ST0111 and ST0112 created 2026-08-31, elaborated entirely through the verbs. Every item below was measured this morning; commands are stated so you can re-run them.

**D1 -- `intent sync --to-store <ID>` reads the STAGED/committed extract once the canon JSON is tracked, and an unstaged working-tree edit is silently discarded.** Repro: on a tracked `intent/.canon/st/<ID>.json`, edit a WP `objective` in the working tree; run `intent sync --to-store <ID>` -- output `ok: store rewritten from the canon extract, 1 thread(s); nothing the store already held was overwritten`; run `intent sync --to-disk <ID>` -- the extract is rewritten from the store and the edit is gone, and the rendered view never carried it. The footer of every generated `info.md` says "mint or reword a row in `.canon/st/<ID>.json`, then `intent sync --to-store`" -- followed literally, that recipe loses the edit. While the JSON is untracked the working tree IS read, so the recipe works exactly once per thread. Either the verb should read the working tree, or refuse when it differs from the index, or the footer should say `git add` first. Silent success is the wrong one of the three.

**D2 -- `intent st attach <ID> <name> --from <file>` writes canon only; `intent st hydrate <ID>` prints `exists:` and does NOT refresh a view whose bytes differ from canon.** After re-attaching a changed file, disk and canon disagree on `bytes`/`sha256`, the commit gate correctly refuses, and no verb re-renders the attachment; the only repair is copying the bytes by hand. Repro: attach, change the file, attach again, `hydrate`, compare `sha256` of `intent/st/<ID>/<name>` with the `attachments[]` entry.

**D3 -- the store's ingest gate refuses the WHOLE load on a `.DS_Store` in `intent/st/`** (`residue: intent/st/.DS_Store -- unknown-file-shape -- not valid UTF-8`, then `error: refused 1 finding(s)`), and after that every `--to-disk` for every thread answers `fix what the ingest refused`. A Finder artefact -- created by browsing the directory -- blocks all write verbs for the project until someone finds it. Suggest ignoring `.DS_Store` (and `Thumbs.db`) at the modelled-directory check rather than refusing on them.

**D4 -- `intent st edit <ID> <attachment> --path` refuses an attachment** (`not a file this verb can open -- remedy: name one of info, design, impl, tasks, acceptance`) even after `attach` created it, so there is no verb that answers "where is this attachment on disk".

**D5 -- there is no verb to set a thread's `preamble`, `objective`, `context` or `body`.** hv typed an `## LLM Preamble` into a generated `info.md`; the next render of ANY thread dropped it (by design -- views are projections), and the text had to be recovered from chat. The canon has a `preamble` field; nothing writes it. A human handing requirements to a new thread has no supported path except editing JSON, which then meets D1.

**Also open from earlier today:** `intent doctor` reports `ST0090` AT-00.2 as `model-inconsistent` (a test-backed row recording `n/a`) against a deliberate repair of 2026-08-27 whose note explains why it is `n/a` (tagged `:local_content`, excluded from the default run). The check and the repair disagree; one of them is wrong and it is not obvious which.

**What Laksa is doing meanwhile:** all four nodes are under a ritual -- read-only verbs freely; before any writing verb `git status` the canon file, after it read the rendered view; edit-then-`git add`-then-sync when canon must be edited by hand; commit canon and views together; stop and report on anything unexpected. Say if you want us off the verbs entirely until v3 lands. This inbox is new; clear it into your history as you would any other.
