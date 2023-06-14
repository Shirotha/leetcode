/*
    Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

    Constraints:

    1 <= haystack.length, needle.length <= 10^4
    haystack and needle consist of only lowercase English characters.
 */

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(needle.as_str()).map_or(-1, |i| i as i32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: haystack = "sadbutsad", needle = "sad"
        Output: 0
        Explanation: "sad" occurs at index 0 and 6.
        The first occurrence is at index 0, so we return 0.
     */
    #[test]
    fn example1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();

        let i = Solution::str_str(haystack, needle);

        assert_eq!(i, 0);
    }

    /*
        Input: haystack = "leetcode", needle = "leeto"
        Output: -1
        Explanation: "leeto" did not occur in "leetcode", so we return -1.
     */
    #[test]
    fn example2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();

        let i = Solution::str_str(haystack, needle);

        assert_eq!(i, -1);
    }
}