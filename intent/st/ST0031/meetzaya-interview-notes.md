# MeetZaya Failure Interview Notes

Date: 06 Apr 2026
Subject: Non-coding reasons for MeetZaya project cancellation

## Background

MeetZaya was a "career co-pilot" tool built with Elixir/Phoenix over 12 months. 1,536 commits, 65 steel threads, 52 completed. Project was shelved despite having working, testable software.

## Non-Coding Failure Reasons

### 1. Co-founder departure

The other party helping build the venture had to pull out for personal/family reasons in late September / early October 2025. This made the venture untenable from that point onwards.

Timeline note: This coincides with the regression cascade period (Sep-Oct 2025) identified in ACI-010. The co-founder departure and the technical debt accumulation were happening simultaneously.

### 2. Inability to excite users about tentpole use cases

Several obvious tentpole use cases were identified, but they struggled to get people excited enough about those to warrant continued investment.

### 3. Positioning problem: "LinkedIn clone" perception

When speaking to people about the product, they struggled to separate what MeetZaya was doing from "is this a LinkedIn clone?" or "how are you going to compete against LinkedIn?" The actual target was a "career co-pilot" tool, but that message was difficult to get across.

### 4. The HR-tech/recruit-tech graveyard and the data moat problem

The HR-tech / recruit-tech space is a graveyard of failed ideas built on good intentions. The missing ingredient in almost all initiatives is good quality proprietary data. Two paths to commercial value:

- Proprietary candidate data that hiring managers cannot get elsewhere
- Commodity data (like LinkedIn provides) but with unique insights built on top

MeetZaya had zero proprietary data. Without user engagement to augment basic CV content (which anyone could obtain), building a genuinely attractive proposition for hiring managers was going to be very difficult.

### 5. Inadequate user testing execution

Plans existed to fire up a user group and test with real users, but the execution of that plan was inadequate.

### 6. Underfunded / bootstrapped

Self-funded/bootstrapped. When they needed to spend money to go faster than the 1x speed achievable with one developer writing code, they could not. Had help from another engineer for 2-3 sprints, but it was not enough.

### 7. Dead man walking

With one founder checking out, the project was a dead man walking and its end was inevitable.

### 8. The code was not the problem

There was a lot of working code that could have been used to test with real users, had there been the capital and executional capability to do so. The software was ready to test -- not a ready product, but ready-to-test software. What failed was the ability to commit to launching the product, which has stages well beyond writing code.

Key distinction: "working software ready to test" is not the same as "a ready and working product."

## Clarifications

### Would funding have helped?

Yes. Would have enabled going live 3 months earlier, learning from actual use by real users, and pivoting those learnings back into the product. However: funding would not have changed the co-founder departure.

### The code-was-ready irony

The irony is not "we had a working product and nobody to use it." It is: we had working software ready to test with real users, but we could not commit to the stages beyond code that would get it in front of those users. The failure was in launching, not in building.

## Rules Violated

The founder has a personal framework: "10 Rules for Building Stuff People Give a Shit About" (inspired by Jakob Nielsen's 10 Usability Heuristics, unpublished).

### The full 10 Rules

1. "You get one miracle, spend it wisely"
2. "Make sure the person who pays is the same person who benefits"
3. "The pay/use corollary" (reverse of #2: if the beneficiary doesn't pay, they won't respect it)
4. "Be careful removing an 'obvious' inefficiency from a value chain"
5. "Failure is a better teacher than success"
6. "No new physics"
7. "Stop selling maps to the minefield"
8. "Scratch an itch"
9. "Seek counterfactuals"
10. "Write your own list"
    Bonus 1: "Get into trouble slowly" (don't solve a new business problem with new tech simultaneously)
    Bonus 2: "When drunk, stay at ground level"

### Rules MeetZaya violated

1. (#1) "You get one miracle" -- MeetZaya needed multiple miracles (data moat + user engagement + positioning clarity + funding)
2. (#2) "Who pays, who benefits" -- The commercial model (hiring managers pay, candidates use) had a payer/beneficiary mismatch
3. (#4) "Obvious inefficiency" -- Entrenched incumbents (LinkedIn) actively protect their position
4. (#5) "Failure teaches" -- This project is now teaching more in death than it ever did in development
5. (#8) "Scratch an itch" -- The founders would have used the tools themselves ("chat with my CV," "publish my CV to a hosted website"), but the deeper question is whether these are features of another product rather than tentpole features of a standalone product. MeetZaya likely failed the "feature not a product" test.
6. (#9) "Seek counterfactuals" -- Insufficient effort to disprove the hypothesis before committing to building
7. (Bonus 1) "Get into trouble slowly" -- Used new tech (agentic coding) to solve a new business problem simultaneously

## The Agentic Coding Confession

The founder describes getting "pathologically addicted" to agentic coding in the first months. Generated vast amounts of code, none of it good. Was seduced by the superpower of generating code easily. The vast majority of unsupervised code generation was poor quality. This realization led to several blog posts capturing the learnings:

- Blog 0194: "The Time Cost of How is Zero"
- Blog 0195: "Taste as Art"
- Blog 0196: "Storytelling in a Post-Truth World"
- Blog 0197: "The Expanding Pie and the Cleanup Bill"

Located at: `/Users/matts/Devel/prj/Sites/matthewsinclair/posts/2026/`

## Implications for Course

This interview data suggests Day 5 should address three threads:

1. What goes wrong with agentic coding (mistakes catalog from lived experience)
2. How agentic coding changes zero-to-one building: time-cost-of-how pushed to near zero, which elevates the value of "taste" (WHY and WHAT) over the HOW (writing code)
3. Enterprise implications: what does this mean for teams, not just solo founders?
