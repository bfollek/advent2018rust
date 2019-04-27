mod day01;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut expected = 592;
    assert_eq!(expected, day01::part1("testdata/day01.txt")?);
    assert_eq!(expected, day01::part1_fold("testdata/day01.txt")?);
    Ok(())
}
