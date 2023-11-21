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

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (n, m) = (s1.len(), s2.len());
        if n + m != s3.len() { return false; }
        if n == 0 { return s2 == s3; }
        if m == 0 { return s1 == s3; }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let (n, m) = (n + 1, m + 1);
        let mut raw = vec![false; n * m];
        let mut lookup = raw.chunks_mut(m).collect::<Vec<_>>();
        lookup[0][0] = true;
        for i in (1..n).take_while( |&i| s1[i - 1] == s3[i - 1] ) { lookup[i][0] = true; }
        for j in (1..m).take_while( |&j| s2[j - 1] == s3[j - 1] ) { lookup[0][j] = true; }
        let (mut i0, mut j0) = (1, 1);
        while i0 != n && j0 != m {
            if j0 != m {
                for j in j0..m {
                    let c = s3[i0 + j - 1];
                    lookup[i0][j] = lookup[i0 - 1][j] && s1[i0 - 1] == c
                                 || lookup[i0][j - 1] && s2[j  - 1] == c;
                }
                i0 += 1;
            }
            if i0 != n {
                for i in i0..n {
                    let c = s3[i + j0 - 1];
                    lookup[i][j0] = lookup[i - 1][j0] && s1[i  - 1] == c
                                 || lookup[i][j0 - 1] && s2[j0 - 1] == c;
                }
                j0  += 1;
            }
        }
        lookup[n - 1][m - 1]
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

    /*
        Input: s1 = "aa", s2 = "ab", s3 = "abaa"
        Output: true
     */
    #[test]
    fn example4() {
        let s1 = "aa".to_string();
        let s2 = "ab".to_string();
        let s3 = "abaa".to_string();

        let b = Solution::is_interleave(s1, s2, s3);

        assert!(b);
    }
}