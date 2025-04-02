// https://leetcode.com/problems/product-of-array-except-self
package main

func productExceptSelf(nums []int) []int {
  // (1, x-1) * (x+1, n)
  length := len(nums)
  l := make([]int, length)
  r := make([]int, length)
  result := make([]int, length)

  l[0] = 1
  r[length-1] = 1

  for i := 1; i < length; i++ {
    l[i] = nums[i-1] * l[i-1]
  }

  for i := length - 2; i >= 0; i-- {
    r[i] = nums[i+1] * r[i+1]
  }

  for i := 0; i < length; i++ {
    result[i] = l[i] * r[i]
  }

  return result
}