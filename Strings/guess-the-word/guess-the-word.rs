// https://leetcode.com/problems/guess-the-word/

struct Master {}

impl Master {
    fn guess(&self, word: String) -> i32 {
        todo!()
    }
}

struct Solution;

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let mut words = words;

        let mut char_counts: [i32; 26] = Self::build_char_counts(&words);

        while let Some(candidate) = Self::find_best_candidate(&words, &char_counts) {
            let correct_chars = master.guess(candidate.clone()) as usize;

            if correct_chars == 6 {
                break;
            }

            words = Self::filter_words(&words, candidate, correct_chars, &mut char_counts)
                .into_iter()
                .cloned()
                .collect();
        }
    }

    fn build_char_counts(words: &[String]) -> [i32; 26] {
        let mut counts = [0; 26];
        for word in words {
            for c in word.bytes() {
                counts[(c - b'a') as usize] += 1;
            }
        }
        counts
    }

    fn filter_words<'a>(
        words: &'a Vec<String>,
        candidate: &String,
        candidate_correct_chars: usize,
        chars: &mut [i32; 26],
    ) -> Vec<&'a String> {
        words
            .iter()
            .filter(|word| {
                let matches = word
                    .bytes()
                    .zip(candidate.bytes())
                    .filter(|(a, b)| a == b)
                    .count();

                if matches != candidate_correct_chars {
                    for c in candidate.bytes() {
                        chars[(c - b'a') as usize] -= 1;
                    }
                }
                matches == candidate_correct_chars
            })
            .collect()
    }

    fn find_best_candidate<'a>(words: &'a [String], char_counts: &[i32; 26]) -> Option<&'a String> {
        words.iter().max_by_key(|word| {
            word.bytes()
                .map(|c| char_counts[(c - b'a') as usize])
                .sum::<i32>()
        })
    }
}

fn main() {
    let words = vec!["acckzz", "ccbazz", "eiowzz", "abcczz"];
    let master = Master {};
    Solution::find_secret_word(words.into_iter().map(|s| s.to_string()).collect(), &master);
}
