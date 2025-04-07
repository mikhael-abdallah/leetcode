function countBits(n: number): number[] {
  const ans = new Array(n + 1).fill(0) as number[];
  for (let i = 0; i <= n; i++) {
    ans[i] = ans[i >> 1] + (i & 1);
  }
  return ans;
}
