/*
    You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.

    Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

    You may assume that you have an infinite number of each kind of coin.

    Constraints:

    1 <= coins.length <= 12
    1 <= coins[i] <= 2^31 - 1
    0 <= amount <= 10^4
 */

pub struct Solution;

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        for i in (0..coins.len()).rev() { if coins[i] > amount { coins.swap_remove(i); } }
        let mut store = vec![0; amount as usize + 1];
        for &coin in coins.iter() { store[coin as usize] = 1; }
        for n in 1..=amount { let i = n as usize;
            if store[i] != 0 { continue; }
            store[i] = 1 + coins.iter()
                .filter_map( |&coin| {
                store.get((n - coin) as usize).copied()
                    .filter( |&x| x != -1 )
            } ).min().unwrap_or(-2);
        }
        *store.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: coins = [1,2,5], amount = 11
        Output: 3
        Explanation: 11 = 5 + 5 + 1
     */
    #[test]
    fn example1() {
        let coins = vec![1,2,5];
        let amount = 11;

        let n = Solution::coin_change(coins, amount);

        assert_eq!(n, 3);
    }

    /*
        Input: coins = [2], amount = 3
        Output: -1
     */
    #[test]
    fn example2() {
        let coins = vec![2];
        let amount = 3;

        let n = Solution::coin_change(coins, amount);

        assert_eq!(n, -1);
    }

    /*
        Input: coins = [1], amount = 0
        Output: 0
     */
    #[test]
    fn example3() {
        let coins = vec![1];
        let amount = 0;

        let n = Solution::coin_change(coins, amount);

        assert_eq!(n, 0);
    }

    /*
        Input: coins = [186,419,83,408], amount = 6249
        Output: 20
     */
    #[test]
    fn example4() {
        let coins = vec![186,419,83,408];
        let amount = 6249;

        let n = Solution::coin_change(coins, amount);

        assert_eq!(n, 20);
    }
}