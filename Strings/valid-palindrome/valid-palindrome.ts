// https://leetcode.com/problems/valid-palindrome/

function isPalindrome(s: string): boolean {
  const lowerCase = s.toLocaleLowerCase().replace(/[^a-zA-Z]/g, "");

  let n = lowerCase.length;
  let half = n / 2;
  for (let i = 0; i < half; i++) {
    const char1 = lowerCase[i];
    const char2 = lowerCase[n - i - 1];
    if (char1 != char2) {
      return false;
    }
  }
  return true;
}

console.log(isPalindrome("A man, a plan, a canal: Panama"));
