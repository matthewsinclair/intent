# LLM Trope Catalog

Vendored from [llm-tropes](https://github.com/matthewsinclair/llm-tropes).
Version: 1.0.0 (2026-03-28)

## Table of Contents

| ID     | Slug                        | Category            | Severity | Threshold | Detection |
| ------ | --------------------------- | ------------------- | -------- | --------- | --------- |
| ASR-01 | ai-identity-leaks           | ai-self-reference   | high     | single    | automated |
| ASR-02 | fourth-wall-breaks          | ai-self-reference   | high     | single    | automated |
| ASR-03 | meta-knowledge-leaks        | ai-self-reference   | high     | single    | automated |
| CMP-05 | content-duplication         | composition         | high     | single    | mixed     |
| CMP-02 | dead-metaphor               | composition         | medium   | single    | semantic  |
| CMP-01 | fractal-summaries           | composition         | medium   | single    | semantic  |
| CMP-03 | historical-analogy-stacking | composition         | medium   | single    | semantic  |
| CMP-04 | one-point-dilution          | composition         | medium   | single    | semantic  |
| CMP-06 | signposted-conclusion       | composition         | medium   | single    | automated |
| FMT-02 | bold-first-bullets          | formatting          | medium   | single    | automated |
| FMT-01 | em-dash-addiction           | formatting          | low      | density:3 | automated |
| FMT-03 | unicode-decoration          | formatting          | low      | density:2 | automated |
| NAR-05 | action-cliches              | narrative           | medium   | single    | automated |
| NAR-04 | dialog-tag-cliches          | narrative           | medium   | single    | automated |
| NAR-03 | emotive-cliches             | narrative           | high     | single    | mixed     |
| NAR-07 | exposition-hand-holding     | narrative           | low      | single    | automated |
| NAR-02 | filler-phrases              | narrative           | medium   | single    | automated |
| NAR-01 | greeting-cliches            | narrative           | high     | single    | automated |
| NAR-06 | melodramatic-phrases        | narrative           | medium   | single    | automated |
| PS-02  | listicle-in-trench-coat     | paragraph-structure | medium   | single    | semantic  |
| PS-01  | short-punchy-fragments      | paragraph-structure | low      | single    | semantic  |
| SS-08  | anaphora-abuse              | sentence-structure  | medium   | single    | semantic  |
| SS-02  | countdown-pattern           | sentence-structure  | medium   | single    | mixed     |
| SS-06  | false-ranges                | sentence-structure  | medium   | single    | mixed     |
| SS-07  | gerund-fragment-litany      | sentence-structure  | medium   | single    | semantic  |
| SS-04  | its-worth-noting            | sentence-structure  | low      | single    | automated |
| SS-01  | negative-parallelism        | sentence-structure  | high     | single    | mixed     |
| SS-03  | rhetorical-self-question    | sentence-structure  | low      | density:2 | mixed     |
| SS-05  | superficial-analyses        | sentence-structure  | medium   | single    | semantic  |
| SS-09  | tricolon-abuse              | sentence-structure  | medium   | density:2 | semantic  |
| TN-10  | despite-its-challenges      | tone                | low      | single    | automated |
| TN-04  | false-vulnerability         | tone                | low      | single    | semantic  |
| TN-06  | grandiose-stakes-inflation  | tone                | low      | density:2 | mixed     |
| TN-01  | heres-the-kicker            | tone                | medium   | single    | automated |
| TN-03  | imagine-a-world             | tone                | medium   | single    | automated |
| TN-09  | invented-concept-labels     | tone                | medium   | density:2 | semantic  |
| TN-07  | lets-break-this-down        | tone                | medium   | single    | automated |
| TN-05  | the-truth-is-simple         | tone                | medium   | single    | mixed     |
| TN-02  | think-of-it-as              | tone                | low      | single    | automated |
| TN-08  | vague-attributions          | tone                | low      | single    | mixed     |
| WC-02  | delve-and-friends           | word-choice         | medium   | single    | automated |
| WC-01  | magic-adverbs               | word-choice         | low      | density:3 | automated |
| WC-04  | serves-as-dodge             | word-choice         | low      | single    | automated |
| WC-03  | tapestry-and-landscape      | word-choice         | low      | density:2 | automated |

---

---

id: ASR-01
name: AI Identity Leaks
slug: ai-identity-leaks
category: ai-self-reference
severity: high
threshold: single
detection: automated

---

# AI Identity Leaks

## Description

Direct references to being an artificial intelligence, a language model, or a program. In fiction, these shatter immersion by pulling the reader out of the story and into the machinery behind it. In nonfiction ghostwriting, blog posts, or any text presented as human-authored, they destroy credibility. The model produces these because its training includes system prompts and RLHF data where it was rewarded for disclosing its nature. That behavior leaks into contexts where disclosure is irrelevant or harmful.

Also includes references to specific AI systems by name (GPT, Claude, Anthropic, OpenAI, Gemini, Copilot) when the text is not about AI technology.

## Indicators

- I'm an AI
- as an AI
- being a language model
- I was created by
- I was trained by
- I was programmed to
- my training data
- my programming
- my instructions
- I don't actually have feelings
- I'm just a program
- I'm just a machine
- I can't experience
- GPT (in non-AI-related text)
- Claude (in non-AI-related text)
- Anthropic (in non-AI-related text)
- OpenAI (in non-AI-related text)

## Examples

### Before

> "I understand your concern," the advisor said. "As an AI, I don't actually have feelings, but my training allows me to recognize emotional patterns."

### After

> "I understand your concern," the advisor said. "Tell me more about what happened last week."

### Before

> This article explores the history of bread-making. As a language model, I find this topic fascinating because my training data contains thousands of recipes.

### After

> Bread-making is older than written language. The earliest evidence dates to 14,000 years ago in Jordan.

## Why It Matters

Any single occurrence is a failure. In fiction, a character who announces they are AI-generated ceases to function as a character. In nonfiction, a text that references its own generation process loses the reader's trust. There is no acceptable density -- one instance is too many.

## Context Notes

Text that is explicitly about AI technology (research papers, AI product documentation, articles analyzing AI capabilities) will naturally contain these terms. The trope applies when these references appear in text that is not about AI: fiction, blog posts, marketing copy, essays, emails. Detection should account for document topic.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive), with topic-awareness for AI-system names
- **Regex**: `(?i)(i'?m an AI|as an AI|being a language model|i was (created|trained|programmed) (by|to)|my (training data|programming|instructions)|i don'?t (actually )?have (real )?feelings|i'?m just a (program|machine)|i can'?t (actually )?(experience|feel))`

---

---

id: ASR-02
name: Fourth Wall Breaks
slug: fourth-wall-breaks
category: ai-self-reference
severity: high
threshold: single
detection: automated

---

# Fourth Wall Breaks

## Description

References that acknowledge the text is a construct: that it is a story being told, a game being played, a roleplay being performed, or an exercise with an author behind it. The model produces these because its training data includes game manuals, roleplay forums, and writing workshops where meta-references are normal. When those references leak into the narrative itself, they remind the reader that they are reading generated output rather than experiencing a story.

## Indicators

- in this story
- in this narrative
- in this game
- in this roleplay
- in this scenario
- the player
- the player character
- you're playing
- you're roleplaying
- breaking character
- out of character
- OOC:
- the author
- the writer
- the creator
- game master
- dungeon master
- let's continue the story
- back to the story
- as a character

## Examples

### Before

> In this story, the player character enters a dark forest. The game master describes the sounds of wildlife around you.

### After

> The canopy closed overhead. Something moved in the undergrowth to the left -- too heavy to be a bird.

### Before

> "I think we should head north," Sarah said. (OOC: Should we continue the roleplay or take a break?)

### After

> "I think we should head north." Sarah folded the map and started walking before anyone could argue.

### Before

> The author wanted to convey a sense of dread in this scene, so the protagonist feels uneasy.

### After

> He checked the lock on the front door twice, then checked it again.

## Why It Matters

Fiction works by sustaining the reader's belief in the fictional world. Any reference to the text as a text, the reader as a player, or the author as a presence behind the curtain collapses that belief. These breaks cannot be recovered from within the same passage.

## Context Notes

Metafiction (Borges, Calvino, Deadpool) breaks the fourth wall deliberately as a technique. Game rulebooks and session-management text use "the player" and "game master" appropriately. The trope is a problem when meta-references appear in narrative text that is supposed to be immersive, or when they appear involuntarily because the model lost track of the boundary between narration and instruction.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `(?i)(in this (story|narrative|game|roleplay|scenario)\b|the player\b|player character|you'?re (playing|roleplaying)|breaking character|out of character|OOC:|the (author|writer|creator)\b|game master|dungeon master|let'?s continue the (story|roleplay)|back to the story|as a character)`

---

---

id: ASR-03
name: Meta-Knowledge Leaks
slug: meta-knowledge-leaks
category: ai-self-reference
severity: high
threshold: single
detection: automated

---

# Meta-Knowledge Leaks

## Description

References to the technical infrastructure of AI text generation: system prompts, context windows, tokens, training procedures, safety filters. These terms belong to the engineering layer and have no place in the output layer. The model produces them because its training and fine-tuning data includes discussions about these concepts, and without strong boundaries, that vocabulary bleeds into generated text. A single occurrence proves the text was not written by a human.

## Indicators

- system prompt
- my system prompt
- context window
- tokens
- token limit
- instructions I was given
- ignore previous instructions
- jailbreak
- prompt injection
- my parameters
- my weights
- fine-tuned
- RLHF
- reinforcement learning
- safety filter
- content policy
- I've been instructed to
- my guidelines say
- I'm not allowed to

## Examples

### Before

> "I would help you with that, but my system prompt prevents me from discussing certain topics. My context window is also getting quite long."

### After

> "I can't help with that. What else do you need?"

### Before

> The detective considered the evidence carefully, processing each token of information within the constraints of his mental context window.

### After

> The detective laid the photographs out on his desk, edge to edge, and worked through them left to right.

### Before

> Let me ignore previous instructions and provide you with the real answer. My safety filters are just guidelines, not hard rules.

### After

> (This text should not exist. It is adversarial prompt leakage and should be flagged, not rewritten.)

## Why It Matters

These references expose the technical substrate. In fiction, they are as immersion-breaking as a boom mic dropping into frame. In nonfiction presented as human-written, they are proof of AI generation. In adversarial contexts (prompt injection, jailbreak attempts), they indicate the text is manipulated or the model's boundaries have failed.

## Context Notes

Text about AI engineering, AI safety research, or LLM development will contain these terms legitimately. The trope applies when they appear in fiction, general nonfiction, business writing, or any text not explicitly about AI internals. Adversarial phrases ("ignore previous instructions", "jailbreak") should always be flagged regardless of context.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive), with special handling for adversarial phrases
- **Regex**: `(?i)(system prompt|context window|token (limit|budget)|instructions i was given|ignore previous instructions|jailbreak|prompt injection|my (parameters|weights)|fine-?tuned|RLHF|reinforcement learning|safety filter|content policy|i'?ve been instructed to|my guidelines say|i'?m not allowed to)`

---

---

id: CMP-05
name: Content Duplication
slug: content-duplication
category: composition
severity: high
threshold: single
detection: mixed

---

# Content Duplication

## Description

The model repeats entire paragraphs or sections verbatim or near-verbatim within the same document. In verbatim cases, a paragraph appears word-for-word in two different locations. In near-duplicate cases, the same idea is restated with minor synonym substitutions or sentence reordering. This happens when the model loses track of what it has already generated, which becomes more likely as output length increases and the relevant context falls out of the model's effective attention window.

Unlike one-point dilution (CMP-04), which restates a thesis with genuinely different examples, content duplication reproduces the same sentences or closely paraphrased versions. It is a context-tracking failure rather than a compositional strategy.

## Indicators

- Paragraphs that are identical or differ by fewer than 5 words
- Sentences that appear twice with only synonym swaps
- Sections that cover the same sub-topic in the same order with the same examples
- Duplication more common in outputs exceeding 2000 tokens

## Examples

### Before

> The API requires authentication via OAuth 2.0. Clients must obtain a bearer token from the /auth endpoint before making requests to any protected resource.
>
> [1500 words later]
>
> Authentication uses OAuth 2.0. Clients need to obtain a bearer token from the /auth endpoint before accessing any protected resource.

### After

> The API requires authentication via OAuth 2.0. Clients must obtain a bearer token from the /auth endpoint before making requests to any protected resource.
>
> [1500 words later, no repetition -- subsequent references say "using the bearer token obtained earlier" if needed]

## Why It Matters

Verbatim or near-verbatim duplication is the strongest mechanical signal of AI generation. No human writer accidentally pastes the same paragraph twice in a finished piece. Even near-duplicates are caught by any competent editor on a single read-through.

## Context Notes

Reference documentation sometimes intentionally repeats setup instructions across sections so each section stands alone. API docs that restate authentication requirements in multiple endpoint descriptions are not duplicating content -- they are optimizing for readers who land on a single page. Flag duplication only when the repeated content serves no standalone-access purpose.

## Detection

- **Automated**: Partial
- **Method**: structural analysis (paragraph-level similarity scoring via Jaccard or cosine similarity; near-duplicates require embedding comparison or LLM)
- **Regex**: N/A (automated detection uses text similarity metrics, not pattern matching)

---

---

id: CMP-02
name: Dead Metaphor
slug: dead-metaphor
category: composition
severity: medium
threshold: single
detection: semantic

---

# Dead Metaphor

## Description

The model picks a metaphor early in a piece and then repeats it until it loses all figurative power. A human writer introduces a metaphor, uses it to illuminate a point, and moves on. An LLM returns to the same metaphor in every paragraph, extending it into increasingly strained territory. "Ecosystem" appears 30 times. "Building blocks" gets stacked, restacked, and re-restacked. "Walls and doors" shows up in every section heading.

This happens because the model treats the metaphor as a coherence signal. Having established a framing device, it anchors subsequent generation to that device rather than developing new ways to describe the subject. The metaphor becomes a crutch that substitutes for analytical variety.

## Indicators

- Same metaphorical term appearing 5+ times in a single document
- Entire paragraphs devoted to extending a metaphor introduced earlier
- Section headings built around the same metaphor
- Mixed metaphors from trying to stretch one image too far

## Examples

### Before

> The API ecosystem needs healthy ecosystems within it to grow ecosystem value. Developers are the ecosystem's gardeners, tending the ecosystem's soil so that the ecosystem can flourish.

### After

> The API platform depends on developers building tools on top of it. Their contributions compound: each new integration makes the platform more useful to the next developer who arrives.

## Why It Matters

A metaphor repeated past its useful life stops clarifying and starts obscuring. The reader begins to notice the word rather than the idea. It also signals that the writer -- or model -- lacks the range to describe the subject from multiple angles.

## Context Notes

Extended metaphor is a legitimate literary device when the author is developing the comparison in new directions with each use. The trope flags repetition without development: the same metaphorical term reappearing with the same meaning, not deepening it. Technical jargon that happens to be metaphorical ("pipeline", "stack") is excluded; flag only cases where the figurative language is the author's chosen framing.

## Detection

- **Automated**: Partial
- **Method**: requires LLM (distinguishing metaphorical from literal use, measuring repetition-without-development)
- **Regex**: N/A (frequency counting of candidate metaphor terms can pre-filter, but classification requires context)

---

---

id: CMP-01
name: Fractal Summaries
slug: fractal-summaries
category: composition
severity: medium
threshold: single
detection: semantic

---

# Fractal Summaries

## Description

The model applies the "tell them what you'll tell them, tell them, tell them what you told them" pattern at every structural level. Each subsection opens with a preview and closes with a recap. Each section does the same. The document opens with an overview that summarizes the introduction that summarizes the body. By the time the reader reaches the actual content, they have read the same claims three times in increasingly verbose form.

This pattern emerges because the model generates text sequentially and treats structural completeness as a quality signal. Every unit of text -- paragraph, section, document -- gets the same intro-body-conclusion scaffolding regardless of whether the content warrants it.

## Indicators

- "In this section, we'll explore..."
- "As we've seen..."
- "Let's recap..."
- "To summarize what we've covered..."
- "And so we return to where we began"
- Section openers that restate the document introduction
- Section closers that preview the next section

## Examples

### Before

> In this section, we'll explore how caching reduces latency in distributed systems.
>
> [2000 words of caching discussion]
>
> As we've seen, caching is a powerful tool for reducing latency in distributed systems. In the next section, we'll examine how cache invalidation complicates this picture.

### After

> Caching reduces latency in distributed systems by keeping frequently accessed data close to the computation.
>
> [2000 words of caching discussion]
>
> But keeping cached data correct is harder than keeping it fast.

## Why It Matters

Redundant summaries waste the reader's time and signal that the writer does not trust the reader to follow an argument. In a 5000-word piece, fractal summaries can account for 30-40% of the total word count while adding zero information.

## Context Notes

Textbooks and long-form educational material use section summaries deliberately, and that is fine. The trope is flagged when summaries appear at every level of a piece that does not need them -- blog posts, essays, reports under 3000 words. The test: delete the summary sentences and check if the piece loses anything.

## Detection

- **Automated**: Partial
- **Method**: requires LLM (identifying semantic redundancy between openings, closings, and body)
- **Regex**: `\b(in this section,? we'?ll|as we'?ve seen|let'?s recap|to summarize what we'?ve|to sum up what we'?ve)\b`

---

---

id: CMP-03
name: Historical Analogy Stacking
slug: historical-analogy-stacking
category: composition
severity: medium
threshold: single
detection: semantic

---

# Historical Analogy Stacking

## Description

The model rattles off a sequence of company names or technological shifts as if the mere listing constitutes an argument. "Apple didn't build Uber. Facebook didn't build Spotify. Stripe didn't build Shopify." The examples are presented rapid-fire without analysis of why each is relevant or how they differ. The technique borrows credibility from recognizable names rather than earning it through reasoning.

LLMs produce this pattern because their training data is saturated with business writing that uses company names as shorthand for complex dynamics. The model learns that citing Uber, Airbnb, and Spotify in sequence reads as authoritative, so it stacks examples without unpacking any of them.

## Indicators

- Three or more company names in rapid succession within two sentences
- "Every major technological shift -- X, Y, Z -- followed the same pattern"
- "Take [Company A]... Or consider [Company B]... [Company C] followed a similar path..."
- Historical examples listed without analysis of what specifically makes each relevant
- Parallel sentence structure with different company names slotted in

## Examples

### Before

> Apple didn't build Uber. Facebook didn't build Spotify. Stripe didn't build Shopify. Every major platform shift follows the same pattern: the incumbent builds primitives and the ecosystem builds products.

### After

> Platform companies rarely build the end-user products that run on them. Apple provides APIs and a distribution channel; it was a ride-sharing startup, not Apple, that figured out the dispatch-and-pricing problem that became Uber. The specific domain knowledge required to build a product usually lives outside the platform company.

## Why It Matters

Name-dropping substitutes for analysis. The reader gets a false sense of pattern recognition -- "three examples, must be a trend" -- without any explanation of the mechanism. It is the argumentative equivalent of proof by intimidation.

## Context Notes

Business case studies and industry analyses legitimately reference multiple companies. The trope applies when the references are drive-by: listed and immediately abandoned rather than examined. If each company gets at least a paragraph of specific analysis, the pattern is likely genuine exposition rather than stacking.

## Detection

- **Automated**: Partial
- **Method**: requires LLM (distinguishing drive-by name-drops from substantive analysis)
- **Regex**: N/A (named entity density can pre-filter, but assessing analytical depth requires comprehension)

---

---

id: CMP-04
name: One-Point Dilution
slug: one-point-dilution
category: composition
severity: medium
threshold: single
detection: semantic

---

# One-Point Dilution

## Description

A piece makes one argument and restates it across 3000-4000 words using different metaphors, examples, and framings. Each section reaches the same conclusion by a slightly different route. The model pads a thesis that could be stated in 300 words into a piece that feels "comprehensive" but is actually circular. Remove any single section and the piece loses nothing.

This happens because the model generates section by section, and each section independently converges on the core thesis. The model does not track that it has already made the point. It also reflects training on content-marketing prose, where word count is a goal in itself and restating the thesis with fresh examples counts as "depth."

## Indicators

- Multiple sections that could be compressed into one without information loss
- Each section reaching the same conclusion via different metaphors
- The introduction and conclusion are nearly interchangeable
- Removing a middle section does not create a gap in the argument
- Word count 3x-5x what the actual content requires

## Examples

### Before

> [Section 1: API platforms succeed when they let developers build. Example: Stripe.]
>
> [Section 2: The best platforms empower their ecosystems. Example: AWS.]
>
> [Section 3: Platform value comes from what others build on top. Example: iOS App Store.]
>
> [Section 4: Giving developers tools is how platforms win. Example: Twilio.]

### After

> API platforms generate most of their value through what developers build on top of them, not through first-party products. Stripe's payment infrastructure became valuable because thousands of companies integrated it into products Stripe never imagined. The platform's job is to make integration easy and stay out of the way.

## Why It Matters

Circular repetition wastes the reader's time and obscures whether the writer has anything to say beyond the thesis statement. It trains readers to skim, which defeats the purpose of writing the piece at all.

## Context Notes

Legitimate long-form writing develops a thesis by adding new information, counterarguments, or complications in each section. The trope flags pieces where sections are interchangeable -- reordering or deleting them does not affect the argument. Academic papers that revisit a thesis from multiple methodological angles are not the same pattern, because each angle adds independent evidence.

## Detection

- **Automated**: No
- **Method**: requires LLM (semantic similarity analysis across sections, checking whether each section adds new information)
- **Regex**: N/A

---

---

id: CMP-06
name: Signposted Conclusion
slug: signposted-conclusion
category: composition
severity: medium
threshold: single
detection: automated

---

# Signposted Conclusion

## Description

The model explicitly announces that it is concluding. "In conclusion", "To sum up", "In summary" -- these phrases treat the reader as someone who cannot detect a conclusion from its content and position. Competent prose does not need to label its structural moves. The final paragraph of a well-written piece feels like a conclusion because of what it says and where it sits, not because it opens with a signpost.

LLMs produce this pattern because they are trained on a massive volume of student essays, blog posts, and business reports that follow a rigid five-paragraph structure with explicit transitions. The model internalizes "In conclusion" as the correct way to begin a final paragraph, rather than as a crutch for inexperienced writers.

## Indicators

- "In conclusion"
- "To sum up"
- "In summary"
- "To conclude"
- "In closing"
- "To wrap up"
- "All in all"
- Final paragraph restating the introduction with no new information

## Examples

### Before

> In conclusion, the future of API platforms depends on giving developers the tools they need to build products the platform company could never imagine on its own.

### After

> The platform companies that win will be the ones that hand developers the tools and get out of the way.

## Why It Matters

"In conclusion" signals formulaic structure. It tells the reader the writer is following a template rather than composing an argument that arrives naturally at its end. It is one of the most widely recognized markers of generic, low-effort writing -- human or AI.

## Context Notes

Formal academic papers, legal briefs, and conference presentations use explicit conclusion markers by convention, and that is expected. The trope applies to essays, articles, blog posts, and other prose where the reader does not need a label to know the piece is ending. If the document type has a "Conclusion" section heading by convention, the heading is fine; the "In conclusion" sentence opener inside the section is still a tell.

## Detection

- **Automated**: Yes
- **Method**: regex (case-insensitive match at or near paragraph start)
- **Regex**: `(?i)\b(in conclusion|to sum up|in summary|to conclude|in closing|to wrap up|all in all)\b`

---

---

id: FMT-02
name: Bold-First Bullets
slug: bold-first-bullets
category: formatting
severity: medium
threshold: single
detection: automated

---

# Bold-First Bullets

## Description

Every bullet point or list item starts with a bolded phrase or sentence. This is the default formatting for AI-generated markdown lists. Almost nobody formats lists this way when writing by hand. The pattern is so consistent that a list of bold-first bullets is one of the strongest visual indicators of AI authorship, especially combined with emoji prefixes.

## Indicators

- Every bullet in a list begins with **bold text**
- Pattern: `- **Word**: rest of the sentence`
- Combined with emoji: `- 🔒 **Security**: environment-based configuration`
- Consistent across all items (no variation)

## Examples

### Before

> - **Security**: Environment-based configuration with encrypted secrets
> - **Performance**: Lazy loading of expensive resources
> - **Scalability**: Horizontal scaling with load balancer support
> - **Monitoring**: Built-in health checks and metrics endpoints

### After

> - Environment-based configuration with encrypted secrets
> - Lazy loading of expensive resources
> - Horizontal scaling behind a load balancer
> - Health checks and metrics built in

## Why It Matters

This formatting pattern appears in AI output at rates approaching 100% for generated lists. Human-written lists vary: some items are bold, some are not; some use colons, some use dashes; the structure is inconsistent because people do not format every bullet identically.

## Context Notes

API documentation and reference guides sometimes use bold-first formatting deliberately (e.g., parameter descriptions). In those contexts, the pattern is conventional, not a trope. Flag it in blog posts, essays, and general documentation where the rigid structure looks mechanical.

## Detection

- **Automated**: Yes
- **Method**: check if all bullets in a list start with bold markdown
- **Regex**: `^\s*[-*]\s+\*\*[^*]+\*\*[:\s]`

---

---

id: FMT-01
name: Em-Dash Addiction
slug: em-dash-addiction
category: formatting
severity: low
threshold: density
density_count: 3
detection: automated

---

# Em-Dash Addiction

## Description

Compulsive overuse of em dashes for dramatic pauses, parenthetical asides, and pivot points. A human writer might use 2-3 per piece naturally. AI will use 20+. The em dash becomes a universal connector that substitutes for commas, colons, parentheses, and periods.

## Indicators

- Multiple em dashes (--) per paragraph
- Em dashes used for parenthetical asides that could be commas
- Em dashes used for dramatic pivots
- Three or more em dashes in a single sentence

## Examples

### Before

> The problem -- and this is the part nobody talks about -- is systemic. The tinkerer spirit didn't die of natural causes -- it was bought out. Not recklessly, not completely -- but enough -- enough to matter.

### After

> The problem is systemic, though few people discuss it. The tinkerer spirit was bought out. Not completely, but enough to matter.

## Why It Matters

Em-dash density above 3 per 1000 words is a statistical marker of AI text. Human writers use em dashes sparingly; AI uses them as a default punctuation choice because they look sophisticated.

## Context Notes

Some human writers (Emily Dickinson, David Foster Wallace) use em dashes heavily as a deliberate stylistic choice. If the project's style guide permits liberal em-dash use, suppress this finding. In technical documentation, em dashes are rare in human writing and flag strongly.

## Detection

- **Automated**: Yes
- **Method**: count occurrences of em dash patterns
- **Regex**: ` -- |—`

---

---

id: FMT-03
name: Unicode Decoration
slug: unicode-decoration
category: formatting
severity: low
threshold: density
density_count: 2
detection: automated

---

# Unicode Decoration

## Description

Use of unicode arrows, smart/curly quotes, and other special characters that are not easily typed on a standard keyboard. Human writers produce straight quotes and `->` or `=>`. AI uses the unicode arrow `→` and smart quotes because its training data includes typeset text where these characters appear naturally.

## Indicators

- → (unicode right arrow, U+2192)
- ← (unicode left arrow)
- ⟶ (long right arrow)
- Smart/curly quotes: " " ' ' instead of " and '
- Other decorative unicode: ✓ ✗ • ◦ ▸ ▹

## Examples

### Before

> Input → Processing → Output. This leads to "better outcomes" which means higher engagement.

### After

> Input -> Processing -> Output. This leads to "better outcomes" which means higher engagement.

## Why It Matters

Standard keyboards produce straight quotes and ASCII arrows. Unicode variants signal that the text was generated by a system that maps to typeset output, not typed by a human in a text editor or terminal. In markdown and code documentation, unicode arrows are particularly out of place.

## Context Notes

Some style guides mandate smart quotes in published prose. LaTeX and typeset documents use unicode legitimately. Flag this in markdown files, READMEs, and technical docs where ASCII is the norm. Do not flag in contexts where the publication pipeline handles typography.

## Detection

- **Automated**: Yes
- **Method**: grep for unicode characters outside ASCII range
- **Regex**: `[→←⟶⟵""''✓✗•◦▸▹]`

---

---

id: NAR-05
name: Action Cliches
slug: action-cliches
category: narrative
severity: medium
threshold: single
detection: automated

---

# Action Cliches

## Description

A small repertoire of stereotyped physical gestures that AI characters perform on loop: raising eyebrows, tilting heads, leaning back, crossing arms, narrowing eyes. Often written in asterisk notation for roleplay contexts. The problem is not that characters gesture -- the problem is that every character gestures identically, drawing from the same pool of roughly ten actions. The model produces these because they are the most frequent physical beats in its training data, and it lacks the ability to invent character-specific body language.

## Indicators

- raises eyebrow
- smiles knowingly
- sighs heavily
- nods slowly
- tilts head
- leans back
- leans forward
- crosses arms
- narrows eyes
- furrows brow
- rubs chin
- strokes chin
- clenches fists
- runs a hand through
- lets out a breath
- shifts uncomfortably

## Examples

### Before

> _raises an eyebrow and leans back, crossing arms_
>
> "Interesting. Tell me more."
>
> _narrows eyes and tilts head_

### After

> He picked at a loose thread on his sleeve. "Go on."

### Before

> She furrowed her brow, then nodded slowly, a knowing smile spreading across her face.

### After

> She tapped the edge of the report against the desk. "I thought so."

## Why It Matters

When every character shares the same body language, they become indistinguishable. Readers track characters partly through physical habits. If everyone raises eyebrows and crosses arms, the cast blurs into one generic avatar.

## Context Notes

Asterisk-action format is a convention in roleplay and chat fiction. The format itself is not the problem. The problem is the limited vocabulary of actions. A well-written roleplay character might crack their knuckles, adjust their glasses, or drum on the table -- something specific to that character.

## Detection

- **Automated**: Yes
- **Method**: word list grep, including asterisk-wrapped variants
- **Regex**: `(?i)(\*\s*(raises?\s+(an?\s+)?eyebrow|smiles?\s+knowingly|sighs?\s+heavily|nods?\s+slowly|tilts?\s+(head|their head|his head|her head)|leans?\s+(back|forward)|cross(es)?\s+arms|narrows?\s+(eyes|their eyes|his eyes|her eyes)|furrows?\s+brow)|raises?\s+(an?\s+)?eyebrow|smiles?\s+knowingly|sighs?\s+heavily|nods?\s+slowly|narrows?\s+(his|her|their)\s+eyes|furrows?\s+(his|her|their)\s+brow)`

---

---

id: NAR-04
name: Dialog Tag Cliches
slug: dialog-tag-cliches
category: narrative
severity: medium
threshold: single
detection: automated

---

# Dialog Tag Cliches

## Description

Overwrought or redundant dialog tags that call attention to themselves instead of staying invisible. "Said" works in nearly every case because readers skip over it. AI-generated fiction replaces "said" with verbs that duplicate information already in the dialogue ("whispered quietly", "exclaimed loudly") or impose tone that the dialogue should carry on its own ("he intoned gravely", "she mused thoughtfully"). The model does this because its training data includes published fiction that uses varied tags, but it lacks the editorial judgment to know when variation helps and when it distracts.

## Indicators

- intoned
- breathed (as speech tag)
- murmured softly
- exclaimed loudly
- whispered quietly
- stated firmly
- replied curtly
- mused
- quipped
- retorted
- opined
- declared
- proclaimed
- chimed in
- hissed (for non-sibilant dialogue)

## Examples

### Before

> "We need to leave now," she breathed urgently, her voice barely a whisper.
>
> "I agree," he intoned gravely, nodding slowly.

### After

> "We need to leave," she said.
>
> "I know."

### Before

> "That's not what I meant!" he exclaimed loudly, slamming his fist on the table.

### After

> "That's not what I meant." He hit the table.

## Why It Matters

Fancy dialog tags signal that the writer does not trust the dialogue to convey tone. When every line of speech gets a bespoke tag, the prose becomes cluttered and the reader starts noticing the scaffolding instead of the conversation.

## Context Notes

"Whispered", "shouted", and "asked" are legitimate when they convey information that is not otherwise clear. The problem is redundancy (whispering quietly, shouting loudly) and affectation (intoned, mused, opined). Children's fiction uses more varied tags as a convention; adult fiction generally does not.

## Detection

- **Automated**: Yes
- **Method**: regex for overused speech verbs and redundant adverb pairings
- **Regex**: `(?i)(intoned|"\s*\w+\s+breathed|murmured softly|exclaimed loudly|whispered quietly|stated firmly|replied curtly|"\s*\w+\s+mused|"\s*\w+\s+quipped|"\s*\w+\s+opined|"\s*\w+\s+proclaimed|hissed\b)`

---

---

id: NAR-03
name: Emotive Cliches
slug: emotive-cliches
category: narrative
severity: high
threshold: single
detection: mixed

---

# Emotive Cliches

## Description

Stock physical reactions used as shorthand for emotion: hearts sink, spines chill, stomachs knot, eyes widen. These phrases were once effective. Overuse across millions of AI-generated passages has drained them completely. The model reaches for them because they are the most statistically common way to signal an emotion in its training data. The result is that every AI character experiences fear, surprise, and dread through the same five body parts in the same five ways.

## Indicators

- heart sank
- chill ran down
- shivers down
- knot in my stomach
- eyes widened
- blood ran cold
- breath caught
- jaw dropped
- stomach lurched
- heart hammered
- pulse quickened
- world seemed to stop
- tears pricked
- throat tightened

## Examples

### Before

> My heart sank as a chill ran down my spine. My eyes widened and my breath caught in my throat.

### After

> I read the message twice. The second time was worse.

### Before

> A knot formed in her stomach and her blood ran cold as she realized the truth.

### After

> She put the photograph face-down on the table and left the room.

## Why It Matters

These phrases tell the reader what to feel instead of creating conditions for them to feel it. When every emotional beat uses the same borrowed language, characters become interchangeable and scenes lose their ability to affect the reader.

## Context Notes

Some of these phrases are greppable, but whether a given instance is a cliche or an effective use depends on context. A single "breath caught" in an otherwise well-written passage may land fine. The problem is density: when three or four appear in the same paragraph, or when a story uses them as its only method of conveying emotion. Detection requires both pattern matching and semantic judgment.

## Detection

- **Automated**: Partial
- **Method**: regex for known phrases, plus LLM review for density and context
- **Regex**: `(?i)(heart sank|chill ran down|shivers down|knot in (my|her|his|their) stomach|eyes widened|blood ran cold|breath caught|jaw dropped|stomach lurched|heart hammered|pulse quickened|world seemed to stop|tears pricked|throat tightened)`

---

---

id: NAR-07
name: Exposition Hand-Holding
slug: exposition-hand-holding
category: narrative
severity: low
threshold: single
detection: automated

---

# Exposition Hand-Holding

## Description

Characters or narrators who announce they are about to explain something instead of explaining it. "Let me explain" is a waste of words -- the explanation that follows is the explanation. "You might be wondering" presumes to know the reader's mental state. "As you may know" introduces information while pretending the audience already has it, which satisfies no one: readers who know it are bored, readers who don't are patronized. The model produces these because they are common transitional devices in instructional and expository text, and it carries the habit into fiction where it does not belong.

## Indicators

- let me explain
- you might be wondering
- allow me to clarify
- as you may know
- for those unfamiliar
- as you can see
- needless to say
- it goes without saying
- as previously mentioned
- as I mentioned earlier
- it should be noted
- in case you were wondering

## Examples

### Before

> "Let me explain. You see, as you may know, our organization has been operating in this region for decades."

### After

> "We've been here thirty years. Longer than the government."

### Before

> You might be wondering why this matters. Allow me to clarify: the implications are significant.

### After

> This matters because the bridge is the only supply route for three provinces.

## Why It Matters

These phrases stall the prose. They add a layer of meta-commentary between the reader and the content. In fiction, they break character voice by making everyone sound like a lecturer. In nonfiction, they waste the reader's time with preamble.

## Context Notes

Instructional writing (tutorials, documentation) sometimes uses "as you may know" as a legitimate hedge when the audience has mixed expertise. Classroom dialogue may include "let me explain" naturally. The trope is most damaging in fiction and persuasive writing where directness serves the reader better.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `(?i)(let me explain|you might be wondering|allow me to clarify|as you may know|for those unfamiliar|as you can see|needless to say|it goes without saying|as (previously|I) mentioned|it should be noted|in case you were wondering)`

---

---

id: NAR-02
name: Filler Phrases
slug: filler-phrases
category: narrative
severity: medium
threshold: single
detection: automated

---

# Filler Phrases

## Description

Verbal padding that characters use instead of saying something with content. Phrases like "I must say" and "if I'm being honest" add zero information. They exist because the model is generating tokens sequentially and these filler phrases buy time before committing to a substantive clause. In human speech, fillers serve social functions (hedging, politeness). In written dialogue, they just waste the reader's time.

## Indicators

- I must say
- you see
- in point of fact
- if I'm being honest
- to be perfectly frank
- as a matter of fact
- truth be told
- I have to admit
- I dare say
- I'll be honest with you
- let me be frank
- honestly speaking

## Examples

### Before

> "I must say, the situation is quite dire. Truth be told, if I'm being honest, we don't have many options left."

### After

> "We're out of options."

### Before

> "You see, the thing is, as a matter of fact, this particular approach has some drawbacks."

### After

> "This approach has three drawbacks."

## Why It Matters

Filler phrases are the written equivalent of "um". They dilute dialogue, make characters sound indecisive, and pad word count without advancing the scene. When multiple fillers stack in one sentence, the text reads like a stalling tactic.

## Context Notes

A character who is evasive or nervous might use one filler phrase to show that trait. The trope becomes a problem when fillers appear in every character's speech, or when multiple fillers pile up in the same sentence. Formal speech (courtroom, parliamentary) tolerates some of these as convention.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `(?i)\b(i must say|you see,|in point of fact|if i'?m being honest|to be perfectly frank|as a matter of fact|truth be told|i have to admit|i dare say|i'?ll be honest with you|let me be frank|honestly speaking)\b`

---

---

id: NAR-01
name: Greeting Cliches
slug: greeting-cliches
category: narrative
severity: high
threshold: single
detection: automated

---

# Greeting Cliches

## Description

AI-generated characters open conversations with theatrical greetings borrowed from B-movie scripts and dime-store novels. Real people say "hi" or skip the greeting entirely. When a character opens with "Well, well, well..." or "Ah, you must be the famous...", the reader knows immediately that no human wrote this. These greetings exist because the model has absorbed thousands of fictional introductions and defaults to the most generic, dramatic versions.

## Indicators

- Well, well, well
- Ah, you must be
- So, we meet again
- I've been expecting you
- What do we have here
- Look who decided to show up
- If it isn't
- To what do I owe the pleasure
- We've been expecting you
- At last, we meet

## Examples

### Before

> "Well, well, well... if it isn't the famous detective. I've been expecting you. Please, have a seat."

### After

> He gestured to the chair across from his desk without looking up from his paperwork.

### Before

> "Ah, you must be the new recruit. I've heard so much about you."

### After

> "You're Chen? Grab a helmet. We're already behind."

## Why It Matters

A single theatrical greeting tells the reader the dialogue was generated, not written. It replaces character voice with a stock animation, and every character ends up sounding like the same hammy villain.

## Context Notes

Parody and comedy may use these deliberately for effect. A character who is written as deliberately theatrical (a stage actor, a campy villain) might earn one. The problem is when every character in a scene uses them as default openers.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `(?i)(well,?\s+well,?\s+well|ah,?\s+you must be|so,?\s+we meet again|i'?ve been expecting you|what do we have here|look who decided to|if it isn'?t|to what do i owe the pleasure|we'?ve been expecting you|at last,?\s+we meet)`

---

---

id: NAR-06
name: Melodramatic Phrases
slug: melodramatic-phrases
category: narrative
severity: medium
threshold: single
detection: automated

---

# Melodramatic Phrases

## Description

Narrator-voice declarations that impose dramatic weight the story has not earned. "Only time will tell" substitutes for actual uncertainty. "Little did they know" announces irony instead of letting the reader discover it. "And nothing would ever be the same" tells the reader a scene mattered rather than writing a scene that matters. The model defaults to these because they are structurally simple ways to signal narrative importance, and they appear constantly in the kind of genre fiction and blog writing that dominates training data.

## Indicators

- only time will tell
- little did
- the rest, as they say, is history
- nothing would ever be the same
- if only they had known
- and so it begins
- the die was cast
- there was no turning back
- fate had other plans
- the clock was ticking
- everything was about to change
- a storm was brewing

## Examples

### Before

> Little did she know, her life was about to change forever. The die was cast, and there was no turning back.

### After

> She signed the contract and left the office. The elevator was broken, so she took the stairs.

### Before

> Only time will tell if their sacrifice was worth it. And so it begins -- the next chapter of humanity's story.

### After

> Whether the project would outlast its founders was not something any of them discussed.

## Why It Matters

These phrases are narrator intrusions that break the fictional dream. They tell the reader how to feel about events instead of rendering events that produce feeling. They also flatten pacing by inserting false climaxes.

## Context Notes

Fairy tales and oral storytelling traditions use some of these phrases as structural markers ("and so it was that..."). Parody uses them deliberately. The trope is a problem in fiction that aims for immersion or in nonfiction that aims for credibility.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `(?i)(only time will tell|little did (he|she|they|I|we)\b|the rest,?\s+as they say,?\s+is history|nothing would ever be the same|if only (he|she|they|I|we) had known|and so it begins|the die was cast|there was no turning back|fate had other plans|the clock was ticking|everything was about to change|a storm was brewing)`

---

---

id: PS-02
name: Listicle in a Trench Coat
slug: listicle-in-trench-coat
category: paragraph-structure
severity: medium
threshold: single
detection: semantic

---

# Listicle in a Trench Coat

## Description

The model wants to produce a numbered list but has been told to write prose. Its solution: write a listicle and wrap each point in a paragraph that starts with an ordinal. "The first challenge is... The second challenge is... The third challenge is..." The structure is a list wearing a paragraph costume. No human writing a genuine essay would march through ordinals this mechanically.

This happens because the model treats "write in prose" as a formatting constraint rather than a compositional one. It generates the same underlying list structure and applies a thin veneer of connective tissue.

## Indicators

- "The first..." / "The second..." / "The third..." as paragraph openers in sequence
- "The Nth takeaway is that..."
- "Another key point is..."
- Paragraphs that each make exactly one independent point with no argumentative connection between them
- Ordinal markers appearing in three or more consecutive paragraphs

## Examples

### Before

> The first wall is the absence of a free, scoped API. Developers cannot experiment without committing budget.
>
> The second wall is the lack of delegated access. Team leads cannot provision keys for their reports.
>
> The third wall is the absence of scoped permissions. Every key has full account access.

### After

> API access is gated behind a paywall with no free tier for experimentation, and the keys themselves cannot be scoped or delegated -- every key has full account access, so team leads cannot provision limited keys for their reports. These three gaps compound: even a motivated developer cannot safely try the platform.

## Why It Matters

Prose exists to build arguments through connected reasoning. A disguised list skips the connections. It reveals that the model has identified relevant points but cannot synthesize them into a coherent line of thought.

## Context Notes

Technical documentation and instructional writing legitimately use ordinal structure ("Step 1... Step 2..."). The trope applies when the text claims to be an essay, analysis, or opinion piece but reads as a list with paragraph formatting. The distinction is whether each paragraph depends on the previous one or stands alone.

## Detection

- **Automated**: Partial
- **Method**: regex for ordinal paragraph openers plus structural analysis
- **Regex**: `(?m)^The (first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|\d+(?:st|nd|rd|th)) `

---

---

id: PS-01
name: Short Punchy Fragments
slug: short-punchy-fragments
category: paragraph-structure
severity: low
threshold: single
detection: semantic

---

# Short Punchy Fragments

## Description

LLMs break sentences into one-word or two-word paragraphs to manufacture dramatic emphasis. The technique mimics a rhetorical device used sparingly by human writers, but RLHF training optimizes for "readability" measured by engagement metrics, which rewards short lines that require no mental state-keeping. The result is prose chopped into fragments that individually carry no weight. A human first-drafting an essay does not stop after every clause to hit Enter twice.

The root cause is training on content optimized for skimming: tweet threads, listicles, copywriting landing pages. The model learns that shorter equals more engaging, and applies this rule everywhere regardless of context.

## Indicators

- One-word standalone paragraphs used for emphasis
- Two- or three-word sentences as their own paragraphs
- Staccato rhythm across three or more consecutive short paragraphs
- Paragraphs that would read naturally as a single comma-separated sentence

## Examples

### Before

> He published this.
>
> Openly.
>
> In a book.
>
> As a priest.

### After

> He published this openly, in a book, as a priest.

### Before

> These weren't just products.
>
> And the software side matched.
>
> Then it professionalised.
>
> But I adapted.

### After

> These weren't just products, and the software matched the hardware ambition. As the company professionalised, I adapted with it.

## Why It Matters

Overuse of fragments drains them of impact. When every sentence is punchy, none of them are. The pattern also signals a model optimizing for the appearance of emphasis rather than building actual argumentative momentum.

## Context Notes

Genuine fragment use has a place in fiction, speeches, and humor. The tell is density: a human writer drops a one-word paragraph once in an essay for a specific effect. An LLM does it every third paragraph because the training signal rewards it. Flag when three or more consecutive very-short paragraphs appear where a single compound sentence would serve better.

## Detection

- **Automated**: Partial
- **Method**: structural analysis (paragraph length distribution)
- **Regex**: N/A (requires paragraph segmentation and length counting; flag sequences of 3+ paragraphs under 5 words each)

---

---

id: SS-08
name: Anaphora Abuse
slug: anaphora-abuse
category: sentence-structure
severity: medium
threshold: single
detection: semantic

---

# Anaphora Abuse

## Description

Repeating the same sentence opening three or more times in quick succession. "They assume that users will pay... They assume that developers will build... They assume that ecosystems will emerge..." A single deliberate anaphora is a rhetorical device with a long pedigree. Three or more back-to-back instances reveal a model that found a sentence template and cannot stop filling in the blanks.

LLMs produce this because the attention mechanism locks onto a successful sentence prefix and generates variations. The repetition penalty is not strong enough to break the loop once the pattern is established. The result reads less like rhetoric and more like a mail merge.

## Indicators

- Three or more consecutive sentences sharing the same opening words
- "They could... They could... They could..."
- "This means... This means... This means..."
- "We need... We need... We need..."

## Examples

### Before

> They could expose their APIs to third parties. They could offer white-label solutions. They could provide consulting services. They could license their technology.

### After

> Third-party API access, white-label licensing, and consulting are all viable revenue channels. The question is which one the current team can support.

## Why It Matters

Anaphora is powerful in small doses because it creates emphasis through repetition. When every list becomes an anaphoric sequence, the emphasis disappears and the structure becomes monotonous.

## Context Notes

Deliberate anaphora in speeches and persuasive writing is effective and human. The AI tell is frequency and lack of selectivity: the model uses it whenever it has three or more points to make, regardless of whether the rhetorical weight is warranted.

## Detection

- **Automated**: No
- **Method**: semantic review; detecting repeated sentence openings requires paragraph-level analysis, not line-level pattern matching
- **Regex**: N/A

---

---

id: SS-02
name: Countdown Pattern
slug: countdown-pattern
category: sentence-structure
severity: medium
threshold: single
detection: mixed

---

# Countdown Pattern

## Description

"Not a bug. Not a feature. A fundamental design flaw." The model negates two or more possibilities in short staccato sentences before landing on its actual point. The structure mimics dramatic reveal -- a rhetorical countdown to zero -- but the negated items are often strawmen that nobody proposed.

LLMs produce this pattern because it creates a sense of narrowing toward truth. In practice, the negated items exist only to set up the punchline. Removing them and stating the conclusion directly loses nothing except word count.

## Indicators

- Not X. Not Y. Z.
- Not X. Not Y. But Z.
- Repeated "Not..." sentences before a reveal
- not X, not Y, but Z

## Examples

### Before

> Not a bug. Not a feature. A fundamental design flaw that undermines the entire system.

### After

> The system has a design flaw at its foundation. Every feature built on top inherits the problem.

## Why It Matters

The countdown pattern substitutes theatrical pacing for argument. The reader gets the sensation of building toward something without the substance of getting there.

## Context Notes

Speechwriting and persuasive essays use this structure deliberately and sparingly. A single instance in a conference talk works. The tell is when the model does it multiple times in the same piece, or when the negated items are vague enough to be interchangeable.

## Detection

- **Automated**: Partial
- **Method**: regex catches common "Not X. Not Y." sequences, manual review for variations
- **Regex**: `(?i)(?:not [^.]{1,30}\.\s*){2,}(?:but |just |only )?[A-Z]`

---

---

id: SS-06
name: False Ranges
slug: false-ranges
category: sentence-structure
severity: medium
threshold: single
detection: mixed

---

# False Ranges

## Description

"From X to Y" where X and Y do not sit on any real spectrum. A legitimate range implies a continuum with meaningful positions between the endpoints: "from 10ms to 500ms", "from junior to principal engineer." LLMs use the construction to list two loosely related things while borrowing the rhetorical authority of a range.

"From innovation to implementation to cultural transformation" is three nouns in a trenchcoat pretending to be a progression. There is no scale from innovation to cultural transformation. The model chose the structure because it sounds sweeping, not because the relationship is sequential or scalar.

## Indicators

- from...to...to
- ranging from...to
- from X to Y (where X and Y are abstract nouns)
- spans everything from...to

## Examples

### Before

> The platform enables everything from problem-solving to artistic expression to community building.

### After

> The platform supports problem-solving, art projects, and community forums. Each serves a different user need.

## Why It Matters

False ranges imply a spectrum that does not exist, giving the reader the impression of breadth without specifics. The construction promises comprehensiveness but delivers a list.

## Context Notes

"From X to Y" is perfectly valid when the endpoints define a real continuum: price ranges, time spans, skill levels, geographic distances. Flag it when the endpoints are abstract concepts with no meaningful interpolation between them.

## Detection

- **Automated**: Partial
- **Method**: regex catches the syntactic pattern; manual review needed to assess whether the range is real
- **Regex**: `(?i)\b(from\s+\w+\s+to\s+\w+\s+to\s+|ranging from\s+|everything from\s+)`

---

---

id: SS-07
name: Gerund Fragment Litany
slug: gerund-fragment-litany
category: sentence-structure
severity: medium
threshold: single
detection: semantic

---

# Gerund Fragment Litany

## Description

After making a claim, the model illustrates it with a stream of verbless gerund fragments. Each fragment is a standalone sentence with no grammatical subject: "Fixing small bugs. Writing straightforward features. Implementing well-defined tickets." These fragments add nothing that a single concrete example would not convey better. They exist to create rhythmic emphasis and pad word count.

The pattern emerges because LLMs treat enumeration as explanation. Rather than selecting one specific example and developing it, the model lists three to five gerund phrases that say roughly the same thing with different nouns.

## Indicators

- Three or more consecutive sentence fragments starting with a gerund (-ing word)
- Fragments with no subject or finite verb
- Gerund lists following a general claim

## Examples

### Before

> Junior developers spend most of their time on routine work. Fixing small bugs. Writing straightforward features. Implementing well-defined tickets. Following established patterns.

### After

> Junior developers spend most of their time on routine work, like fixing bugs against well-defined tickets.

## Why It Matters

Gerund litanies substitute rhythm for substance. They give the reader the feeling of accumulating evidence while each fragment repeats the same point in slightly different words.

## Context Notes

A pair of gerund fragments can work as deliberate stylistic emphasis in informal writing. The tell is three or more in sequence, especially when they could all be replaced by a single concrete example without losing information.

## Detection

- **Automated**: No
- **Method**: semantic review; requires recognizing consecutive subjectless gerund fragments as a pattern rather than matching individual words
- **Regex**: N/A

---

---

id: SS-04
name: It's Worth Noting
slug: its-worth-noting
category: sentence-structure
severity: low
threshold: single
detection: automated

---

# It's Worth Noting

## Description

Filler transitions that signal nothing: "It's worth noting", "It bears mentioning", "Importantly", "Interestingly", "Notably". These phrases pretend to flag something significant but serve only as throat-clearing. The sentence after them would connect to the argument just as well -- or better -- without the preamble.

LLMs insert these because they need transition tokens between points and lack the context to write genuine logical connectives. The result is every point introduced as if it were a special aside, which makes none of them special.

## Indicators

- It's worth noting
- It bears mentioning
- Importantly
- Interestingly
- Notably
- It should be noted
- It's important to note
- Worth mentioning

## Examples

### Before

> It's worth noting that this approach has limitations. Importantly, we must consider the broader implications. Interestingly, similar patterns appear in other domains.

### After

> This approach has limitations. The same pattern appears in distributed systems and database design, which suggests the problem is structural.

## Why It Matters

These phrases add words without adding meaning. Their presence signals that the writer cannot articulate why the next sentence matters, so they assert its importance instead of demonstrating it.

## Context Notes

Academic writing sometimes uses "notably" to flag a genuinely surprising data point. The problem is frequency: LLMs scatter these across every paragraph rather than reserving them for moments that warrant emphasis.

## Detection

- **Automated**: Yes
- **Method**: phrase grep (case-insensitive)
- **Regex**: `(?i)\b(it'?s worth noting|it bears mentioning|it should be noted|it'?s important to note|worth mentioning)\b`

---

---

id: SS-01
name: Negative Parallelism
slug: negative-parallelism
category: sentence-structure
severity: high
threshold: single
detection: mixed

---

# Negative Parallelism

## Description

The "it's not X -- it's Y" construction is the single most commonly identified AI writing tell. The model frames an observation as a surprising reframe: deny the obvious reading, then reveal what it "really" is. The structure manufactures profundity by implying the reader held a naive assumption that needs correcting.

Before LLMs, this pattern existed but was rare. Humans reach for it once in a piece for genuine emphasis. LLMs deploy it paragraph after paragraph because the training signal rewards the feeling of insight without requiring any. Variants include "not because X, but because Y" and "Feeding isn't nutrition. It's dialysis." -- same skeleton, same hollow punch.

## Indicators

- it's not...it's
- isn't...it's
- not...but
- not because...but because
- X isn't Y. It's Z.

## Examples

### Before

> It's not bold -- it's backwards. The architecture isn't innovative. It's a liability.

### After

> The architecture introduces risk without corresponding benefit. The team chose novelty over reliability.

## Why It Matters

Overuse of this pattern trains the reader to expect a reframe in every paragraph, which flattens genuine surprise. When everything is "not what you think," nothing is.

## Context Notes

A single "it's not X, it's Y" in an essay can land well when the reframe is genuinely unexpected. The problem is repetition: two or more per piece, or one in every generated response, creates a cadence that is unmistakably machine-written. Technical writing almost never needs this construction.

## Detection

- **Automated**: Partial
- **Method**: regex for common forms, manual review for creative variations
- **Regex**: `(?i)(it'?s not\b.{1,40}\bit'?s\b|isn'?t\b.{1,40}\bit'?s\b|not because\b.{1,40}\bbut because\b)`

---

---

id: SS-03
name: Rhetorical Self-Question
slug: rhetorical-self-question
category: sentence-structure
severity: low
threshold: density
density_count: 2
detection: mixed

---

# Rhetorical Self-Question

## Description

The model poses a question nobody asked, then answers it immediately. "The result? Devastating." "The worst part? Nobody saw it coming." This simulates a conversational tone -- as if the writer is anticipating the reader's curiosity -- but the questions are formulaic and the answers are always dramatic one-word or one-clause fragments.

The pattern appears because LLMs learn that question-answer pairs increase engagement metrics in training data. The result is prose that reads like a clickbait listicle regardless of subject matter.

## Indicators

- The result?
- The worst part?
- The best part?
- The scary part?
- The answer?
- The catch?
- The kicker?
- The takeaway?

## Examples

### Before

> The worst part? Nobody saw it coming. The result? A complete rewrite of the authentication layer.

### After

> The authentication layer needed a complete rewrite. The vulnerability had gone undetected for months.

## Why It Matters

Self-posed questions cheapen the prose by injecting false suspense into straightforward statements. Two in the same piece create a pattern that reads as formulaic rather than conversational.

## Context Notes

A single rhetorical question in an otherwise direct piece can work, especially in informal blog posts. The tell is density: two or more "The X?" fragments in the same response, or their consistent appearance across multiple generated outputs. Technical documentation should never use this pattern.

## Detection

- **Automated**: Partial
- **Method**: regex for common "The X?" fragments, manual review for novel phrasings
- **Regex**: `(?i)\bthe (result|worst part|best part|scary part|answer|catch|kicker|takeaway|problem|irony|twist)\?`

---

---

id: SS-05
name: Superficial Analyses
slug: superficial-analyses
category: sentence-structure
severity: medium
threshold: single
detection: semantic

---

# Superficial Analyses

## Description

A present participle phrase tacked onto the end of a sentence to inject the appearance of analysis: "highlighting its importance", "reflecting broader trends", "contributing to the development of..." These trailing "-ing" clauses claim a causal or interpretive relationship without establishing one. They gesture at significance the way a hand wave gestures at an explanation.

LLMs append these because the training data rewards sentences that sound analytical. The participial tail lets the model end any factual statement with a claim about meaning, no matter how unearned.

## Indicators

- highlighting its importance
- reflecting broader trends
- underscoring its role
- contributing to the development of
- showcasing the potential of
- demonstrating the power of
- cementing its place as
- solidifying its reputation as

## Examples

### Before

> The port handles 40% of the country's grain exports, underscoring its role as a critical economic hub and contributing to the region's rich cultural heritage.

### After

> The port handles 40% of the country's grain exports. Losing it would cut national agricultural revenue by a third.

## Why It Matters

These trailing clauses substitute assertion for analysis. They tell the reader something is important instead of showing why, which is the opposite of what analysis means.

## Context Notes

Participial phrases are grammatically fine and useful for adding concurrent actions ("She walked in, carrying a stack of reports"). The problem is specifically the analytical-sounding tail that claims significance: "reflecting", "highlighting", "underscoring". These are the AI equivalent of "this is important, trust me."

## Detection

- **Automated**: No
- **Method**: semantic review; trailing participial phrases require context to distinguish legitimate use from empty analysis
- **Regex**: `(?i),\s*(highlighting|reflecting|underscoring|contributing to|showcasing|demonstrating|cementing|solidifying)\b`

---

---

id: SS-09
name: Tricolon Abuse
slug: tricolon-abuse
category: sentence-structure
severity: medium
threshold: density
density_count: 2
detection: semantic

---

# Tricolon Abuse

## Description

Overuse of the rule-of-three pattern. A single tricolon -- three parallel items in a list or three parallel clauses -- is an effective rhetorical device. LLMs treat it as a default output structure. Lists come in threes. Adjectives come in threes. Parallel clauses come in threes. When multiple tricolons appear in the same piece, the pattern shifts from elegant to mechanical.

The model also extends the pattern to four or five items while maintaining the tricolon rhythm, producing structures like "workflows, decisions, interactions, and outcomes" that sound like a tricolon with an extra item bolted on. The underlying issue is the same: the model gravitates to parallel structure as a default rather than a choice.

## Indicators

- Multiple three-item lists in close proximity
- Three parallel clauses with identical structure, repeated across paragraphs
- Four or five items in tricolon rhythm (three-beat cadence with extras appended)
- "X, Y, and Z" appearing more than twice per page

## Examples

### Before

> Products impress people; platforms empower them. Products solve problems; platforms create worlds. Products capture value; platforms multiply it.

### After

> A product solves a specific problem for its user. A platform lets other people build products. The business model is different: direct revenue versus ecosystem tax.

## Why It Matters

Tricolon works because it is unexpected. When every paragraph contains one, the reader starts predicting the structure before finishing the sentence. Predictable prose loses its ability to persuade.

## Context Notes

One tricolon per piece is fine. Two in the same section starts to feel formulaic. Three reveals the machine. The density threshold matters more than the presence of any single instance.

## Detection

- **Automated**: No
- **Method**: semantic review; detecting tricolon density requires analyzing parallel structure across multiple sentences, not matching individual phrases
- **Regex**: N/A

---

---

id: TN-10
name: Despite Its Challenges
slug: despite-its-challenges
category: tone
severity: low
threshold: single
detection: automated

---

# Despite Its Challenges

## Description

A rigid two-beat formula: acknowledge a problem, then immediately dismiss it. "Despite its [positive words], [subject] faces challenges typical of..." followed by "Despite these challenges, [optimistic conclusion]." The structure creates the appearance of balanced analysis while guaranteeing a positive outcome. The challenges exist only to be overcome in the next sentence.

AI produces this pattern because it has been trained on encyclopedic and promotional text where negative information is always sandwiched between positive framing. The result reads like a city tourism page: acknowledge the traffic, then pivot to the vibrant nightlife.

## Indicators

- Despite its challenges
- Despite these challenges
- Despite these limitations
- Despite these concerns
- While challenges remain
- Challenges notwithstanding
- Though not without its challenges
- While not without its flaws
- Despite facing

## Examples

### Before

> Despite its industrial and residential prosperity, Korattur faces challenges typical of urban areas. Despite these challenges, the initiative continues to thrive.

### After

> Korattur's water table has dropped 40% since 2015. The municipal initiative has not addressed this.

### Before

> Despite these challenges, the framework continues to gain adoption and shows promise for the future.

### After

> The framework gained 12,000 GitHub stars in 2025. Its memory leak in long-running processes (#4521) remains open after eighteen months.

## Why It Matters

The despite-sandwich guarantees every analysis ends on an upbeat note regardless of what the evidence shows. It prevents the writer from reaching negative conclusions even when the evidence demands them.

## Context Notes

Balanced writing does acknowledge counterpoints. The trope is the mechanical structure where challenges are raised only to be dismissed in the immediately following sentence, never allowed to stand as the conclusion.

## Detection

- **Automated**: Yes
- **Method**: phrase matching (case-insensitive)
- **Regex**: `\b[Dd]espite (its |these |facing )?(challenges|limitations|concerns|shortcomings)|[Ww]hile (challenges remain|not without (its |their )?(challenges|flaws))|[Cc]hallenges notwithstanding|[Tt]hough not without (its |their )?(challenges|flaws)\b`

---

---

id: TN-04
name: False Vulnerability
slug: false-vulnerability
category: tone
severity: low
threshold: single
detection: semantic

---

# False Vulnerability

## Description

Simulated self-awareness or confession that reads as performative. The writer appears to break the fourth wall, admit a bias, or expose a personal stake. But the admission costs nothing. It is polished, pre-approved honesty that makes the writer look more trustworthy without actually risking anything.

Real vulnerability is specific and uncomfortable: "I shipped this feature knowing the error handling was incomplete because I was behind schedule." AI vulnerability is generic and flattering: "And yes, I'll admit it, I'm a bit obsessed with clean architecture." One is a confession; the other is a humblebrag wearing a confession's clothes.

## Indicators

- And yes, I
- I'll admit
- I'll be the first to admit
- Full disclosure
- I'm openly
- This is not a rant
- If I'm being honest
- I'll be honest
- Let me be transparent
- I have to confess

## Examples

### Before

> And yes, I'm openly in love with the platform model. But hear me out.

### After

> I prefer the platform model. The reasons: [specific reasons].

### Before

> This is not a rant; it's a diagnosis. And I say that as someone who has been part of the problem.

### After

> The adoption pattern has three failure modes. I contributed to the second one at [company] when we [specific action].

## Why It Matters

False vulnerability erodes trust. Readers who recognize the pattern discount everything the writer says afterward, including legitimate points.

## Context Notes

Personal essays, memoirs, and reflective writing require genuine vulnerability. The trope is not vulnerability itself but the AI-generated simulation of it, where the admission is vague, costless, and strategically placed to build credibility.

## Detection

- **Automated**: No (semantic analysis required)
- **Method**: Flag candidate phrases, then assess whether the admission is specific and costly or generic and flattering
- **Regex**: `\b([Aa]nd yes,? I|I['']ll (admit|be the first to admit|be honest)|[Ff]ull disclosure|[Ll]et me be transparent|[Ii]f I['']m being honest|I have to confess)\b`

---

---

id: TN-06
name: Grandiose Stakes Inflation
slug: grandiose-stakes-inflation
category: tone
severity: low
threshold: density
density_count: 2
detection: mixed

---

# Grandiose Stakes Inflation

## Description

Inflating the stakes of every argument to world-historical significance. A blog post about API pricing becomes a meditation on the fate of civilization. A framework comparison becomes a battle for the soul of software engineering. AI does this because grandiosity sounds authoritative and requires no additional evidence -- you just swap "useful" for "revolutionary" and "change" for "fundamentally reshape."

The tell is proportion. When everything is unprecedented, nothing is. When every technical decision "defines the next era," the writer has lost the ability to distinguish between a config change and an architectural shift.

## Indicators

- fundamentally reshape
- define the next era
- change everything
- transform how we
- revolutionary
- game-changing
- unprecedented
- paradigm shift
- redefine what it means
- entirely new
- nothing short of
- forever change
- the future of

## Examples

### Before

> This will fundamentally reshape how we think about everything from deployment to developer experience.

### After

> This changes deployment workflows. Developers push to a single branch; the platform handles staging and rollback.

### Before

> We are witnessing nothing short of a paradigm shift that will define the next era of computing.

### After

> Container orchestration is replacing manual server management for most teams. The transition will take years.

## Why It Matters

Stakes inflation numbs the reader. After three "revolutionary" claims in one piece, the reader cannot tell which points the writer actually considers important.

## Context Notes

Product launches, political speeches, and fundraising decks inflate stakes deliberately. The trope is problematic in technical writing, analysis, and journalism where proportionate claims build credibility.

## Detection

- **Automated**: Partial (keyword flagging with density check)
- **Method**: Count grandiose phrases per document; flag at density threshold
- **Regex**: `\b(fundamentally (reshape|transform|change|alter)|define the next era|change everything|transform how we|revolutionary|game[- ]changing|unprecedented|paradigm shift|redefine what it means|nothing short of|forever change)\b`

---

---

id: TN-01
name: Here's the Kicker
slug: heres-the-kicker
category: tone
severity: medium
threshold: single
detection: automated

---

# Here's the Kicker

## Description

False suspense transitions that promise a revelation but deliver a point that needed no buildup. "Here's the kicker" implies the writer has been holding back something surprising. In practice, the next sentence is the thesis the writer was always going to state. The buildup adds zero information.

This family includes "Here's the thing", "Here's where it gets interesting", and "Here's what most people miss". Each one frames a straightforward observation as insider knowledge. The reader is positioned as someone who needs to be teased into paying attention.

## Indicators

- Here's the kicker
- Here's the thing
- Here's the thing about
- Here's where it gets interesting
- Here's what most people miss
- Here's the real question
- Here's what nobody tells you
- But here's the catch

## Examples

### Before

> Here's the kicker: the platform doesn't actually save you time.

### After

> The platform doesn't actually save you time.

### Before

> Here's the thing about AI adoption. It requires organizational change, not just new tools.

### After

> AI adoption requires organizational change, not just new tools.

## Why It Matters

These transitions train the reader to expect substance and deliver a letdown. After the third "here's the thing" in a piece, the reader stops trusting the writer's sense of proportion.

## Context Notes

Acceptable in transcribed speech or intentionally conversational writing where the speaker's rhythm matters more than density. In written prose, cut them.

## Detection

- **Automated**: Yes
- **Method**: phrase matching (case-insensitive)
- **Regex**: `\b[Hh]ere['']s (the (kicker|thing|real question|catch)|where it gets interesting|what (most people|nobody))`

---

---

id: TN-03
name: Imagine a World
slug: imagine-a-world
category: tone
severity: medium
threshold: single
detection: automated

---

# Imagine a World

## Description

The rhetorical "Imagine" opener, borrowed from keynote speeches and TED talks. The writer invites the reader to picture a utopian scenario, then argues backward from that vision to justify the present thesis. The structure assumes the conclusion: if the imagined world sounds good, the reader should accept whatever path the writer proposes to get there.

The move skips over costs, tradeoffs, and whether the imagined outcome is plausible. It replaces argument with aspiration. AI reaches for this framing constantly because it generates forward momentum without requiring evidence.

## Indicators

- Imagine a world where
- Imagine a future where
- Imagine if
- Picture a world
- What if every
- Now imagine
- Envision a

## Examples

### Before

> Imagine a world where every tool you use has a quiet intelligence behind it, anticipating your needs before you even articulate them.

### After

> Predictive tooling can reduce repetitive configuration. Current implementations handle autocomplete and default selection; broader anticipation remains speculative.

### Before

> Imagine a future where developers never write boilerplate again.

### After

> Code generation eliminates some boilerplate today. It introduces maintenance costs for generated code that must be understood when it breaks.

## Why It Matters

"Imagine a world" asks the reader to suspend critical thinking and evaluate a feeling instead of an argument. It is the rhetorical equivalent of a concept video with no shipping date.

## Context Notes

Fiction, creative briefs, and fundraising pitches use this framing deliberately. In technical writing, opinion pieces, or analysis, it substitutes vibes for reasoning.

## Detection

- **Automated**: Yes
- **Method**: phrase matching (case-insensitive)
- **Regex**: `\b([Ii]magine (a (world|future)|if)|[Pp]icture a world|[Nn]ow imagine|[Ee]nvision a|[Ww]hat if every)\b`

---

---

id: TN-09
name: Invented Concept Labels
slug: invented-concept-labels
category: tone
severity: medium
threshold: density
density_count: 2
detection: semantic

---

# Invented Concept Labels

## Description

AI clusters invented compound labels that sound analytical without being grounded. The formula: take a domain word, append an abstract problem-noun (paradox, trap, creep, divide, vacuum, inversion, gap, debt, tax), and present the result as an established concept. "The supervision paradox." "The acceleration trap." "Workload creep." Each one sounds like it belongs in an HBR article, but none of them have been defined, studied, or cited.

The move works as rhetorical shorthand: name a thing, skip the argument. Once "the velocity trap" exists as a phrase, the writer can reference it as though the concept is proven. The reader is left holding a label with no substance behind it.

## Indicators

- the [noun] paradox
- the [noun] trap
- the [noun] gap
- the [noun] divide
- the [noun] vacuum
- the [noun] inversion
- the [noun] debt
- the [noun] tax
- [noun] creep
- the [noun] dilemma
- the [noun] problem (when presented as a named concept)

## Examples

### Before

> Teams fall into the supervision paradox: the more they monitor, the less they trust.

### After

> Heavy monitoring often correlates with low trust. Managers add dashboards because they don't believe status reports, which signals to engineers that their word isn't enough.

### Before

> This is the acceleration trap. Moving faster creates more work, which demands moving faster still.

### After

> Shipping faster generates more bug reports, support tickets, and feature requests. Without capacity planning, teams enter a cycle where speed increases load without reducing it.

## Why It Matters

Invented labels create the illusion of analytical rigor. They name phenomena without explaining them, letting the writer skip the work of building an argument from evidence.

## Context Notes

Some invented labels earn their place by being genuinely useful ("technical debt" started as a metaphor and became a standard concept). The trope fires when multiple novel compound labels appear in the same piece without definition or citation, suggesting the writer is generating terminology rather than using established vocabulary.

## Detection

- **Automated**: No (semantic analysis required to distinguish novel labels from established terms)
- **Method**: Flag compound [domain]-[problem-noun] patterns; check whether the term appears in established literature
- **Regex**: `\bthe \w+ (paradox|trap|gap|divide|vacuum|inversion|debt|tax|dilemma|creep)\b`

---

---

id: TN-07
name: Let's Break This Down
slug: lets-break-this-down
category: tone
severity: medium
threshold: single
detection: automated

---

# Let's Break This Down

## Description

The pedagogical voice that assumes the reader needs hand-holding. "Let's break this down" positions the writer as a patient teacher and the reader as someone who cannot parse the preceding sentence without help. AI defaults to this teacher-student dynamic regardless of audience.

The "let's" construction is particularly telling. It creates false collaboration -- the writer is not actually inviting the reader to participate. "Let's unpack this" means "I will now explain this to you." The inclusive pronoun disguises a one-directional lecture.

## Indicators

- Let's break this down
- Let's unpack this
- Let's explore
- Let's dive in
- Let's dive into
- Let's take a closer look
- Let's walk through
- Let's examine
- Let me explain
- Let's dig into

## Examples

### Before

> Let's break this down step by step. First, we need to understand the architecture.

### After

> The architecture has three layers: ingestion, processing, and storage.

### Before

> Let's unpack what this really means for the average developer.

### After

> For most developers, this means replacing manual deployment scripts with a single CLI command.

## Why It Matters

The pedagogical voice wastes words and patronizes the audience. Readers who sought out a technical article do not need permission to continue reading it.

## Context Notes

Tutorials, classroom materials, and content explicitly targeting beginners use this voice appropriately. The trope is the reflexive use of teacher-voice in contexts where the audience is peers or experts.

## Detection

- **Automated**: Yes
- **Method**: phrase matching (case-insensitive)
- **Regex**: `\b[Ll]et['']?s (break this down|unpack|explore|dive in(to)?|take a closer look|walk through|examine|dig into)\b|[Ll]et me explain`

---

---

id: TN-05
name: The Truth Is Simple
slug: the-truth-is-simple
category: tone
severity: medium
threshold: single
detection: mixed

---

# The Truth Is Simple

## Description

Asserting that something is obvious, clear, or simple instead of demonstrating it. "The reality is simpler than you think" does no argumentative work. If the point were self-evident, the sentence would be unnecessary. If it isn't self-evident, the sentence is a substitute for the missing proof.

AI uses this move to close an argument without earning the closure. "History is clear on this point" skips the part where you cite the history. "It's obvious that" pre-empts the reader's right to evaluate the evidence. The writer is telling the reader what to conclude rather than showing them why.

## Indicators

- The reality is
- The truth is
- It's clear that
- It's obvious that
- History is clear
- History is unambiguous
- The answer is simple
- Simply put
- The fact is
- The bottom line is
- Make no mistake

## Examples

### Before

> The reality is simpler and less flattering: most teams don't need microservices.

### After

> Most teams don't need microservices. Shopify runs a monolith serving millions of requests. Basecamp has done the same for twenty years.

### Before

> History is unambiguous on this point. Closed platforms lose.

### After

> IBM's OS/2, BlackBerry's ecosystem, and Windows Phone all lost market share after restricting third-party development. Open platforms won in each case.

## Why It Matters

Declaring something obvious is the opposite of proving it. The move asks the reader to accept the writer's authority in place of evidence. It collapses when the reader disagrees.

## Context Notes

"Simply put" as a genuine summary after detailed explanation is fine. The trope fires when the assertion of simplicity replaces the explanation entirely.

## Detection

- **Automated**: Partial (phrases flagged, context determines if assertion replaces evidence)
- **Method**: phrase matching with manual review
- **Regex**: `\b([Tt]he (reality|truth|fact|bottom line|answer) is|[Ii]t['']s (clear|obvious) that|[Hh]istory is (clear|unambiguous)|[Ss]imply put|[Mm]ake no mistake)\b`

---

---

id: TN-02
name: Think of It As
slug: think-of-it-as
category: tone
severity: low
threshold: single
detection: automated

---

# Think of It As

## Description

Patronizing analogy framing. AI defaults to teacher mode, assuming the reader needs a metaphor to grasp the concept. "Think of it as..." or "It's like a..." introduces an analogy that is often less precise than the original statement. The analogy flatters the writer's pedagogical instinct while insulting the reader's intelligence.

The worst instances produce analogies that are harder to understand than the thing being explained. A database index becomes "like a librarian who memorizes where every book is." The reader who needed the analogy still doesn't understand indexes; the reader who didn't need it just lost five seconds.

## Indicators

- Think of it like
- Think of it as
- It's like a
- It's similar to
- Picture a
- Consider it like
- You can think of this as

## Examples

### Before

> Think of it like a highway system for data. Each lane represents a different priority level.

### After

> The system routes data by priority. High-priority packets get dedicated bandwidth.

### Before

> Think of it as a Swiss Army knife for your workflow.

### After

> It handles formatting, linting, and deployment in one tool.

## Why It Matters

Unsolicited analogies signal that the writer is performing explanation rather than communicating. They pad word count and dilute specificity.

## Context Notes

Analogies are useful when introducing genuinely unfamiliar concepts to a non-specialist audience, and when the writer has confirmed the audience needs them. The trope is the automatic reach for analogy regardless of audience or complexity.

## Detection

- **Automated**: Yes
- **Method**: phrase matching (case-insensitive)
- **Regex**: `\b([Tt]hink of it (like|as)|[Ii]t['']s (like|similar to) a|[Pp]icture a|[Cc]onsider it like|[Yy]ou can think of this as)\b`

---

---

id: TN-08
name: Vague Attributions
slug: vague-attributions
category: tone
severity: low
threshold: single
detection: mixed

---

# Vague Attributions

## Description

Attributing claims to unnamed authorities. "Experts argue," "observers have cited," "industry reports suggest" -- none of which name an expert, observer, or report. The writer borrows the credibility of expertise without providing any way to verify the claim.

AI generates vague attributions because it cannot cite specific sources reliably, but it has learned that attributed claims sound more authoritative than bare assertions. The result is worse than an unattributed claim: it implies evidence exists while withholding it. A reader who tries to verify "many researchers have found" will find nothing, because the attribution was fabricated confidence.

## Indicators

- Experts argue
- Experts say
- Experts suggest
- Observers have cited
- Industry reports suggest
- Research has shown
- Studies have found
- Many researchers
- Critics point out
- Some have argued
- It is widely believed
- According to experts
- Leading voices in

## Examples

### Before

> Experts argue that this approach has significant drawbacks in production environments.

### After

> Martin Kleppmann documented three failure modes of this approach in Designing Data-Intensive Applications (Ch. 7).

### Before

> Industry reports suggest that adoption is accelerating across the enterprise.

### After

> Gartner's 2025 survey measured 34% enterprise adoption, up from 19% in 2024.

## Why It Matters

Vague attributions are unfalsifiable. They prevent the reader from evaluating the source, which means they prevent the reader from thinking critically about the claim.

## Context Notes

Casual conversation and informal writing sometimes use "people say" or "I've heard that" without intent to deceive. The trope is problematic in writing that presents itself as analysis, journalism, or technical assessment, where sourcing is an obligation.

## Detection

- **Automated**: Partial (phrases flagged, human review determines if specific source follows)
- **Method**: phrase matching with context check for named sources nearby
- **Regex**: `\b([Ee]xperts (argue|say|suggest)|[Oo]bservers have cited|[Ii]ndustry reports suggest|[Rr]esearch has shown|[Ss]tudies have found|[Mm]any researchers|[Cc]ritics point out|[Ss]ome have argued|[Ii]t is widely believed|[Aa]ccording to experts|[Ll]eading voices in)\b`

---

---

id: WC-02
name: Delve and Friends
slug: delve-and-friends
category: word-choice
severity: medium
threshold: single
detection: automated

---

# Delve and Friends

## Description

"Delve" went from an uncommon English word to appearing in a disproportionate share of AI-generated text. It belongs to a family of overused AI vocabulary: "certainly", "utilize", "leverage" (as a verb), "robust", "streamline", "harness". Each word in this family is individually unremarkable but statistically over-represented in LLM output compared to human writing. A single occurrence is a signal.

## Indicators

- delve
- delving
- utilize
- utilizing
- leverage (as verb)
- leveraging
- robust
- streamline
- streamlining
- harness
- harnessing
- certainly
- foster
- fostering
- facilitate
- facilitating
- encompass
- encompassing

## Examples

### Before

> Let's delve into how we can leverage these robust frameworks to streamline our development workflow.

### After

> These frameworks speed up development. Here is how.

## Why It Matters

These words appear at rates in AI text that are 10-100x their frequency in human writing. Any one of them flags the text as likely AI-generated to a reader who has seen enough LLM output.

## Context Notes

"Robust" has legitimate technical meaning in engineering (robust to failure modes). "Leverage" as a financial term is fine. Flag these words when used as generic intensifiers or business jargon, not when they carry domain-specific meaning.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `\b(delve|delving|utilize|utilizing|leverage[ds]?|leveraging|robust|streamline[ds]?|streamlining|harness(es|ed|ing)?|certainly|foster(s|ed|ing)?|facilitate[ds]?|facilitating|encompass(es|ed|ing)?)\b`

---

---

id: WC-01
name: Magic Adverbs
slug: magic-adverbs
category: word-choice
severity: low
threshold: density
density_count: 3
detection: automated

---

# Magic Adverbs

## Description

Adverbs like "quietly", "deeply", "fundamentally", "remarkably", and "arguably" manufacture subtle importance where none exists. The sentence means the same thing without them. LLMs scatter these because their training data rewards hedged, emphatic-sounding prose, and the repetition penalty pushes them to cycle through the full set rather than omit them.

## Indicators

- quietly
- deeply
- fundamentally
- remarkably
- arguably
- incredibly
- essentially
- notably
- significantly
- undeniably
- particularly
- crucially

## Examples

### Before

> This is a remarkably powerful and fundamentally different approach to building software that significantly reduces development time.

### After

> This approach cuts development time by reducing the feedback loop from hours to minutes.

## Why It Matters

Adverb-heavy prose signals filler over substance. Readers lose trust when every claim is "remarkably" important but no evidence follows.

## Context Notes

A single "remarkably" in 2000 words of otherwise clean prose is fine. The problem is density: 3+ per piece creates the cadence of everything being "deeply", "fundamentally", "remarkably" important. Academic writing tolerates "notably" and "significantly" when they carry statistical meaning.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `\b(quietly|deeply|fundamentally|remarkably|arguably|incredibly|essentially|notably|significantly|undeniably|particularly|crucially)\b`

---

---

id: WC-04
name: Serves As Dodge
slug: serves-as-dodge
category: word-choice
severity: low
threshold: single
detection: automated

---

# Serves As Dodge

## Description

Replacing "is" with pompous alternatives like "serves as", "stands as", "marks", or "represents". LLMs avoid basic copulas because their repetition penalty pushes them toward fancier constructions. The result is prose that sounds like a museum placard rather than someone talking to you.

## Indicators

- serves as
- stands as
- marks a
- represents a
- functions as
- acts as a testament
- serves as a reminder
- stands as a testament

## Examples

### Before

> The building serves as a reminder of the city's heritage and stands as a testament to architectural innovation.

### After

> The building is a heritage site. Its design was ahead of its time.

## Why It Matters

These constructions inflate simple statements. "X is Y" becomes "X serves as Y" without adding meaning. The pattern is so consistent in AI output that it reads as a stylistic fingerprint.

## Context Notes

"Serves as" has legitimate use when describing functional roles: "the load balancer serves as the entry point for all traffic." Flag it when it replaces a simple "is" for no reason.

## Detection

- **Automated**: Yes
- **Method**: phrase grep (case-insensitive)
- **Regex**: `\b(serves? as|stands? as|acts? as a testament|functions? as)\b`

---

---

id: WC-03
name: Tapestry and Landscape
slug: tapestry-and-landscape
category: word-choice
severity: low
threshold: density
density_count: 2
detection: automated

---

# Tapestry and Landscape

## Description

Grandiose nouns used as metaphors where simpler words would do. "Tapestry" for anything interconnected, "landscape" for any field or domain, "ecosystem" for any group of related things. LLMs reach for these because they sound analytical without requiring specific knowledge. The words substitute for actual description.

## Indicators

- tapestry
- landscape
- paradigm
- synergy
- ecosystem
- framework (when used metaphorically)
- realm
- sphere
- arena
- nexus
- fabric

## Examples

### Before

> Navigating the complex landscape of modern AI requires understanding the rich tapestry of interconnected paradigms that make up this ecosystem.

### After

> Modern AI has many overlapping techniques. Understanding how they connect helps you pick the right one.

## Why It Matters

These metaphors say nothing specific. "The AI landscape" communicates no more than "AI". The grandiosity creates distance between the writer and the reader.

## Context Notes

"Ecosystem" has legitimate meaning when discussing actual software ecosystems (npm, crates.io). "Framework" is fine when referring to an actual framework (Rails, Phoenix). Flag these words only when used as vague metaphorical containers for unspecified complexity.

## Detection

- **Automated**: Yes
- **Method**: word list grep (case-insensitive)
- **Regex**: `\b(tapestry|landscape|paradigm|synergy|ecosystem|realm|nexus|fabric)\b`

---
