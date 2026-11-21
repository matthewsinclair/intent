#!/usr/bin/env bash
# gen_explorer_shots.sh -- the screenshots on docs/explorer.md, taken from the
# explorer itself.
#
# It seeds two demo projects under a short HOME of their own, runs `intent
# explore` in a pty for each shot (the `explorer_shots` example of intent-cli,
# which declares the shots), and rasterises the SVGs it writes with
# rsvg-convert. Re-run it when the explorer's screens change, and at a cut, and
# commit the PNGs with the page.
#
# THE HOME IS FIXED RATHER THAN MADE BY mktemp, because `/projects` prints each
# project's absolute path: a random directory would put a different path into
# that shot on every run. An existing one is refused rather than reused, since
# it is either a run in progress or one kept with --keep.
#
# THE SCREEN IS READ IN RUST: the example reads it through the vt100 crate,
# and this builds it with cargo before it seeds anything, because cargo finds
# its own home through HOME. rsvg-convert comes from librsvg, and
# ImageMagick's `magick` reduces each PNG to a 64-colour palette: a terminal
# draws in a handful of colours, and the result reads the same at a quarter of
# the size.
#
# THE SEED'S DATES ARE DRAWN AS ONE FIXED DATE (issue 0623). The demo projects
# are seeded on the day of the run, and no verb sets a thread's created date,
# so the dates that day can stamp, in UTC and in local time, are read on both
# sides of seeding and passed with --seed-date. Two runs over an unchanged
# explorer then write byte-identical files on any day.
#
# Exit codes: 0 the shots are written, 1 a step failed, 2 usage or a missing
# prerequisite.

# inputs: native/rust/crates/intent-cli/examples/explorer_shots.rs native/rust/Cargo.lock
# inputs-exempt: INTENT -- the explorer is driven, not read: the shots show whichever `intent` is on PATH, which `bin/devbin build all` builds from this tree

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"

OUTDIR="$ROOT/docs/images/explorer"
KEEP=""
DEMO="/tmp/intent-demo"

usage() {
  cat >&2 <<'USAGE'
usage: gen_explorer_shots.sh [--out <dir>] [--keep]

  --out     write the PNGs here (default: docs/images/explorer)
  --keep    leave the demo HOME at /tmp/intent-demo for a look afterwards

Seeds the demo projects, takes every shot the explorer_shots example declares,
and writes one PNG per shot. The version and commit the explorer draws are
elided, and the seed's dates are drawn as one fixed date.
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
command -v cargo >/dev/null || die "no cargo on PATH" "install the Rust toolchain (https://rustup.rs), which builds the explorer_shots example"
[ ! -e "$DEMO" ] \
  || die "$DEMO already exists" "a run is in progress, or one was kept with --keep: remove it with \`rm -rf $DEMO\` once nothing is using it"

# `intent 3.2.7 (<tree>, <sha12>[+dirty]) dev` (ST0087 WP-01, issue 0647) -- the
# version and the commit are the two things every shot must not show.
LINE="$(intent --version)"
read -r _ VERSION _ <<EOF
$LINE
EOF
COMMIT="${LINE#*(*, }"
COMMIT="${COMMIT%%)*}"
COMMIT="${COMMIT%+dirty}"
[ "$COMMIT" != "$LINE" ] || COMMIT=""
if [ -z "$VERSION" ] || [ -z "$COMMIT" ]; then
  echo "error: \`intent --version\` did not name a version and a commit, so no shot can be checked for them" >&2
  exit 1
fi

# built_executable <target> <what> <cargo build args...> -- build one target,
# --quiet and --locked because a run of this must never rewrite the lock, and
# print the executable cargo reports for it. A failed build, or one that names
# no executable, refuses: the one reading of a cargo build this script makes.
built_executable() {
  local target="$1" what="$2" path
  shift 2
  path="$(cargo build --quiet --locked --manifest-path "$ROOT/native/rust/Cargo.toml" "$@" \
    --message-format=json-render-diagnostics \
    | jq -r --arg t "$target" \
      'select(.reason == "compiler-artifact" and .target.name == $t and .executable != null) | .executable')" \
    || { echo "error: cargo could not build $what (its diagnostics are above)" >&2; exit 1; }
  if [ -z "$path" ] || [ ! -x "$path" ]; then
    echo "error: cargo built no $what" >&2
    exit 1
  fi
  printf '%s\n' "$path"
}

# Built before HOME moves.
SHOTS="$(built_executable explorer_shots "the explorer_shots example" -p intent-cli --example explorer_shots)"

# THE SEED'S OWN `intent` (ST0095 AC-03.3): a debug build with the `seeded-ids`
# feature, which takes its event ids from the sequence file INTENT_SEEDED_IDS
# names, so two runs of this script mint the same ids and write the same images.
# A release build cannot carry the feature, and the binary a person installs
# has no seed at all. Built into a target directory of its own, never the
# shared one, so the tree's own pair is never a seeded one. Only `intent` is
# built: the seeding verbs never reach intentd.
SEEDED="$(built_executable intent "the seeded intent" \
  --target-dir "$ROOT/native/rust/target/seeded-ids" -p intent-cli --bin intent --features seeded-ids)"
SEEDED_BIN="$(dirname "$SEEDED")"

# The day the seed's ids are minted on: the example's PINNED_DATE, the one date
# every shot draws, read from its source so the date has one home. A run on any
# day then mints the same ids, so a regeneration moves no published pixel.
EXAMPLE="$ROOT/native/rust/crates/intent-cli/examples/explorer_shots.rs"
ID_DATE="$(sed -n 's/^const PINNED_DATE: &str = "\([0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}\)";$/\1/p' "$EXAMPLE")"
[ -n "$ID_DATE" ] \
  || die "no PINNED_DATE in $EXAMPLE" "the seed mints its ids on the example's pinned date: restore \`const PINNED_DATE: &str = \"YYYY-MM-DD\";\` there"

# The start of the sequence: a ULID whose time half is $1 (YYYY-MM-DD) at
# 00:00Z and whose random half is zero, so the ids read as that day. Each mint
# takes the next ULID after the one the file holds (event.rs), so the first id
# minted is this one's increment.
seed_ulid() {
  local secs ms alphabet out i
  secs="$(date -u -j -f '%Y-%m-%d %H:%M:%S' "$1 00:00:00" +%s 2>/dev/null)" \
    || secs="$(date -u -d "$1 00:00:00" +%s 2>/dev/null)" \
    || die "cannot turn the date $1 into epoch seconds" "check PINNED_DATE in $EXAMPLE is a real YYYY-MM-DD date; this date(1) takes neither BSD's -j -f nor GNU's -d"
  ms=$((secs * 1000))
  alphabet=0123456789ABCDEFGHJKMNPQRSTVWXYZ
  out=""
  for i in 0 1 2 3 4 5 6 7 8 9; do
    out="${alphabet:$((ms % 32)):1}$out"
    ms=$((ms / 32))
  done
  printf '%s0000000000000000\n' "$out"
}

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

# mint <verb...> -- run a verb that mints, keep its output in LOG, and print the id
# it names on its `created:` line, which is how a script reads a minted id (ST0095
# WP-02). A project mints git-style ids, so no id is spelled in this file: each
# is the one the verb printed (ST0095 AC-03.3). A verb that refuses, or mints
# nothing, is a step that failed: its whole output goes to stderr, since LOG is
# removed with the run, and it exits 1, which under -e the assignment it feeds
# carries out of the seed.
mint() {
  local out id
  out="$(intent "$@" 2>&1)" || {
    printf '%s\n' "$out" >> "$LOG"
    printf 'error: `intent %s` refused while seeding:\n%s\n' "$*" "$out" >&2
    exit 1
  }
  printf '%s\n' "$out" >> "$LOG"
  id="$(printf '%s\n' "$out" | awk '$1 == "created:" { print $2; exit }')"
  [ -n "$id" ] || {
    printf 'error: `intent %s` printed no created: line:\n%s\n' "$*" "$out" >&2
    exit 1
  }
  printf '%s' "$id"
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

  T1="$(mint st new "Accept card payments" --start)"
  set_from "$T1" objective <<'TXT'
Take card payments at checkout without the card number ever reaching our servers.

The payment form hands the card to the processor and gets back a token; everything after that works on the token alone. An order exists only once a charge has been captured, and every captured charge ends in exactly one receipt.
TXT
  set_from "$T1" context <<'TXT'
Today checkout takes orders on account only. Card payments are the most requested feature from the shop's customers, and the processor's hosted form keeps us out of card-data scope.
TXT
  W1="$(mint wp new "$T1" "Tokenise the card in the payment form")"
  W2="$(mint wp new "$T1" "Charge, capture and record the order")"
  W3="$(mint wp new "$T1" "Email a receipt")"
  set_from "$W1" objective <<'TXT'
The processor's hosted field takes the card number and hands back a token, so the number never reaches our servers.
TXT
  set_from "$W2" objective <<'TXT'
Charge the token, capture on success, and write the order only once the capture is confirmed.
TXT
  set_from "$W3" objective <<'TXT'
Send one receipt per captured charge, even when the processor calls back twice.
TXT
  intent wp start "$W1" >> "$LOG"

  intent ac new "$T1" AC-00.1 --text "The payment page names the processor that handles the card" >> "$LOG"
  intent ac new "$T1" AC-01.1 --kind test --text "The card number never reaches our servers: the form posts a token" >> "$LOG"
  intent ac new "$T1" AC-02.1 --kind test --text "A card the processor declines leaves no order behind" >> "$LOG"
  intent ac new "$T1" AC-02.2 --kind test --text "A captured charge is recorded as exactly one order" >> "$LOG"
  intent ac new "$T1" AC-03.1 --kind test --text "A captured charge sends exactly one receipt" >> "$LOG"
  mkdir -p test/payments
  printf '# AT-01.1: the payment form posts a token, never the card number\n' > test/payments/tokenise_test.exs
  printf '# AT-02.1: a declined card leaves no order behind\n' > test/payments/decline_test.exs
  printf '# AT-02.2: a captured charge is recorded as exactly one order\n' > test/payments/capture_test.exs
  intent at new "$T1" AT-01.1 --covers AC-01.1 --file test/payments/tokenise_test.exs >> "$LOG"
  intent at new "$T1" AT-02.1 --covers AC-02.1 --file test/payments/decline_test.exs >> "$LOG"
  intent at new "$T1" AT-02.2 --covers AC-02.2 --file test/payments/capture_test.exs >> "$LOG"
  intent at red "$T1" AT-01.1 >> "$LOG"
  intent at green "$T1" AT-01.1 >> "$LOG"
  intent at red "$T1" AT-02.1 >> "$LOG"
  intent at green "$T1" AT-02.1 >> "$LOG"
  intent at red "$T1" AT-02.2 --note "fails when the processor sends the capture callback twice" >> "$LOG"
  intent ac satisfy "$T1" AC-00.1 --evidence "Checked on the staging payment page" >> "$LOG"
  intent wp done "$W1" >> "$LOG"
  intent wp start "$W2" >> "$LOG"

  T2="$(mint st new "Refunds from the order page")"
  set_from "$T2" objective <<'TXT'
Let staff refund all or part of a captured charge from the order page, and record who refunded what and why.
TXT

  T3="$(mint st new "Rate-limit the public API" --start)"
  set_from "$T3" objective <<'TXT'
Stop one API key from starving the others: every key gets a token bucket, and a request over its limit is answered 429 with a Retry-After header.
TXT
  W4="$(mint wp new "$T3" "A token bucket per API key")"
  set_from "$W4" objective <<'TXT'
Keep one bucket per key, refill it at the key's rate, and refuse a request that finds it empty.
TXT
  intent wp start "$W4" >> "$LOG"

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

# The dates the seed can stamp, in UTC and in local time, read on both sides
# of seeding so a run that crosses midnight is covered too.
SEED_DATES=("$(date -u +%F)" "$(date +%F)")
# Seed with the seeded intent first on PATH and its sequence file named, and with
# nothing else: the shots render as they always have.
seed_ulid "$ID_DATE" > "$TMP/ids"
UNSEEDED_PATH="$PATH"
export PATH="$SEEDED_BIN:$PATH" INTENT_SEEDED_IDS="$TMP/ids"
seed_shopfront
seed_warehouse
intent discover "$DEMO" >> "$LOG"
export PATH="$UNSEEDED_PATH"
# THE LIVE SHOT MINTS TOO: its `intent issues add` runs while the list is on
# screen, so its id must come from the same sequence, or each run draws a
# different one. The shots run with a one-verb shim first on PATH: `issues add`
# goes to the seeded intent, with INTENT_SEEDED_IDS still naming the sequence,
# and everything else, the explorer included, to the tree's own pair.
UNSEEDED="$(command -v intent)"
mkdir -p "$TMP/shim"
cat > "$TMP/shim/intent" <<EOF
#!/bin/sh
[ "\$1 \$2" = "issues add" ] && exec "$SEEDED" "\$@"
exec "$UNSEEDED" "\$@"
EOF
chmod +x "$TMP/shim/intent"
SEED_DATES+=("$(date -u +%F)" "$(date +%F)")
DATE_ARGS=()
while read -r d; do
  DATE_ARGS+=(--seed-date "$d")
done < <(printf '%s\n' "${SEED_DATES[@]}" | sort -u)

PATH="$TMP/shim:$PATH" "$SHOTS" --cwd "$DEMO/shopfront" --out "$TMP/svg" --thread "$T1" \
  --version "$VERSION" --commit "$COMMIT" "${DATE_ARGS[@]}"
unset INTENT_SEEDED_IDS

mkdir -p "$OUTDIR"
for svg in "$TMP"/svg/*.svg; do
  png="$OUTDIR/$(basename "$svg" .svg).png"
  rsvg-convert --zoom 2 "$svg" -o "$TMP/full.png"
  magick "$TMP/full.png" +dither -colors 64 -strip -define png:compression-level=9 "$png"
done
echo "ok: wrote the explorer's shots to ${OUTDIR#"$ROOT"/}"
