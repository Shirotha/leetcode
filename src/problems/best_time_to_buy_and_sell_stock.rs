/*
    You are given an array prices where prices[i] is the price of a given stock on the ith day.

    You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

    Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

    Constraints:

    1 <= prices.length <= 105
    0 <= prices[i] <= 104
 */

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut x = 0;
        for j in 1..prices.len() {
            let y = prices[j] - prices[i];
            if y < 0 {
                i = j;
            } else if y > x {
                x = y;
            }
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: prices = [7,1,5,3,6,4]
        Output: 5
        Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
        Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
     */
    #[test]
    fn example1() {
        let prices = vec![7,1,5,3,6,4];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 5);
    }

    /*
        Input: prices = [7,6,4,3,1]
        Output: 0
        Explanation: In this case, no transactions are done and the max profit = 0.     */
    #[test]
    fn example2() {
        let prices = vec![7,6,4,3,1];

        let p = Solution::max_profit(prices);

        assert_eq!(p, 0);
    }
}