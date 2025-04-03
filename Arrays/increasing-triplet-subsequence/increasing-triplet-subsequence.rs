// https://leetcode.com/problems/increasing-triplet-subsequence/
struct Solution {}

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min = i32::MAX;
        let mut min2 = i32::MAX;

        for num in nums {
            if num <= min {
                min = num;
            } else if num <= min2 {
                min2 = num;
            } else {
                return true;
            }
        }
        
        false
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let result = Solution::increasing_triplet(nums);
    println!("{}", result);
}
