# dc -- archived DOING, WP-11 signing pipeline (2026-08-19)

Archived at the localfold. The work is landed and the criteria are vc's to move;
kept verbatim because the artefact hashes and the notary submission id are the
only durable record of a run that cannot be repeated -- `publish` refuses a dev
version, so this pipeline cannot be exercised again until 3.0.0 has a real one.

## DOING

**WP-11: THE SIGNING PIPELINE IS EXERCISED END TO END AND APPLE ACCEPTED IT.** `doctor` -> build at HEAD -> `stage` -> `sign` -> `notarize` -> `checksum` -> `formula` -> `publish --dry-run`, every exit code read from a file. Notary submission `850a8625-5566-4996-964f-d3bb732ab915`, status Accepted, ticket visible to Gatekeeper after 0s; both binaries proven from a QUARANTINED copy before being hashed. Formula: 71 lines, **0 offences under `brew style` at a tap path**, with the bare-path control reproducing the documented 4 false offences. `publish` refuses on `3.0.0-dev` before touching the remote. Artefacts: `intent` `d259f66c0820719e`, `intentd` `8218d4247032fc2d`, support `076e4ca6a3e8a9c8`.

**hv's WP-11 QUESTION IS ANSWERED: the tap needs ONLY the release asset, no repo access.** Read off `cmd_formula` -- three `releases/download/` URLs plus `homepage` as a display string, no git block, no clone, no token -- and confirmed by an anonymous-GET control (`GITHUB_TOKEN`/`GH_TOKEN` unset) returning http=200. **v2.19.0 carries ZERO release assets**, so D39/D40's "artefacts on the source repo's own releases" had never been exercised once; it now is, up to publication.

Committed today: `26fe1aea` (board), `db35e178` (the false stage-refusal claim corrected).
