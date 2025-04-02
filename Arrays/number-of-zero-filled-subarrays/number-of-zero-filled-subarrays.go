// https://leetcode.com/problems/number-of-zero-filled-subarrays/

package main

func zeroFilledSubarray(nums []int) int64 {
  // Arithmetric Progression Sum for each sequence of zeroes
  res := int64(0)
  count := int64(0)

  for _, num := range nums {
    if num == 0 {
      count++
    } else {
      res += (count * (count + 1)) / 2
      count = 0
    }
  }
  res += (count * (count + 1)) / 2
  return res
}