// https://leetcode.com/problems/majority-element
// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
package main

func majorityElement(nums []int) int {
    count := 0
    candidate := nums[0]

    for _, num := range nums {
        if count == 0 {
            candidate = num
            count = 1
        } else if candidate == num {
            count++
        } else {
            count--
        }
    }
    return candidate
}


