// https://leetcode.com/problems/longest-common-prefix/
package main

func longestCommonPrefix(strs []string) string {
   prefix := ""
    
   for i := 0; i < len(strs[0]); i++ {
    c := strs[0][i]
    for j := 1; j < len(strs); j++ {
        if i >= len(strs[j]) || strs[j][i] != c {
            return prefix
        }
    }
    prefix += string(c)
   }

   return prefix
    
}
