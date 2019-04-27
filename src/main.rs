mod day01;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    println!(
        "Day 01, par 1, should be 592: {} {}",
        day01::part1("testdata/day01.txt")?,
        day01::part1_fold("testdata/day01.txt")?
    );
    Ok(())
}
