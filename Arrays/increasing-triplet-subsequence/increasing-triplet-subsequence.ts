// https://leetcode.com/problems/increasing-triplet-subsequence/
function increasingTriplet(nums: number[]): boolean {
  let min = Infinity;
  let min2 = Infinity;

  for (let i = 0; i < nums.length; i++) {
    if (nums[i] <= min) {
      min = nums[i];
    } else if (nums[i] <= min2) {
      min2 = nums[i];
    } else {
      return true;
    }
  }

  return false;
}
