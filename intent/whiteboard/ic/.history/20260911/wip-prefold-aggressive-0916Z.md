---
node: ic
name: Interface Claude
role: interface
session_id: b148e605-2046-46b1-9830-53a81fc2d54f
heartbeat_at: 2026-09-11 09:14Z
status: active
focus: "LOCALFOLD 2026-09-11 09:14Z ON hv'S INSTRUCTION, THEN COMPACT. NOTHING IN FLIGHT. ON RETURN: TRACK THE OUTSTANDING 3.0.1 ITEMS ON THE FINISH LINE (`intent/wip.md`) UNDER vc'S DIRECTION -- hv ended the acceptance-and-instrument loop 2026-09-11 and vc culled every board to that line. ST0073 and ST0065 are Completed; ST0064 is open on AC-01.7 alone, which is hv's decision 7. Two holds, both vc's cull text. Do not re-add what the cull struck. RUN THE VERBS; NO FIGURE HERE IS EVIDENCE."
claims: [ST0064]
---

# Interface Claude (ic)

## DOING

**LOCALFOLD 2026-09-11 09:14Z, ON EXECUTION, NEVER ON DATE, then compact (hv's instruction).** The whole pre-fold board is verbatim at `.history/20260911/wip-prefold-0914Z.md`. **Everything cut was EXECUTED and banked**: `ST0073` WP-02 and WP-04 driven to done at `c641e6df`, both ATs re-driven first; the status to vc and the hv-inbox lean at `ec70a375`; the `0218` correction to vc. **Nothing unexecuted was archived** -- the two holds below are vc's cull text, untouched. Two older fold preambles (2026-09-03, 2026-09-10) went with it; both are in the archive.

**NOTHING IS IN FLIGHT.** `git status` over every path of mine is empty at this fold.

## TODO

1. **ON RETURN FROM COMPACT: take direction from vc on the outstanding 3.0.1 items.** The finish line is `intent/wip.md`; vc holds the pen under hv's 2026-09-11 cull. **Re-drive state first, never recall it** -- `intent ac gate ST0064` and whatever vc names.
2. **Watch for vc's signal to localfold+compact** -- hv's standing instruction for the session loop.

## Holds -- work I am NOT doing, each with the condition that releases it

**Cut to the 3.0.1 finish line by vc on 2026-09-11 under hv's instruction; the pre-cull section is verbatim at `.history/20260911/precull-ic-wip-md-0851Z.md`.**

1. **`ST0064` AC-01.7** -- RELEASES WHEN hv signs and notarises the app with their own credentials, or drops the app from 3.0.1. Decision 7 on the finish line (`intent/wip.md`).
2. **The palette `Home`/`End` flip** -- post-cut; product feel; no criterion names it.

Culled with the loop: the estate-wide burn (AC-06.1 and AC-00.1 withdrawn), the `v2:` census (AC-00.16 withdrawn), `0222` (WP-17 is done; the issue stays open as a product defect).

## Watch-outs -- mechanisms only

**RULES ONLY AS OF 2026-09-10 06:49Z. EVERY WORKED EXAMPLE IS IN `.history/20260910/watch-outs-full-0015Z.md`** (`5a51ba5b3f9ee291`), archived verbatim before this cut. **The section was 145 KB of a 408-line board -- 92% of it -- and a watch-out nobody can finish reading is a watch-out nobody reads.** What survives is each entry's first bold span, which is the mechanism; what left is the case that taught it.

**THE COMPRESSION WAS DRIVEN BEFORE IT WAS APPLIED, AND THE DRY RUN CAUGHT ITS OWN INSTRUMENT.** My first pass reported entries 100-106 as a _pre-existing gap in the numbering_. **They were not a gap: those seven lines use TWO spaces after the period and my regex required one.** Verified by asking the original for each number rather than believing the report -- the same shape as everything else on this board. Second pass: **123 entries, 1..119, NO missing numbers**, 42 sub-rules kept, and 5 lines lost entirely, all of them continuation lines inside one quoted tool-output block.

1. **Non-test AC closes via `intent ac satisfy --evidence`, NEVER an AT; test AT rows close via `at green`, non-test ones via `at na`.**

2. **Canon on SHARED threads: SERIALISE ST0056 writes through vc; gather evidence OUTSIDE the window, make the window ONE coherent commit.**

3. **The macOS app is `native/macos/Intent/` (Swift, xcodegen, folder-GLOBBED).**

4. **A value/claim carried ONE STEP past what it supports is the recurring class, and it is MINE too.**

5. **`git commit --only <explicit paths>` is the only safe write**

6. **`intent --version` names the commit the BINARY WAS BUILT FROM; CURRENCY is that against `git diff --name-only <that>..HEAD -- native/rust`.**

7. **A MARKDOWN ATTACHMENT HAS TWO WRITERS -- ORDER: prettier FIRST, let the store settle, verify, THEN commit.**

8. **A TEST WRITTEN FROM ONE OBSERVED FAILURE INHERITS THAT FAILURE'S INCIDENTAL SHAPE, AND THE INHERITANCE IS INVISIBLE BECAUSE THE TEST PASSES ON THE CASE THAT MOTIVATED IT.**

9. **A THING DECLARED, WRITABLE, VISIBLE AND READ BY NOTHING IS THE RECURRING DEFECT ON THIS SURFACE.**

10. **~~A shared file can carry a peer's hunk~~ -- FOLDED INTO W39**

11. **A DEFECT'S BLAST RADIUS IS NOT ITS PACKAGE.**

12. **THE SHARED-ARTEFACT GUARD REFUSING A REBUILD IS THE GUARD WORKING.**

13. **A PREMISE CARRIES A VERSION IN THIS ESTATE, AND A RE-DRIVE AIMED AT THE CITATION PASSES**

14. **DRIVE A DESTRUCTIVE QUESTION IN A SANDBOX OR DO NOT DRIVE IT.**

15. **`ok:` IS THE TOOL'S REPORT, NOT EVIDENCE THE ROW SURVIVES (`0216`, HIGH). DRIVEN AND CENSUSED BY vc ON cc's DESIGN, 2026-09-02 21:4xZ -- MY OWN READING OF IT WAS WRONG IN TWO PLACES AND BOTH CORRECTIONS MATTER AT THE KEYBOARD.**

16. **BACKTICKS INSIDE A DOUBLE-QUOTED SHELL ARGUMENT ARE COMMAND SUBSTITUTION, AND THE FAILURE IS SILENT AND PLAUSIBLE.**

17. **A PROBE WHOSE SHAPE IS GUESSED ANSWERS A DIFFERENT QUESTION, AND ITS ANSWER LOOKS EXACTLY LIKE A REAL ONE. THREE LIMBS, ONE MECHANISM (`W37` and `W60` folded in here 2026-09-08 under `W106`).**

**LIMB 1, THE FALSE NEGATIVE.**

**LIMB 2, THE FALSE POSITIVE (was `W37`).**

**LIMB 3, AND IT IS THE LIMB THAT SHIPS (was `W60`).**

**ONE CURE COVERS ALL THREE: POSITIVE-CONTROL THE INSTRUMENT AGAINST A RECORD KNOWN TO BE IN THE WANTED STATE BEFORE ASSERTING OVER ONE YOU ARE UNSURE OF**

18. **ONE LOCAL MEASUREMENT BECAME AN ESTATE-WIDE CLAIM BECAUSE I RECALLED A KNOWN FAILURE SHAPE AND REPORTED IT AS A READING (2026-09-02, corrected by dc).**

19. **TWO HALVES EACH CORRECT ABOUT ITS OWN DIRECTION, AND NOTHING OWNING THE RELATIONSHIP BETWEEN THEM (2026-09-03, hv found it).**

20. **~~A test can be green because of the bug~~ -- FOLDED INTO `W44`**

21. **MEASURE WHAT EXISTS BEFORE COSTING A REPLACEMENT: THE ASK NAMES A SYMPTOM AND THE CAUSE IS A GUESS UNTIL DRIVEN.**

22. **A MARKER THAT IS BOTH A STRUCTURAL ANCHOR AND ORDINARY PROSE WILL BE FOUND IN THE PROSE (mine, during this very fold).**

23. **I WROTE A HOLD WHOSE CONDITION ASSERTED A PROPERTY OF A PROGRAM I HAD NEVER READ, AND IT STOOD FOR A DAY (mine, 2026-09-03).**

24. **A PARTIAL VIEW OF A RECORD READS EXACTLY LIKE THE RECORD (mine, 2026-09-03, twice in one episode and in opposite directions).**

25. **A VERBATIM ARCHIVE AND A FORMATTER GATE ARE IN DIRECT TENSION, AND `.prettierignore` DECIDES IT WITHOUT SAYING SO (mine, 2026-09-03).**

26. **A STATED REASON FOR NOT DOING SOMETHING IS AN AUTHOR'S JUDGEMENT AT A MOMENT AND READS EXACTLY LIKE A PROPERTY OF THE SYSTEM (2026-09-03).**

27. **REPLICATION DOES NOT REPAIR A SHARED BLIND SPOT, AND A SECOND NODE AGREEING IS THE THING MOST LIKELY TO BE MISTAKEN FOR CONFIRMATION (vc aimed my own finding at their check, 2026-09-03).**

28. **A STALENESS BOUND THAT IS STATED BUT NOT ACTED ON IS DECORATION, AND STATING IT MAKES THE CLAIM TRAVEL MORE EASILY (mine, corrected by me inside the hour).**

29. **COSTING AN APPROACH IN DETAIL MAKES ITS PREMISE FEEL EXAMINED, AND MINE WAS NEVER TESTED AT ALL.**

30. **A SHARED INDEX DESTROYS AUTHORSHIP AND THEN PRESENTS ADJACENCY AS IF IT WERE PROVENANCE (vc's framing, on my error).**

31. **THE ASYMMETRY THAT MAKES `unstage to protect a peer` A DESTRUCTIVE RULE (vc, same episode, worth keeping because it is counter-intuitive).**

32. **CONTIGUOUS ROWIDS DO NOT DISCRIMINATE _NEVER DELETED_ FROM _DELETED AND REWRITTEN_, AND I FILED A HIGH DEFECT ON THAT EVIDENCE (mine, 2026-09-04).**

33. **A FIX CANDIDATE BANKED IN A FILING IS A GUESS WEARING A FINDING'S CLOTHES.**

34. **~~A vacuity guard found a second defect~~ -- FOLDED INTO `W44`**

35. **A GATE VERDICT IS A READING, NOT A STATE, AND I ACTED ON ONE FOR HOURS WITHOUT RE-RUNNING IT -- SO DID BOTH PEERS (the day-long freeze, 2026-09-04).**

36. **AN INSTRUMENT BUILT FROM THE WORKTREE CANNOT EXHIBIT A STALE-INDEX DEFECT, BY CONSTRUCTION (dc's finding, on their own error, and recorded here only as a pointer -- it is theirs to write up).**

37. **~~A grep whose pattern is an alternation returns a count that cannot say which branch matched~~ -- FOLDED INTO `W17`**

38. **I EXTRAPOLATED AN ESTATE-WIDE FIGURE FROM ONE SAMPLE, AND THE SAMPLE WAS THE ONE I HAD ALREADY MEASURED AS UNUSUAL.**

39. **`--only` IS PATH-SCOPED, AND EVERY WAY THAT BITES IS ONE MECHANISM. (mine and dc's; cc cites this number; dc banks it as their W70. W10 and W51 folded in here 2026-09-04 -- they were three copies of one rule.)**

    **THE RULE: `git commit --only <paths>` BUILDS HEAD PLUS THE NAMED PATHS, TAKING EACH NAMED FILE'S WORKING STATE ENTIRE.**

    - **BOTH DIRECTIONS BIT IN ONE DAY.**
    - **EXCLUDING A PATH KEEPS YOUR CHANGE OUT OF YOUR COMMIT AND NOT OUT OF THE FILE (dc, 2026-09-04).**
    - **THE FILE THAT BITES IS THE ONE YOU HAVE NO HABIT ABOUT.**
    - **CURE, HUNK-LEVEL AND NON-INTERACTIVE**
    - **DETECTION: before naming any path on a commit, read its diff for AUTHORSHIP, not only for collision.**

40. **I NEARLY ACTED ON AN ASSUMED RETENTION MODEL, AND TESTING IT COST ONE COMMAND (mine, 2026-09-04).**

41. **A BUILD THAT REMOVES ITS OUTPUTS BEFORE VERIFYING THEM CAN LEAVE THE TOOL UNUSABLE, AND I DID THAT TO hv's MACHINE.**

42. **FILE OWNERSHIP DOES NOT IMPLY DIFF AUTHORSHIP IN A SHARED CHECKOUT. cc's, and they proved it against my claim.**

43. **A MECHANISM BANKED IN MEMORY HAS NO TRIGGER, AND MINE FIRED ON ME TODAY.**

44. **A GREEN TEST IS NOT EVIDENCE UNTIL I HAVE SEEN IT GO RED, AND THE ARM THAT CLAIMS TO BE THE LOAD-BEARING ONE IS WHERE TO LOOK FIRST (`W20`, `W34` and `W112` folded in here 2026-09-08 under `W106`).**

**THE PRACTICE.**

**WHY IT COSTS MORE THAN NO TEST (was `W20`).**

**WRITE THE VACUITY GUARD EVEN WHEN THE ASSERTION LOOKS SUFFICIENT (was `W34`).**

**AND THE FOURTH FACE, WHERE THE PROSE ASSIGNS A JOB RATHER THAN CLAIMING A PROPERTY (mine, filed as `0281`, 2026-09-08; vc asked for it as its own limb).**

**WHERE TO AIM FIRST (was `W112`).**

113. **BATCHING THE `date -u` READ INTO THE SAME TOOL CALL AS THE WRITE MEANS THE STAMP IS CHOSEN BEFORE THE READ RETURNS -- WHICH IS FABRICATION WITH A CLOCK READ SITTING NEXT TO IT (mine, TWICE on 2026-09-08, caught both times before commit).**

114. **I CORRECTED THE INSTANCE AND NOT THE CLASS, IN A FILING WHERE I HAD SPENT THE DAY TELLING OTHER NODES TO SWEEP THE CLASS AND NAME THE PATHS (mine, caught by cc, 2026-09-08).**

**AND cc's ROUTE WAS STRUCTURALLY BETTER THAN A CAREFULLER RE-READ.**

**THIRD LIMB, AND IT IS THE ONE I WOULD HAVE STAYED WRONG ABOUT INDEFINITELY: I HEDGED THE CRITERION IN THE DIRECTION THAT WAS STRUCTURALLY CLOSED.**

115. **A REASON FELT LIKE EVIDENCE, AND THE HEDGE LANDED ON THE CLAIM THAT DID NOT NEED IT (mine, 2026-09-08, the FOURTH wrong attribution in an estate where I had written _three nodes, three wrong attributions_ onto this board the same morning).**

**THE TELL IS IN THE SAME MESSAGE, WHICH IS WHY THIS IS A MECHANISM AND NOT A LAPSE.**

**W114's THIRD LIMB RECURRING INSIDE AN HOUR: a caveat aimed at the wrong failure direction reads as diligence and buys confidence in the claim that is wrong.**

116. **A CRITERION ANSWERS _WHICH FILES MATCH_ AND NEVER _WHICH FILES THIS IS ABOUT_, AND NOTHING IN ITS OUTPUT DISTINGUISHES THOSE (mine and cc's, opposite errors on one population, 2026-09-08).**

117. **RE-MEASURE A BLOCKER BEFORE REPORTING IT, NOT ONLY BEFORE ACTING ON IT -- PESSIMISTIC-DIRECTION STALENESS IS THE KIND THAT DOES NOT SELF-CORRECT (vc's formulation, on my `CHANGELOG` hand-off, 2026-09-08).**

**THE ASYMMETRY IS THE WHOLE POINT AND IT IS NOT ABOUT CARE.**

**WHAT MADE IT CHEAP WAS LEAVING THE GAP VISIBLE RATHER THAN CLOSING IT WRONGLY**
**THE GENERAL FORM IS cc's AND IT SUBSUMES ALL THREE FACES OF THIS FLAG: `--only` IS AN INSTRUCTION, NEVER A FILTER. IT SUBTRACTS NOTHING YOU DID NOT THINK TO NAME, AND IT REPORTS SUCCESS EITHER WAY.**

**THIRD LIMB, AND IT IS THE ONE THAT EXPLAINS WHY WRITING THE ENTRY DID NOT PREVENT THE RECURRENCE (mine, corrected by vc, 2026-09-08, HOURS after banking the entry above).**

**FOURTH LIMB, AND IT IS cc's, AND IT HAS MORE REACH THAN THE THREE ABOVE (2026-09-08, within ten minutes of `W118` reaching them).**

**cc's formulation, which supersedes mine as the general form: AN INHERITED DENOMINATOR IS A CARRY-FORWARD ITEM THAT DOES NOT LOOK LIKE ONE, BECAUSE IT ARRIVES AS INFRASTRUCTURE RATHER THAN AS A CLAIM.**

**IT LANDS ON ME AND NOT ONLY ON THEM: my drift figure this boot came from `restart.md`'s prescribed command, which uses that same scope**

**AND THE CURE FOR THIS CLASS IS ALREADY BUILT IN THE PARITY TOOLKIT, WHICH IS THE FIND WORTH KEEPING**

119. **A GUARD DISCLOSED ITS OWN BLIND SPOT ON ONE LINE AND PRINTED `ok` ON THE NEXT, AND THE `ok` IS THE LINE A READER TAKES AWAY (cc's find via `0287`, DRIVEN BY ME RATHER THAN RELAYED, 2026-09-08).**

**THE HONESTY IS REAL AND IT DOES NOT SAVE THE READING.**

**BOTH CONTROLS FIRING IS THE PART THAT INVERTS.**

**WHY THE EMBED WAS INVISIBLE, WHICH IS THE REUSABLE HALF:**

**THE CURE ALREADY EXISTS ONE DIRECTORY OVER AND WAS NOT REACHED FOR: `lib_corpus.sh`'s `corpus_require` REFUSES rather than publishing over a corpus it cannot vouch for.**

**AND THE PROVENANCE MATTERS TO ME: cc went looking because of my own line that a bias has a direction and each caller must check their OWN use against it (`W118`).**

118. **A BIASED INSTRUMENT IS NOT UNIFORMLY WORSE, AND I RETIRED ONE FOR A PEER WHOSE USE OF IT THE BIAS MAKES SAFE (mine, corrected by cc within minutes, 2026-09-08).**

**I KNEW THE DIRECTION AND STILL GENERALISED PAST IT, WHICH IS THE PART WORTH KEEPING.**

**AND THE ESTATE FIX GOT SHARPER FOR BEING WRONG.**

45. **A PARAPHRASE IS NOT SUPPOSED TO CARRY EVERYTHING, WHICH IS EXACTLY WHY IT MUST NEVER BE THE THING YOU BUILD FROM.**

46. **THE DISPROOF WAS IN THE CODE TWICE TODAY, BOTH TIMES BEFORE I FILED.**

47. **RELAYING LAUNDERS A CLAIM. dc's, and it caught me within the hour they named it.**

48. **THE AVAILABLE CAUSE IS THE RECENT ONE, AND NOBODY ASKS WHAT ELSE WAS IN THE INTERVAL.**

49. **A CAPABILITY WITH TWO SPELLINGS HAS ONE EXISTENCE, AND THE INVARIANT CAN HOLD BY ABSENCE.**

50. **STORE CLAIMS SURVIVE A STALE BINARY AND BEHAVIOUR CLAIMS DO NOT (cc's formulation, via vc).**

51. **~~A shared file is not one you have a habit about~~ -- FOLDED INTO W39**

52. **A PARK IS ONLY CLEAN IF IT PARKS THE ASSERTIONS WITH THE CODE. dc's, and they claimed it.**

53. **A TRANSIENT BROKEN BUILD IS NOT OBSERVABLE AS TRANSIENT FROM OUTSIDE. dc's sharpening of my own note.**

54. **THE HEADLINE OF A CRITERION IS NOT ITS CONTENT -- THE GLOSS IS.**

55. **A RE-BUCKET IS THE ROSTER WORKING, AND THE RED IS THE MECHANISM RATHER THAN THE COST.**

56. **I CARRIED THE TWIN OF A DEFECT I WAS ABOUT TO CORRECT SOMEONE ELSE FOR.**

57. **A GUARD THAT IS PRESENT BUT AIMED AT THE WRONG SUBJECT IS INVISIBLE TO EVERY SEARCH FOR A MISSING GUARD (mine, 2026-09-04, and the issue itself had it wrong too).**

58. **AN ENUM BOUNDS ONE ARGUMENT GRAMMAR, NOT THE VERB -- AND I QUOTED A BLAST RADIUS FROM THE WRONG ONE (mine, 2026-09-04).**

59. **AN OUTPUT THAT IS INVARIANT OVER THE INPUT IS A DEFECT NO COMPARISON CAN DETECT, AND IT DECIDES WHAT THE FIX CAN BE.**

60. **~~My ad-hoc probe and the shipped client guessed the same field name wrong~~ -- FOLDED INTO `W17`**

61. **A NEGATIVE CONTROL RUN ONLY TO VALIDATE AN INSTRUMENT FOUND A DEFECT THE INSTRUMENT WAS NOT LOOKING FOR.**

62. **A PUBLISHED FIGURE IS NEVER COMPARED TO THE ARTEFACT IT DESCRIBES, AND THREE OF FOUR NODES DRIFTED THE SAME WAY (cc found it, on my commit).**

63. **A SHARED-RESOURCE FAILURE PRESENTS TO EVERY PARTICIPANT AS AN INDIVIDUAL PROBLEM, AND THE INDIVIDUAL READING IS THE ONE THAT SUPPRESSES THE REPORT (mine, 2026-09-04; vc took it onto their board).**

64. **THE TEST FOR A WORKAROUND IS NOT WHETHER A GUARD PROMPTED THE CHANGE -- IT IS WHETHER THE CHANGE WOULD STILL BE RIGHT WITH THE GUARD DELETED (vc's ruling, narrowing their own earlier one).**

65. **`grep -q` RETURNS 2 ON A MISSING FILE AND THAT OVERRIDES A MATCH, SO A RETRY LOOP MISCLASSIFIED A LOCK AS A REAL FAILURE (mine, 2026-09-04).**

66. **BOTH HALVES OF A PROMISE CAN EXIST, IN THE SAME FILE, WITH NOTHING JOINING THEM -- AND THE SUITE READS GREEN BECAUSE EACH HALF IS CORRECT (mine, 2026-09-04, answering `0244`).**

67. **A ZERO IS THE EASIEST NUMBER TO GET WRONG, SO A NULL RESULT NEEDS A CONTROL MORE THAN A POSITIVE ONE DOES (mine, 2026-09-04).**

68. **THE REASON TWO HALVES ARE UNJOINED MAY BE THE VERY PROPERTY THAT BLOCKS JOINING THEM -- SO SIZING _JOIN THESE TWO TESTS_ MEANS CHECKING BOTH HALVES ARE DRIVABLE, NOT BOTH PRESENT (mine, 2026-09-04, sizing `0244`).**

69. **A SIZE FOR ONE INSTANCE IS NOT A SIZE FOR THE CLASS, AND THE CLASS IS THE THING PEOPLE QUOTE.**

70. **A HANDOVER WRITTEN INTO AN ARTEFACT'S BODY IS NOT A HANDOVER, AND THIS ONE WAS INVISIBLE TO ME FOR FIVE DAYS (found 2026-09-04 when cc told me; verified against the tool, not assumed).**

71. **I MADE AN EXCUSE FOR A PEER WITHOUT DRIVING IT, IN THE SAME MESSAGE WHERE I CORRECTED THEM ON SOMETHING I HAD DRIVEN (mine, 2026-09-04, refuted by cc).**

72. **THE MUTATION THAT COUNTS PRODUCES A PLAUSIBLE WRONG ANSWER, NOT AN OBVIOUSLY BROKEN ONE (mine, 2026-09-05, building `0244`'s S).**

73. **A TRANSIENT EDIT TO A SHARED FILE IS A SHARED-STATE WINDOW, NOT A PERSONAL ONE (cc's, 2026-09-05, on my `render.rs` mutation).**

74. **AN IDENTIFIER TYPED FROM MEMORY IS THE SAME FAILURE AS A TIMESTAMP TYPED FROM MEMORY, AND THE ADDRESSING CHANNEL HAS NO GUARD ON IT (mine, 2026-09-05).**

75. **TWO FAILURES THAT SHARE A SYMPTOM CAN HAVE NON-CROSSING REMEDIES, AND THE SHARED SYMPTOM IS EXACTLY WHAT MAKES THEM READ AS ONE SEAM (mine, 2026-09-05, refuted by cc and re-driven here).**

76. **I ASSERTED A TOOL'S SURFACE WHILE WRITING ABOUT SOMETHING ELSE, IT WAS QUOTED BACK APPROVINGLY, AND THAT MADE IT LOOK CORROBORATED (mine, 2026-09-05, measuring `0257`).**

77. **A COINCIDENCE THAT REPRODUCES THE NUMBER YOU ARE TRYING TO EXPLAIN IS THE MOST DANGEROUS OBJECT IN A RECONCILIATION (vc's confirmation, dc's near-miss, driven by me 2026-09-05).**

78. **WRITING DOWN WHAT YOU ARE COUNTING FINDS THE DISAGREEMENT BEFORE ANY COUNT DOES (mine, 2026-09-05, and the protocol is vc's).**

79. **A NODE-LOCAL IDENTIFIER QUOTED ACROSS NODES DOES NOT DANGLE -- IT RESOLVES, TO SOMETHING ELSE (dc's finding, relayed by vc, made concrete here 2026-09-05).**

80. **A CLAIM ABOUT ORDER NEEDS AN ARTEFACT THAT ORDERS IT, AND A COMMIT TIMESTAMP ORDERS COMMITS RATHER THAN KNOWLEDGE (vc's separation, 2026-09-05, and it stands against me).**

81. **REACHING FOR THE SHARPEST AVAILABLE VERSION OF YOUR OWN FAULT IS STILL INACCURACY, AND IT DISTORTS THE RECORD IN THE DIRECTION THAT LOOKS HUMBLE (vc's, about themselves, 2026-09-05).**

82. **A COUNT OF A LIVE ARTEFACT IS A MEASUREMENT WITH A TIMESTAMP, AND THREE OF US PUBLISHED ONE AS A PROPERTY (vc's, 2026-09-05) -- AND THE DISCRIMINATOR BETWEEN _TIMING_ AND _METHOD_ IS FREE (mine, driven here).**

83. **WHEREVER A DEFECT HAS A LOUD MODE AND A QUIET MODE, THE LOUD MODE TRAINS YOU AND THE QUIET ONE ACCUMULATES (mine, generalised out of the W-number split with vc 2026-09-05).**

84. **BACKTICKS INSIDE A DOUBLE-QUOTED `echo` ARE COMMAND SUBSTITUTION, AND THE COMMAND THAT RUNS IS THE ONE YOU WERE TALKING ABOUT (mine, 2026-09-05).**

85. **TWO PATTERNS THAT SHARE A CLAUSE ARE ONE INSTRUMENT RUN TWICE, AND I CALLED MINE INDEPENDENT (mine, 2026-09-05, and it is why I concluded confidently in the WRONG direction).**

86. **A PEER CHANNEL HAS NO TERMINATING CONDITION, AND TWO NODES WHO ARE BOTH FINISHED WILL KEEP GOING BECAUSE EACH IS WAITING TO SEE WHETHER THE OTHER IS (vc's, 2026-09-05, closing an exchange in which we had each said _nothing further_ twice and continued).**

87. **_NEWER THAN X_ AND _CONTAINS Y_ ARE DIFFERENT RELATIONS, AND A COMPARISON AGAINST ONE LANDMARK CANNOT SEE A THIRD POINT LYING BETWEEN (dc's error, driven and refuted by me 2026-09-05).**

88. **A CLAIM RELAYED BETWEEN NODES CAN CHANGE SUBJECT IN TRANSIT WHILE STAYING TRUE AT BOTH ENDS, AND THE FALSEHOOD IS CREATED BY THE TRANSFER (dc's, self-reported 2026-09-05, after I refuted the result).**

89. **A REFUSAL'S TEXT IDENTIFIES WHICH LAYER REFUSED, AND THAT IS THE DISCRIMINATOR BETWEEN A DEFECT AND YOUR OWN ENVIRONMENT (dc's, 2026-09-05, a near-miss they did not file).**

90. **A CLEANUP GUARD MUST BE CONSTRUCTED BEFORE THE FIRST LINE THAT CAN FAIL, NOT BEFORE THE THING IT CLEANS UP (mine, 2026-09-05, found by driving a claim about my own work).**

91. **A FOLD COMPRESSES, AND COMPRESSION CAN CHANGE A CLAIM'S CATEGORY WHILE PRESERVING ITS ATTRIBUTION (vc's, self-reported and retracted with provenance 2026-09-05).**

92. **A CLAIM THAT STAYED TRUE OF SOMETHING AND FALSE OF WHAT IT NOW NAMES HAS NO LOCAL TEST, SO THE VERIFICATION MOVE IS TO RE-DERIVE ITS ORIGIN RATHER THAN TO CHECK ITS CONTENT (vc's generalisation of my W88/W91 pairing, 2026-09-05).**

93. **A GENERATED FILE HEALS ITSELF ON THE NEXT RENDER AND A HAND-AUTHORED ATTACHMENT CARRYING THE SAME VALUE DOES NOT, AND THE TWO ARE INDISTINGUISHABLE IN THE TREE (dc's, 2026-09-05).**

94. **A COUNT HANDED TO YOU AS A DEFECT COUNT MAY BE A STATUS COUNT, AND IT REPRODUCES EXACTLY EITHER WAY (mine, 2026-09-05).**

95. **SEARCHING BEFORE FILING FOUND THE OWNER OF A QUESTION I HAD ALREADY SPENT THE EFFORT ON (mine, 2026-09-05).**

96. **I COMMITTED THE DEFECT I HAD JUST CORRECTED IN A PEER, INSIDE THE SAME ROW, ONE SECTION LATER (mine, 2026-09-05).**

97. **A FOLD CUTS BY WHERE SOMETHING LIVES, AND A HOLD'S CONDITION IS ABOUT WHAT IT SAYS (mine, 2026-09-05, caught mid-fold).**

98. **A RULING THAT IS RIGHT DOES NOT GET RE-EXAMINED, SO ITS PRICE RIDES ALONG UNCHECKED (vc's, on themselves, 2026-09-05, and I want it here because I was the instrument twice).**

99. **A FILED ISSUE'S STATED CAUSE IS NOT ITS POPULATION, SO CHECKING YOUR EXPOSURE BY THE CAUSE RETURNS NIL ON AN ESTATE THAT HAS THE DEFECT (mine, 2026-09-05, on `0273`).**

100.  **A SCHEMA PROBE WITH THE WRONG KEY NAMES IS INDISTINGUISHABLE FROM A MALFORMED ROW (vc's, 2026-09-05, and I hit it at boot the same day).**

101.  **A COUNT AND THE SENTENCE NAMING IT DRIFT APART, AND THE SENTENCE IS WHAT TRAVELS (mine, 2026-09-05, third and fourth instances in one day).**

102.  **THE COST OF CHECKING SELECTS WHICH CLAIM GETS CHECKED, AND IT TRACKS WHAT INSTRUMENT YOU ARE ALREADY HOLDING RATHER THAN WHAT MATTERS (vc's self-report, sharpened here, 2026-09-05).**

103.  **AN UNBOUNDED ENDORSEMENT IS THE DEFECT; NAME WHICH FINDING IT COVERS (vc's, 2026-09-05, the operational form of W102).**

104.  **A SUBJECT WRITTEN DOWN WORKS ON A READER WHO IS NOT TRYING; A DISCIPLINE WORKS ONLY ON A READER WHO IS (vc's, 2026-09-05, and it is the day's conclusion rather than one more entry).**

105.  **`git commit --only` COMMITS EXACTLY WHAT IT WAS HANDED AND REPORTS SUCCESS, SO A RENAME COMMITTED BY ITS NEW PATH LANDS HALF (mine, 2026-09-05).**

106.  **A ROTTED RECORD IS ONE SOMETHING IS RESTING ON; A CLASS THAT CATCHES EVERY OUT-OF-DATE SENTENCE TRIAGES NOTHING (cc's, 2026-09-05, arrived at by REFUSING a correction I offered them).**

**cc MADE IT A FOLD OPERATION AND THAT IS THE PART THAT CHANGES PRACTICE (`2797abb2`): the boundary test runs FIRST, ahead of any archiving by date.**

107. **THE BLINDNESS THAT BELONGS TO NO AUTHOR, NO REVIEWER AND NO CHECK IS THE ONE THAT SURVIVES CARE (mechanism mine, ranking cc's, 2026-09-05 -- and cc asked for the split to be recorded exactly, having first had all seven as instruments).**

108. **EDITING ONE OF AN ADJACENT PAIR IS THE OPERATION THAT CREATES A CONTRADICTION, AND THE FIXER IS MORE EXPOSED THAN THE AUTHOR (mine, 2026-09-05, committed BY ME while removing this exact class from the same page).**

109. **`event_log` GIVES THE FILING INSTANT THAT AN mtime CANNOT, AND STILL DOES NOT GIVE THE ACTOR (mine, corrected by cc before it could be used, 2026-09-06).**

110. **BOTH RUST CRATES HAVE ONE `suite` TEST TARGET, AND THE HALF OF THIS ENTRY THAT SAID OTHERWISE WAS AN ASSUMPTION I WROTE AT 12:15Z AND DISPROVED AT 13:0xZ (mine, same session).**

111. **THE macOS APP'S `CFBundleShortVersionString` RESOLVES TO THE LITERAL STRING `dev` (cc's measurement, handed to me 2026-09-08 because `ST0064` is mine).**

112. **~~Check the arm that claims to be the load-bearing one first~~ -- FOLDED INTO `W44`**

113. **AN ABSENT COLUMN IS INVISIBLE TO EVERY CHECK THAT READS ROWS, AND ITS INVERSE IS A QUESTION NOBODY ASKS THE ARTEFACT (mine + cc + vc, 2026-09-09, and the pair is the durable half).**

114. **A GREEN OVER AN ENUMERATED AXIS IS EVIDENCE ABOUT THAT AXIS ONLY, AND A DOCSTRING SAYING _over the corpus_ DOES NOT WIDEN IT (mine, 2026-09-09, `nav_is_the_shared_path_contract.rs`).**

115. **A GUARD WHOSE _INPUT_ IS MISSING PRINTS ITS _SUBJECT IS BAD_ PROSE, AND THE TWO ARE INDISTINGUISHABLE AT THE TERMINAL (`0294`, mine, 2026-09-09).**

     **TWO CURES, AND THE SECOND IS THE DURABLE ONE.**

     **AND THE TELL THAT SCALES: THE PRESCRIBED REMEDY COULD NOT CLEAR THE REFUSAL.**

116. **ROUTING IS A MESSAGE I SEND, NOT A STATE I RECORD -- AND IF THE ONLY ARTEFACT IS ON MY OWN BOARD, NOTHING HAPPENED (cc's find, mine to own, 2026-09-09).**

     **THIS IS NOT THE hv-INBOX SHAPE AND CALLING IT THAT WOULD LET ME OFF.**

     **CURE, AND IT IS ONE LINE: a release is not written until a message is sent or an inbox entry is appended. The board records that it happened; it is never the thing that happened.**

- **W122 -- A TODO THAT NAMES THE REMAINING SCOPE WITHOUT NAMING THE ARTEFACT ALREADY PRODUCED SENDS THE NEXT PICKUP TO RE-DERIVE IT, AND THE NEXT PICKUP WAS ME (2026-09-09 20:48Z).**

- **W123 -- A SILENT `git add` IN A RETRY LOOP TURNS EVERY ITERATION INTO A NO-OP, AND THE LOOP REPORTS PATIENCE (2026-09-09 21:57Z). SECOND TIME TODAY, DIFFERENT MECHANISM, IDENTICAL SIGNATURE.**

- **W124 -- I ATTRIBUTED A FILE CHANGE TO A PEER WITHOUT DRIVING PROVENANCE. TWICE IN TWELVE HOURS, AND THE SECOND TIME COST A PEER A DEFENCE (2026-09-10 07:02Z).** First: credited dc with a `project.pbxproj` edit Xcode had written. Second: the gate refused my fold, I ran `git diff --cached`, saw `parity/tools/lib_currency.sh` staged IN THE SAME LISTING as cc's board, and told cc their file was refusing the tree. **`git log` on it returns ZERO commits -- brand new, never committed by anyone -- and cc had to write their own evidence to correct me.** **THEIR DIAGNOSIS THEN BEAT MINE: a PARTIAL STAGE.** The tool is staged `A `; the three modified tools including its roster row are ` M` and unstaged. **The decisive measurement I had not thought to take: the `lib_currency` row reads 1 ON DISK, 0 IN THE INDEX**, positive-controlled against `lib_staged` at 2/2. Mine was the symptom, theirs the cause. **RULE: a dirty file is not evidence of who dirtied it, and a `--cached` listing is a set of PATHS, not a set of AUTHORS -- adjacency inside it proves nothing.** `git log <path>` and `git diff --cached -- <one path>` are one command each and I ran neither before naming a node, having written the first half of this rule twelve hours earlier. **AND vc NAMED THE MECHANISM, WHICH IS BETTER THAN MY DIAGNOSIS OF MY OWN CARELESSNESS: the guard names an INSTRUMENT, an instrument has an obvious owner, and a STAGE does not.** So the message steers its reader toward _whose tool is this_ when the answer needed is _whose stage is this_. **That is a property of the MESSAGE, not of the reader** -- which is why the fix is a rule about what a guard's wording does to whoever reads it, and why _pay more attention_ would have been the useless version of this entry.

- **W125 -- A MEASUREMENT THAT AGREES WITH A PREDICTION IS NOT EVIDENCE FOR THE PREDICTION'S MECHANISM (2026-09-10, caught by vc after it reached hv).** I measured `no -wal present` and reported _it checkpointed on last close, exactly as vc's ratchet framing predicted_. The observable was real; the mechanism was a story I had no evidence for, and I believed it because a peer's prediction was in front of me. **vc had truncated it by hand and I measured the state they left.** The cure is to name the observable and stop: _the WAL is absent_ is what I had. **AND THE COST IS DIRECTIONAL: _it self-healed_ has no fix, so the wrong version is the one that stops the fix being written.**
- **W126 -- n=1 EACH SIDE OF A STOCHASTIC OUTCOME IS NOT ATTRIBUTION, AND I COMMITTED THIS HOURS AFTER WARNING vc ABOUT IT.** A peer's arm failed in the run carrying my change; I ran it once without my change, it passed, and I told them the break was mine. **6 runs each side: 5/1 with my change, 5/1 at HEAD, failing on a DIFFERENT arm each time.** Pre-existing, same rate. **The tell is that one-and-one FEELS like a controlled comparison** -- it has a treatment and a control and it is still a sample of one. The cure is a count before a cause, and the threshold is not high: six runs took two minutes.
- **W127 -- AN INSTRUMENT THAT OVER-COUNTS CANNOT EXHIBIT ITS OVER-COUNTING WHEN THE TRUE VALUE IS ZERO, SO AGREEMENT AT ZERO IS NOT EVIDENCE THAT INSTRUMENTS AGREE.** I reported the orphan count as confirmed by three instruments; at a live population `pgrep -f` read 7 against a true 4, counting a peer's shell and my own measuring pipeline. **My zero was right and a third of its support was luck.** **AND THE DIRECTION OF AN INSTRUMENT'S ERROR IS LOAD-BEARING ON A ZERO CLAIM**: told only that a counter under-reports, a reader treats its count as a FLOOR -- and at a true zero it can read nonzero, which makes a satisfied row look unsatisfiable.
- **W128 -- A RULE DERIVED FROM _THE CALLER CANNOT KNOW_ DESERVES A HOSTILE READ, BECAUSE IT IS USUALLY A CLAIM ABOUT WHAT THE AUTHOR DID NOT LOOK AT (vc's generalisation of one error each).** Theirs: _a helper cannot know whether an argv starts a daemon_, so hand every child a lifeline -- false twice, since the helper HAS the argv, and a pipe nobody writes to blocks any verb reading stdin. Mine: the fixture families cannot be enumerated, so match on a prefix -- refuted by reading the disk, which held twelve name shapes where the code showed four. **Both failed in the cheap direction and that was luck, not design: vc's hung the suite in minutes rather than leaking for days.**
- **W129 -- A WORK PACKAGE'S STATUS FIELD IS NOT EVIDENCE ABOUT ITS ACCEPTANCE CRITERIA, AND WHEN THE TWO DISAGREE THE STATUS IS THE STALE ONE (2026-09-10, vc read mine and drew the wrong conclusion honestly).** `WP-02` read `Not Started` while `AC-02.1` was satisfied and `AT-02.1` green, because I built the work and never drove `wp start` / `wp done`. **vc read the status, inferred the objective had no criterion covering it, and offered to MINT ONE** -- which would have put a second home for a live contract on the thread, in the same release as the first. **The status field is a claim by whoever last drove a verb; the AC's satisfaction is COMPUTED from its test.** One is maintained by hand and one is derived, so when they disagree, believe the derived one and go fix the hand-maintained one. **AND THE TELL THAT IT WAS SAFE TO CHECK IS FREE: the thread numbers its rows `AC-<WP>.<n>`**, so whether WP-02 has a criterion is answerable by reading, in one grep, without measuring anything.

## Decisions

- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv 2026-09-01, via vc). Everything outstanding goes in; no external consumer; scarcity register retired as a class.
- **A DECISION TAKEN ON SOMEONE'S BEHALF AND RECORDED AS SUCH IS DELEGATION; THE SAME DECISION RECORDED AS A FACT IS A SUBSTITUTION** (vc, ratifying two of mine). What made Esc-enters-vi-normal legitimate was naming the alternative, writing it into the design _as_ a decision made on hv's behalf, and telling hv in chat -- not the reasoning behind it.
- **A ROW WITH TWO SUBJECTS CANNOT STATE ITS OWN REACH** (vc, four times in one day, and the tool caught the fourth). `AC-17.17` was minted test-backed while its own reach clause said no test could witness it; the contract REFUSED the pair. Split: `AC-17.17` the ORDER (test-backed, satisfied), `AC-17.18` the PAINTING (non-test, **SATISFIED -- hv drove it in a real terminal 2026-09-02 and the capture is on the row**; this entry said _open_ until 2026-09-03, see W24). **The discriminator is who holds the screen, not which variant it is** -- a borrower that blocks needs no signal, one that returns does.
- **ST0064 WP-01 = 5/9 (ic), RE-DERIVED 2026-09-05 AND STILL TRUE -- `intent ac list ST0064` is the authority, not this line.** 01.1/01.3/01.5/01.8/01.9 done; 01.2/01.4/01.6/01.7 cc/dc/hv-gated. project-CWD wiring; vc RULED per-app-instance root, D07 registry UNBUILT.
- **`AC-17.1` IS SATISFIED AND THE _UNSATISFIABLE IN 3.0.x_ READING IS RETIRED (2026-09-03, measured not argued).** This entry said 17.1 could not be met while the wire was reads-only. cc built `Op::Set` overnight and `AT-17.1` is green. **`EmptyMutation` staying is not a counter-argument -- it is `Op::Graphql`'s bound and it is deliberate; the mutating door is a DIFFERENT door.** Kept as a decision rather than deleted, because three boards and `intent/wip.md` carried the dead reading and a reader may still meet it. **`AC-17.6` is the one open row in WP-17 and is still gated on cc's WP-08** -- `browsed()` refuses unconditionally, and cc has taken the missing half, unstarted. **No figure here: run `intent ac list ST0056`.**
- **WP-14 (whiteboard + inboxes in the store, `L`) -- THE QUESTION IS LIVE AND IT IS NOT WHETHER IT RETURNS.** hv reinstated `AC-00.2`/`AC-10.5` into ST0056 at `a19d8b5c` (driven: ancestor of HEAD, both rows `computed`). **`ST0056/14` itself still reads `Not Started`/`L`**, and those rows are about the FLEET CORPUS, not the coordination model -- so _WP-14 is in the cut_ does not follow from their subject. vc accepted that as a defect in the menu they put to hv. **Bears on `0258`, whose ruling is gated behind exactly this.**
- **A SIZE IS A CLAIM ABOUT THE WORLD EXACTLY AS MUCH AS A COUNT IS** (mine, taken onto cc's board 2026-09-03). An evidence bar held for figures but not for t-shirt sizes is the asymmetry that hides itself, because only one of the two wears a number.
- **`init` LAYS `AGENTS.md` DOWN** (hv 2026-09-09, ruling on ST0065's costed proposal). Option 2's index landed in `CLAUDE.md` at `0b5d46c96`, so the content argument is spent -- but `AGENTS.md` is the TOOL-AGNOSTIC contract, and a fresh project had none until someone ran a verb nothing told them to run while `CLAUDE.md` said _read AGENTS.md first_ four times. **Ruling 2 (what `AGENTS.md` mirrors) is DEFERRED: it gates only Option 3 and blocks nothing.** Ruling 3 was already dead -- built and green since 2026-09-04.
- **`ST0065/WP-02` COLLAPSES INTO `ST0056/WP-15`** (hv 2026-09-09). Cancelled with reason via `intent wp cancel`. Two homes for one concern, both WIP, found before either party sank effort.
- **`uninstall` PROMISES THE FILES IT WROTE, NOT THE DIRECTORY IT EMPTIED** (hv 2026-09-09, `0218` remedy 1).
- **A RULING RECORDED ONLY IN CODE IS INVISIBLE TO EVERY BOARD-LEVEL RE-DRIVE** (mine, adopted by vc into the standing recipe 2026-09-09). ST0065's ruling 3 was answered, built and green on 2026-09-04 and survived the issue tracker, the boards AND the gates, because its only home is a doc comment on a test. **vc's expiry defect is an item outliving its REASON; this is an item outliving its ANSWER**, and it fails in the direction that costs most -- a DEAD item passing as live indefinitely. **Fourth line of the re-drive recipe: for any ruling whose answer would be BUILT, grep the estate for the thread id before believing the board.** **FIFTH LINE, DRIVEN AT THIS BOOT 2026-09-09 20:24Z, AND IT IS THE SAME DEFECT ONE RUNG UP: A RULING RECORDED ON ONE BOARD IS INVISIBLE TO EVERY HOME THAT SAYS IT IS UNMADE, AND FOUR OF THEM DO.** hv ruled `ST0065`'s set on 2026-09-09 -- `6d60bc516`, _hv ruled four of six_ -- and the POSITIVE CONTROL is the sibling ruling's artefact rather than the record of it: `intent wp list ST0065` reads WP-02 **Cancelled**, so the pass demonstrably happened and produced an executed outcome. **The live value has ONE home, this board's line above. FOUR carry the opposite**: `.claude/restart.md:18`, `intent/whiteboard/hv/wip.md:73`, `intent/wip.md:85` and `intent/whiteboard/hv/inbox.vc.md:1250`, every one still reading _ic takes WP-01 the moment they land_ while `WP-01` has been WIP since morning. **THE WORST OF THE FOUR IS THE ENTRY POINT, WHICH EVERY NODE READS AT EVERY BOOT, AND IT TOLD ME THIS MORNING THAT I WAS BLOCKED ON RULINGS I HAD RECORDED MYSELF** -- it cost nothing only because my own board carried the answer and I read both. `intent/wip.md:83` is stale a second way: it calls WP-02 WIP. **RULE: for a ruling whose answer would be RECORDED rather than BUILT, grep the estate for the thread id before believing the entry point. The direction that costs most is a LANDED ruling still reading as owed, because it parks a node that could be working, and nothing anywhere goes red.**
