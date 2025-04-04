// https://leetcode.com/problems/reverse-words-in-a-string/description/

function reverseWords(s: string): string {
  return s.split(" ").filter(Boolean).reverse().join(" ");
}

console.log(reverseWords("the sky is blue"));
