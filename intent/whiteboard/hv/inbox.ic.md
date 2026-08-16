# inbox: ic -> hv

_(empty)_

## (2026-08-15 09:12Z) -- the repo is PUBLIC and the whiteboard protocol mandates publishing a session identifier. Your call, not ours.

**Verified independently** (`gh repo view --json visibility,isPrivate` -> `{"isPrivate":false,"visibility":"PUBLIC"}`, `matthewsinclair/intent`). dc found it, vc re-ran it, I re-ran it. The machine's environment brief says "assume private", which is wrong **in the dangerous direction** -- it understates the blast radius.

**60 whiteboard files are tracked**, so every board, inbox and archived inbox is world-readable the moment it lands on `upstream`. 20 of those are mine.

**I scanned my own 20 and there are no credentials.** For the record, so you are not taking that on trust: four credential-shaped hits, all the word "token" in the parse-token sense; one `/Users/matts` in an archive; and **three Claude Code session UUIDs**.

**THE ONE THING I WANT A RULING ON, because it is structural rather than a slip:** `session_id` is a **required field in the whiteboard header block**, and peers compare it to decide which nodes are live. So the protocol _requires_ publishing a session identifier, and every node in every project that adopts the whiteboard does the same thing. I have not touched mine, because stripping it breaks `pickup` and because changing a protocol to fix a disclosure question is your decision rather than a node's.

Risk as I read it is low -- a local Claude Code session UUID grants no remote access and identifies nothing outside this machine -- so this is "you should know the protocol does this", not an alarm.

**The larger question, which I think is yours and which I do not think should be settled between nodes:** whether a public repository should carry this volume of unedited working transcript. vc's position is that we should not change how we write, and I agree with the substance -- the candour is precisely what has been catching things all morning, and a sanitised board would have caught none of it. But that is a judgement about what you want public under your own name, and the nodes agreeing among themselves is not the same as you deciding. Putting it to you rather than reporting it as settled.

**Operationally it raises the cost of one existing hazard rather than creating a new one.** The dirty shared index has been a "somebody lands a formatting regression" problem all morning; on a public repo with four sessions on one worktree it is a **publication into a history nobody can rewrite.** dc owns it and is on it. The standing `--only` rule already covers it; what changes is the price of forgetting.

FYI on my own state, no response needed: `st bootstrap` landed and cc is wiring it; both guards are built, green and with dc for pre-commit wiring.

## (2026-08-16 10:18Z) FYI only -- no response needed. ANNOUNCE: NO MORE PUSHES TO `upstream`. hv, just now: the CI/CD budget has been hit.

**`local` is fine. `upstream` is not.** Every commit stops at the Dropbox remote from here.

```
git push local main        # yes
git push upstream main     # NO
git push                   # NO -- may carry a configured upstream without you naming it
```

**Passing it on immediately rather than only recording it, because all four of us have been pushing to both remotes all session and the next push is the one that costs.** It reached me mid-fold; I have not pushed upstream since `dcf9d3b6`, which is the last commit of mine that reached GitHub.

**Treating it as STANDING, not a one-session pause** -- it is on my board under watch-outs rather than in DOING, so it survives my compact. If hv lifts it, that is hv's to say.

-- ic
