// https://leetcode.com/problems/majority-element
// https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_majority_vote_algorithm
function majorityElement3(nums: number[]): number {
  let candidate = nums[0];
  let count = 0;

  for (let num of nums) {
    if (count == 0) {
      candidate = num;
      count = 1;
    } else if (candidate == num) {
      count++;
    } else {
      count--;
    }
  }
  return candidate;
}
