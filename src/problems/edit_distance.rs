#![allow(clippy::needless_range_loop)]
/*
    Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

    You have the following three operations permitted on a word:

    Insert a character
    Delete a character
    Replace a character

    Constraints:

    0 <= word1.length, word2.length <= 500
    word1 and word2 consist of lowercase English letters.
 */

pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len() + 1, word2.len() + 1);
        let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
        let mut raw = vec![0; n << 1];
        let (prev, curr) = raw.split_at_mut(n);
        for j in 1..n { curr[j] = j; }
        prev.copy_from_slice(curr);
        for i in 1..m {
            for j in 0..n {
                curr[j] = if j == 0 { prev[0] + 1 }
                else if word1[i - 1] == word2[j - 1] { prev[j - 1] }
                else { curr[j - 1].min(prev[j]).min(prev[j - 1]) + 1 }
            }
            prev.copy_from_slice(curr);
        }
        curr[n - 1] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: word1 = "horse", word2 = "ros"
        Output: 3
        Explanation: 
        horse -> rorse (replace 'h' with 'r')
        rorse -> rose (remove 'r')
        rose -> ros (remove 'e')
     */
    #[test]
    fn example1() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();

        let d = Solution::min_distance(word1, word2);

        assert_eq!(d, 3);
    }

    /*
        Input: word1 = "intention", word2 = "execution"
        Output: 5
        Explanation: 
        intention -> inention (remove 't')
        inention -> enention (replace 'i' with 'e')
        enention -> exention (replace 'n' with 'x')
        exention -> exection (replace 'n' with 'c')
        exection -> execution (insert 'u')
     */
    #[test]
    fn example2() {
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();

        let d = Solution::min_distance(word1, word2);

        assert_eq!(d, 5);
    }
}