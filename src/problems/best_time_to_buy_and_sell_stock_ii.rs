/*
    You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

    On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

    Find and return the maximum profit you can achieve.

    Constraints:

    1 <= prices.length <= 3 * 104
    0 <= prices[i] <= 104
 */

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut x = 0;
        for j in 1..prices.len() {
            let y = prices[j] - prices[j - 1];
            if y > 0 { x += y; }
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: prices = [7,1,5,3,6,4]
        Output: 7
        Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
        Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
        Total profit is 4 + 3 = 7.
     */
    #[test]
    fn example1() {
        let prices = vec![7,1,5,3,6,4];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 7);
    }

    /*
        Input: prices = [1,2,3,4,5]
        Output: 4
        Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
        Total profit is 4.     
     */
    #[test]
    fn example2() {
        let prices = vec![1,2,3,4,5];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 4);
    }

    /*
        Input: prices = [7,6,4,3,1]
        Output: 0
        Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0. 
     */
    #[test]
    fn example3() {
        let prices = vec![7,6,4,3,1];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 0);
    }
}