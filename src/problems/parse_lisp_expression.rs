/*
    You are given a string expression representing a Lisp-like expression to return the integer value of.

    The syntax for these expressions is given as follows.

    An expression is either an integer, let expression, add expression, mult expression, or an assigned variable. Expressions always evaluate to a single integer.
    (An integer could be positive or negative.)
    A let expression takes the form "(let v1 e1 v2 e2 ... vn en expr)", where let is always the string "let", then there are one or more pairs of alternating variables and expressions, meaning that the first variable v1 is assigned the value of the expression e1, the second variable v2 is assigned the value of the expression e2, and so on sequentially; and then the value of this let expression is the value of the expression expr.
    An add expression takes the form "(add e1 e2)" where add is always the string "add", there are always two expressions e1, e2 and the result is the addition of the evaluation of e1 and the evaluation of e2.
    A mult expression takes the form "(mult e1 e2)" where mult is always the string "mult", there are always two expressions e1, e2 and the result is the multiplication of the evaluation of e1 and the evaluation of e2.
    For this question, we will use a smaller subset of variable names. A variable starts with a lowercase letter, then zero or more lowercase letters or digits. Additionally, for your convenience, the names "add", "let", and "mult" are protected and will never be used as variable names.
    Finally, there is the concept of scope. When an expression of a variable name is evaluated, within the context of that evaluation, the innermost scope (in terms of parentheses) is checked first for the value of that variable, and then outer scopes are checked sequentially. It is guaranteed that every expression is legal. Please see the examples for more details on the scope.

    Constraints:

    1 <= expression.length <= 2000
    There are no leading or trailing spaces in expression.
    All tokens are separated by a single space in expression.
    The answer and all intermediate calculations of that answer are guaranteed to fit in a 32-bit integer.
    The expression is guaranteed to be legal and evaluate to an integer.
 */

pub struct Solution;

use std::collections::HashMap;
use std::str::from_utf8_unchecked;

struct Stack<'a> {
    depth: u8,
    vars: HashMap<&'a str, Vec<(i32, u8)>>
}
impl<'a> Stack<'a> {
    #[inline] fn new() -> Self { Stack { depth: 0, vars: HashMap::new() } }
    #[inline] fn push(&mut self) { self.depth += 1; }
    #[inline] fn pop(&mut self) {
        for var in self.vars.values_mut() {
            if let Some((_, depth)) = var.last() {
                if *depth == self.depth { var.pop(); }
            }
        }
        self.depth -= 1;
    }
    #[inline] fn set(&mut self, name: &'a str, value: i32) {
        let depth = self.depth;
        let var = self.vars.entry(name).or_insert_with(Vec::new);
        if let Some((x, _)) = var.last_mut()
            .filter( |(_, d)| *d == depth )
        { *x = value; }
        else { var.push((value, depth)); }
    }
    #[inline] fn get(&self, name: &'a str) -> i32 { self.vars.get(name).unwrap().last().unwrap().0 }
}
struct Parser<'a> {
    code: &'a [u8],
    pos: usize,
    stack: Stack<'a>
}
impl<'a> Parser<'a> {
    #[inline] fn parse(code: &'a str) -> i32 { Self::new(code).parse_expression() }
    #[inline] fn new(code: &'a str) -> Self {
        Parser { code: code.as_bytes(), pos: 0, stack: Stack::new() }
    }
    #[inline(always)] fn get(&self) -> u8 { unsafe { *self.code.get_unchecked(self.pos) } }
    #[inline(always)] fn inc(&mut self) { self.pos += 1; }
    #[inline] fn next(&mut self) -> Option<u8> {
        let char = self.get();
        if char == b' ' || char == b')' { return None; }
        self.inc();
        Some(char)
    }
    fn parse_expression(&mut self) -> i32 {
        match self.get() {
            b'(' => {
                self.inc();
                let name = self.parse_identifier();
                self.inc();
                let value = match name {
                    "let" => self.parse_let(),
                    "add" => self.parse_add(),
                    "mult" => self.parse_mult(),
                    _ => panic!()
                };
                self.inc();
                value
            },
            b'a'..=b'z' => self.parse_variable(),
            _ => self.parse_literal()
        }
    }
    #[inline] fn parse_let(&mut self) -> i32 {
        self.stack.push();
        while self.get() != b'(' {
            let pos = self.pos;
            let name = self.parse_identifier();
            if self.get() == b')' { self.pos = pos; break; }
            self.inc();
            let value = self.parse_expression();
            self.stack.set(name, value);
            self.inc();
        }
        let value = self.parse_expression();
        self.stack.pop();
        value
    }
    #[inline] fn parse_add(&mut self) -> i32 {
        let lhs = self.parse_expression();
        self.inc();
        let rhs = self.parse_expression();
        lhs + rhs
    }
    #[inline] fn parse_mult(&mut self) -> i32 {
        let lhs = self.parse_expression();
        self.inc();
        let rhs = self.parse_expression();
        lhs * rhs
    }
    #[inline] fn parse_variable(&mut self) -> i32 {
        let name = self.parse_identifier();
        self.stack.get(name)
    }
    #[inline] fn parse_identifier(&mut self) -> &'a str {
        let start = self.pos;
        while self.next().is_some() {}
        unsafe { from_utf8_unchecked(&self.code[start..self.pos]) }
    }
    #[inline] fn parse_literal(&mut self) -> i32 {
        let mut value = 0;
        let sign = if self.get() == b'-' { self.inc(); -1 } else { 1 };
        while let Some(char) = self.next() {
            let digit = (char - b'0') as i32;
            value = 10 * value + digit;
        }
        sign * value
    }
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        Parser::parse(&expression)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: expression = "(let x 2 (mult x (let x 3 y 4 (add x y))))"
        Output: 14
        Explanation: In the expression (add x y), when checking for the value of the variable x,
        we check from the innermost scope to the outermost in the context of the variable we are trying to evaluate.
        Since x = 3 is found first, the value of x is 3.
     */
    #[test]
    fn example1() {
        let expression = "(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string();

        let r = Solution::evaluate(expression);

        assert_eq!(r, 14);
    }

    /*
        Input: expression = "(let x 3 x 2 x)"
        Output: 2
        Explanation: Assignment in let statements is processed sequentially.
     */
    #[test]
    fn example2() {
        let expression = "(let x 3 x 2 x)".to_string();

        let r = Solution::evaluate(expression);

        assert_eq!(r, 2);
    }

    /*
        Input: expression = "(let x 1 y 2 x (add x y) (add x y))"
        Output: 5
        Explanation: The first (add x y) evaluates as 3, and is assigned to x.
        The second (add x y) evaluates as 3+2 = 5.
     */
    #[test]
    fn example3() {
        let expression = "(let x 1 y 2 x (add x y) (add x y))".to_string();

        let r = Solution::evaluate(expression);

        assert_eq!(r, 5);
    }

    #[test]
    fn example4() {
        let expression = "(let x 2 (add (let x 3 (let x 4 x)) x))".to_string();

        let r = Solution::evaluate(expression);

        assert_eq!(r, 6);
    }

    #[test]
    fn example5() {
        let expression = "(let x 7 -12)".to_string();

        let r = Solution::evaluate(expression);

        assert_eq!(r, -12);
    }
}