#![allow(non_snake_case)]
/*
    Write a function that takes the binary representation of an unsigned integer and returns the number of '1' bits it has (also known as the Hamming weight).

    Note:

    Note that in some languages, such as Java, there is no unsigned integer type. In this case, the input will be given as a signed integer type. It should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
    In Java, the compiler represents the signed integers using 2's complement notation. Therefore, in Example 3, the input represents the signed integer. -3.

    Constraints:

    The input must be a binary string of length 32.
 */

pub struct Solution;

impl Solution {
    pub fn hammingWeight (mut n: u32) -> i32 {
        const M1: u32 = 0x55555555;
        const M2: u32 = 0x33333333;
        const M3: u32 = 0x0f0f0f0f;
        const M4: u32 = 0x00ff00ff;
        const M5: u32 = 0x0000ffff;
        n -= n >> 1 & M1;
        n = (n >> 2 & M2) + (n & M2);
        n = ((n >>  4) + n) & M3;
        n = ((n >>  8) + n) & M4;
           (((n >> 16) + n) & M5) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: n = 00000000000000000000000000001011
        Output: 3
        Explanation: The input binary string 00000000000000000000000000001011 has a total of three '1' bits.
     */
    #[test]
    fn example1() {
        let n = 0b00000000000000000000000000001011;

        let w = Solution::hammingWeight(n);

        assert_eq!(w, 3);
    }

    /*
        Input: n = 00000000000000000000000010000000
        Output: 1
        Explanation: The input binary string 00000000000000000000000010000000 has a total of one '1' bit.
     */
    #[test]
    fn example2() {
        let n = 0b00000000000000000000000010000000;

        let w = Solution::hammingWeight(n);

        assert_eq!(w, 1);
    }

    /*
        Input: n = 11111111111111111111111111111101
        Output: 31
        Explanation: The input binary string 11111111111111111111111111111101 has a total of thirty one '1' bits.
     */
    #[test]
    fn example3() {
        let n = 0b11111111111111111111111111111101;

        let w = Solution::hammingWeight(n);

        assert_eq!(w, 31);
    }
}