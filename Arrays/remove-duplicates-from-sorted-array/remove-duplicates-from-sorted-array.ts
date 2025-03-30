// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

function removeDuplicates(nums: number[]): number {
  if (nums.length == 0) return 0;
  let distinctNumbers = 1;
  for (let i = 1; i < nums.length; i++) {
    if (nums[i] !== nums[distinctNumbers - 1]) {
      nums[distinctNumbers++] = nums[i];
    }
  }

  return distinctNumbers;
}
