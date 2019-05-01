mod day_01;
mod day_02;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let mut expected;

    expected = 592;

    assert_eq!(expected, day_01::part_1("testdata/day01.txt")?);
    assert_eq!(expected, day_01::part_1_fold("testdata/day01.txt")?);
    assert_eq!(expected, day_01::part_1_sum("testdata/day01.txt")?);

    expected = 241;
    assert_eq!(expected, day_01::part_2("testdata/day01.txt")?);

    expected = 8715;
    assert_eq!(expected, day_02::part_1("testdata/day02.txt")?);

    Ok(())
}
