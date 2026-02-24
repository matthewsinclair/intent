# WP07: CI/CD, Examples, and Cleanup

## Scope

CI pipeline, example projects, project backlog directory.

## Directories to DELETE

- backlog/ (project's own)
- examples/hello-world/backlog/
- examples/v0.0.0-project/backlog/
- examples/v1.2.0-project/backlog/
- examples/v1.2.1-project/backlog/

## Files to Edit

- .github/workflows/tests.yml - Remove Node.js setup
- examples/hello-world/README.md, CLAUDE.md, intent/st/ST0001/design.md, info.md
- examples/v0.0.0-project/.gitignore
- examples/v1.2.0-project/.gitignore
- examples/v1.2.1-project/.gitignore

## Acceptance Criteria

- CI has no Node.js or Backlog.md steps
- No example has backlog/ directory
- Project backlog/ removed
