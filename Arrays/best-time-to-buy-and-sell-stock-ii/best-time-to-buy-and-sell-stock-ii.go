// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
package main

func maxProfit(prices []int) int {
    maxProfit := 0
    
    for i := 1; i < len(prices); i++ {
        if prices[i] > prices[i-1] {
            maxProfit += prices[i] - prices[i-1]
        }
    }

    return maxProfit
}