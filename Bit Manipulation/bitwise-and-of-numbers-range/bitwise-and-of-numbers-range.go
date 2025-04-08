// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/
package main
func rangeBitwiseAnd(left int, right int) int {
    prefixSize := 0;

    for left < right {
        left = left >> 1;
        right = right >> 1;
        prefixSize++;
    }

    return left << prefixSize;
    
}