mod day01;
mod util;

fn main() {
    let mut expected = 592;
    assert_eq!(expected, day01::part1("testdata/day01.txt").unwrap());
    assert_eq!(expected, day01::part1_fold("testdata/day01.txt").unwrap());
}
