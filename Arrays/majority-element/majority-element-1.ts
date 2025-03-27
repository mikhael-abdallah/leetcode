// https://leetcode.com/problems/majority-element
function majorityElement(nums: number[]): number {
  const countMap = new Map<number, number>();
  const halfLength = nums.length / 2;

  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];
    const count = countMap.get(num) || 0;
    countMap.set(num, count + 1);
  }
  for (const [num, count] of countMap.entries()) {
    if (count > halfLength) {
      return num;
    }
  }
  return -1;
}
