# Dependency Graph -- [[PROJECT_NAME]]

> Cross-app dependency rules for umbrella applications.
> Each row declares which apps a given app may depend on, and which it must NOT.
> Enforced by the `Mix.Checks.DependencyGraph` Credo check (D11).

## Rules

| App | May depend on | Must NOT depend on |
| --- | ------------- | ------------------ |
|     |               |                    |

## Notes

- "May depend on" is an allowlist -- unlisted apps are implicitly allowed unless forbidden.
- "Must NOT depend on" is a denylist -- any alias/import/use of a forbidden app triggers a violation.
- Dependencies are detected via `alias`, `import`, and `use` statements in source files.
- The app is determined from the file path: `apps/<app>/lib/...`

## Examples

```markdown
| App       | May depend on        | Must NOT depend on     |
| --------- | -------------------- | ---------------------- |
| core      |                      | web, admin             |
| web       | core                 | admin                  |
| admin     | core                 |                        |
| reporting | core                 | web, admin             |
```

In this example, `core` cannot depend on `web` or `admin` (it is a foundation layer),
while `web` can use `core` but not `admin`.
