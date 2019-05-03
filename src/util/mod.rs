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

pub fn parse_text_file_to_vector<T: Parses>(file_name: &str) -> Result<Vec<T>, Box<Error>> {
  let mut f = fs::File::open(file_name)?;
  let mut txt = String::new();
  f.read_to_string(&mut txt)?;
  let mut v: Vec<T> = Vec::new();
  let lines = txt.split("\n");
  for l in lines {
    let t = T::parse(l);
    v.push(t);
  }
  Ok(v)
}

pub trait Parses {
  fn parse(raw: &str) -> Self;
}

impl Parses for i32 {
  fn parse(raw: &str) -> Self {
    raw.to_string().parse().unwrap()
  }
}

impl Parses for String {
  fn parse(raw: &str) -> Self {
    raw.to_string()
  }
}
