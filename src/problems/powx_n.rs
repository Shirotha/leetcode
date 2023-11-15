/*
    Implement pow(x, n), which calculates x raised to the power n (i.e., xn).

    Constraints:

    -100.0 < x < 100.0
    -2^31 <= n <= 2^31-1
    n is an integer.
    Either x is not zero or n > 0.
    -10^4 <= xn <= 10^4
 */

pub struct Solution;

use std::cmp::Ordering::*;

#[inline] fn pow(mut x: f64, mut n: u32) -> f64 {
    let mut p = 1.0;
    loop {
        if n & 1 == 1 { p *= x; }
        n >>= 1;
        if n == 0 { return p; }
        x *= x;
    }
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n.cmp(&0) {
            Greater => pow(x, n as u32),
            Equal => 1.0,
            Less => pow(x.recip(), n.unsigned_abs())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(x: f64, y: f64) {
        const E: f64 = 1e-5;
        assert!((x - y).abs() <= E);
    }

    /*
        Input: x = 2.00000, n = 10
        Output: 1024.00000
     */
    #[test]
    fn example1() {
        let x = 2.0;
        let n = 10;

        let p = Solution::my_pow(x, n);

        judge(p, 1024.0);
    }

    /*
        Input: x = 2.10000, n = 3
        Output: 9.26100
     */
    #[test]
    fn example2() {
        let x = 2.1;
        let n = 3;

        let p = Solution::my_pow(x, n);

        judge(p, 9.261);
    }

    /*
        Input: x = 2.00000, n = -2
        Output: 0.25000
        Explanation: 2-2 = 1/22 = 1/4 = 0.25
     */
    #[test]
    fn example3() {
        let x = 2.0;
        let n = -2;

        let p = Solution::my_pow(x, n);

        judge(p, 0.25);
    }

    /*
        Input: x = 1.00000, n = -2147483648
        Output: 1.0
     */
    #[test]
    fn example4() {
        let x = 1.0;
        let n = -2147483648;

        let p = Solution::my_pow(x, n);

        judge(p, 1.0);
    }
}