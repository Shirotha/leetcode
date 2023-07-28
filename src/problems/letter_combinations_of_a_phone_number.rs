/*
    Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

    A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

    Constraints:

    0 <= digits.length <= 4
    digits[i] is a digit in the range ['2', '9'].
 */

pub struct Solution;

use std::collections::VecDeque;

enum PeekAction<T> {
    Push(T),
    Next,
    Pop,
}
use PeekAction::*;
struct PeekResult<T, R>(PeekAction<T>, Option<R>);
impl<T, R> PeekResult<T, R> {
    fn found(action: PeekAction<T>, result: R) -> Self {
        PeekResult(action, Some(result))
    }
    fn skip(action: PeekAction<T>) -> Self {
        PeekResult(action, None)
    }
}
struct PathIter<T> {
    stack: VecDeque<T>,
}
impl<T> PathIter<T> {
    fn new(root: T) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(root);
        PathIter { stack }
    }
    fn forward<P, R>(&mut self, peek: P) -> Result<Option<R>,()> where 
        P: FnOnce(&mut T) -> PeekResult<T, R>
    {
        if let Some(state) = self.stack.back_mut() {
            let PeekResult(action, result) = peek(state);
            match action {
                Push(next) => self.stack.push_back(next),
                Next => (),
                Pop => { self.stack.pop_back(); },
            }
            Ok(result)
        } else { Err(()) }
    }
}

fn char_to_index(chr: &u8) -> usize { *chr as usize - 0x32 }

const BUTTONS: &[&[char]] = &[
    &['a','b','c'],
    &['d','e','f'],
    &['g','h','i'],
    &['j','k','l'],
    &['m','n','o'],
    &['p','q','r','s'],
    &['t','u','v'],
    &['w','x','y','z'],
];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits = digits.as_bytes();
        let n = digits.len();
        let mut result = Vec::new();
        if n == 0 { return result; }
        let mut builder = String::with_capacity(n);
        let mut iter = PathIter::new((digits[0], 0u8));
        while let Ok(path) = iter.forward( |(button, option)| {
            let options = BUTTONS[char_to_index(button)];
            let i = *option as usize;
            if i == options.len() {
                builder.pop();
                return PeekResult::skip(Pop);
            }
            *option += 1;
            builder.push(options[i]);
            let j = builder.len();
            if j == n {
                PeekResult::found(Next, ())
            } else {
                PeekResult::skip(Push((digits[j], 0)))
            }
        } ) {
            if path.is_some() {
                result.push(builder.clone());
                builder.pop();
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(ws: Vec<String>, result: &[&str]) {
        assert_eq!(ws.len(), result.len());
        for w in ws {
            assert!(result.contains(&w.as_str()));
        }
    }

    /*
        Input: digits = "23"
        Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
     */
    #[test]
    fn example1() {
        let digits = "23".to_string();

        let ws = Solution::letter_combinations(digits);

        judge(ws, &["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }

    /*
        Input: digits = ""
        Output: []
     */
    #[test]
    fn example2() {
        let digits = "".to_string();

        let ws = Solution::letter_combinations(digits);

        judge(ws, &[]);
    }

    /*
        Input: digits = "2"
        Output: ["a","b","c"]
     */
    #[test]
    fn example3() {
        let digits = "2".to_string();

        let ws = Solution::letter_combinations(digits);

        judge(ws, &["a","b","c"]);
    }
}