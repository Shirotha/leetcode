/*
    You are given a string s and an array of strings words. All the strings of words are of the same length.

    A concatenated substring in s is a substring that contains all the strings of any permutation of words concatenated.

    For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated substring because it is not the concatenation of any permutation of words.
    Return the starting indices of all the concatenated substrings in s. You can return the answer in any order.

    Constraints:

    1 <= s.length <= 10^4
    1 <= words.length <= 5000
    1 <= words[i].length <= 30
    s and words[i] consist of lowercase English letters.
 */

pub struct Solution;

use std::collections::{HashMap, VecDeque};
use std::cmp::Ordering::*;

struct Word {
    index: usize,
    count: usize,
}

impl Word {
    fn new(index: usize, count: usize) -> Word {
        Word { index, count }
    }
}

struct State {
    indices: Vec<VecDeque<usize>>,
    start: usize,
    length: usize,
    sequence: usize,
}

impl State {
    fn new(offset: usize, unique_words: usize, word_length: usize, sequence_length: usize) -> State {
        State { 
            indices: vec![VecDeque::new(); unique_words], 
            start: offset, 
            length: word_length,
            sequence: sequence_length - word_length,
        }
    }

    fn update(&mut self, word: &Word, index: usize) -> Option<usize> {
        unsafe {
            let state = self.indices.get_unchecked_mut(word.index);
            state.push_front(index);
            while *state.back().unwrap_unchecked() < self.start {
                state.pop_back();
            }
            match state.len().cmp(&word.count) {
                Greater => {
                    self.start = state.pop_back().unwrap_unchecked() + self.length;
                },
                Equal => {
                    if self.start + self.sequence == index {
                        let result = Some(self.start);
                        self.start += self.length;
                        return result;
                    }
                },
                Less => ()
            }
        }
        None
    }

    fn clear(&mut self, index: usize) {
        if self.start < index {
            self.indices.iter_mut().for_each(VecDeque::clear);
        }
        self.start = index + self.length;
    }
}

impl Solution {
    fn create_lookup(words: &[String]) -> HashMap<&str, Word> {
        let mut lookup: HashMap<&str, Word> = HashMap::new();
        let mut i = 0;
        for w in words {
            if let Some(word) = lookup.get_mut(w.as_str()) {
                word.count += 1;
            } else {
                lookup.insert(w, Word::new(i, 1));
                i += 1;
            }
        }
        lookup
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let n = words.len();
        if n == 0 { return vec![]; }
        let l = words[0].len();
        let seq = n * l;
        if seq > s.len() { return vec![]; }
        let mut result = vec![];
        let words = Self::create_lookup(&words);
        let n = words.len();
        let mut state = HashMap::new();
        for i in 0..=(s.len() - l) {
            if let Some(word) = words.get(&s[i..(i+l)]) {
                let current = state.entry(i % l)
                    .or_insert_with(|| State::new(i, n, l, seq));
                if let Some(x) = current.update(word, i) {
                    result.push(x as i32);
                }
            } else {
                state.entry(i % l).and_modify(|current| current.clear(i) );
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "barfoothefoobarman", words = ["foo","bar"]
        Output: [0,9]
        Explanation: Since words.length == 2 and words[i].length == 3, the concatenated substring has to be of length 6.
        The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
        The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
        The output order does not matter. Returning [9,0] is fine too.
     */
    #[test]
    fn example1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo","bar"].into_iter()
            .map(str::to_string).collect();

        let is = Solution::find_substring(s, words);

        assert_eq!(is, vec![0,9]);
    }

    /*
        Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
        Output: []
        Explanation: Since words.length == 4 and words[i].length == 4, the concatenated substring has to be of length 16.
        There is no substring of length 16 is s that is equal to the concatenation of any permutation of words.
        We return an empty array.
     */
    #[test]
    fn example2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec!["word","good","best","word"].into_iter()
            .map(str::to_string).collect();

        let is = Solution::find_substring(s, words);

        assert!(is.is_empty());
    }

    /*
        Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
        Output: [6,9,12]
        Explanation: Since words.length == 3 and words[i].length == 3, the concatenated substring has to be of length 9.
        The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"] which is a permutation of words.
        The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"] which is a permutation of words.
        The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"] which is a permutation of words.
     */
    #[test]
    fn example3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar","foo","the"].into_iter()
            .map(str::to_string).collect();

        let is = Solution::find_substring(s, words);

        assert_eq!(is, vec![6,9,12]);
    }

    /*
        Input: s = "aaaaaaaaaaaaaa", words = ["aa","aa"]
        Output: [0,1,2,3,4,5,6,7,8,9,10]
     */
    #[test]
    fn example4() {
        let s = "aaaaaaaaaaaaaa".to_string();
        let words = vec!["aa","aa"].into_iter()
            .map(str::to_string).collect();

        let is = Solution::find_substring(s, words);

        assert_eq!(is, vec![0,1,2,3,4,5,6,7,8,9,10]);
    }

    /*
        Input: s = "lingmindraboofooowingdingbarrwingmonkeypoundcake", words = ["fooo","barr","wing","ding","wing"]
        Output: [13]
     */
    #[test]
    fn example5() {
        let s = "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string();
        let words = vec!["fooo","barr","wing","ding","wing"].into_iter()
            .map(str::to_string).collect();

        let is = Solution::find_substring(s, words);

        assert_eq!(is, vec![13]);
    }

    /*
        Input: s = "bcabbcaabbccacacbabccacaababcbb", words = ["c","b","a","c","a","a","a","b","c"]
        Output: [6,16,17,18,19,20]
     */
    #[test]
    fn example6() {
        let s = "bcabbcaabbccacacbabccacaababcbb".to_string();
        let words = vec!["c","b","a","c","a","a","a","b","c"].into_iter()
            .map(str::to_string).collect();

        let is = Solution::find_substring(s, words);

        assert_eq!(is, vec![6,16,17,18,19,20]);
    }
}