// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/

function rangeBitwiseAnd(left: number, right: number): number {
  let prefixSize = 0;
  while (left < right) {
    left = left >> 1;
    right = right >> 1;
    prefixSize++;
  }
  return left << prefixSize;
}
