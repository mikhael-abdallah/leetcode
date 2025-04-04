// https://leetcode.com/problems/valid-palindrome/

package main

import "strings"

func isPalindrome(s string) bool {
	s = strings.ToLower(s)
	b := []byte(s)
	i := 0
	j := len(b) - 1

	for i < j {
		if i < j && !isAlphanumeric(b[i]) {
			i++
		} else if i < j && !isAlphanumeric(b[j]) {
			j--
		} else if b[i] != b[j] {
			return false
		} else {
			i++
			j--
		}
	}

	return true
}

func isAlphanumeric(b byte) bool {
	return (b >= 'a' && b <= 'z') || (b >= '0' && b <= '9')
}
