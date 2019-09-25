mod day_01;
mod day_02;
mod util;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Day 1
    assert_eq!(592, day_01::part_1("testdata/day01.txt")?);
    assert_eq!(592, day_01::part_1_fold("testdata/day01.txt")?);
    assert_eq!(592, day_01::part_1_sum("testdata/day01.txt")?);
    assert_eq!(241, day_01::part_2("testdata/day01.txt")?);
    // Day 2
    assert_eq!(8715, day_02::part_1("testdata/day02.txt")?);
    assert_eq!(
        String::from("fvstwblgqkhpuixdrnevmaycd"),
        day_02::part_2("testdata/day02.txt")?
    );

    Ok(())
}
