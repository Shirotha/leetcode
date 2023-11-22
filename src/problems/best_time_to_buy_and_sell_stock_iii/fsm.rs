/*
    You are given an array prices where prices[i] is the price of a given stock on the ith day.

    Find the maximum profit you can achieve. You may complete at most two transactions.

    Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

    Constraints:

    1 <= prices.length <= 10^5
    0 <= prices[i] <= 10^5
 */

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut s = [-prices[0], i32::MIN, i32::MIN, i32::MIN];
        for p in prices.into_iter().skip(1) {
            s[0] = s[0].max(-p);
            s[1] = s[1].max(s[0] + p);
            s[2] = s[2].max(s[1] - p);
            s[3] = s[3].max(s[2] + p);
        }
        s[3].max(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: prices = [3,3,5,0,0,3,1,4]
        Output: 6
        Explanation: Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
        Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
     */
    #[test]
    fn example1() {
        let prices = vec![3,3,5,0,0,3,1,4];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 6)
    }

    /*
        Input: prices = [1,2,3,4,5]
        Output: 4
        Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
        Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
     */
    #[test]
    fn example2() {
        let prices = vec![1,2,3,4,5];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 4)
    }

    /*
        Input: prices = [7,6,4,3,1]
        Output: 0
        Explanation: In this case, no transaction is done, i.e. max profit = 0.
     */
    #[test]
    fn example3() {
        let prices = vec![7,6,4,3,1];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 0)
    }
}