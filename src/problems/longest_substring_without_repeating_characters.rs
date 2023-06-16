/*
    Given a string s, find the length of the longest substring without repeating characters.

    Constraints:

    0 <= s.length <= 5 * 10^4
    s consists of English letters, digits, symbols and spaces.
 */

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut lookup = HashMap::new();
        let mut j = 0;
        let mut best = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(&k) = lookup.get(&c) {
                if k >= j {
                    let d = i - j;
                    if d > best { best = d; }
                    j = k + 1;
                }
            }
            lookup.insert(c, i);
        }
        best.max(s.len() - j) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "abcabcbb"
        Output: 3
        Explanation: The answer is "abc", with the length of 3.
     */
    #[test]
    fn example1() {
        let s = "abcabcbb".to_string();

        let l = Solution::length_of_longest_substring(s);

        assert_eq!(l, 3);
    }

    /*
        Input: s = "bbbbb"
        Output: 1
        Explanation: The answer is "b", with the length of 1.
     */
    #[test]
    fn example2() {
        let s = "bbbbb".to_string();

        let l = Solution::length_of_longest_substring(s);

        assert_eq!(l, 1);
    }

    /*
        Input: s = "pwwkew"
        Output: 3
        Explanation: The answer is "wke", with the length of 3.
        Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.     
     */
    #[test]
    fn example3() {
        let s = "pwwkew".to_string();

        let l = Solution::length_of_longest_substring(s);

        assert_eq!(l, 3);
    }

    /*
        Input: s = "aabaab!bb"
        Output: 3
     */
    #[test]
    fn example4() {
        let s = "aabaab!bb".to_string();

        let l = Solution::length_of_longest_substring(s);

        assert_eq!(l, 3);
    }
}