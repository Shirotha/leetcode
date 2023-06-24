/*
    Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.

    Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

    Constraints:

    1 <= s.length <= 3 * 105
    s consists of digits, '+', '-', '(', ')', and ' '.
    s represents a valid expression.
    '+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
    '-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
    There will be no two consecutive operators in the input.
    Every number and running calculation will fit in a signed 32-bit integer.
 */

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    #[inline] 
    fn add(result: &mut i32, right: &mut String, operator: bool) {
        if let Ok(x) = right.parse::<i32>() {
            right.clear();
            if operator { 
                *result -= x; 
            } else { 
                *result += x; 
            }
        }
    }

    pub fn calculate(s: String) -> i32 {
        let mut builder = String::new();
        let mut inverse = VecDeque::new();
        let mut operator = false;
        let mut result = 0;
        for c in s.chars() {
            match c {
                '0'..='9' => builder.push(c),
                '+' => {
                    let op = operator ^ inverse.back().copied().unwrap_or(false);
                    Self::add(&mut result, &mut builder, op);
                    operator = false;
                },
                '-' => {
                    let op = operator ^ inverse.back().copied().unwrap_or(false);
                    Self::add(&mut result, &mut builder, op);
                    operator = true;
                },
                '(' => {
                    let op = operator ^ inverse.back().copied().unwrap_or(false);
                    inverse.push_back(op);
                    operator = false;
                },
                ')' => {
                    let op = operator ^ inverse.back().copied().unwrap_or(false);
                    Self::add(&mut result, &mut builder, op);
                    inverse.pop_back();
                },
                _ => ()
            }
        }
        Self::add(&mut result, &mut builder, operator);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "1 + 1"
        Output: 2
     */
    #[test]
    fn example1() {
        let s = "1 + 1".to_string();

        let x = Solution::calculate(s);

        assert_eq!(x, 2);
    }

    /*
        Input: s = " 2-1 + 2 "
        Output: 3
     */
    #[test]
    fn example2() {
        let s = " 2-1 + 2 ".to_string();

        let x = Solution::calculate(s);

        assert_eq!(x, 3);
    }

    /*
        Input: s = "(1+(4+5+2)-3)+(6+8)"
        Output: 23
     */
    #[test]
    fn example3() {
        let s = "(1+(4+5+2)-3)+(6+8)".to_string();

        let x = Solution::calculate(s);

        assert_eq!(x, 23);
    }

    /*
        Input: s = "- (3 + (4 + 5))"
        Output: -12
     */
    #[test]
    fn example4() {
        let s = "- (3 + (4 + 5))".to_string();

        let x = Solution::calculate(s);

        assert_eq!(x, -12);
    }
}