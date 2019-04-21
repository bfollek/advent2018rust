use std::io::Result;

mod day01;

fn main() -> Result<()> {
    println!("Day 01, par 1: {}", day01::part1("../testdata/day1.txt")?);
    Ok(())
}
