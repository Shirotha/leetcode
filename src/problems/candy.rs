/*
    There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.

    You are giving candies to these children subjected to the following requirements:

    Each child must have at least one candy.
    Children with a higher rating get more candies than their neighbors.
    Return the minimum number of candies you need to have to distribute the candies to the children.

    Constraints:

    n == ratings.length
    1 <= n <= 2 * 10^4
    0 <= ratings[i] <= 2 * 10^4
 */

pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut n = 1;
        let mut d = 0;
        let mut last = 1;
        for i in 1..ratings.len() {
            match ratings[i].cmp(&ratings[i - 1]) {
                Less => {
                    if d < 0 { d -= 1; } else { d = -1; }
                    last -= 1;
                    n -= d;
                    if last <= 0 { n += 1; }
                },
                Equal => {
                    d = 0; last = 1;
                    n += 1;
                }
                Greater => {
                    if d > 0 { d += 1; } else { d = 1; last = 1; }
                    last += 1;
                    n += last;
                }
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: ratings = [1,0,2]
        Output: 5
        Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
     */
    #[test]
    fn example1() {
        let ratings = vec![1,0,2];

        let n = Solution::candy(ratings);

        assert_eq!(n, 5);
    }

    /*
        Input: ratings = [1,2,2]
        Output: 4
        Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
        The third child gets 1 candy because it satisfies the above two conditions.
     */
    #[test]
    fn example2() {//                1 2 1 1 5 4 3 2 1
        let ratings = vec![1,5,5,5,5,4,3,2,1];

        let n = Solution::candy(ratings);

        assert_eq!(n, 20);
    }

    /*
        Input: ratings = [1,6,10,8,7,3,2]
        Output: 18
     */
    #[test]
    fn example3() {
        let ratings = vec![1,6,10,8,7,3,2];

        let n = Solution::candy(ratings);

        assert_eq!(n, 18);
    }

    /*
        Input: ratings = [3,2,1,1,4,3,3]
        Output: 11
     */
    #[test]
    fn example4() {//                3 2 1 1 2 1 1
        let ratings = vec![3,2,1,1,4,3,3];

        let n = Solution::candy(ratings);

        assert_eq!(n, 11);
    }
}