#!/usr/bin/env bash
# gen_explorer_shots.sh -- the screenshots on docs/explorer.md, taken from the
# explorer itself.
#
# It seeds two demo projects under a short HOME of their own, runs `intent
# explore` in a pty for each shot (explorer_shots.py, which declares the shots),
# and rasterises the SVGs it writes with rsvg-convert. Re-run it when the
# explorer's screens change, and at a cut, and commit the PNGs with the page.
#
# THE HOME IS FIXED RATHER THAN MADE BY mktemp, because `/projects` prints each
# project's absolute path: a random directory would put a different path into
# that shot on every run. An existing one is refused rather than reused, since
# it is either a run in progress or one kept with --keep.
#
# THE VENV STEP, once per machine, because pyte is what reads the screen:
#
#   python3 -m venv <dir> && <dir>/bin/pip install pyte
#
# then pass `--python <dir>/bin/python`. rsvg-convert comes from librsvg, and
# ImageMagick's `magick` reduces each PNG to a 64-colour palette: a terminal
# draws in a handful of colours, and the result reads the same at a quarter of
# the size.
#
# Exit codes: 0 the shots are written, 1 a step failed, 2 usage or a missing
# prerequisite.

# inputs: intent/st/ST0056/parity/tools/explorer_shots.py
# inputs-exempt: INTENT -- the explorer is driven, not read: the shots show whichever `intent` is on PATH, which `bin/devbin build all` builds from this tree
# inputs-exempt: PYTHON -- a scratch venv holding pyte, made by the venv step above; it reads the screen and adds nothing to it

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"

PYTHON="python3"
OUTDIR="$ROOT/docs/images/explorer"
KEEP=""
DEMO="/tmp/intent-demo"

usage() {
  cat >&2 <<'USAGE'
usage: gen_explorer_shots.sh [--python <path>] [--out <dir>] [--keep]

  --python  a python3 that can import pyte (default: python3)
  --out     write the PNGs here (default: docs/images/explorer)
  --keep    leave the demo HOME at /tmp/intent-demo for a look afterwards

Seeds the demo projects, takes every shot explorer_shots.py declares, and
writes one PNG per shot. The version and commit the explorer draws are elided.
USAGE
  exit 2
}

die() {
  echo "error: $1" >&2
  echo "  remedy: $2" >&2
  exit 2
}

while [ $# -gt 0 ]; do
  case "$1" in
    --python) PYTHON="${2:?--python needs a path}"; shift 2 ;;
    --out)    OUTDIR="${2:?--out needs a directory}"; shift 2 ;;
    --keep)   KEEP=1; shift ;;
    -h|--help) usage ;;
    *)        echo "error: unknown argument \`$1\`" >&2; usage ;;
  esac
done

command -v intent >/dev/null || die "no \`intent\` on PATH" "run \`bin/devbin build all\`, which links the pair this tree builds"
command -v rsvg-convert >/dev/null || die "no rsvg-convert on PATH" "install librsvg (\`brew install librsvg\`)"
command -v magick >/dev/null || die "no magick on PATH" "install ImageMagick (\`brew install imagemagick\`)"
command -v jq >/dev/null || die "no jq on PATH" "install jq (\`brew install jq\`)"
"$PYTHON" -c 'import pyte, wcwidth' 2>/dev/null \
  || die "\`$PYTHON\` cannot import pyte" "python3 -m venv <dir> && <dir>/bin/pip install pyte, then pass --python <dir>/bin/python"
[ ! -e "$DEMO" ] \
  || die "$DEMO already exists" "a run is in progress, or one was kept with --keep: remove it with \`rm -rf $DEMO\` once nothing is using it"

# `intent 3.2.0 (<sha>) dev` -- the two things every shot must not show.
read -r _ VERSION COMMIT _ <<EOF
$(intent --version)
EOF
COMMIT="${COMMIT#(}"
COMMIT="${COMMIT%)}"
if [ -z "$VERSION" ] || [ -z "$COMMIT" ]; then
  echo "error: \`intent --version\` did not name a version and a commit, so no shot can be checked for them" >&2
  exit 1
fi

TMP="$(mktemp -d)"
mkdir "$DEMO"
cleanup() {
  rm -rf "$TMP"
  [ -n "$KEEP" ] || rm -rf "$DEMO"
}
trap cleanup EXIT

export HOME="$DEMO"
export XDG_CONFIG_HOME="$DEMO/.config" XDG_DATA_HOME="$DEMO/.local/share"
export XDG_STATE_HOME="$DEMO/.local/state" XDG_CACHE_HOME="$DEMO/.cache"
export XDG_RUNTIME_DIR="$DEMO/run"
mkdir -p "$XDG_RUNTIME_DIR" "$TMP/svg"
LOG="$TMP/seed.log"
PROSE="$TMP/prose.md"

# Write PROSE from stdin, then set one field from it.
set_from() {
  cat > "$PROSE"
  intent set "$1" "$2" --from "$PROSE" >> "$LOG"
}

# Every command below is one a reader could type. The test files exist
# because `at green` refuses a row whose file is missing or does not carry
# the row's id.
seed_shopfront() {
  mkdir "$DEMO/shopfront"
  cd "$DEMO/shopfront"
  git init -q
  intent init shopfront >> "$LOG" 2>&1
  jq '.author = "Demo"' intent/.config/config.json > "$TMP/config.json"
  cp "$TMP/config.json" intent/.config/config.json

  intent st new "Accept card payments" --start >> "$LOG"
  set_from ST0001 objective <<'TXT'
Take card payments at checkout without the card number ever reaching our servers.

The payment form hands the card to the processor and gets back a token; everything after that works on the token alone. An order exists only once a charge has been captured, and every captured charge ends in exactly one receipt.
TXT
  set_from ST0001 context <<'TXT'
Today checkout takes orders on account only. Card payments are the most requested feature from the shop's customers, and the processor's hosted form keeps us out of card-data scope.
TXT
  intent wp new ST0001 "Tokenise the card in the payment form" >> "$LOG"
  intent wp new ST0001 "Charge, capture and record the order" >> "$LOG"
  intent wp new ST0001 "Email a receipt" >> "$LOG"
  set_from intent:///threads/ST0001/wp/01 objective <<'TXT'
The processor's hosted field takes the card number and hands back a token, so the number never reaches our servers.
TXT
  set_from intent:///threads/ST0001/wp/02 objective <<'TXT'
Charge the token, capture on success, and write the order only once the capture is confirmed.
TXT
  set_from intent:///threads/ST0001/wp/03 objective <<'TXT'
Send one receipt per captured charge, even when the processor calls back twice.
TXT
  intent wp start ST0001/01 >> "$LOG"

  intent ac new ST0001 AC-00.1 --text "The payment page names the processor that handles the card" >> "$LOG"
  intent ac new ST0001 AC-01.1 --kind test --text "The card number never reaches our servers: the form posts a token" >> "$LOG"
  intent ac new ST0001 AC-02.1 --kind test --text "A card the processor declines leaves no order behind" >> "$LOG"
  intent ac new ST0001 AC-02.2 --kind test --text "A captured charge is recorded as exactly one order" >> "$LOG"
  intent ac new ST0001 AC-03.1 --kind test --text "A captured charge sends exactly one receipt" >> "$LOG"
  mkdir -p test/payments
  printf '# AT-01.1: the payment form posts a token, never the card number\n' > test/payments/tokenise_test.exs
  printf '# AT-02.1: a declined card leaves no order behind\n' > test/payments/decline_test.exs
  printf '# AT-02.2: a captured charge is recorded as exactly one order\n' > test/payments/capture_test.exs
  intent at new ST0001 AT-01.1 --covers AC-01.1 --file test/payments/tokenise_test.exs >> "$LOG"
  intent at new ST0001 AT-02.1 --covers AC-02.1 --file test/payments/decline_test.exs >> "$LOG"
  intent at new ST0001 AT-02.2 --covers AC-02.2 --file test/payments/capture_test.exs >> "$LOG"
  intent at red ST0001 AT-01.1 >> "$LOG"
  intent at green ST0001 AT-01.1 >> "$LOG"
  intent at red ST0001 AT-02.1 >> "$LOG"
  intent at green ST0001 AT-02.1 >> "$LOG"
  intent at red ST0001 AT-02.2 --note "fails when the processor sends the capture callback twice" >> "$LOG"
  intent ac satisfy ST0001 AC-00.1 --evidence "Checked on the staging payment page" >> "$LOG"
  intent wp done ST0001/01 >> "$LOG"
  intent wp start ST0001/02 >> "$LOG"

  intent st new "Refunds from the order page" >> "$LOG"
  set_from ST0002 objective <<'TXT'
Let staff refund all or part of a captured charge from the order page, and record who refunded what and why.
TXT

  intent st new "Rate-limit the public API" --start >> "$LOG"
  set_from ST0003 objective <<'TXT'
Stop one API key from starving the others: every key gets a token bucket, and a request over its limit is answered 429 with a Retry-After header.
TXT
  intent wp new ST0003 "A token bucket per API key" >> "$LOG"
  set_from intent:///threads/ST0003/wp/01 objective <<'TXT'
Keep one bucket per key, refill it at the key's rate, and refuse a request that finds it empty.
TXT
  intent wp start ST0003/01 >> "$LOG"

  intent issues add "The pay button submits twice on a slow connection" --severity high \
    --body "On a throttled connection a second click on Pay starts a second charge before the first returns." >> "$LOG"
  intent issues add "The refund window is fixed at 30 days" --severity medium \
    --body "Some products need a longer window, and the 30 is a literal in the order service." >> "$LOG"
  intent issues add "Receipt emails show the currency symbol twice" --severity low \
    --body "The template prints the symbol and the formatter adds another." >> "$LOG"
}

# A second project, so `/projects` has somewhere to go.
seed_warehouse() {
  mkdir "$DEMO/warehouse"
  cd "$DEMO/warehouse"
  git init -q
  intent init warehouse >> "$LOG" 2>&1
  intent st new "Count stock by bin" --start >> "$LOG"
}

seed_shopfront
seed_warehouse
intent discover "$DEMO" >> "$LOG"

"$PYTHON" "$HERE/explorer_shots.py" --cwd "$DEMO/shopfront" --out "$TMP/svg" \
  --version "$VERSION" --commit "$COMMIT"

mkdir -p "$OUTDIR"
for svg in "$TMP"/svg/*.svg; do
  png="$OUTDIR/$(basename "$svg" .svg).png"
  rsvg-convert --zoom 2 "$svg" -o "$TMP/full.png"
  magick "$TMP/full.png" +dither -colors 64 -strip -define png:compression-level=9 "$png"
done
echo "ok: wrote the explorer's shots to ${OUTDIR#"$ROOT"/}"
