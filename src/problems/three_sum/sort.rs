/*
    Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

    Notice that the solution set must not contain duplicate triplets.

    Constraints:

    3 <= nums.length <= 3000
    -10^5 <= nums[i] <= 10^5
 */

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.to_vec();
        nums.sort_unstable();
        let mut result = HashSet::new();
        let mut last = i32::MAX;
        for i in 0..nums.len() {
            let n = nums[i];
            if n == last { continue; }
            for j in (i + 1)..nums.len() {
                let m = nums[j];
                let l = -n - m;
                if nums[(j + 1)..].binary_search(&l).is_ok() {
                    result.insert(vec![n, m, l]);
                }
            }
            last = n;
        }
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [-1,0,1,2,-1,-4]
        Output: [[-1,-1,2],[-1,0,1]]
        Explanation: 
        nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
        nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
        nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
        The distinct triplets are [-1,0,1] and [-1,-1,2].
        Notice that the order of the output and the order of the triplets does not matter.
     */
    #[test]
    fn example1() {
        let nums = vec![-1,0,1,2,-1,-4];

        let ts = Solution::three_sum(nums);

        assert_eq!(ts, vec![vec![-1,-1,2],vec![-1,0,1]]);
    }

    /*
        Input: nums = [0,1,1]
        Output: []
        Explanation: The only possible triplet does not sum up to 0.
     */
    #[test]
    fn example2() {
        let nums = vec![0,1,1];

        let ts = Solution::three_sum(nums);

        assert!(ts.is_empty());
    }

    /*
        Input: nums = [0,0,0,0]
        Output: [[0,0,0]]
        Explanation: The only possible triplet sums up to 0.
     */
    #[test]
    fn example3() {
        let nums = vec![0,0,0];

        let ts = Solution::three_sum(nums);

        assert_eq!(ts, vec![vec![0,0,0]]);
    }
}