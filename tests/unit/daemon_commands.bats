#!/usr/bin/env bats
#
# daemon_commands -- the `intent daemon` surface, which had ZERO tests.
#
# WHY IT EXISTS. `coverage_map.sh` reports command families nothing exercises,
# and `daemon` was one of two at zero (the other is `config`). Measured before
# writing a line: `grep -rn 'intent daemon' tests/` returned nothing, and the
# single estate-wide `daemon` hit was a COMMENT in release_sidecars.bats:133.
# The family is four verbs on a surface a menubar app now consumes (ST0064),
# so "no test names it" and "it works" were resting on the same evidence: none.
#
# NOTHING HERE STARTS A DAEMON, AND THAT IS A DESIGN CONSTRAINT RATHER THAN A
# GAP. Four Claude sessions write this checkout concurrently, and `daemon start`
# binds a socket under `$HOME`. Every test below drives a verb whose contract
# can be established WITHOUT a live daemon: the absent state, the two rendered
# faces, the refusals, and the two documented cases where a naive reading would
# expect something other than `absent`. The live/stale arms need a daemon in a
# sandbox and are deliberately left to a separate file, so that this one can run
# on a shared machine without a second writer appearing on it.
#
# THE SANDBOXED $HOME IS THE ISOLATION, and it is sound rather than hopeful.
# `$HOME` is read in exactly ONE module (`userstate.rs`, which says so and is
# enforced over every `src/**/*.rs`), and the socket path is
# `daemon_socket_under(home)` = `$HOME/.local/share/intent/intentd.sock`. So a
# fake `$HOME` relocates the address the CLI probes, by construction and not by
# convention.
#
# TWO ARMS PIN BEHAVIOUR THAT IS RULED AND WOULD OTHERWISE READ AS A BUG:
#
#   - NO `$HOME` IS `absent` AT rc=0, NOT A FAILURE. Ruled in daemon.rs's
#     `candidates` doc: "EMPTY IS A LEGITIMATE ANSWER, NOT AN ERROR. Without
#     `$HOME` there is no per-user state and so no socket path -- and the honest
#     consequence is that no daemon can be found, not that the command fails."
#     The project verbs must not start needing `$HOME` because a routing probe
#     could not resolve a directory. Nothing tested that, so a refactor that
#     "fixed" it into an error would have broken a documented contract silently.
#
#   - A DEAD SOCKET *FILE* IS STILL `absent`. A unix socket file outlives the
#     process that bound it, so `Path::exists` is the wrong question and
#     `daemon_status` asks `route()` instead. The failure this prevents is a
#     status verb reporting a daemon is running while every command routes
#     in-process -- and status is exactly where an operator goes to find out
#     why that is happening.

load "../lib/test_helper"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-daemon-XXXXXX)"
  # A SHORT PATH ON PURPOSE. `sockaddr_un` holds a fixed-size path, and
  # daemon.rs names "a long $HOME, or a temporary directory used as one" as the
  # usual cause of a bind refusal. `/tmp/intent-test-daemon-XXXXXX/h` keeps the
  # derived socket path far under the limit, so a failure here is the subject
  # and never the fixture.
  export HOME="$TEST_TEMP_DIR/h"
  mkdir -p "$HOME"
  cd "$TEST_TEMP_DIR" || exit 1
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

@test "daemon status: reports absent at rc=0 when nothing is running" {
  run "$INTENT_BIN" daemon status
  [ "$status" -eq 0 ]
  [[ "$output" == *"no intentd is answering"* ]]
  # The remedy half matters as much as the state: it tells the operator that
  # commands still work rather than leaving "absent" to read as broken.
  [[ "$output" == *"in-process"* ]]
}

@test "daemon status --format json: the machine face is a bare lowercase discriminator" {
  run "$INTENT_BIN" daemon status --format json
  [ "$status" -eq 0 ]
  # ic's decoder is `enum State: String` (ST0064), so `state` carries the bare
  # variant name and nothing decorative. Asserted as a parsed field rather than
  # a substring: a substring match would pass on `{"state":"absentish"}`.
  [ "$(echo "$output" | jq -r '.state')" = "absent" ]
  # `endpoint` is present iff live and `pid` iff stale, so both are absent here.
  [ "$(echo "$output" | jq -r 'has("endpoint")')" = "false" ]
  [ "$(echo "$output" | jq -r 'has("pid")')" = "false" ]
}

@test "daemon status --format terminal: names the default explicitly" {
  run "$INTENT_BIN" daemon status --format terminal
  [ "$status" -eq 0 ]
  local explicit="$output"
  run "$INTENT_BIN" daemon status
  [ "$status" -eq 0 ]
  # The two faces are one projection of one value. If the default ever stops
  # being `terminal` this fails rather than drifting quietly.
  [ "$output" = "$explicit" ]
}

@test "daemon status --format: refuses an undeclared value and names the roster" {
  run "$INTENT_BIN" daemon status --format zzz
  [ "$status" -eq 1 ]
  # Naming the roster is the point. A bare "invalid value" leaves the operator
  # guessing at a closed set the tool already knows.
  [[ "$output" == *"terminal"* ]]
  [[ "$output" == *"json"* ]]
}

@test "daemon: bare family requires a subcommand" {
  run "$INTENT_BIN" daemon
  [ "$status" -eq 1 ]
  [[ "$output" == *"requires a subcommand"* ]]
}

@test "daemon: an unrecognised verb is refused rather than ignored" {
  run "$INTENT_BIN" daemon nosuchverb
  [ "$status" -eq 1 ]
  [[ "$output" == *"nosuchverb"* ]]
}

@test "daemon status: no \$HOME is a legitimate absent, not a failure" {
  # RULED, not incidental -- see the header. A routing probe that cannot resolve
  # a directory must not turn a project verb into a failure.
  run env -u HOME "$INTENT_BIN" daemon status
  [ "$status" -eq 0 ]
  [[ "$output" == *"no intentd is answering"* ]]
}

@test "daemon status: an empty \$HOME is treated the same as an unset one" {
  # `home()` matches `Ok(h) if !h.is_empty()`, so the empty string takes the
  # same branch as unset. Pinned because the two are easy to split by accident.
  run env HOME= "$INTENT_BIN" daemon status
  [ "$status" -eq 0 ]
  [[ "$output" == *"no intentd is answering"* ]]
}

@test "daemon status: a leftover socket FILE with no listener is still absent" {
  # A unix socket file outlives the process that bound it. Planting one is the
  # whole fixture: bind it, close it, leave the inode. A status verb keying on
  # `Path::exists` reports a running daemon here and is wrong.
  mkdir -p "$HOME/.local/share/intent"
  python3 -c "
import socket
s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
s.bind('$HOME/.local/share/intent/intentd.sock')
s.close()
"
  # The fixture must actually be a socket, or this test passes for the wrong
  # reason -- an absent file would also report absent.
  [ -S "$HOME/.local/share/intent/intentd.sock" ]

  run "$INTENT_BIN" daemon status
  [ "$status" -eq 0 ]
  [[ "$output" == *"no intentd is answering"* ]]
}

@test "daemon --help: the family names all four verbs" {
  run "$INTENT_BIN" daemon --help
  [ "$status" -eq 0 ]
  # `--help` renders for a DECLARED command whether or not an arm exists, so
  # this asserts the roster is declared and nothing more. The verbs' behaviour
  # is the subject of the tests above, driven rather than read off help text.
  [[ "$output" == *"start"* ]]
  [[ "$output" == *"stop"* ]]
  [[ "$output" == *"status"* ]]
  [[ "$output" == *"run"* ]]
}
