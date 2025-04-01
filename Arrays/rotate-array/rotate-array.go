package main

// https://leetcode.com/problems/rotate-array/description/

func rotate(nums []int, k int)  {
    n := len(nums)
	if n == 0 {
		return
	}
    k = k % n
    if k == 0 {
        return
    }

    rotated_count := 0

	// Iterate until all elements have been rotated
    for start := 0; rotated_count < n; start++ {
        current_index := start
        previous_value := nums[start]
        
		// Rotate elements in steps of k until returning to the starting position
        for {
            next_index := (current_index + k) % n
            nums[next_index], previous_value = previous_value, nums[next_index]
            current_index = next_index
            rotated_count++
            if current_index == start {
                break
            }
        }
    }
}
