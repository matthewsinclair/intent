    bin/devbin macos <doctor|store-creds|app-sign|app-notarize|app-verify|publish>

Release a macOS app: sign a staged copy of the project's `.app` with the Developer ID and the hardened runtime, notarise and staple it, check it as a download meets it, and attach it zipped to the release at its tag. devbin implements it, on macOS only, for the app the `macos:` block of `bin/.devbin/config.yaml` declares.

It is opt-in: absent until a project asks for it with `commands.macos.enabled: true`. `enabled: false` switches it off again. A project's own `bin/.devbin/cmd/macos` handler, or a `commands.macos.run:` line, still wins over devbin's, as for any built-in, and then `bin/devbin help macos` prints the project's own page.

## The declaration

    macos.app           the .app bundle the build leaves, from the project
                        root or absolute; required
    macos.build         the command line that builds it, run at the root
                        before app-sign stages; none: it is built already
    macos.bundle_id     the bundle id the app carries and is signed as;
                        required
    macos.entitlements  the entitlements file signed in; none: signed without
    macos.keychain      the login keychain's service prefix the notary
                        credentials live under; default devbin.apple
    macos.asset         the release asset's name; default the app's name
                        with .zip, as Demo.app.zip
    macos.version_from  the file the app's own version is read from, for an
                        app that versions apart; default project.version_from
    macos.commit_stamp  the Info.plist key the build writes its commit into;
                        required when the app versions apart
    macos.kind_stamp    the Info.plist key the build writes its kind into
    macos.kind_value    the value macos.kind_stamp must carry; default release
    macos.build_root    where publish's last Launch Services drop looks;
                        default the directory holding macos.app

A path key may open with `~`, `$HOME` or an XDG base-directory variable, expanded as the XDG spec reads it, so an app built outside the project needs no machine's own path. Each stands alone as a leading component, and an XDG variable that is unset falls back to its XDG default; any other leading `~` or `$`, such as `~user` or `${XDG_STATE_HOME:-...}`, is refused, naming it. An app that versions apart is still released under the project's tag and version. Every `macos:` key is annotated in devbin's reference config, which `bin/devbin devbin` names.

## A release

On the maintainer's machine, once the release core has made the release object, in a tree whose HEAD is the tag. Once main has moved past it, that is a worktree: `git worktree add --detach <dir> <tag>`. In order:

    app-sign      refused, building nothing, unless HEAD is the release tag
                  of the project's version in a clean tree; then
                  macos.build, a copy of macos.app staged under XDG state,
                  outside the build tree, refused and removed, with nothing
                  signed, unless its bundle id is macos.bundle_id, its
                  version the app's and its declared stamps the tag's build's
                  (its kind macos.kind_value, its commit a prefix of the
                  tag's); then signed inside-out (nested code,
                  then the bundle) and its identifier, team and hardened
                  runtime read back off the signature
    app-notarize  submit the signed copy to Apple and wait; staple the ticket
                  only when it is Accepted, else print Apple's log; then
                  check a quarantined copy with spctl
    publish       zip the stapled copy as macos.asset, unpack the zip as a
                  user does and check the bundle inside, then attach it with
                  `release attach <version> --from`, the release core's one
                  way an asset reaches a release: it never replaces an asset,
                  writes SHA256SUMS over every asset, and reads each upload
                  back, so publish run again uploads nothing more

    app-verify    the staged copy's signature and ticket; changes nothing
    doctor        the platform, the declaration, the tools, the Developer ID
                  identity (counted, with its team and a test-sign) and the
                  notary credentials, each PRESENT or MISSING; exit 1 when
                  anything is missing or the identity cannot sign
    store-creds   store APPLE_ID, APPLE_APP_PASSWORD and APPLE_TEAM_ID in the
                  login keychain under macos.keychain's prefix; interactive,
                  in a terminal only

The stage is one per checkout, under the XDG state directory in `devbin/macos/`, and a stage at or under the project root or the build product's directory is refused before it is made. The stage, the root and the products are compared as the filesystem spells them, so on a volume that folds case a stage typed in another case is still read as inside. A stage whose path names `.` or `..` beneath a directory that does not exist yet is refused too, since where it lands cannot be known, and so is one whose existing part cannot be entered. Each copy the built-in makes has its Launch Services record dropped on every path out, with INT, TERM and HUP ignored through the drop and the record list read back, so the bundle id opens only the installed app. A record left on the staged copy is a warning, and every step drops it again, publish the last; one left on a copy that is gone fails the run, after the upload for publish's proof copy, saying first that the asset is published. After the upload, publish drops every record at or under macos.build_root, and one that stays is a warning. A build root that is `/`, or holds the home directory or a directory installed apps live in, is refused before anything is built or uploaded.

## The credentials

They live in the login keychain and nowhere else: no key material in the tree, in CI config or in any file the built-in writes, and nothing prints a value. They are read only by the subcommands that need them: app-sign reads the team id alone, to choose the identity, and app-notarize reads the Apple id and the app password too. `store-creds` hands each value to security(1), which prompts for it and echoes nothing, so no value passes through devbin. `doctor` names the identity by count and team, never whole.

## Exit codes

    0    done
    1    refused or failed, naming why; or a usage error: no subcommand, an
         unknown one, or an argument after one
    2    not macOS: every subcommand refuses off macOS, before the config or
         any tool is read; --help works anywhere
