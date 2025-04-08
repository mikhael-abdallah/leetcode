// https://leetcode.com/problems/reverse-bits/
struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut res = 0;
        let mut x = x;
        for _ in 0..32 {
            res = res << 1;
            let bit = x & 1;
            res += bit;
            x = x >> 1;
        }

        return res;
    }
}

fn main() {
    let x = 43261596;
    let res = Solution::reverse_bits(x);
    println!("{}", res);
}
