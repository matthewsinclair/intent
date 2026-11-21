//! **The first `st hydrate` of an upgraded project counts the files it wrote**
//! (ic, 2026-11-21). The pin is a write of the record there, and that write
//! renders the thread's views, so the organize run after it found them in
//! place: the summary said `0 written by this run` above `wrote:` and `re-rendered:` lines for
//! `info.md` and `acceptance.md`. The count and the lines are one set.

use crate::common::run_joined as intent;

#[test]
fn the_first_hydrate_counts_the_views_its_pin_wrote() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  let (ok, out) = intent(root, &["init", "probe"]);
  assert!(ok, "init: {out}");
  let (ok, out) = intent(root, &["st", "new", "a thread to hydrate"]);
  assert!(ok, "st new: {out}");
  let st = crate::common::created(&out);

  let (ok, out) = intent(root, &["st", "hydrate", st.as_str()]);
  assert!(ok, "st hydrate: {out}");
  let home = format!("intent/st/{st}/");
  let listed = out
    .lines()
    .filter(|l| {
      let l = l.trim_start();
      (l.starts_with("wrote: ") || l.starts_with("re-rendered: ")) && l.contains(&home)
    })
    .count();
  let counted: usize = out
    .lines()
    .find_map(|l| {
      l.split(" written by this run")
        .next()
        .filter(|_| l.contains(" written by this run"))
        .and_then(|head| head.rsplit(' ').next())
        .and_then(|n| n.parse().ok())
    })
    .unwrap_or_else(|| panic!("no count in the summary: {out}"));
  assert!(
    listed > 0,
    "the first hydrate writes the thread's views: {out}"
  );
  assert_eq!(
    counted, listed,
    "the summary counts the files the written lines name: {out}"
  );
}
