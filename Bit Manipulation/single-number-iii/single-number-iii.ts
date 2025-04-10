function singleNumber(nums: number[]): number[] {
  let xor = 0;
  const n = nums.length;
  for (let i = 0; i < n; i++) {
    xor ^= nums[i];
  }
  const rightMostBit = xor & -xor;

  let x = 0;

  for (let i = 0; i < nums.length; i++) {
    if ((nums[i] & rightMostBit) == rightMostBit) {
      x ^= nums[i];
    }
  }

  return [x, xor ^ x];
}
