use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = prices[0];

        for price in prices {
            min_price = cmp::min(min_price, price);
            max_profit = cmp::max(max_profit, price - min_price);
        }

        max_profit
    }
}