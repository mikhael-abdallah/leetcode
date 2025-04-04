// https://leetcode.com/problems/longest-common-prefix/

struct Solution;

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();

    let strs_chars = strs.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for i in 0..strs_chars[0].len() {
      let c = strs_chars[0][i];
      for j in 1..strs_chars.len() {
        if i >= strs_chars[j].len() || strs_chars[j][i] != c {
          return prefix;
        }
      }
      prefix.push(c);
    }

    prefix
  }
}

fn main() {
  let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
  let prefix = Solution::longest_common_prefix(strs);
  println!("{}", prefix);
}
