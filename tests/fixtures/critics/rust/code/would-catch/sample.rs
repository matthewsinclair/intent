// Fixture: triggers IN-RS-CODE-001 (result-over-panic) and
// IN-RS-CODE-002 (ownership-before-clone) by design.

pub fn first_digit(s: &str) -> u32 {
  s.chars()
    .find(|c| c.is_ascii_digit())
    .unwrap()
    .to_digit(10)
    .unwrap()
}

pub fn parse_required(input: Option<String>) -> i32 {
  let raw = input.expect("input was None");
  raw.parse::<i32>().expect("parse failed")
}

pub fn debug_identify(name: &str) {
  if name.is_empty() {
    panic!("name must not be empty");
  }
}

pub fn print_all(items: Vec<String>) {
  for item in items {
    println!("{}", item.clone());
  }
}

pub fn pick(items: Vec<String>, index: usize) -> String {
  items[index].clone()
}
