// https://leetcode.com/problems/longest-common-prefix/

function longestCommonPrefix(strs: string[]): string {
  for (let i = 0; i < 200; i++) {
    let first = strs[0][i];
    for (let j = 1; j < strs.length; j++) {
      if (strs[j][i] != first) {
        return strs[0].substring(0, i);
      }
    }
  }
  return strs[0];
}
