// https://leetcode.com/problems/zigzag-conversion/description/

struct Solution;

impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
      if num_rows == 1 {
        return s;
      }

      let mut res = vec![String::new(); num_rows as usize];
      let mut direction = 1;
      let mut actual_column = 0;
      let chars = s.chars();

      for c in chars {
        res[actual_column as usize].push(c);

        actual_column = actual_column + direction;

        if actual_column < 0 || actual_column >= num_rows {
            direction *= -1;
            actual_column += 2 * direction;
        }
      }

      res.concat()
  }
}

fn main() {
    let s = String::from("PAYPALISHIRING");
    let num_rows = 3;
    let result = Solution::convert(s, num_rows);
    println!("{}", result);
}