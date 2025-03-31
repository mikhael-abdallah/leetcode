package main

func maxProfit(prices []int) int {
    maxProfit := 0
    minPrice := prices[0]
    
    for i := 1; i < len(prices); i++ {
        price := prices[i]
        minPrice = min(minPrice, price)
        maxProfit = max(maxProfit, price - minPrice)
    }
    
    return maxProfit
}
