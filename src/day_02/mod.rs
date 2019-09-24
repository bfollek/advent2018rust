use crate::util;
use std::collections::HashMap;
use std::error::Error;

// part_1 does a checksum on the box IDs.
pub fn part_1(file_name: &str) -> Result<i32, Box<dyn Error>> {
  let mut cnt_2 = 0;
  let mut cnt_3 = 0;
  let ids = util::text_file_to_vector(file_name)?;
  for id in ids {
    let (has_2, has_3) = check_id(id);
    if has_2 {
      cnt_2 += 1;
    }
    if has_3 {
      cnt_3 += 1;
    }
  }
  Ok(cnt_2 * cnt_3)
}

// check_id checks IDs for letters that appear exactly 2 or 3 times.
fn check_id(id: String) -> (bool, bool) {
  let mut char_map = HashMap::new();
  for c in id.chars() {
    let cnt = char_map.entry(c).or_insert(0);
    *cnt += 1;
  }
  let (mut has_2, mut has_3) = (false, false);
  for cnt in char_map.values() {
    match cnt {
      2 => has_2 = true,
      3 => has_3 = true,
      _ => (),
    }
    if has_2 && has_3 {
      break; // No need to keep checking
    }
  }
  (has_2, has_3)
}

pub fn part_2(file_name: &str) -> Result<String, Box<dyn Error>> {
  let ids: Vec<String> = util::text_file_to_vector(file_name)?;
  for (i, id1) in ids.iter().enumerate() {
    for (j, id2) in ids.iter().enumerate() {
      if i != j {
        let (b, common_chars) = diff_by_1(id1.to_string(), id2.to_string());
        if b {
          return Ok(common_chars.to_string())
        }
      }
    }
  }
  Err("id's that diff by 1 char not found")?
}

fn diff_by_1(s1: String, s2: String) -> (bool, String) {
  (false, "test".to_string())
}
