/*
    Given two strings s and t, return true if t is an anagram of s, and false otherwise.

    An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

    Constraints:

    1 <= s.length, t.length <= 5 * 10^4
    s and t consist of lowercase English letters.
 */

pub struct Solution;

trait Letter {
    fn index(self) -> usize;
}
impl Letter for char {
    #[inline]
    fn index(self) -> usize { self as usize - 0x61 }
}

impl Solution {
    #[inline]
    fn create_lookup(s: &str) -> [usize; 26] {
        let mut lookup = [0; 26];
        for c in s.chars() {
            lookup[c.index()] += 1;
        }
        lookup
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut lookup = Self::create_lookup(&s);
        for c in t.chars() {
            let n = unsafe { lookup.get_unchecked_mut(c.index()) };
            if n == &0 { return false; }
            *n -= 1;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "anagram", t = "nagaram"
        Output: true
     */
    #[test]
    fn example1() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();

        let b = Solution::is_anagram(s, t);

        assert!(b);
    }

    /*
        Input: s = "rat", t = "car"
        Output: false
     */
    #[test]
    fn example2() {
        let s = "rat".to_string();
        let t = "car".to_string();

        let b = Solution::is_anagram(s, t);

        assert!(!b);
    }
}