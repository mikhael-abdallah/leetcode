// https://leetcode.com/problems/first-missing-positive/

package main
func firstMissingPositive(nums []int) int {
    n := len(nums)
    i := 0;
    for i < n {
        index := nums[i] - 1
        if index >= 0 && index < n && nums[index] != nums[i] {
            nums[index], nums[i] = nums[i], nums[index]
        } else {
            i++
        }
    }
    for i := 0; i < n; i++ {
        if nums[i] != i+1 {
            return i + 1
        }
    }
    return n + 1
}