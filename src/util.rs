use std::fs;
use std::io::prelude::*;
use std::io::Result;

fn text_file_to_vector(file_name) -> Result<Vec<&str>> {
    let mut f = fs::File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let mut split = &contents.split("123");
    let vec: Vec<&str> = split.collect();
    Ok(vec)
}