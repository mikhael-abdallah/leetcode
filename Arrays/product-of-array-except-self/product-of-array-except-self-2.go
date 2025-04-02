// https://leetcode.com/problems/product-of-array-except-self
package main

func productExceptSelf2(nums []int) []int {
  // (1, x-1) * (x+1, n)
  // this approach save space by using the result array as L and a variable to R
  length := len(nums)
  l := make([]int, length)
  l[0] = 1

  for i := 1; i < length; i++ {
    l[i] = nums[i-1] * l[i-1]
  }

  r := 1

  for i := length - 1; i >= 0; i-- {
    l[i] = l[i] * r
    r *= nums[i]
  }

  return l
}