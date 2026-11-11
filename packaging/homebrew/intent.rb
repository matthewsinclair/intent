# Intent's Homebrew formula, as a template. devbin's built-in `formula` renders
# it from a release in assets mode (bin/.devbin/config.yaml, `formula:`), filling
# every placeholder from the tag and the release's SHA256SUMS, and replaces this
# header with its own. Moved here from bin/.devbin/cmd/shared/brew.lib's renderer
# with ST0091 WP-03.
class Intent < Formula
  RELEASE_VERSION = "@@VERSION@@".freeze
  SOURCE_COMMIT = "@@REVISION@@".freeze

  desc "Steel thread process for helping LLMs help you work with your code"
  homepage "https://github.com/matthewsinclair/intent"
  # THE URL IS AT TOP LEVEL, NOT INSIDE `on_macos do on_arm do`. Until 3.0.1 it
  # was nested there, and `brew tap` validates a tap's formulae under EVERY
  # simulated OS and arch: on Linux and on Intel the nested form has no url, so
  # the tap refused with "formula requires at least a URL" and "invalid syntax
  # in tap", for everyone.
  # It is macOS arm64's, and on Linux the package for the machine's
  # architecture overrides it inside `on_linux`, below.
  # There is no `version` line because brew reads it from the /v<version>/ path,
  # and `brew audit --strict` refuses the redundant one.
  url "https://github.com/matthewsinclair/intent/releases/download/v@@VERSION@@/intent-aarch64-apple-darwin.tar.gz"
  sha256 "@@SHA256:intent-aarch64-apple-darwin.tar.gz@@"
  license "MIT"

  # On macOS it is arm64 only, by ruling (hv, 2026-08-15) rather than by
  # omission, and TRUE BY CONSTRUCTION through the dependency line inside
  # `on_macos`, which is what a platform limit in a formula is for. On macOS one
  # archive carries both binaries, Developer ID signed and notarised, and the
  # support tree (devbin 0329); nothing is stapled, because a Mach-O has nowhere
  # to hold a ticket and Gatekeeper checks online. On Linux (hv, 2026-10-14) the
  # package for the machine's architecture carries both binaries, built static
  # against musl, and the support tree. (No comment
  # line here may open with the dependency keyword: `brew audit` reads that as a
  # commented-out dependency and refuses it.)
  on_macos do
    depends_on arch: :arm64
  end

  on_linux do
    on_intel do
      url "https://github.com/matthewsinclair/intent/releases/download/v@@VERSION@@/intent-x86_64-unknown-linux-musl.tar.gz"
      sha256 "@@SHA256:intent-x86_64-unknown-linux-musl.tar.gz@@"
    end
    on_arm do
      url "https://github.com/matthewsinclair/intent/releases/download/v@@VERSION@@/intent-aarch64-unknown-linux-musl.tar.gz"
      sha256 "@@SHA256:intent-aarch64-unknown-linux-musl.tar.gz@@"
    end
  end

  # EVERYTHING LANDS IN libexec AND bin GETS SYMLINKS, which is not the obvious
  # layout and is not a style preference.
  #
  # `intent` locates its own install by walking up from its symlink-resolved
  # location to the directory containing `lib/templates/`, so the support tree
  # has to sit beside the binary inside the keg -- a plain `bin.install` leaves
  # the binary unable to find the hooks it execs, and both whiteboard guards and
  # every session hook stop working with nothing reporting a failure.
  #
  # `prefix/lib/templates` would resolve, but `lib` is a LINKED directory: brew
  # symlinks keg `lib` subdirectories into the shared prefix, so it would publish
  # a directory called `templates` into `#{HOMEBREW_PREFIX}/lib`. `libexec` is
  # not linked, so the tree stays private to this keg.
  #
  # `install` is devbin's generated block alone (T8). Each platform's archive is
  # the install root under one top-level directory, which Homebrew enters, so the
  # block lays it into libexec; sets the executable bit on each binary the link
  # set names, which a release asset arrives without and Homebrew's `.install`
  # keeps (the v3.0.0 tap installed and nothing ran, 2026-08-26); writes T3's
  # manifest under libexec, with the version line, which makes `intent doctor`
  # and `intent use` read this keg as the brew tree; and links intent and intentd
  # into bin.
  def install
    @@INSTALL@@
  end

  # A CAVEAT, NOT A post_install, AND THE CHOICE WAS DRIVEN (dc, 2026-09-11).
  # The pre-commit gate a project installs finds this keg through
  # ~/.local/share/intent/home, and only `intent bootstrap` writes that file. A
  # post_install cannot run it: Homebrew gives post_install a throwaway HOME
  # inside its sandbox and denies writes outside the keg. The caveat is
  # conditional because bootstrap REPOINTS an existing pointer.
  #
  # THE UPGRADE CASE (issues 0527 and 0540). Intent 3.2.0 and earlier recorded
  # the versioned keg, which `brew upgrade` deletes, so the pointer names nothing
  # and every gated commit is refused. A pointer naming any keg under
  # Cellar/intent/ is re-recorded, whether or not that keg is still on disk, and
  # `intent bootstrap --check` (issue 0533) says which case a machine is in.
  def caveats
    <<~EOS
      Intent's pre-commit gate finds this install through ~/.local/share/intent/home, which
      only `intent bootstrap` writes, and `intent bootstrap --check` says where it points.
      Run
        intent bootstrap
      if that file does not exist yet, or if it names an Intent under Cellar/intent/, whether
      or not that version is still installed: Intent 3.2.0 and earlier recorded the versioned
      keg, which `brew upgrade` removes, and from then on every commit in a project with the
      gate installed is refused. Later versions record a path that survives upgrades, and a
      file naming anything else, such as a source checkout, needs nothing.

      `brew services start` logs to intentd.log and intentd.err.log under ~/.local/state/intent,
      the files `intent daemon logs` reads, and `intent bootstrap` makes that directory, so run
      it once before the first `brew services start`.

      A maintainer with a dev or opt tree beside this keg chooses which one answers
      `intent` with `intent use`, and `intent use brew` hands it back to this keg.
    EOS
  end

  # IT RUNS `intentd` DIRECTLY RATHER THAN `intent daemon run`, on the daemon's
  # own words: serving is what that binary does with no arguments, and `intent
  # daemon run` execs it, so going through the CLI would put a process in front
  # of the one launchd should supervise.
  #
  # THE LOG PATHS ARE THE DAEMON'S OWN FILES (issue 0667, hv 2026-10-31).
  # intentd writes every line it logs to stdout or stderr and keeps no log file
  # of its own: stdout goes to intentd.log and stderr to intentd.err.log, D19's
  # two files under the user's state dir, which `intent daemon logs` reads. The
  # state dir is `$XDG_STATE_HOME/intent`, by default `~/.local/state/intent`
  # (intentsvcs userstate.rs), and the default is the one written here because a
  # service's environment carries no XDG_STATE_HOME. On Linux `brew services`
  # runs it under systemd's user manager, which no run has proven yet.
  service do
    run [opt_bin/"intentd"]
    keep_alive true
    log_path "#{Dir.home}/.local/state/intent/intentd.log"
    error_log_path "#{Dir.home}/.local/state/intent/intentd.err.log"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/intent --version")
  end
end
