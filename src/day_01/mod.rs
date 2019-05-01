use crate::util;
use std::collections::HashMap;
use std::error::Error;

// part_1 sums the frequency changes.
pub fn part_1(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::text_file_to_vector(file_name)?;
  let mut res = 0;
  for s in v {
    let i: i32 = s.parse()?;
    res += i;
  }
  Ok(res)
}

pub fn part_1_fold(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::text_file_to_vector(file_name)?;
  // The function passed to fold returns an i32, not a Result, so
  // I can't use the ? operator on s.parse(). Instead, unwrap()
  // works, but it panics on Error instead of returning it.
  let res: i32 = v.iter().fold(0, |sum, s| sum + s.parse::<i32>().unwrap());
  Ok(res)
}

pub fn part_1_sum(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::text_file_to_vector_i32(file_name)?;
  let res = v.iter().sum();
  Ok(res)
}

// part_2 returns the first frequency sum that repeats.
pub fn part_2(file_name: &str) -> Result<i32, Box<Error>> {
  let mut freq = 0;
  let mut seen = HashMap::new();
  let v = util::text_file_to_vector_i32(file_name)?;
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
