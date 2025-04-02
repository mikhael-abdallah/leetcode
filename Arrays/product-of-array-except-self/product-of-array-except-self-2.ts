// https://leetcode.com/problems/product-of-array-except-self
function productExceptSelf(nums: number[]): number[] {
  // (1, x-1) * (x+1, n)
  // this approach save space by using the result array as L and a variable to R
  const length = nums.length;
  const result = Array.from({ length }) as number[];
  result[0] = 1;
  for (let i = 1; i < length; i++) {
    result[i] = nums[i - 1] * result[i - 1];
  }

  let R = 1;

  for (let i = 1; i < length; i++) {
    result[i] *= R;
    R *= nums[i];
  }

  return result;
}
