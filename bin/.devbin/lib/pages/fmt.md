    bin/devbin fmt <elixir|rust|swift|md|all> [<arg>...]

Format sources in place. `fmt` MUTATES files. Its read-only counterpart is `check format`, and the two are deliberately different commands (devbin design D21): there is no `fmt --check`.

## Options

    elixir    mix format                       when the project declares elixir
    rust      cargo fmt                        when it declares rust
    swift     swift-format over every *.swift  when it declares swift
    md        prettier over Markdown -- root *.md only, unless given paths;
              in every project
    all       every option above that is offered here, one pass

Each runs at the project root as a sealed gate, logged under `tmp/fmt/`. Arguments after `md`, `elixir` or `rust` go to prettier, `mix format` or `cargo fmt`, and paths are relative to the root. `swift` ignores them, and skips `build`, `.build` and `Carthage` directories.

## Why md is root-only

prettier rewrites Markdown tables into its own layout, so a sweep of the whole tree would re-lay every hand-aligned table not already in it. So bare `fmt md` formats `./*.md` and says so, and a file elsewhere is formatted by naming it:

    bin/devbin fmt md guides/setup.md

It runs prettier through `npx --yes` with no `--prose-wrap` flag. A command-line flag overrides a config file, so passing one would impose a prose style on every project that vendors devbin. The project's own `.prettierrc` decides.

## A missing formatter skips, loudly

If `npx` (Node) or `swift-format` is absent, that option says so and skips: it exits 0 and its seal reads green, with the skip in its log. A missing `mix` or `cargo` fails instead. The gate that goes red on unformatted sources is `check format`, which fails rather than skips when a formatter is missing.

## Markdown has no check form, on purpose

`check format` does not check Markdown. A gate over it would fail on any table a project aligns by hand in a layout prettier does not produce.
