// https://leetcode.com/problems/number-of-1-bits/
package main

func hammingWeight(n int) int {
	count := 0
	for n != 0 {
		count += n & 1
		n = n >> 1
	}
	return count
}
