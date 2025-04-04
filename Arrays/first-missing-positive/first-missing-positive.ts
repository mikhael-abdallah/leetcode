// https://leetcode.com/problems/first-missing-positive/

function firstMissingPositive(nums: number[]): number {
  const n = nums.length;
  let i = 0;
  while (i < n) {
    let index = nums[i] - 1;
    if (index >= 0 && index < n && nums[index] != nums[i]) {
      let temp = nums[index];
      nums[index] = nums[i];
      nums[i] = temp;
    } else {
      i++;
    }
  }

  for (let i = 0; i < n; i++) {
    if (nums[i] != i + 1) {
      return i + 1;
    }
  }
  return n + 1;
}
