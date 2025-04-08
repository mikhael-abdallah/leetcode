// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/
struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut prefix_size = 0;
        let mut left = left;
        let mut right = right;

        while left < right {
            left = left >> 1;
            right = right >> 1;
            prefix_size += 1;
        }

        left << prefix_size
    }
}

fn main() {
    let left = 2147483646;
    let right = 2147483647;
    let res = Solution::range_bitwise_and(left, right);
    println!("{}", res);
}
