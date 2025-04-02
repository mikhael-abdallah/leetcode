// https://leetcode.com/problems/number-of-zero-filled-subarrays/

function zeroFilledSubarray(nums: number[]): number {
  // Arithmetric Progression Sum for each sequence of zeroes
  let res = 0;
  let zeroCount = 0;
  const length = nums.length;

  for (let i = 0; i < length; i++) {
    if (nums[i] == 0) {
      zeroCount++;
    } else {
      res += ((1 + zeroCount) * zeroCount) / 2;
      zeroCount = 0;
    }
  }
  if (zeroCount > 0) {
    res += ((1 + zeroCount) * zeroCount) / 2;
  }

  return res;
}
