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
  let mut v = Vec::new();
  let lines = txt.split("\n");
  // I couldn't see how to get the error handling right if I used map/collect instead of the loop.
  // I had to use unwrap() after the parse, which broke the goal of returning errors as a Result.
  for l in lines {
    let i = l.to_string().parse::<i32>()?;
    v.push(i);
  }
  Ok(v)
}

// ---------------------------------------------------

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

fn read_from_file<T: ReadsValue>(file_name: &str) -> T {
  T::read_from_file(file_name)
}

trait ReadsValue {
  fn read_from_file(file_name: &str) -> Self;
}

impl ReadsValue for i32 {
  fn read_from_file(file_name: &str) -> Self {
    0
  }
}

impl ReadsValue for &str {
  fn read_from_file(file_name: &str) -> Self {
    &"foo"
  }
}

fn test(file_name: &str) {
  let value: &str = ReadsValue::read_from_file(file_name);
  let value2 = i32::read_from_file(file_name);
  let mut value3 = 0;
  value3 = read_from_file(file_name);
}

// ---------------------------------------------------

// trait Convertible {
//   type Item;

//   fn convert(self) -> Self::Item;
// }

// impl Convertible for i32 {
//   type Item = i32;

//   fn convert(self) -> Self::Item {
//     self.parse().unwrap()
//   }
// }

// pub fn parse_text_file_to_vector<T: Convertible>(file_name: &str) -> Result<Vec<T>, Box<Error>> {
//   let mut f = fs::File::open(file_name)?;
//   let mut txt = String::new();
//   f.read_to_string(&mut txt)?;
//   let mut v = Vec::new();
//   let lines = txt.split("\n");
//   for l in lines {
//     let t = l.convert();
//     v.push(t);
//   }
//   Ok(v)
// }

// fn test() {
//   let foo: Vec<i32> = parse_text_file_to_vector("foo.txt")?;
//   println!("{:?}", foo);
// }

// //https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html

// trait Convert<T> {
//   fn convert(self) -> T;
// }

// impl Convert<String> for &str {
//   fn convert(self) -> String {
//     self.to_string()
//   }
// }

// impl Convert<i32> for &str {
//   fn convert(self) -> i32 {
//     self.parse().unwrap()
//   }
// }

// // The way to do this would be to define a trait, e.g. convert, then implement it for String, i32, etc.
// // Is there such a trait existing?
// // String - to_string()
// // Others - .to_string().parse::<T>()?;
// // default is to_string()?

// // But how does the code know what type we want returned? Is this something I do via the let?

// // https://blog.jcoglan.com/2019/04/22/generic-returns-in-rust/
// trait MyInto<T> {
//   fn into(self) -> T;
// }

// #[derive(Clone)]
// struct Person {
//   name: String,
//   age: u8,
// }

// impl MyInto<String> for Person {
//   fn into(self) -> String {
//     format!("{} ({})", self.name, self.age)
//   }
// }

// Why, I believe I need an associated type, and a generic one at that. Chapter 20.
// Is this insane yet?

// pub fn parse_text_file_to_vector<T: Convert<T>>(file_name: &str) -> Result<Vec<T>, Box<Error>> {
//   let mut f = fs::File::open(file_name)?;
//   let mut txt = String::new();
//   f.read_to_string(&mut txt)?;
//   let mut v = Vec::new();
//   let lines = txt.split("\n");
//   for l in lines {
//     let t = l.convert();
//     v.push(t);
//   }
//   Ok(v)
// }
