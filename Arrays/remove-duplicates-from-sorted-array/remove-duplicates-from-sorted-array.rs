// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut distinct_count: usize = 1;
        
        for i in 1..nums.len() {
            if nums[i] != nums[distinct_count - 1] {
                nums[distinct_count] = nums[i];
        distinct_count += 1;
      }
    }

    distinct_count as i32
  }
}

fn main() {
    let mut nums = vec![1, 1, 2];
    let result = Solution::remove_duplicates(&mut nums);
    println!("{}", result);
}
