/*
    Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.

    An interleaving of two strings s and t is a configuration where s and t are divided into n and m 
    substrings
    respectively, such that:

    s = s_1 + s_2 + ... + s_n
    t = t_1 + t_2 + ... + t_m
    |n - m| <= 1
    The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
    Note: a + b is the concatenation of strings a and b.

    Constraints:

    0 <= s1.length, s2.length <= 100
    0 <= s3.length <= 200
    s1, s2, and s3 consist of lowercase English letters.
 */

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (n, m, l) = (s1.len(), s2.len(), s3.len());
        if n + m != l { return false; }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut stack = VecDeque::with_capacity(n.min(m) << 1);
        let (mut i, mut j) = (0, 0);
        'outer: loop {
            if i == n {
                while j != m {
                    if s2[j] == s3[i + j] { j += 1; }
                    else if let Some((ii, jj)) = stack.pop_back() {
                        i = ii; j = jj;
                        continue 'outer;
                    } else { return false; }
                }
                return true;
            } else if j == m {
                while i != n {
                    if s1[i] == s3[i + j] { i += 1; }
                    else if let Some((ii, jj)) = stack.pop_back() {
                        i = ii; j = jj;
                        continue 'outer;
                    } else { return false; }
                }
                return true;
            }
            let k = i + j;
            let (a, b) = (s1[i] == s3[k], s2[j] == s3[k]);
            if a {
                if b { stack.push_back((i, j + 1)); }
                i += 1;
            } else if b { j += 1; }
            else if let Some((ii, jj)) = stack.pop_back() {
                i = ii; j = jj;
            } else { return false; }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
        Output: true
        Explanation: One way to obtain s3 is:
        Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
        Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
        Since s3 can be obtained by interleaving s1 and s2, we return true.
     */
    #[test]
    fn example1() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();

        let b = Solution::is_interleave(s1, s2, s3);

        assert!(b);
    }
    
    /*
        Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
        Output: false
        Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
     */
    #[test]
    fn example2() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();

        let b = Solution::is_interleave(s1, s2, s3);

        assert!(!b);
    }
    
    /*
        Input: s1 = "", s2 = "", s3 = ""
        Output: true
     */
    #[test]
    fn example3() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();

        let b = Solution::is_interleave(s1, s2, s3);

        assert!(b);
    }
}