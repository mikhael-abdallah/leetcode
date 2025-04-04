// https://leetcode.com/problems/reverse-words-in-a-string/description/
package main

import "strings"
func reverseWords(s string) string {
    words := strings.Split(s, " ")
    // filter empty strings
    // filter undefined
    words = filter(words, func(word string) bool {
        return word != ""
    })
    for i, j := 0, len(words)-1; i < j; i, j = i+1, j-1 {
        words[i], words[j] = words[j], words[i]
    }
    return strings.Join(words, " ")
}


func filter(ss []string, test func(string) bool) (ret []string) {
    for _, s := range ss {
        if test(s) {
            ret = append(ret, s)
        }
    }
    return
}