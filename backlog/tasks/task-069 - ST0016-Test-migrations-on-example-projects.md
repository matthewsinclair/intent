---
id: task-069
title: ST0016 - Test migrations on example projects
status: done
assignee: []
created_date: "2025-07-16"
completed_date: "2025-07-17"
labels: []
dependencies: []
---

## Description

Test the `intent upgrade` command on all example projects (v0.0.0, v1.2.0, v1.2.1) to ensure migrations work correctly.

## Results

✅ All example projects successfully migrated to Intent v2.0.0:

- v0.0.0-project: Migrated (manual fix required for missing YAML frontmatter)
- v1.2.0-project: Migrated (manual fix required for frontmatter conversion issue)
- v1.2.1-project: Migrated successfully with automatic upgrade

All projects now:

- Use Intent v2.0.0 directory structure (intent/ instead of stp/)
- Have .intent/config.json configuration
- Pass `intent doctor` checks
- Show steel threads correctly with `intent st list`

## Notes

The upgrade script has an issue with the convert_yaml_frontmatter function that needs fixing for v0.0.0 and v1.2.0 projects. The function doesn't properly handle files without YAML frontmatter.
