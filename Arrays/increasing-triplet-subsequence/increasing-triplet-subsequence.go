// https://leetcode.com/problems/increasing-triplet-subsequence/
package main

import "math"
func increasingTriplet(nums []int) bool {
    min := math.MaxInt32
    min2 := math.MaxInt32

    for _, num := range nums {
        if num <= min {
            min = num
        } else if num <= min2 {
            min2 = num
        } else {
            return true
        }
    }

    return false
}
