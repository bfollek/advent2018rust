use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::marker::Sized;

pub fn text_file_to_vector<T: Parses>(file_name: &str) -> Result<Vec<T>, Box<Error>> {
  let mut f = fs::File::open(file_name)?;
  let mut txt = String::new();
  f.read_to_string(&mut txt)?;
  let mut v: Vec<T> = Vec::new();
  let lines = txt.split("\n");
  for l in lines {
    let t = T::parse(l)?;
    v.push(t);
  }
  Ok(v)
}

pub trait Parses {
  fn parse(raw: &str) -> Result<Self, Box<Error>>
  where
    Self: Sized;
}

impl Parses for i32 {
  fn parse(raw: &str) -> Result<Self, Box<Error>> {
    raw
      .to_string()
      .parse()
      .map_err(|e: std::num::ParseIntError| e.into()) // Converts to Box
  }
}

impl Parses for String {
  fn parse(raw: &str) -> Result<Self, Box<Error>> {
    Ok(raw.to_string())
  }
}
