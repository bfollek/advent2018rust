use std::error::Error;
use std::fs;
use std::io::prelude::*;

pub fn text_file_to_vector(file_name: &str) -> Result<Vec<String>, Box<Error>> {
  let mut f = fs::File::open(file_name)?;
  let mut txt = String::new();
  f.read_to_string(&mut txt)?;
  let res = txt.split("\n").map(|s| s.to_string()).collect();
  Ok(res)
}

pub fn text_file_to_vector_i32(file_name: &str) -> Result<Vec<i32>, Box<Error>> {
  let mut f = fs::File::open(file_name)?;
  let mut txt = String::new();
  f.read_to_string(&mut txt)?;
  let res = txt
    .split("\n")
    .map(|s| s.to_string().parse::<i32>().unwrap())
    .collect();
  Ok(res)
}
