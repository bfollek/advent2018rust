use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::marker::Sized;

pub fn text_file_to_vector<T: ConvertsToText>(file_name: &str) -> Result<Vec<T>, Box<dyn Error>> {
  let mut f = fs::File::open(file_name)?;
  let mut txt = String::new();
  f.read_to_string(&mut txt)?;
  let mut v: Vec<T> = Vec::new();
  let lines = txt.split('\n');
  for l in lines {
    let t = T::convert_to_text(l)?;
    v.push(t);
  }
  Ok(v)
}

pub trait ConvertsToText {
  fn convert_to_text(raw: &str) -> Result<Self, Box<dyn Error>>
  where
    Self: Sized;
}

impl ConvertsToText for i32 {
  fn convert_to_text(raw: &str) -> Result<Self, Box<dyn Error>> {
    raw.to_string().parse().map_err(std::convert::Into::into) // Box the error
  }
}

impl ConvertsToText for String {
  fn convert_to_text(raw: &str) -> Result<Self, Box<dyn Error>> {
    Ok(raw.to_string())
  }
}
