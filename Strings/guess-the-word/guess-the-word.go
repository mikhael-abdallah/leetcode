// https://leetcode.com/problems/guess-the-word/
package main

type Master struct {
}

func (this *Master) Guess(word string) int {
	return 0
}

func findSecretWord(words []string, master *Master) {
	charCounts := make([]int, 26)
	for _, word := range words {
		for _, char := range word {
			charCounts[char-'a']++
		}
	}

	correctChars := 0

	for correctChars < 6 && len(words) > 0 {
		bestWord := findBestCandidate(words, charCounts)
		correctChars = master.Guess(bestWord)
		words = filterWords(words, bestWord, correctChars, charCounts)
	}
}

func findBestCandidate(words []string, charCounts []int) string {
	maxSum := 0
	bestWord := ""
	for _, word := range words {
		sum := 0
		for _, char := range word {
			sum += charCounts[char-'a']
		}
		if sum > maxSum {
			maxSum = sum
			bestWord = word
		}
	}
	return bestWord
}

func filterWords(words []string, candidate string, candidateCorrectChars int, charCounts []int) []string {
	filteredWords := make([]string, 0)
	for _, word := range words {
		matches := 0
		for i := 0; i < len(candidate); i++ {
            if candidate[i] == word[i] {
                matches++
            }
        }
		if matches == candidateCorrectChars {
			filteredWords = append(filteredWords, word)
		} else {
			for _, char := range candidate {
				charCounts[char-'a']--
			}
		}
	}
	return filteredWords
}
