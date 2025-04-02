// https://leetcode.com/problems/majority-element
// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm

pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = nums[0];

        for num in nums {
            if count == 0 {
                candidate = num;
                count = 1;
            } else if candidate == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        candidate
    }
}

fn main() {
    let nums = vec![3, 2, 3];
    let result = Solution::majority_element(nums);
    println!("{}", result);
}
