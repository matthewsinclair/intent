    bin/devbin devbin [templates|--version]

Help about the framework itself, not about this project. `help` answers "what can this project do"; this answers "what is the thing under bin/ and how do I work on it".

## Forms

    (bare)       the seam, the name this install answers as, where the
                 reference config and long-form guides are, and, when
                 vendored, where upgrades come from
    templates    what `bin/devbin new <cmd>` scaffolds from
    --version    devbin's own version and the commit its bytes came from

`help` is the bare form; `-v` and `version` are `--version`. Printing is the job, so each form exits 0 when it can answer. An unknown form prints the usage line and exits 1, and a broken install manifest makes the bare form and `--version` refuse (below).

## The seam

In a project, devbin is vendored:

    bin/devbin, devbin's runtime   devbin's -- replaced wholesale by upgrade
    config.yaml, cmd/, help/       yours -- never overwritten by install or upgrade

`install` writes a starter `config.yaml` only where there is none. The overview prints where the runtime is, derived from where it is actually running.

## Whose version --version reports

devbin's, as recorded in the install manifest, not the project's: that is `bin/devbin --version`. If a project's manifest exists but records no devbin version, the bare form and `--version` refuse with exit 1 rather than print the project's version under devbin's name. With no manifest at all they fall back to the project's version file: right in devbin's own source checkout, where the project is devbin, and wrong in a vendored project, where `bin/devbin doctor` fails the missing manifest.
