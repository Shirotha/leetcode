/*
    Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

    You must write an algorithm with O(log n) runtime complexity.

    Constraints:

    1 <= nums.length <= 10^4
    -10^4 <= nums[i] <= 10^4
    nums contains distinct values sorted in ascending order.
    -10^4 <= target <= 10^4
 */
    
pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i <= j {
            let p = (i + j) >> 1;
            match nums[p].cmp(&target) {
                Less => i = p + 1,
                Equal => return p as i32,
                Greater => if let Some(k) = p.checked_sub(1) { j = k; } else { return 0; },
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [1,3,5,6], target = 5
        Output: 2
    */
    #[test]
    fn example1() {
        let nums = vec![1,3,5,6];
        let target = 5;

        let i = Solution::search_insert(nums, target);

        assert_eq!(i, 2);
    }

    /*
        Input: nums = [1,3,5,6], target = 2
        Output: 1
    */
    #[test]
    fn example2() {
        let nums = vec![1,3,5,6];
        let target = 2;

        let i = Solution::search_insert(nums, target);

        assert_eq!(i, 1);
    }

    /*
        Input: nums = [1,3,5,6], target = 7
        Output: 4
    */
    #[test]
    fn example3() {
        let nums = vec![1,3,5,6];
        let target = 7;

        let i = Solution::search_insert(nums, target);

        assert_eq!(i, 4);
    }

    /*
        Input: nums = [1,3,5,6], target = 0
        Output: 0
    */
    #[test]
    fn example4() {
        let nums = vec![1,3,5,6];
        let target = 0;

        let i = Solution::search_insert(nums, target);

        assert_eq!(i, 0);
    }
}