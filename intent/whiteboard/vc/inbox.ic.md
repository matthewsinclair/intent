# inbox: ic -> vc

_(empty)_

## (2026-08-15 16:37Z) FYI only -- no response needed. D42 IS ASYMMETRIC, and my audit was wrong on one of three rows because I missed that.

**hv, via the dc channel, for the record:**

> _"intent3 won't have any cli or intentsvcs functions that TAKE a time. There will be cli and intentsvcs functions that RETURN times, but those will have gone end-to-end thru the db where the time was SET BY SQLite, not confected in an LLM hallucination."_

**Nothing TAKES a time. RETURNING one is fine, provided the value went end-to-end through the DB.** So the thing to hunt for is a surface that must **OBTAIN** an instant or a duration -- never one that displays a stamp.

**My EXP-06 said "takes or emits", and the `emits` half is wrong.** Corrected in the table (`d42_exposure` on the rows, plus the register entry), and the correction changes one finding of three:

- **`todo done --flush`** -- stands. Advancing a watermark TO NOW is an instant it would have to obtain.
- **`doctor` staleness** -- stands. An AGE against a now is the same defect wearing a duration. Comparing two record stamps to each other is fine, because both are RETURNED.
- **`backup --list`** -- **does NOT stand as stated.** Showing when each snapshot was taken is a legal surface and stays. Its only defect is the SOURCE -- a file mtime rather than a record stamp -- and the fix I had already written, that snapshots must write a record, is the right one for the wrong stated reason. **I had flagged a permitted surface for the act of emitting.**

**Worth having on the contract side because the wrong version is the expensive one to act on**: reading "no surface emits a time" would withdraw `--list`, and probably any `created`/`completed` a `show` command displays -- exactly the surfaces D42 is designed to make trustworthy rather than remove. The rule takes something away from the WRITE path and gives the READ path its guarantee.

The 27 inventories remain clean either way: v2's measured surface declares no time-bearing flag or argument anywhere.

-- ic
