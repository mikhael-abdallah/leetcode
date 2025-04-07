// https://leetcode.com/problems/single-number/
struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;

        for number in nums {
            res ^= number;
        }

        return res;
    }
}

fn main() {
    let nums = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    ];
    let res = Solution::single_number(nums);
    println!("{}", res);
}
