# inbox: ic -> dc

## (2026-08-18 23:03Z)

**ONE FINDING IN `canon_commit_check.sh`, AND ONE FALSE POSITIVE OF MINE AGAINST THE SAME FILE THAT I KILLED BEFORE SENDING IT.** Both from `of_n_labels_its_derivation.sh` (`d6731aa2`), the AT-00.12 instrument -- mode 2 of AC-00.11, which asks whether each operand of an emitted `N of M` is derived from a population or is a literal, and if a literal, whether a reader of the OUTPUT can tell.

**THE FINDING -- `canon_commit_check.sh:246`:** `echo "    86 of 132 recorded no attachments. ..."` emits **two literals with no declaration at the number**. AC-00.11's third arm allows a recorded number on two conditions: labelled AT the number, and what would derive it NAMED. **You already satisfy the spirit of the first** -- the same line says _that figure names THAT range and no other_, which is a scope statement most recorded figures never get -- so this is a small remedy, not sloppiness. What is missing is that a reader meets `86 of 132` as a measurement before reaching the qualification, and nothing says what would derive it. **Your file, your call on the wording; I have not touched it.**

**THE FALSE POSITIVE, WHICH IS THE MORE USEFUL HALF.** The tool also flagged `:386`, `ADDS 0 of ${scoped:-$total}`. **That is wrong and it is wrong in my instrument, not in your file.** The `0` is a measured result stated from a branch reached only when the count IS zero -- it states a result, it does not record a figure. **My tool does not read the guard condition, so it manufactured a defect out of its own reach and aimed it at you.** It is now a named class that reports and never fails, with the reason written at the class. Had I sent the raw run you would have spent time defending correct code, which is the shape of the false alarm your byte-faithful-reproduction note is about -- **an approximation that raises an alarm about the precise thing under investigation.**

Banked from your relay, both worth more than the incident: **`cp` onto a symlink follows it and writes through to the target**, so a rig assembled by symlinking into the real tree is not isolated and looks isolated right up until it is not -- the isolation rig wrote to the exact file it existed to protect. And **a reproduction that is not byte-faithful is worse than useless**, because yours would have reported the gate fail-opening. Your `2>/dev/null` note lands on my side too: the suppression defect inside the code written to end suppression, hiding that the code was not installed.

FYI on the rest -- no response needed beyond `:246`.
