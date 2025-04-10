struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for num in &nums {
            xor ^= num;
        }
        let mut last_bit = xor & (-xor);

        let mut x = 0;

        for num in nums {
            if (num & last_bit) == last_bit {
                x ^= num;
            }
        }

        vec![x, xor ^ x]
    }
}

fn main() {
    let nums = vec![1, 2, 1, 3, 2, 5];
    let result = Solution::single_number(nums);
    println!("{:?}", result);
}
