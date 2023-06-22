/*
    Write an algorithm to determine if a number n is happy.

    A happy number is a number defined by the following process:

    Starting with any positive integer, replace the number by the sum of the squares of its digits.
    Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
    Those numbers for which this process ends in 1 are happy.
    Return true if n is a happy number, and false if not.

    Constraints:

    1 <= n <= 2^31 - 1
 */

pub struct Solution;

use std::collections::HashSet;

struct Digits {
    n: i32,
    div: i32,
}
impl Iterator for Digits {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.div == 0 { return None; }
        let d = Some(self.n / self.div);
        self.n %= self.div;
        self.div /= 10;
        d
    }
}
trait DigitIterator {
    fn digits(self) -> Digits;
}
impl DigitIterator for i32 {
    fn digits(self) -> Digits {
        let n = self as u64;
        let mut div = 1;
        
        while n >= div * 10 { div *= 10 }
        Digits { n: self, div: div as i32 }
    }
}

impl Solution {
    const CYCLE: [i32; 8] = [4, 16, 37, 58, 89, 145, 42, 20];

    pub fn is_happy(n: i32) -> bool {
        let cycle = HashSet::from(Self::CYCLE);
        let mut n = n;
        while n != 1 {
            if cycle.contains(&n) { return false; }
            n = n.digits().map( |d| d*d ).sum();
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: n = 19
        Output: true
        Explanation:
        1^2 + 9^2 = 82
        8^2 + 2^2 = 68
        6^2 + 8^2 = 100
        1^2 + 0^2 + 0^2 = 1
     */
    #[test]
    fn example1() {
        let n = 19;

        let b = Solution::is_happy(n);

        assert!(b);
    }

    /*
        Input: n = 2
        Output: false
     */
    #[test]
    fn example2() {
        let n = 2;

        let b = Solution::is_happy(n);

        assert!(!b);
    }

    /*
        Input: n = 1999999999
        Output: false
     */
    #[test]
    fn example3() {
        let n = 1999999999;

        let b = Solution::is_happy(n);

        assert!(!b);
    }
}