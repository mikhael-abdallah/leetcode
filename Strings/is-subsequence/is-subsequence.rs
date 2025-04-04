// https://leetcode.com/problems/is-subsequence/

struct Solution;

impl Solution {
  pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;

    let s_chars = s.chars().collect::<Vec<char>>();
    let t_chars = t.chars().collect::<Vec<char>>();

    while i < s_chars.len() && j < t_chars.len() {
      if s_chars[i] == t_chars[j] {
        i += 1;
      }
      j += 1;
    }
    i == s.len()
  }
}

fn main() {
  let s = String::from("abc");
  let t = String::from("ahbgdc");
  let result = Solution::is_subsequence(s, t);
  println!("{}", result);
}
