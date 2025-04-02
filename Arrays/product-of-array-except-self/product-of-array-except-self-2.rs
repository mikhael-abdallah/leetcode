// https://leetcode.com/problems/product-of-array-except-self

pub struct Solution {}
impl Solution {
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // (1, x-1) * (x+1, n)
    // this approach save space by using the result array as L and a variable to R
    let length = nums.len();
    let mut l = vec![1; length];

    for i in 1..length {
      l[i] = nums[i - 1] * l[i - 1];
    }

    let mut r = 1;

    for i in (0..length).rev() {
      l[i] = l[i] * r;
      r *= nums[i];
    }

    l
  }
}

fn main() {
  let nums = vec![1, 2, 3, 4];
  let result = Solution::product_except_self(nums);
  println!("{:?}", result);
}
