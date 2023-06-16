/*
    Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

    A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

    Constraints:

    0 <= s.length <= 100
    0 <= t.length <= 10^4
    s and t consist only of lowercase English letters.
 */

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = s.chars().peekable();
        for c in t.chars() {
            if let Some(&x) = iter.peek() {
                if c == x { iter.next(); }
            } else { return true; }
        }
        iter.peek().is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "abc", t = "ahbgdc"
        Output: true
     */
    #[test]
    fn example1() {
        let s = "abc".to_string();
        let t = "ahbgdc".to_string();

        let b = Solution::is_subsequence(s, t);

        assert!(b);
    }

    /*
        Input: s = "axc", t = "ahbgdc"
        Output: false
     */
    #[test]
    fn example2() {
        let s = "axc".to_string();
        let t = "ahbgdc".to_string();

        let b = Solution::is_subsequence(s, t);

        assert!(!b);
    }
}