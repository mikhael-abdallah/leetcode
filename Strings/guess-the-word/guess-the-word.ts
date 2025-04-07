// https://leetcode.com/problems/guess-the-word/

interface Master {
  guess(word: string): number;
}

function findSecretWord(words: string[], master: Master) {
  let candidateCorrectChars = 0;
  while (candidateCorrectChars != 6) {
    let candidate = findBestCandidate(words);
    candidateCorrectChars = master.guess(candidate);
    words = filterWords(words, candidate, candidateCorrectChars);
  }
}

function filterWords(
  words: string[],
  candidate: string,
  candidateCorrectChars: number
): string[] {
  return words.filter((word) => {
    let wordCorrectChars = 0;
    for (let i = 0; i < candidate.length; i++) {
      if (candidate[i] == word[i]) {
        wordCorrectChars++;
      }
    }
    const isMatch = wordCorrectChars == candidateCorrectChars;
    return isMatch;
  });
}
/*
    This function finds the word with the most common characters.
*/
function findBestCandidate(words: string[]): string {
  const chars = Array.from({ length: 26 }).fill(0) as number[];
  for (const word of words) {
    for (let j = 0; j < 6; j++) {
      chars[word.charCodeAt(j) - 97]++;
    }
  }
  let max = 0;
  let maxWord = "";
  for (const word of words) {
    let sum = 0;
    for (let i = 0; i < 6; i++) {
      sum += chars[word.charCodeAt(i) - 97];
    }
    if (sum > max) {
      max = sum;
      maxWord = word;
    }
  }
  return maxWord;
}
