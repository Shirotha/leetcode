/*
    Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

    Each letter in magazine can only be used once in ransomNote.

    Constraints:

    1 <= ransomNote.length, magazine.length <= 10^5
    ransomNote and magazine consist of lowercase English letters.
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
    fn create_lookup(note: &str) -> [usize; 26] {
        let mut lookup = [0; 26];
        for c in note.chars() {
            lookup[c.index()] += 1;
        }
        lookup
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut lookup = Self::create_lookup(&ransom_note);
        let mut rem = ransom_note.len();
        for c in magazine.chars() {
            let n = unsafe { lookup.get_unchecked_mut(c.index()) };
            if *n > 0 {
                *n -= 1;
                rem -= 1;
                if rem == 0 { return true; }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: ransomNote = "a", magazine = "b"
        Output: false
     */
    #[test]
    fn example1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();

        let b = Solution::can_construct(ransom_note, magazine);

        assert!(!b);
    }

    /*
        Input: ransomNote = "aa", magazine = "ab"
        Output: false
     */
    #[test]
    fn example2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();

        let b = Solution::can_construct(ransom_note, magazine);

        assert!(!b);
    }

    /*
        Input: ransomNote = "aa", magazine = "aab"
        Output: true
     */
    #[test]
    fn example3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();

        let b = Solution::can_construct(ransom_note, magazine);

        assert!(b);
    }
}