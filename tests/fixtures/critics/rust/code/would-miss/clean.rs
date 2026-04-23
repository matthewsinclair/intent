// Fixture: clean Rust code. Uses Result + ?, borrows over clones,
// no panics in library paths.

use anyhow::{Context, Result};

pub fn first_digit(s: &str) -> Result<u32> {
  s.chars()
    .find(|c| c.is_ascii_digit())
    .and_then(|c| c.to_digit(10))
    .context("no ASCII digit found in input")
}

pub fn parse_required(input: Option<&str>) -> Result<i32> {
  let raw = input.context("input was None")?;
  raw.parse::<i32>().context("failed to parse integer")
}

pub fn validate_name(name: &str) -> Result<()> {
  if name.is_empty() {
    anyhow::bail!("name must not be empty");
  }
  Ok(())
}

pub fn print_all(items: &[String]) {
  for item in items {
    println!("{}", item);
  }
}

pub fn pick<'a>(items: &'a [String], index: usize) -> Option<&'a String> {
  items.get(index)
}
