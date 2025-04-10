package main

func singleNumber(nums []int) []int {
	xor := 0
	for _, num := range nums {
		xor ^= num
	}
	lastBit := xor & -xor

	x := 0
	for _, num := range nums {
		if num&lastBit == lastBit {
			x ^= num
		}
	}

	return []int{x, xor ^ x}
}


