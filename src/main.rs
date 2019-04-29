mod day01;
mod day02;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut expected;

    expected = 592;

    assert_eq!(expected, day01::part1("testdata/day01.txt")?);
    assert_eq!(expected, day01::part1_fold("testdata/day01.txt")?);
    assert_eq!(expected, day01::part1_sum("testdata/day01.txt")?);

    expected = 241;
    assert_eq!(expected, day01::part2("testdata/day01.txt")?);

    expected = 8715;
    assert_eq!(expected, day02::part1("testdata/day02.txt")?);

    Ok(())
}
