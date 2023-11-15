/*
    You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

    Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

    Constraints:

    1 <= nums.length <= 100
    0 <= nums[i] <= 400
 */

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, nums[0]);
        for n in nums.into_iter().skip(1) {
            let tmp = b; b = b.max(a + n); a = tmp;
        }
        b
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [1,2,3,1]
        Output: 4
        Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
        Total amount you can rob = 1 + 3 = 4.
     */
    #[test]
    fn example1() {
        let nums = vec![1,2,3,1];

        let m = Solution::rob(nums);

        assert_eq!(m, 4);
    }

    /*
        Input: nums = [2,7,9,3,1]
        Output: 12
        Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
        Total amount you can rob = 2 + 9 + 1 = 12.
     */
    #[test]
    fn example2() {
        let nums = vec![2,7,9,3,1];

        let m = Solution::rob(nums);

        assert_eq!(m, 12);
    }

    /*
        Input: nums = [1,3,1,3,100]
        Output: 103
     */
    #[test]
    fn example3() {
        let nums = vec![1,3,1,3,100];

        let m = Solution::rob(nums);

        assert_eq!(m, 103);
    }

    /*
        Input: nums = [4,1,2,7,5,3,1]
        Output: 14
     */
    #[test]
    fn example4() {
        let nums = vec![4,1,2,7,5,3,1];

        let m = Solution::rob(nums);

        assert_eq!(m, 14);
    }
}