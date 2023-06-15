/*
    A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

    Given a string s, return true if it is a palindrome, or false otherwise.

    Constraints:

    1 <= s.length <= 2 * 10^5
    s consists only of printable ASCII characters.
 */

pub struct Solution;

impl Solution {
    fn get_ascii_lowercase_alphanumeric(c: char) -> Option<char> {
        let c = c.to_ascii_lowercase();
        match c {
            'a'..='z' | '0'..='9' => Some(c),
            _ => None
        }
    }

    pub fn is_palindrome(s: String) -> bool {
        let n = s.len();
        if n <= 1 { return true; }
        let l = (n + 1) / 2;
        let f = s.chars()
            .filter_map(Solution::get_ascii_lowercase_alphanumeric)
            .take(l);
        let r = s.chars().rev()
            .filter_map(Solution::get_ascii_lowercase_alphanumeric)
            .take(l);
        f.eq(r)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "A man, a plan, a canal: Panama"
        Output: true
        Explanation: "amanaplanacanalpanama" is a palindrome.
     */
    #[test]
    fn example1() {
        let s = "A man, a plan, a canal: Panama".to_string();

        let p = Solution::is_palindrome(s);

        assert!(p);
    }

    /*
        Input: s = "race a car"
        Output: false
        Explanation: "raceacar" is not a palindrome.
     */
    #[test]
    fn example2() {
        let s = "race a car".to_string();

        let p = Solution::is_palindrome(s);

        assert!(!p);
    }

    /*
        Input: s = " "
        Output: true
        Explanation: s is an empty string "" after removing non-alphanumeric characters.
        Since an empty string reads the same forward and backward, it is a palindrome.
     */
    #[test]
    fn example3() {
        let s = " ".to_string();

        let p = Solution::is_palindrome(s);

        assert!(p);
    }
}