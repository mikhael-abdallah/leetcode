// https://leetcode.com/problems/zigzag-conversion/description/

package main

import "strings"
func convert(s string, numRows int) string {
    if numRows == 1 {
        return s
    }

    res := make([]string, numRows)
    direction := 1
    actualColumn := 0
    chars := []rune(s)

    for _, c := range chars {
        res[actualColumn] += string(c)

        actualColumn += direction

        if actualColumn < 0 || actualColumn >= numRows {
            direction *= -1
            actualColumn += 2 * direction
        }
    }

    return strings.Join(res, "")
}
