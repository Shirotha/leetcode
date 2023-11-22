/*
    You are given an integer array prices where prices[i] is the price of a given stock on the ith day, and an integer k.

    Find the maximum profit you can achieve. You may complete at most k transactions: i.e. you may buy at most k times and sell at most k times.

    Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

    Constraints:

    1 <= k <= 100
    1 <= prices.length <= 1000
    0 <= prices[i] <= 1000
 */

pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut s = vec![i32::MIN; k << 1];
        for p in prices {
            s[0] = s[0].max(-p);
            s[1] = s[1].max(s[0] + p);
            for i in 1..k {
                let i = i << 1;
                s[i] = s[i].max(s[i - 1] - p);
                s[i + 1] = s[i + 1].max(s[i] + p);
            }
        }
        s[(k << 1) - 1].max(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: k = 2, prices = [2,4,1]
        Output: 2
        Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
     */
    #[test]
    fn example1() {
        let k = 2;
        let prices = vec![2,4,1];

        let p = Solution::max_profit(k, prices);

        assert_eq!(p, 2);
    }

    /*
        Input: k = 2, prices = [3,2,6,5,0,3]
        Output: 7
        Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4. Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
     */
    #[test]
    fn example2() {
        let k = 2;
        let prices = vec![3,2,6,5,0,3];

        let p = Solution::max_profit(k, prices);

        assert_eq!(p, 7);
    }
}