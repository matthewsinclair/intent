---
id: IN-CO-STYLE-002
language: content
category: style
severity: warning
title: Every content image has meaningful alt text
summary: >
  Every image that carries meaning has descriptive alt text; decorative images
  declare themselves empty (`alt=""`). Alt text is what a screen-reader user, a
  reader on a broken connection, and the image-search index all receive in place
  of the picture.
principles:
  - accessibility
  - discoverability
applies_when:
  - "Any web content page containing images"
  - "Markdown image syntax, MDX <Image>, or HTML <img>"
applies_to:
  - "**/*.md"
  - "**/*.mdx"
  - "**/*.html"
does_not_apply_when:
  - 'Purely decorative images correctly marked alt="" (empty is the right answer, not a violation)'
  - "Icon fonts / CSS background images that carry no content meaning"
tags:
  - content
  - web
  - accessibility
  - a11y
  - mechanical
references: []
related_rules: []
aliases: []
status: active
version: 1
---

# Every content image has meaningful alt text

An image that means something says what; an image that means nothing says so explicitly.

## Problem

An image with no alt text is invisible to a screen-reader user and to image search, and shows nothing but a broken-image icon when the asset fails to load. Worse is alt text that repeats the filename (`alt="IMG_2043.jpg"`) or the word "image" -- noise that the reader must sit through. Meaningful content deserves a meaningful description; a decorative flourish deserves an explicit empty alt so assistive tech skips it rather than announcing a filename.

## Detection

- Markdown `![](...)` or `![ ]` with an empty or whitespace-only alt on a content image.
- HTML `<img>` with no `alt` attribute at all.
- Alt text that is a filename, an extension (`.jpg`, `.png`), or the literal word "image" / "photo".

## Bad

```markdown
![](/img/dashboard.png)

![IMG_2043.jpg](/img/team.jpg)
```

## Good

```markdown
![The analytics dashboard showing weekly active users trending up](/img/dashboard.png)

<!-- decorative divider: explicitly empty so screen readers skip it -->

![](/img/flourish.svg)
```

## When This Applies

- Every image that conveys information, on any indexable page.
- Especially diagrams, screenshots, and charts, where the alt text carries the content.

## When This Does Not Apply

- Decorative images correctly marked `alt=""` -- empty is the right answer.
- CSS background images and icon fonts that carry no meaning.

## Further Reading

- IN-CO-STYLE-001 -- page meta, the sibling discoverability rule.
