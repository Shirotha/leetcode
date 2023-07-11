/*
    Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.

    Constraints:

    1 <= s.length <= 16
    s contains only lowercase English letters.
 */

pub struct Solution;

use std::{alloc::{alloc_zeroed, Layout}, slice};

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        if s.len() == 1 { return vec![vec![s]]; }
        let b = s.as_bytes();
        let n = b.len();
        let (odds, stack, evens) = unsafe {
            let ptr = alloc_zeroed(Layout::array::<u8>(3 * n - 1).unwrap());
            ( slice::from_raw_parts_mut(ptr, n),
              slice::from_raw_parts_mut(ptr.add(n), n),
              slice::from_raw_parts_mut(ptr.add(n << 1), n - 1))
        };
        for (i, v) in odds.iter_mut().enumerate() {
            *v = 1;
            for j in 1..=i.min(n - i - 1) {
                if b[i - j] == b[i + j] { *v += 2; } else { break; }
            }
        }
        for (i, v) in evens.iter_mut().enumerate() {
            for j in 1..=(i + 1).min(n - i - 1) {
                if b[i + 1 - j] == b[i + j] { *v += 2; } else { break; }
            }
        }
        let mut result = Vec::new();
        let mut builder: Vec<String> = Vec::with_capacity(n);
        let b = b.as_ptr() as *mut u8;
        let mut i = 0;
        loop {
            loop {
                let len = stack[i] as usize + 1;
                if len > n - i { break; }
                stack[i] += 1;
                let max = if len & 1 == 1 { odds[i + ((len - 1) >> 1)] } 
                else { evens[i + (len >> 1) - 1] };
                if len as u8 > max { continue; }
                builder.push( unsafe { String::from_raw_parts(b.add(i), len, len) } );
                if i + len == n {
                    result.push(builder.clone());
                    unsafe { builder.set_len(builder.len() - 1) }
                    stack[i] = u8::MAX - 1;
                } else { i += len; }
            }
            stack[i] = 0;
            if let Some(len) = builder.len().checked_sub(1) {
                i -= builder[len].len();
                unsafe { builder.set_len(len); }
            } else { break; }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "aab"
        Output: [["a","a","b"],["aa","b"]]
     */
    #[test]
    fn example1() {
        let s = "aab".to_string();

        let p = Solution::partition(s);

        assert_eq!(p, vec![vec!["a","a","b"],vec!["aa","b"]])
    }

    /*
        Input: s = "a"
        Output: [["a"]]
     */
    #[test]
    fn example2() {
        let s = "a".to_string();

        let p = Solution::partition(s);

        assert_eq!(p, vec![vec!["a"]])
    }

    /*
        Input: s = "ab"
        Output: [["a","b"]]
     */
    #[test]
    fn example3() {
        let s = "ab".to_string();

        let p = Solution::partition(s);

        assert_eq!(p, vec![vec!["a","b"]])
    }

    /*
        Input: s = "abcba"
        Output: [["a","b","c","b","a"],["a","bcb","a"],["abcba"]]
     */
    #[test]
    fn example4() {
        let s = "abcba".to_string();

        let p = Solution::partition(s);

        assert_eq!(p, vec![vec!["a","b","c","b","a"],vec!["a","bcb","a"],vec!["abcba"]])
    }

    /*
        Input: s = "fff"
        Output: [["f","f","f"],["f","ff"],["ff","f"],["fff"]]
     */
    #[test]
    fn example5() {
        let s = "fff".to_string();

        let p = Solution::partition(s);

        assert_eq!(p, vec![vec!["f","f","f"],vec!["f","ff"],vec!["ff","f"],vec!["fff"]])
    }
}