/*
    Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

    An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.
    Every close bracket has a corresponding open bracket of the same type.

    Constraints:

    1 <= s.length <= 10^4
    s consists of parentheses only '()[]{}'.
 */

pub struct Solution;

use std::collections::VecDeque;

trait Paren {
    fn is_open(&self) -> bool;
    fn closing(&self) -> u8;
}
impl Paren for u8 {
    #[inline] fn closing(&self) -> u8 {
        match self {
            0x28 => 0x29,
            0x5b => 0x5d,
            0x7b => 0x7d,
            _ => panic!("invalid char!")
        }
    }
    #[inline] fn is_open(&self) -> bool {
        matches!(self, 0x28 | 0x5b | 0x7b)
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = VecDeque::new();
        for c in s.as_bytes() {
            if c.is_open() {
                stack.push_back(c.closing());
            } else if let Some(top) = stack.pop_back() {
                if &top != c { return false; }
            } else { return false; }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "()"
        Output: true
     */
    #[test]
    fn example1() {
        let s = "()".to_string();

        let b = Solution::is_valid(s);

        assert!(b);
    }

    /*
        Input: s = "()[]{}"
        Output: true
     */
    #[test]
    fn example2() {
        let s = "()[]{}".to_string();

        let b = Solution::is_valid(s);

        assert!(b);
    }

    /*
        Input: s = "(]"
        Output: false
     */
    #[test]
    fn example3() {
        let s = "(]".to_string();

        let b = Solution::is_valid(s);

        assert!(!b);
    }
}