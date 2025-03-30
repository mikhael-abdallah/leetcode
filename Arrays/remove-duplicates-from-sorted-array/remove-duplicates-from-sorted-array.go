// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
package main

func removeDuplicates(nums []int) int {
    if len(nums) == 0 {
        return 0
    }

    distinctCount := 1
    for i := 1; i < len(nums); i++ {
        if nums[i] != nums[distinctCount-1] {
            nums[distinctCount] = nums[i]
            distinctCount++
        }
    }

    return distinctCount
}