/*
    Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.

    If target is not found in the array, return [-1, -1].

    You must write an algorithm with O(log n) runtime complexity.

    Constraints:

    0 <= nums.length <= 10^5
    -10^9 <= nums[i] <= 10^9
    nums is a non-decreasing array.
    -10^9 <= target <= 10^9
 */

pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1,-1];
        if nums.is_empty() { return result; }
        let n = nums.len() - 1;
        let mut i = 0;
        let mut j = n;
        while i <= j {
            let p = (i + j) >> 1;
            match nums[p].cmp(&target) {
                Less => if p == n { return result; } else { i = p + 1 },
                Equal => if p != n && nums[p + 1] == target { i = p + 1 } else { j = p; break; },
                Greater => if p == 0 { return result; } else { j = p - 1 },
            }
        }
        if j < i { return result; }
        result[1] = j as i32;
        i = 0;
        loop {
            let p = (i + j) >> 1;
            match nums[p].cmp(&target) {
                Less => i = p + 1,
                Equal => if p != 0 && nums[p - 1] == target { j = p - 1 } else { i = p; break; }
                Greater => j = p - 1,
            }
        }
        result[0] = i as i32;
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [5,7,7,8,8,10], target = 8
        Output: [3,4]
     */
    #[test]
    fn example1() {
        let nums = vec![5,7,7,8,8,10];
        let target = 8;

        let ij = Solution::search_range(nums, target);

        assert_eq!(ij, vec![3,4]);
    }

    /*
        Input: nums = [5,7,7,8,8,10], target = 6
        Output: [-1,-1]
     */
    #[test]
    fn example2() {
        let nums = vec![5,7,7,8,8,10];
        let target = 6;

        let ij = Solution::search_range(nums, target);

        assert_eq!(ij, vec![-1,-1]);
    }

    /*
        Input: nums = [], target = 0
        Output: [-1,-1]
     */
    #[test]
    fn example3() {
        let nums = vec![];
        let target = 0;

        let ij = Solution::search_range(nums, target);

        assert_eq!(ij, vec![-1,-1]);
    }

    /*
        Input: nums = [1], target = 1
        Output: [0,0]
     */
    #[test]
    fn example4() {
        let nums = vec![1];
        let target = 1;

        let ij = Solution::search_range(nums, target);

        assert_eq!(ij, vec![0,0]);
    }

    /*
        Input: nums = [2,2], target = 2
        Output: [0,1]
     */
    #[test]
    fn example5() {
        let nums = vec![2,2];
        let target = 2;

        let ij = Solution::search_range(nums, target);

        assert_eq!(ij, vec![0,1]);
    }

    /*
        Input: nums = [0,0,1,1,1,2,4,4,4,4,5,5,5,6,8,8,9,9,10,10,10], target = 8
        Output: [14,15]
     */
    #[test]
    fn example6() {
        let nums = vec![2,2];
        let target = 2;

        let ij = Solution::search_range(nums, target);

        assert_eq!(ij, vec![0,1]);
    }
}