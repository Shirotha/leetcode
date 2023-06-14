/*
    Write a function to find the longest common prefix string amongst an array of strings.

    If there is no common prefix, return an empty string "".

    Constraints:

    1 <= strs.length <= 200
    0 <= strs[i].length <= 200
    strs[i] consists of only lowercase English letters.
 */

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            let n = prefix.chars().zip(s.chars()).take_while(|&(a, b)| a == b ).count();
            prefix.truncate(n);
            if n == 0 { return prefix; }
        }
        prefix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: strs = ["flower","flow","flight"]
        Output: "fl"
     */
    #[test]
    fn example1() {
        let strs = vec!["flower","flow","flight"].iter().map(|s| s.to_string() ).collect();

        let prefix = Solution::longest_common_prefix(strs);

        assert_eq!(prefix.as_str(), "fl");
    }

    /*
        Input: strs = ["dog","racecar","car"]
        Output: ""
        Explanation: There is no common prefix among the input strings.
     */
    #[test]
    fn example2() {
        let strs = vec!["dog","racecar","car"].iter().map(|s| s.to_string() ).collect();

        let prefix = Solution::longest_common_prefix(strs);

        assert_eq!(prefix.as_str(), "");
    }
}