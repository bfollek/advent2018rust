use crate::util;
use std::error::Error;

// part_1 does a checksum on the box IDs.
pub fn part_1(file_name: &str) -> Result<i32, Box<Error>> {
  let mut cnt_2 = 0;
  let mut cnt_3 = 0;
  let ids = util::text_file_to_vector(file_name)?;
  for id in ids {}
  Ok(0)
}

// func Part1(fileName string) int {
// 	cnt2 := 0
// 	cnt3 := 0
// 	ids := util.MustLoadStringSlice(fileName)
// 	for _, id := range ids {
// 		has2, has3 := checkID(id)
// 		if has2 {
// 			cnt2++
// 		}
// 		if has3 {
// 			cnt3++
// 		}
// 	}
// 	return cnt2 * cnt3
// }
