// https://leetcode.com/problems/single-number/
package main

func singleNumber(nums []int) int {
	res := 0
	for _, num := range nums {
		res ^= num
	}
	return res
}

