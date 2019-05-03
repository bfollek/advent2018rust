use crate::util;
use std::collections::HashMap;
use std::error::Error;

// part_1 sums the frequency changes.
pub fn part_1(file_name: &str) -> Result<i32, Box<Error>> {
  let v: Vec<i32> = util::parse_text_file_to_vector(file_name)?;
  let mut res = 0;
  for i in v {
    res += i;
  }
  Ok(res)
}

pub fn part_1_fold(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::parse_text_file_to_vector(file_name)?;
  let res = v.iter().fold(0, |sum, i| sum + i);
  Ok(res)
}

pub fn part_1_sum(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::parse_text_file_to_vector(file_name)?;
  let res = v.iter().sum();
  Ok(res)
}

// part_2 returns the first frequency sum that repeats.
pub fn part_2(file_name: &str) -> Result<i32, Box<Error>> {
  let mut freq = 0;
  let mut seen = HashMap::new();
  let v = util::parse_text_file_to_vector(file_name)?;
  loop {
    // We need &v to avoid a borrow checking error.
    // http://xion.io/post/code/rust-for-loop.html
    for i in &v {
      freq += i;
      // Reference required here. HashMap.get() apparently dereferences
      // automatically, though - we're using the value, not the address,
      // as the key.
      match seen.get(&freq) {
        None => seen.insert(freq, true),
        Some(_) => return Ok(freq),
      };
    }
  }
}
