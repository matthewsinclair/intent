# The 3.2.0 install on gyges, 2026-09-21

The clean-machine proof AC-00.5 and AC-11.1 of ST0056 waited on: a machine that had never seen this repository installs Intent from the tap over the network, and both binaries answer at the tag's commit. hv's terminal, verbatim, read by vc at 16:24Z; the tap was cloned fresh in this run and every artefact was fetched and verified from the published release.

```
matts@gyges  ~  ♥ 17:22
❯ brew install matthewsinclair/intent/intent
==> Auto-updating Homebrew...
==> Tapping matthewsinclair/intent
Cloning into '/opt/homebrew/Library/Taps/matthewsinclair/homebrew-intent'...
Tapped 1 formula (14 files, 32.4KB).
==> Trusted formula matthewsinclair/intent/intent
==> Would install 1 formula:
matthewsinclair/intent/intent 3.2.0
==> Fetching downloads for: intent
✔︎ Resource intent--support                                                                   Verified    412.6KB/412.6KB
✔︎ Resource intent--intentd                                                                   Verified     18.3MB/ 18.3MB
✔︎ Formula intent (3.2.0)                                                                     Verified     20.8MB/ 20.8MB
==> Installing intent from matthewsinclair/intent
🍺  /opt/homebrew/Cellar/intent/3.2.0: 190 files, 40.2MB, built in 3 seconds
==> Caveats
==> intent
Intent's pre-commit gate finds this install through ~/.local/share/intent/home, which
only `intent bootstrap` writes. If that file does not exist yet, run:
  intent bootstrap
Until it exists, every commit in a project with the gate installed is
refused.

To start matthewsinclair/intent/intent now and restart at login:
  brew services start matthewsinclair/intent/intent
Or, if you don't want/need a background service you can just run:
  /opt/homebrew/opt/intent/bin/intentd

matts@gyges  ~  ♥ 17:23
❯ intent bootstrap
note: moved skills to /Users/matts/.local/share/intent/skills, agents to /Users/matts/.local/share/intent/agents out of /Users/matts/.intent into the XDG layout, and removed it
created: install root recorded -- /opt/homebrew/Cellar/intent/3.2.0/libexec
created: /Users/matts/.config/intent/config.json
  author: matts
done: this machine is set up

matts@gyges  ~  ♥ 17:23
❯ intent --version && intent daemon start && intent daemon status
intent 3.2.0 (4e2f908a5857c4ac878aa8f071c5f09b60753993)
ok: intentd is answering at /Users/matts/.local/state/intent/run/intentd.sock
     logs: /Users/matts/.local/state/intent/intentd.log
ok: intentd is answering at /Users/matts/.local/state/intent/run/intentd.sock
note: its web face is at http://127.0.0.1:51737
note: it is running intentd 3.2.0 (4e2f908a5857c4ac878aa8f071c5f09b60753993), the commit the intentd beside this intent names
```

`intent explore` was then opened over ssh: its status bar read `Intent 3.2.0 (4e2f908a)` and its projects pane said no project is registered and named `intent discover <dir>`, the right answer on a machine that has never run it.

What the log does and does not show. It shows the network install path whole: the tap validates and clones, the formula and both resources are fetched and verified from the release, the keg installs, the caveat prints, bootstrap records the install root under the keg's libexec, and `intent`, `intentd` and the explorer answer at the tag. The machine was not bare: a `~/.intent` from an earlier Intent was found and moved into the XDG layout by bootstrap itself, and that is the move's own line above. `intent daemon stop` was not driven in this run.
