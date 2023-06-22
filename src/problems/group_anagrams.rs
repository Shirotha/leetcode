/*
    Given an array of strings strs, group the anagrams together. You can return the answer in any order.

    An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

    Constraints:

    1 <= strs.length <= 10^4
    0 <= strs[i].length <= 100
    strs[i] consists of lowercase English letters.
 */

pub struct Solution;

use std::collections::HashMap;
use std::ops::BitOr;

trait Letter {
    fn index(self) -> usize;
}
impl Letter for char {
    #[inline]
    fn index(self) -> usize { self as usize - 0x61 }
}

impl Solution {
    fn hash(s: &str) -> u128 {
        let mut counts = [2 | 1; 26];
        for c in s.chars() {
            counts[c.index()] += 1;
        }
        counts.into_iter().enumerate()
            .map( |(i, n)| n << (i << 2) )
            .fold(0, u128::bitor)
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::new();
        for s in strs {
            let h = Self::hash(&s);
            let v = groups.entry(h).or_insert_with(Vec::new);
            v.push(s);
        }
        groups.into_values().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(g: &mut Vec<Vec<String>>, result: Vec<Vec<&str>>) {
        g.iter_mut().for_each( |s| s.sort_unstable() );
        g.sort_unstable_by( |a, b| a[0].cmp(&b[0]) );

        let result: Vec<Vec<String>> = result.into_iter()
            .map( |g| g.into_iter().map(str::to_string).collect() )
            .collect();

        assert_eq!(g, &result);
    }

    /*
        Input: strs = ["eat","tea","tan","ate","nat","bat"]
        Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
     */
    #[test]
    fn example1() {
        let strs: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"].into_iter()
            .map(str::to_string).collect();

        let mut g = Solution::group_anagrams(strs);

        let result = vec![
            vec!["ate","eat","tea"],
            vec!["bat"],
            vec!["nat","tan"],
        ];

        judge(&mut g, result);
    }

    /*
        Input: strs = [""]
        Output: [[""]]
     */
    #[test]
    fn example2() {
        let strs: Vec<String> = vec![""].into_iter()
            .map(str::to_string).collect();

        let mut g = Solution::group_anagrams(strs);

        let result = vec![
            vec![""]
        ];
        
        judge(&mut g, result);
    }

    /*
        Input: strs = ["a"]
        Output: [["a"]]
     */
    #[test]
    fn example3() {
        let strs: Vec<String> = vec!["a"].into_iter()
            .map(str::to_string).collect();

        let mut g = Solution::group_anagrams(strs);

        let result = vec![
            vec!["a"]
        ];
        
        judge(&mut g, result);
    }
    
}