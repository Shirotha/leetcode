/*
    Given two strings s and t, determine if they are isomorphic.

    Two strings s and t are isomorphic if the characters in s can be replaced to get t.

    All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

    Constraints:

    1 <= s.length <= 5 * 10^4
    t.length == s.length
    s and t consist of any valid ascii character.
 */

pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = HashMap::new();
        let mut taken = HashSet::new();
        for (a, b) in s.chars().zip(t.chars()) {
            if let Some(c) = map.get(&a) {
                if c != &b { return false; }
            } else if taken.contains(&b) {
                return false;
            } else {
                map.insert(a, b);
                taken.insert(b);
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "egg", t = "add"
        Output: true
     */
    #[test]
    fn example1() {
        let s = "egg".to_string();
        let t = "add".to_string();

        let b = Solution::is_isomorphic(s, t);

        assert!(b);
    }

    /*
        Input: s = "foo", t = "bar"
        Output: false
     */
    #[test]
    fn example2() {
        let s = "foo".to_string();
        let t = "bar".to_string();

        let b = Solution::is_isomorphic(s, t);

        assert!(!b);
    }

    /*
        Input: s = "paper", t = "title"
        Output: true
     */
    #[test]
    fn example3() {
        let s = "paper".to_string();
        let t = "title".to_string();

        let b = Solution::is_isomorphic(s, t);

        assert!(b);
    }

    /*
        Input: s = "badc", t = "baba"
        Output: false
     */
    #[test]
    fn example4() {
        let s = "badc".to_string();
        let t = "baba".to_string();

        let b = Solution::is_isomorphic(s, t);

        assert!(!b);
    }
}