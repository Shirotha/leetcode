/*
    Given two binary strings a and b, return their sum as a binary string.

    Constraints:

    1 <= a.length, b.length <= 10^4
    a and b consist only of '0' or '1' characters.
    Each string does not contain leading zeros except for the zero itself.
 */

pub struct Solution;

use std::iter::repeat;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        const ZERO: u8 = b'0';
        let (mut n, a, b) = if a.len() >= b.len() {
            (a.len(), a, b)
        } else { (b.len(), b, a) }; 
        let mut digits = String::with_capacity(n + 1);
        let mut carry = 0;
        for (&a, &b) in a.as_bytes().iter().rev()
            .zip(b.as_bytes().iter().rev().chain(repeat(&ZERO))) 
        {
            let sum = (a - ZERO) + (b - ZERO) + carry;
            digits.push((ZERO + (sum & 1)) as char);
            carry = (sum & 2) >> 1; n -= 1;
        }
        if carry != 0 { digits.push((ZERO + 1) as char); }
        unsafe {
            digits.as_mut_vec().reverse();
        }
        digits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: a = "11", b = "1"
        Output: "100"
     */
    #[test]
    fn example1() {
        let a = "11".to_string();
        let b =  "1".to_string();

        let s = Solution::add_binary(a, b);

        assert_eq!(&s, &"100");
    }

    /*
        Input: a = "1010", b = "1011"
        Output: "10101"
     */
    #[test]
    fn example2() {
        let a = "1010".to_string();
        let b = "1011".to_string();

        let s = Solution::add_binary(a, b);

        assert_eq!(&s, &"10101");
    }

    /*
        Input: a = "100", b = "110010"
        Output: "110110"
     */
    #[test]
    fn example3() {
        let a =    "100".to_string();
        let b = "110010".to_string();

        let s = Solution::add_binary(a, b);

        assert_eq!(&s, &"110110");
    }
}
