// https://leetcode.com/problems/number-of-zero-filled-subarrays/


pub struct Solution {}
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        // Arithmetic Progression Sum for each sequence of zeroes
        let mut count = 0;
        let mut res = 0;

        for num in nums {
            if num == 0 {
                count += 1;
            } else {
                res += (count * (count + 1)) / 2;
                count = 0;
            }
        }
        res += (count * (count + 1)) / 2;

        res
    }
}

fn main() {
    let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
    let res = Solution::zero_filled_subarray(nums);
    // expect 6
    assert_eq!(res, 6);

}

