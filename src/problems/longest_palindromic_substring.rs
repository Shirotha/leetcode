/*
    Given a string s, return the longest palindromic substring in s.

    Constraints:

    1 <= s.length <= 1000
    s consist of only digits and English letters.
 */

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n == 1 { return s; }
        let b = s.as_bytes();
        let mut length = vec![0u16; (n << 1) - 1];
        for (i, v) in length[0..n].iter_mut().enumerate() {
            *v = 1;
            for j in 1..=i.min(n - i - 1) {
                if b[i - j] == b[i + j] { *v += 2; } else { break; }
            }
        }
        for (i, v) in length[n..].iter_mut().enumerate() {
            for j in 1..=(i + 1).min(n - i - 1) {
                if b[i + 1 - j] == b[i + j] { *v += 2; } else { break; }
            }
        }
        let (mut i, mut max) = (0, 0);
        for (j, l) in length.into_iter().enumerate() {
            if l > max { i = j; max = l; }
        }
        let r = (max >> 1) as usize;
        if i < n { s[(i - r)..=(i + r)].to_string() }
        else { s[(i + 1 - r - n)..=(i + r - n)].to_string() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "babad"
        Output: "bab"
        Explanation: "aba" is also a valid answer.
     */
    #[test]
    fn example1() {
        let s = "babad".to_string();

        let p = Solution::longest_palindrome(s);

        assert_eq!(p.as_str(), "bab");
    }

    /*
        Input: s = "cbbd"
        Output: "bb"
     */
    #[test]
    fn example2() {
        let s = "cbbd".to_string();

        let p = Solution::longest_palindrome(s);

        assert_eq!(p.as_str(), "bb");
    }
}