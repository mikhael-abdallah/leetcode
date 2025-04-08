// https://leetcode.com/problems/reverse-bits/
const reverseBits = (n: number) => {
  let res = 0;

  for (let i = 0; i < 32; i++) {
    res = res << 1;
    res += n & 1;
    n = n >> 1;
  }

  return res >>> 0;
};
