/*
    Reverse bits of a given 32 bits unsigned integer.

    Note:

    Note that in some languages, such as Java, there is no unsigned integer type. In this case, both input and output will be given as a signed integer type. They should not affect your implementation, as the integer's internal binary representation is the same, whether it is signed or unsigned.
    In Java, the compiler represents the signed integers using 2's complement notation. Therefore, in Example 2 above, the input represents the signed integer -3 and the output represents the signed integer -1073741825.

    Constraints:

    The input must be a binary string of length 32
 */

pub struct Solution;

impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        const M1: u32 = 0xffff0000;
        const M2: u32 = 0xff00ff00;
        const M3: u32 = 0xf0f0f0f0;
        const M4: u32 = 0xcccccccc;
        const M5: u32 = 0xaaaaaaaa;
        x = (x & M1) >> 16 | (x & !M1) << 16;
        x = (x & M2) >>  8 | (x & !M2) <<  8;
        x = (x & M3) >>  4 | (x & !M3) <<  4;
        x = (x & M4) >>  2 | (x & !M4) <<  2;
            (x & M5) >>  1 | (x & !M5) <<  1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: n = 00000010100101000001111010011100
        Output:    964176192 (00111001011110000010100101000000)
        Explanation: The input binary string 00000010100101000001111010011100 represents the unsigned integer 43261596, so return 964176192 which its binary representation is 00111001011110000010100101000000.
     */
    #[test]
    fn example1() {
        let n = 0b00000010100101000001111010011100;

        let r = Solution::reverse_bits(n);

        assert_eq!(r, 0b00111001011110000010100101000000);
    }

    /*
        Input: n = 11111111111111111111111111111101
        Output:   3221225471 (10111111111111111111111111111111)
        Explanation: The input binary string 11111111111111111111111111111101 represents the unsigned integer 4294967293, so return 3221225471 which its binary representation is 10111111111111111111111111111111.
     */
    #[test]
    fn example2() {
        let n = 0b11111111111111111111111111111101;

        let r = Solution::reverse_bits(n);

        assert_eq!(r, 0b10111111111111111111111111111111);
    }
}