/*
    Given an integer x, return true if x is a palindrome, and false otherwise.

    Constraints:

    -2^31 <= x <= 2^31 - 1
 */

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        let mut y = 0; let mut z = x;
        while z != 0 { y = 10 * y + z % 10; z /= 10; }
        x == y
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: x = 121
        Output: true
        Explanation: 121 reads as 121 from left to right and from right to left.
     */
    #[test]
    fn example1() {
        let x = 121;

        let b = Solution::is_palindrome(x);

        assert!(b);
    }

    /*
        Input: x = -121
        Output: false
        Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
     */
    #[test]
    fn example2() {
        let x = -121;

        let b = Solution::is_palindrome(x);

        assert!(!b);
    }

    /*
        Input: x = 10
        Output: false
        Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
     */
    #[test]
    fn example3() {
        let x = 10;

        let b = Solution::is_palindrome(x);

        assert!(!b);
    }
}