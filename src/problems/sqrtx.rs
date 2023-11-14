/*
    Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

    You must not use any built-in exponent function or operator.

    For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.

    Constraints:

    0 <= x <= 2^31 - 1
 */

pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u64;
        let mut l = 0; let mut r = (x >> 1) + 1;
        while l <= r {
            let i = (l + r) >> 1;
            match (i * i).cmp(&x) {
                Less => l = i + 1,
                Equal => { r = i; break; },
                Greater => r = i - 1,
            }
        }
        r as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: x = 4
        Output: 2
        Explanation: The square root of 4 is 2, so we return 2.
     */
    #[test]
    fn example1() {
        let x = 4;

        let r = Solution::my_sqrt(x);

        assert_eq!(r, 2);
    }

    /*
        Input: x = 8
        Output: 2
        Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
     */
    #[test]
    fn example2() {
        let x = 8;

        let r = Solution::my_sqrt(x);

        assert_eq!(r, 2);
    }

    /*
        Input: x = 1
        Output: 1
     */
    #[test]
    fn example3() {
        let x = 1;

        let r = Solution::my_sqrt(x);

        assert_eq!(r, 1);
    }
}