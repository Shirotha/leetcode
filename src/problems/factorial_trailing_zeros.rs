/*
    Given an integer n, return the number of trailing zeroes in n!.

    Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.

    Constraints:

    0 <= n <= 10^4
 */

pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        n/5 + n/25 + n/125 + n/625 + n/3125
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: n = 3
        Output: 0
        Explanation: 3! = 6, no trailing zero.
     */
    #[test]
    fn example1() {
        let n = 3;

        let z = Solution::trailing_zeroes(n);

        assert_eq!(z, 0);
    }

    /*
        Input: n = 5
        Output: 1
        Explanation: 5! = 120, one trailing zero.
     */
    #[test]
    fn example2() {
        let n = 5;

        let z = Solution::trailing_zeroes(n);

        assert_eq!(z, 1);
    }

    /*
        Input: n = 0
        Output: 0
     */
    #[test]
    fn example3() {
        let n = 0;

        let z = Solution::trailing_zeroes(n);

        assert_eq!(z, 0);
    }
}