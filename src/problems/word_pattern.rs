/*
    Given a pattern and a string s, find if s follows the same pattern.

    Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.

    Constraints:

    1 <= pattern.length <= 300
    pattern contains only lower-case English letters.
    1 <= s.length <= 3000
    s contains only lowercase English letters and spaces ' '.
    s does not contain any leading or trailing spaces.
    All the words in s are separated by a single space.
 */

pub struct Solution;

use std::iter::repeat;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut map = HashMap::new();
        let mut taken = HashSet::new();
        let mut n = 0;
        for (a, b) in pattern.chars().chain(repeat('\0'))
            .zip(s.split_ascii_whitespace()) 
        {
            n += 1;
            if a == '\0' { return false; }
            if let Some(c) = map.get(&a) {
                if c != &b { return false; }
            } else if taken.contains(&b) {
                return false;
            } else {
                map.insert(a, b);
                taken.insert(b);
            }
        }
        n == pattern.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: pattern = "abba", s = "dog cat cat dog"
        Output: true
     */
    #[test]
    fn example1() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();

        let b = Solution::word_pattern(pattern, s);

        assert!(b);
    }

    /*
        Input: pattern = "abba", s = "dog cat cat fish"
        Output: false
     */
    #[test]
    fn example2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();

        let b = Solution::word_pattern(pattern, s);

        assert!(!b);
    }

    /*
        Input: pattern = "aaaa", s = "dog cat cat dog"
        Output: false
     */
    #[test]
    fn example3() {
        let pattern = "aaaa".to_string();
        let s = "dog cat cat dog".to_string();

        let b = Solution::word_pattern(pattern, s);

        assert!(!b);
    }

    /*
        Input: pattern = "aaa", s = "aa aa aa aa"
        Output: false
     */
    #[test]
    fn example4() {
        let pattern = "aaa".to_string();
        let s = "aa aa aa aa".to_string();

        let b = Solution::word_pattern(pattern, s);

        assert!(!b);
    }

    /*
        Input: pattern = "aaa", s = "aa aa"
        Output: false
     */
    #[test]
    fn example5() {
        let pattern = "aaa".to_string();
        let s = "aa aa".to_string();

        let b = Solution::word_pattern(pattern, s);

        assert!(!b);
    }
}