// https://leetcode.com/problems/first-missing-positive/

struct Solution;

impl Solution {
  pub fn first_missing_positive(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    let mut i = 0;
    while i < n {
      let index = nums[i] - 1;
      if index >= 0 && nums[index as usize] != nums[i] {
        nums.swap(index as usize, i);
      } else {
        i += 1;
      }
    }

    for i in 0..n {
      if nums[i] != i as i32 + 1 {
        return i as i32 + 1;
      }
    }
    n as i32 + 1
  }
}

fn main() {
  let mut nums = vec![1, 2, 0];
  let result = Solution::first_missing_positive(&mut nums);
  println!("{}", result);
}