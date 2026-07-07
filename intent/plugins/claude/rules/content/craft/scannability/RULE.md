---
id: IN-CO-CRAFT-001
language: content
category: craft
severity: recommendation
title: Web copy is scannable and front-loaded
summary: >
  Web readers scan before they read: they take the first line, the headings,
  the bolded phrases, and the bullets, then decide whether to read the prose.
  Content is written for that behaviour -- the conclusion first, short
  paragraphs, meaningful subheadings, and lists where the shape is a list --
  not as an undifferentiated wall inherited from print.
principles:
  - reader-first
  - web-native
applies_when:
  - "Any web content page meant to be read on screen (landing pages, posts, docs)"
  - "Prose ported from a print or long-form source into a web page"
does_not_apply_when:
  - "Long-form narrative deliberately written to be read start-to-finish (an essay, a story)"
  - "Reference material whose structure is dictated by an external spec"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
  - "**/*.html"
tags:
  - content
  - web
  - voice
  - scannability
references: []
related_rules:
  - IN-PR-STYLE-003
aliases: []
status: active
version: 1
---

# Web copy is scannable and front-loaded

Write for the reader who scans first and reads second: put the point up top and give the eye landmarks.

## Problem

Web reading is not print reading. A visitor scans the first sentence, the subheadings, and the emphasised phrases, then decides in seconds whether the page is worth their attention. Copy that buries its conclusion under three paragraphs of preamble, runs 200-word paragraphs, or offers no subheadings to scan loses that visitor before the payoff. This is not a mechanical tell a grep can catch -- it is a judgement about whether the page rewards a scan: does the first line state the value, do the headings carry the argument on their own, is a set of parallel items actually a list.

## Detection

This is a judgement read, not a pattern match. The critic reads the page as a scanning visitor would and asks:

- Does the opening line state the point, or warm up to it?
- Do the subheadings, read alone, convey the page's argument?
- Are paragraphs short enough to scan (roughly three-to-five lines), or are they print-length walls?
- Where the content is a set of parallel items, is it a list, or prose pretending not to be?

## Bad

```markdown
# Welcome

Thank you for taking the time to visit our website today. We are a company that
was founded some years ago with a mission, and over the course of that time we
have grown considerably and learned a great deal about our customers and their
needs, which brings us to the product we would like to tell you about, which
after all this preamble is a project management tool.
```

## Good

```markdown
# Ship projects on time

Acme is a project management tool that cuts status meetings by half.

## Why teams switch

- **See blockers early** -- risks surface before they slip the date.
- **One source of truth** -- no more reconciling three trackers.

Founded in 2019, we have learned one thing above all: teams want fewer meetings.
```

## When This Applies

- Marketing pages, blog posts, and docs read on screen.
- Any content ported from print, where paragraph length and structure need re-thinking for the web.

## When This Does Not Apply

- Long-form narrative meant to be read linearly (an essay, an interview).
- Reference material whose structure follows an external specification.

## Further Reading

- IN-PR-STYLE-003 -- heading hygiene; scannable headings depend on a clean heading tree.
