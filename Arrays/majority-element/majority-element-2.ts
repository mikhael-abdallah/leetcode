// https://leetcode.com/problems/majority-element
function majorityElement2(nums: number[]): number {
  nums = nums.sort((a, b) => a - b);
  return nums[Math.floor(nums.length / 2)];
}
