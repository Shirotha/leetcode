/*
    Given an input string s, reverse the order of the words.

    A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

    Return a string of the words in reverse order concatenated by a single space.

    Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

    Constraints:

    1 <= s.length <= 10^4
    s contains English letters (upper-case and lower-case), digits, and spaces ' '.
    There is at least one word in s.
 */

pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        for w in s.split_ascii_whitespace().rev() {
            result.push_str(w);
            result.push(' ');
        }
        result.truncate(result.len() - 1);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "the sky is blue"
        Output: "blue is sky the"
     */
    #[test]
    fn example1() {
        let s = "the sky is blue".to_string();

        let r = Solution::reverse_words(s);

        assert_eq!(r.as_str(), "blue is sky the");
    }

    /*
        Input: s = "  hello world  "
        Output: "world hello"
        Explanation: Your reversed string should not contain leading or trailing spaces.
     */
    #[test]
    fn example2() {
        let s = "  hello world  ".to_string();

        let r = Solution::reverse_words(s);

        assert_eq!(r.as_str(), "world hello");
    }

    /*
        Input: s = "a good   example"
        Output: "example good a"
        Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
     */
    #[test]
    fn example3() {
        let s = "a good   example".to_string();

        let r = Solution::reverse_words(s);

        assert_eq!(r.as_str(), "example good a");
    }
}