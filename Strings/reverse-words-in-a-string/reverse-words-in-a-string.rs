// https://leetcode.com/problems/reverse-words-in-a-string/description/

struct Solution;
impl Solution {
  pub fn reverse_words(s: String) -> String {
    s.split_whitespace()
      .rev()
      .collect::<Vec<&str>>()
      .join(" ")
  }
}

fn main() {
  let s = String::from("the sky is blue");
  let result = Solution::reverse_words(s);
  println!("{}", result);
}
