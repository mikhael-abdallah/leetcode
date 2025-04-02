pub struct Solution {}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total_profit = 0;

        for i in 1..prices.len() {
            let profit = prices[i] - prices[i - 1];
            if profit > 0 {
                total_profit += profit;
            }
        }

        total_profit
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = Solution::max_profit(prices);
    println!("{}", result);
}