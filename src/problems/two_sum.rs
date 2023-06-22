/*
    Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

    You may assume that each input would have exactly one solution, and you may not use the same element twice.

    You can return the answer in any order.

    Constraints:

    2 <= nums.length <= 10^4
    -10^9 <= nums[i] <= 10^9
    -10^9 <= target <= 10^9
    Only one valid answer exists.
 */

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut lookup = HashMap::new();
        for (i, n) in nums.into_iter().enumerate() {
            if let Some(&j) = lookup.get(&n) {
                return vec![j, i as i32];
            } else {
                lookup.insert(target - n, i as i32);
            }
        }
        panic!("no solution!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [2,7,11,15], target = 9
        Output: [0,1]
        Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
     */
    #[test]
    fn example1() {
        let nums = vec![2,7,11,15];
        let target = 9;

        let i = Solution::two_sum(nums, target);

        assert_eq!(i, vec![0,1]);
    }

    /*
        Input: nums = [3,2,4], target = 6
        Output: [1,2]
     */
    #[test]
    fn example2() {
        let nums = vec![3,2,4];
        let target = 6;

        let i = Solution::two_sum(nums, target);

        assert_eq!(i, vec![1,2]);
    }

    /*
        Input: nums = [3,3], target = 6
        Output: [0,1]
     */
    #[test]
    fn example3() {
        let nums = vec![3,3];
        let target = 6;

        let i = Solution::two_sum(nums, target);

        assert_eq!(i, vec![0,1]);
    }
}