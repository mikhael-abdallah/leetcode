// https://leetcode.com/problems/number-of-1-bits/

struct Solution;
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n != 0 {
            count += n & 1;
            n = n >> 1;
        }
        return count;
    }
}

fn main() {
    let n = 11;
    let res = Solution::hamming_weight(n);
    println!("{}", res);
}
