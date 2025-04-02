// https://leetcode.com/problems/rotate-array/description/

pub struct Solution {}
impl Solution {
  pub fn rotate(nums: &mut Vec<i32>, k: i32) {
      let len = nums.len();
      if len == 0 {
          return;
      }
      
      let k = (k as usize) % len;
      if k == 0 {
          return;
      }
      
      let mut rotated_count = 0;
      
      // Iterate until all elements have been rotated
      for start in 0..len {
          if rotated_count >= len {
              break;
          }
          
          let mut current_index = start;
          let mut previous_value = nums[start];
          
          // Rotate elements in steps of k until returning to the starting position
          loop {
              let next_index = (current_index + k) % len;
              
              std::mem::swap(&mut nums[next_index], &mut previous_value);
              
              current_index = next_index;
              rotated_count += 1;
              
              if current_index == start {
                  break;
              }
          }
      }
  }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut nums, k);
    println!("{:?}", nums);
}