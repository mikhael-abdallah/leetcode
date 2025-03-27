// https://leetcode.com/problems/move-zeroes/description/
/**
 Do not return anything, modify nums in-place instead.
 */
function moveZeroes(nums: number[]): void {
  const length = nums.length;
  let zeroCount = 0;
  for (let i = 0; i < length; i++) {
    if (nums[i] == 0) {
      zeroCount++;
    } else {
      nums[i - zeroCount] = nums[i]; // move the non-zero element to the left
    }
  }
  for (let i = 0; i < zeroCount; i++) {
    nums[length - i - 1] = 0; // move the zero to the right
  }
}
