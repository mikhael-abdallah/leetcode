struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0; (n + 1) as usize];
        for i in 0..n + 1 {
            ans[i as usize] = ans[(i >> 1) as usize] + (i & 1)
        }

        return ans;
    }
}

fn main() {
    let n = 5;
    let res = Solution::count_bits(n);
    println!("{:?}", res);
}
