use crate::util;
use std::error::Error;

// part_1 sums the frequency changes.
pub fn part1(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::text_file_to_vector(file_name)?;
  let mut res = 0;
  for s in v {
    let i: i32 = s.parse()?;
    res += i;
  }
  Ok(res)
}

pub fn part1_fold(file_name: &str) -> Result<i32, Box<Error>> {
  let v = util::text_file_to_vector(file_name)?;
  // The function passed to fold returns an i32, not a Result, so
  // I can't use the ? operator on s.parse(). Instead, unwrap()
  // works, but it panics on Error instead of returning it.
  let res: i32 = v.iter().fold(0, |sum, s| sum + s.parse::<i32>().unwrap());
  Ok(res)
}
