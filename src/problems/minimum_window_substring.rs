/*
    Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

    The testcases will be generated such that the answer is unique.

    Constraints:

    m == s.length
    n == t.length
    1 <= m, n <= 10^5
    s and t consist of uppercase and lowercase English letters.
 */

pub struct Solution;

trait Letter {
    fn index(self) -> usize;
}
impl Letter for u8 {
    #[inline]
    fn index(self) -> usize {
        (if self >= 0x61 { self - 0x47 } else { self - 0x41 }) as usize
    }
}
impl Letter for &u8 {
    #[inline]
    fn index(self) -> usize { (*self).index() }
}

impl Solution {
    fn create_lookup(t: &[u8]) -> [i32; 52] {
        let mut lookup = [-1_000_000; 52];
        for c in t.iter() { unsafe {
            let n = lookup.get_unchecked_mut(c.index());
            if *n < 0 { *n = 1; } else { *n += 1; }
        } }
        lookup
    }

    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() { return String::new(); }
        let min = t.len();
        let mut rem = min;
        let mut t = Self::create_lookup(t.as_bytes());
        let b = s.as_bytes();
        let mut best = 0..(s.len()+1);
        let mut i = 0;
        for (j, c) in b.iter().enumerate() {
            let n = unsafe { t.get_unchecked_mut(c.index()) };
            *n -= 1; if *n >= 0 { rem -= 1; }
            while rem == 0 {
                let l = j - i;
                if l + 1 == min { best = i..(j+1); break; }
                if l < best.len() { best = i..(j+1); }
                unsafe {
                    let n = t.get_unchecked_mut(b.get_unchecked(i).index());
                    *n += 1; if *n > 0 { rem += 1; }
                }
                i += 1;
            }
        }
        if best.end > s.len() { String::new() } else { s[best].to_string() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "ADOBECODEBANC", t = "ABC"
        Output: "BANC"
        Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
     */
    #[test]
    fn example1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();

        let w = Solution::min_window(s, t);

        assert_eq!(w.as_str(), "BANC");
    }

    /*
        Input: s = "a", t = "a"
        Output: "a"
        Explanation: The entire string s is the minimum window.
     */
    #[test]
    fn example2() {
        let s = "a".to_string();
        let t = "a".to_string();

        let w = Solution::min_window(s, t);

        assert_eq!(w.as_str(), "a");
    }

    /*
        Input: s = "a", t = "aa"
        Output: ""
        Explanation: Both 'a's from t must be included in the window.
        Since the largest window of s only has one 'a', return empty string.
     */
    #[test]
    fn example3() {
        let s = "a".to_string();
        let t = "aa".to_string();

        let w = Solution::min_window(s, t);

        assert_eq!(w.as_str(), "");
    }

    /*
        Input: s = "a", t = "b"
        Output: ""
     */
    #[test]
    fn example4() {
        let s = "a".to_string();
        let t = "b".to_string();

        let w = Solution::min_window(s, t);

        assert_eq!(w.as_str(), "");
    }
}