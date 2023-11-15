/*
    You are climbing a staircase. It takes n steps to reach the top.

    Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

    Constraints:

    1 <= n <= 45
 */

pub struct Solution;

impl Solution {
    pub fn climb_stairs(mut n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        while n != 0 { let tmp = b; b += a; a = tmp; n -= 1; }
        b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: n = 2
        Output: 2
        Explanation: There are two ways to climb to the top.
        1. 1 step + 1 step
        2. 2 steps
     */
    #[test]
    fn example1() {
        let n = 2;

        let c = Solution::climb_stairs(n);

        assert_eq!(c, 2);
    }

    /*
        Input: n = 3
        Output: 3
        Explanation: There are three ways to climb to the top.
        1. 1 step + 1 step + 1 step
        2. 1 step + 2 steps
        3. 2 steps + 1 step
     */
    #[test]
    fn example2() {
        let n = 3;

        let c = Solution::climb_stairs(n);

        assert_eq!(c, 3);
    }
}