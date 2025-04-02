// https://leetcode.com/problems/product-of-array-except-self

pub struct Solution {}

impl Solution {
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // (1, x-1) * (x+1, n)
    let length = nums.len();
    let mut l = vec![1; length];
    let mut r = vec![1; length];
    let mut result = vec![0; length];

    for i in 1..length {
      l[i] = nums[i - 1] * l[i - 1];
    }

    for i in (0..length - 1).rev() {
      r[i] = nums[i + 1] * r[i + 1];
    }
    
    for i in 0..length {
      result[i] = l[i] * r[i];
    }

    result
  }
}

fn main() {
  let nums = vec![1, 2, 3, 4];
  let result = Solution::product_except_self(nums);
  println!("{:?}", result);
}