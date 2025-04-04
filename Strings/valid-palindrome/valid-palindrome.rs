
// https://leetcode.com/problems/valid-palindrome/

struct Solution;

impl Solution {
  pub fn is_palindrome(s: String) -> bool {
    let lower_case = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    let chars: Vec<char> = lower_case.chars().collect();

    let half = lower_case.len() / 2;

    for i in 0..half {
      if chars[i] != chars[chars.len() - i - 1] {
        return false;
      }
    }

    true
  }
}

fn main() {
  let s = String::from("A man, a plan, a canal: Panama");
  let result = Solution::is_palindrome(s);
  println!("{}", result);
}
