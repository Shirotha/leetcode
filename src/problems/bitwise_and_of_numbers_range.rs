/*
    Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.

    Constraints:

    0 <= left <= right <= 2^31 - 1
 */

pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        left & right & (!0 << (32 - (right - left).leading_zeros()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: left = 5, right = 7
        Output: 4
     */
    #[test]
    fn example1() {
        let left = 5;
        let right = 7;

        let a = Solution::range_bitwise_and(left, right);

        assert_eq!(a, 4);
    }

    /*
        Input: left = 0, right = 0
        Output: 0
     */
    #[test]
    fn example2() {
        let left = 0;
        let right = 0;

        let a = Solution::range_bitwise_and(left, right);

        assert_eq!(a, 0);
    }

    /*
        Input: left = 1, right = 2147483647
        Output: 0
     */
    #[test]
    fn example3() {
        let left = 1;
        let right = 2147483647;

        let a = Solution::range_bitwise_and(left, right);

        assert_eq!(a, 0);
    }
}