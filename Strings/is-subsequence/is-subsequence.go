// https://leetcode.com/problems/is-subsequence/
package main

func isSubsequence(s string, t string) bool {
  i := 0;
  j := 0;

  for i < len(s) && j < len(t) {
    if s[i] == t[j] {
      i++
    }
    j++
  }

  return i == len(s)
}
