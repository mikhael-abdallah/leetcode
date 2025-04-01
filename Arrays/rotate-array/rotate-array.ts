// https://leetcode.com/problems/rotate-array/description/

/**
 Do not return anything, modify nums in-place instead.
 */
function rotate(nums: number[], k: number): void {
  // Normalize k to avoid unnecessary full rotations (when k > nums.length)
  k = k % nums.length;

  let rotatedCount = 0;

  // Iterate until all elements have been rotated
  for (let start = 0; rotatedCount < nums.length; start++) {
    let currentIndex = start;
    let previousValue = nums[start];

    // Rotate elements in steps of k until returning to the starting position
    do {
      const nextIndex = (currentIndex + k) % nums.length;

      // Swap current and next elements
      const tempValue = nums[nextIndex];
      nums[nextIndex] = previousValue;
      previousValue = tempValue;

      currentIndex = nextIndex;
      rotatedCount++;
    } while (start !== currentIndex); // Stop when the cycle completes
  }
}
