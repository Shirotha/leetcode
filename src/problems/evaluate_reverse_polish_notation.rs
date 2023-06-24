#![allow(clippy::upper_case_acronyms)]
/*
    You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

    Evaluate the expression. Return an integer that represents the value of the expression.

    Note that:

    The valid operators are '+', '-', '*', and '/'.
    Each operand may be an integer or another expression.
    The division between two integers always truncates toward zero.
    There will not be any division by zero.
    The input represents a valid arithmetic expression in a reverse polish notation.
    The answer and all the intermediate calculations can be represented in a 32-bit integer.

    Constraints:

    1 <= tokens.length <= 10^4
    tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].
 */

pub struct Solution;

use std::collections::VecDeque;

trait RPN {
    unsafe fn rpn_add(&mut self);
    unsafe fn rpn_sub(&mut self);
    unsafe fn rpn_mul(&mut self);
    unsafe fn rpn_div(&mut self);
}
impl RPN for VecDeque<i32> {
    #[inline] unsafe fn rpn_add(&mut self) {
        let back = self.pop_back().unwrap_unchecked();
        *self.back_mut().unwrap_unchecked() += back;
    }
    #[inline] unsafe fn rpn_sub(&mut self) {
        let back = self.pop_back().unwrap_unchecked();
        *self.back_mut().unwrap_unchecked() -= back;
    }
    #[inline] unsafe fn rpn_mul(&mut self) {
        let back = self.pop_back().unwrap_unchecked();
        *self.back_mut().unwrap_unchecked() *= back;
    }
    #[inline] unsafe fn rpn_div(&mut self) {
        let back = self.pop_back().unwrap_unchecked();
        *self.back_mut().unwrap_unchecked() /= back;
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 { unsafe {
        let mut stack: VecDeque<i32> = VecDeque::new();
        for token in tokens.iter() {
            match token.as_str() {
                "+" => stack.rpn_add(),
                "-" => stack.rpn_sub(),
                "*" => stack.rpn_mul(),
                "/" => stack.rpn_div(),
                _ => stack.push_back(token.parse::<i32>().unwrap_unchecked())
            }
        }
        *stack.back().unwrap_unchecked()
    } }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: tokens = ["2","1","+","3","*"]
        Output: 9
        Explanation: ((2 + 1) * 3) = 9
     */
    #[test]
    fn example1() {
        let tokens = vec!["2","1","+","3","*"]
            .into_iter().map(str::to_string).collect();

        let x = Solution::eval_rpn(tokens);

        assert_eq!(x, 9);
    }

    /*
        Input: tokens = ["4","13","5","/","+"]
        Output: 6
        Explanation: (4 + (13 / 5)) = 6
     */
    #[test]
    fn example2() {
        let tokens = vec!["4","13","5","/","+"]
            .into_iter().map(str::to_string).collect();

        let x = Solution::eval_rpn(tokens);

        assert_eq!(x, 6);
    }

    /*
        Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
        Output: 22
        Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
        = ((10 * (6 / (12 * -11))) + 17) + 5
        = ((10 * (6 / -132)) + 17) + 5
        = ((10 * 0) + 17) + 5
        = (0 + 17) + 5
        = 17 + 5
        = 22
     */
    #[test]
    fn example3() {
        let tokens = vec!["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
            .into_iter().map(str::to_string).collect();

        let x = Solution::eval_rpn(tokens);

        assert_eq!(x, 22);
    }
}