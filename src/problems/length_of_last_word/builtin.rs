/*
    Given a string s consisting of words and spaces, return the length of the last word in the string.

    A word is a maximal substring consisting of non-space characters only.

    Constraints:

    1 <= s.length <= 10^4
    s consists of only English letters and spaces ' '.
    There will be at least one word in s.
 */

pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace().last().map(|w| w.len()).unwrap_or_default() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "Hello World"
        Output: 5
        Explanation: The last word is "World" with length 5.
    */
    #[test]
    fn example1() {
        let s = "Hello World".to_string();

        let n = Solution::length_of_last_word(s);

        assert_eq!(n, 5);
    }

    /*
        Input: s = "   fly me   to   the moon  "
        Output: 4
        Explanation: The last word is "moon" with length 4.
    */
    #[test]
    fn example2() {
        let s = "   fly me   to   the moon  ".to_string();

        let n = Solution::length_of_last_word(s);

        assert_eq!(n, 4);
    }

    /*
        Input: s = "luffy is still joyboy"
        Output: 6
        Explanation: The last word is "joyboy" with length 6.
    */
    #[test]
    fn example3() {
        let s = "luffy is still joyboy".to_string();

        let n = Solution::length_of_last_word(s);

        assert_eq!(n, 6);
    }
}