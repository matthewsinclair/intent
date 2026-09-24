#!/usr/bin/env python3
# explorer_shots.py -- the screenshots on docs/explorer.md, taken from the
# explorer itself.
#
# `intent explore` runs in a pty, pyte reads the screen it draws, and each
# screen is written out as an SVG for rsvg-convert to rasterise. It is driven
# by gen_explorer_shots.sh, which owns the scratch HOME and seeds the demo
# projects this reads; run that rather than this.
#
# WHY A PTY AND NOT A SCREEN GRAB. A grab needs a window on somebody's desktop,
# the Screen Recording permission and a person to arrange it, so it is taken
# once and never again, and the page drifts from the explorer it shows. A pty
# needs none of them: the size is fixed, the input is scripted, and the same
# command re-takes every shot at the next cut.
#
# THE VERSION AND THE COMMIT THE EXPLORER DRAWS ARE ELIDED AS `...`, the way a
# transcript in these docs elides them: a shot is published with a release it
# was not taken on. A shot that still shows either after elision is refused
# rather than written, so a new place the explorer draws them cannot slip into
# the docs unnoticed.
#
# Exit codes: 0 every shot written, 1 a shot could not be taken, 2 usage.

import argparse
import fcntl
import json
import os
import pty
import re
import select
import struct
import subprocess
import sys
import termios
import time

import pyte
from wcwidth import wcwidth

# Each shot is a starting command and the steps taken before the screen is
# read. A step is a key name from KEYS, `text:<chars>` to type, `wait:<secs>`,
# or `run:<json argv>` to run a command beside the explorer, as a second
# terminal would. The steps are keys a reader can press, and nothing else.
SHOTS = [
  {"name": "landing", "argv": ["intent", "explore"], "steps": []},
  {"name": "find", "argv": ["intent", "explore"], "steps": ["text:pay"]},
  {"name": "thread", "argv": ["intent", "explore", "ST0001"], "steps": ["DOWN"] * 5},
  {
    "name": "criteria",
    "argv": ["intent", "explore", "ST0001"],
    "steps": ["DOWN"] * 11 + ["ENTER"] + ["DOWN"] * 3,
  },
  {"name": "edit", "argv": ["intent", "explore", "ST0001"], "steps": ["ENTER", "text: online"]},
  {"name": "menu", "argv": ["intent", "explore"], "steps": ["/"]},
  {"name": "outstanding", "argv": ["intent", "explore"], "steps": ["/", "text:outstanding", "ENTER"]},
  {"name": "search", "argv": ["intent", "explore"], "steps": ["/", "text:search card", "ENTER", "wait:1.5"]},
  {"name": "projects", "argv": ["intent", "explore"], "steps": ["/", "text:projects", "ENTER"]},
  {"name": "help", "argv": ["intent", "explore"], "steps": ["/", "text:help", "ENTER"]},
  # LAST, because it writes: an issue added from outside while the list is on
  # screen, and no key pressed after it.
  {
    "name": "live",
    "argv": ["intent", "explore"],
    "steps": [
      "/",
      "text:issues",
      "ENTER",
      'run:["intent", "issues", "add", "The order page times out on a large basket", "--severity", "high"]',
      "wait:1.5",
    ],
  },
]

COLS, ROWS = 100, 30

KEYS = {
  "ESC": b"\x1b",
  "ENTER": b"\r",
  "TAB": b"\t",
  "UP": b"\x1b[A",
  "DOWN": b"\x1b[B",
  "RIGHT": b"\x1b[C",
  "LEFT": b"\x1b[D",
  "PGDN": b"\x1b[6~",
  "PGUP": b"\x1b[5~",
  "BS": b"\x7f",
  "/": b"/",
}

CTRL_C = b"\x03"


class Screen(pyte.Screen):
  """A pyte screen that answers the terminal's queries back into the pty.

  The explorer asks for the cursor position and the device attributes as it
  starts, and a terminal that never answers is refused.
  """

  def __init__(self, cols, rows, fd):
    super().__init__(cols, rows)
    self._fd = fd

  def write_process_input(self, data):
    os.write(self._fd, data.encode())

  # pyte has one screen buffer and ignores the switch to the alternate one. A
  # terminal clears the alternate screen as it enters it, and the explorer
  # draws its first frame on that promise, so without this a second session in
  # one pty (a project switch) paints over the last one's leftovers.
  def set_mode(self, *modes, **kwargs):
    if kwargs.get("private") and any(m in (47, 1047, 1049) for m in modes):
      self.erase_in_display(2)
      self.cursor_position()
    super().set_mode(*modes, **kwargs)


class Session:
  """One explorer, running in a pty of a fixed size."""

  def __init__(self, argv, env, cwd):
    pid, fd = pty.fork()
    if pid == 0:
      fcntl.ioctl(0, termios.TIOCSWINSZ, struct.pack("HHHH", ROWS, COLS, 0, 0))
      os.chdir(cwd)
      os.execvpe(argv[0], argv, env)
    self.pid, self.fd = pid, fd
    self.screen = Screen(COLS, ROWS, fd)
    self.stream = pyte.ByteStream(self.screen)

  def pump(self, timeout=4.0, quiet=0.4):
    """Read what the explorer draws until it has been quiet for `quiet` seconds."""
    end = time.time() + timeout
    last = time.time()
    while time.time() < end:
      ready, _, _ = select.select([self.fd], [], [], 0.05)
      if ready:
        try:
          data = os.read(self.fd, 65536)
        except OSError:
          return
        if not data:
          return
        self.stream.feed(data)
        last = time.time()
      elif time.time() - last >= quiet:
        return

  def alive(self):
    pid, _ = os.waitpid(self.pid, os.WNOHANG)
    return pid == 0

  def close(self):
    try:
      os.write(self.fd, CTRL_C)
      self.pump(timeout=1.0)
    except OSError:
      pass
    try:
      os.kill(self.pid, 9)
      os.waitpid(self.pid, 0)
    except (ProcessLookupError, ChildProcessError):
      pass


def step(sess, token, env, cwd):
  if token.startswith("text:"):
    os.write(sess.fd, token[len("text:"):].encode())
  elif token.startswith("wait:"):
    time.sleep(float(token[len("wait:"):]))
  elif token.startswith("run:"):
    argv = json.loads(token[len("run:"):])
    subprocess.run(argv, cwd=cwd, env=env, check=True, capture_output=True)
  elif token in KEYS:
    os.write(sess.fd, KEYS[token])
  else:
    raise ValueError(f"unknown step `{token}`")
  sess.pump()


# -- rendering ---------------------------------------------------------------

# pyte reports the sixteen ANSI colours as xterm's values, and each maps to a
# palette that reads on a dark page. Any other colour passes through.
PALETTE = {
  "000000": "#000000", "cd0000": "#cd3131", "00cd00": "#0dbc79", "cdcd00": "#e5e510",
  "0000ee": "#2472c8", "cd00cd": "#bc3fbc", "00cdcd": "#11a8cd", "e5e5e5": "#e5e5e5",
  "7f7f7f": "#808080", "ff0000": "#f14c4c", "00ff00": "#23d18b", "ffff00": "#f5f543",
  "5c5cff": "#3b8eea", "ff00ff": "#d670d6", "00ffff": "#29b8db", "ffffff": "#ffffff",
  "black": "#000000", "red": "#cd3131", "green": "#0dbc79", "brown": "#e5e510",
  "yellow": "#e5e510", "blue": "#2472c8", "magenta": "#bc3fbc", "cyan": "#11a8cd",
  "white": "#e5e5e5",
}
FG, BG = "#d4d4d4", "#1e1e1e"
FONT = "Menlo, Monaco, 'DejaVu Sans Mono', monospace"
SIZE, CW, CH, PAD = 14.0, 8.43, 17.0, 18.0

# What the explorer draws that names the build, and what each becomes.
ELIDE = [
  (r"🐢 Intent \d+\.\d+\.\d+\S* \([0-9a-f]{7,40}\)", "🐢 Intent ... (...)"),
  (r"Intent \d+\.\d+\.\d+\S* \([0-9a-f]{7,40}\)", "Intent ... (...)"),
]


def colour(value, default):
  if value == "default":
    return default
  if value in PALETTE:
    return PALETTE[value]
  if re.fullmatch(r"[0-9a-fA-F]{6}", value):
    return "#" + value.lower()
  return default


def cells(screen):
  """The screen as rows of [text, style, columns]; a wide glyph spans two."""
  grid = []
  for y in range(ROWS):
    line, x = [], 0
    while x < COLS:
      c = screen.buffer[y][x]
      wide = c.data != "" and x + 1 < COLS and screen.buffer[y][x + 1].data == ""
      fg, bg = colour(c.fg, FG), colour(c.bg, BG)
      if c.reverse:
        fg, bg = bg, fg
      line.append([c.data or " ", (fg, bg, c.bold, c.italics, c.underscore), 2 if wide else 1])
      x += 2 if wide else 1
    grid.append(line)
  return grid


def text_of(line):
  return "".join(cell[0] for cell in line)


def elide(grid):
  """Rewrite each ELIDE match in place, right-aligned inside the columns it took."""
  for y, line in enumerate(grid):
    for pattern, replacement in ELIDE:
      m = re.search(pattern, text_of(line))
      if not m:
        continue
      start, end = m.span()
      taken = sum(cell[2] for cell in line[start:end])
      style = line[end - 1][1]
      new = [[ch, style, 2 if wcwidth(ch) == 2 else 1] for ch in replacement]
      pad = taken - sum(cell[2] for cell in new)
      grid[y] = line = line[:start] + [[" ", line[start][1], 1]] * pad + new + line[end:]
  return grid


def svg(grid):
  width, height = PAD * 2 + COLS * CW, PAD * 2 + ROWS * CH
  out = [
    f'<svg xmlns="http://www.w3.org/2000/svg" width="{width:.0f}" height="{height:.0f}" viewBox="0 0 {width:.2f} {height:.2f}">',
    f'<rect width="100%" height="100%" rx="10" fill="{BG}"/>',
    f'<g font-family="{FONT}" font-size="{SIZE}">',
  ]
  for y, line in enumerate(grid):
    top = PAD + y * CH
    runs, x = [], 0
    for text, style, w in line:
      if runs and runs[-1][1] == style:
        runs[-1][0].append((x, text, w))
      else:
        runs.append([[(x, text, w)], style])
      x += w
    for chars, (fg, bg, bold, italic, under) in runs:
      x0, span = chars[0][0], sum(w for _, _, w in chars)
      if bg != BG:
        out.append(f'<rect x="{PAD + x0 * CW:.2f}" y="{top:.2f}" width="{span * CW:.2f}" height="{CH:.2f}" fill="{bg}"/>')
      if all(t == " " for _, t, _ in chars):
        continue
      xs = " ".join(f"{PAD + cx * CW:.2f}" for cx, _, _ in chars)
      body = "".join(t for _, t, _ in chars).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")
      attrs = f'fill="{fg}"'
      attrs += ' font-weight="bold"' if bold else ""
      attrs += ' font-style="italic"' if italic else ""
      attrs += ' text-decoration="underline"' if under else ""
      out.append(f'<text x="{xs}" y="{top + CH * 0.76:.2f}" {attrs} xml:space="preserve">{body}</text>')
  out += ["</g>", "</svg>"]
  return "\n".join(out)


def take(shot, env, cwd, out, version, commit):
  sess = Session(shot["argv"], env, cwd)
  try:
    sess.pump(timeout=8.0, quiet=1.0)
    for token in shot["steps"]:
      step(sess, token, env, cwd)
    if not sess.alive():
      return f"the explorer exited before `{shot['name']}` could be read"
    grid = elide(cells(sess.screen))
    shown = "\n".join(text_of(line) for line in grid)
    if not shown.strip():
      return f"`{shot['name']}` read a blank screen"
    for label, needle in (("version", version), ("commit", commit[:7])):
      if needle and needle in shown:
        return f"`{shot['name']}` still shows the {label} `{needle}` after elision -- add a pattern to ELIDE for where the explorer draws it"
    with open(os.path.join(out, shot["name"] + ".svg"), "w") as fh:
      fh.write(svg(grid))
    return None
  finally:
    sess.close()


def main():
  ap = argparse.ArgumentParser(description="Take the explorer's screenshots as SVG.")
  ap.add_argument("--cwd", required=True, help="the demo project to run the explorer in")
  ap.add_argument("--out", required=True, help="the directory the SVGs are written to")
  ap.add_argument("--version", required=True, help="the version `intent --version` names, refused in any shot")
  ap.add_argument("--commit", required=True, help="the commit `intent --version` names, refused in any shot")
  ap.add_argument("--only", default="", help="comma-separated shot names; default every shot")
  args = ap.parse_args()

  wanted = [s for s in args.only.split(",") if s]
  unknown = sorted(set(wanted) - {s["name"] for s in SHOTS})
  if unknown:
    print(f"error: no shot named {', '.join(unknown)}", file=sys.stderr)
    return 2
  env = dict(os.environ)
  env.update({"TERM": "xterm-256color", "COLORTERM": "truecolor", "LANG": "en_US.UTF-8"})
  for shot in SHOTS:
    if wanted and shot["name"] not in wanted:
      continue
    why = take(shot, env, args.cwd, args.out, args.version, args.commit)
    if why:
      print(f"error: {why}", file=sys.stderr)
      return 1
    print(f"shot: {shot['name']}")
  return 0


if __name__ == "__main__":
  sys.exit(main())
