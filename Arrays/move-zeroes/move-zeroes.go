// https://leetcode.com/problems/move-zeroes/description/
package main

func moveZeroes(nums []int)  {
    zeroCount := 0
    for i := 0; i < len(nums); i++ {
        if nums[i] == 0 {
            zeroCount++
        } else {
            nums[i-zeroCount] = nums[i]
        }
    }
    for i := 0; i < zeroCount; i++ {
        nums[len(nums)-i-1] = 0
    }
}
