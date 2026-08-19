# Parse `- AC-x.y <text> [-- satisfied: ...]` into id \t text, stripping the COMPUTED satisfied trailer.
/^- AC-[0-9]+\.[0-9]+ / {
  line = substr($0, 3)
  sp = index(line, " ")
  id = substr(line, 1, sp-1)
  txt = substr(line, sp+1)
  p = index(txt, " -- satisfied: ")
  if (p > 0) txt = substr(txt, 1, p-1)
  printf "%s\t%s\n", id, txt
}
