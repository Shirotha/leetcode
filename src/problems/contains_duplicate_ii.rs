/*
    Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

    Constraints:

    1 <= nums.length <= 10^5
    -10^9 <= nums[i] <= 10^9
    0 <= k <= 10^5
 */

pub struct Solution;

use std::collections::HashSet;
use std::iter::repeat;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 { return false; }
        let k = k as usize;
        let mut lookup = HashSet::with_capacity(k);
        for (n, m) in nums.iter()
            .zip(repeat(None).take(k).chain(nums.iter().map(Some))) 
        {
            if lookup.contains(n) { return true; }
            if let Some(l) = m { lookup.remove(l); }
            lookup.insert(n);
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [1,2,3,1], k = 3
        Output: true
     */
    #[test]
    fn example1() {
        let nums = vec![1,2,3,1];
        let k = 3;

        let b = Solution::contains_nearby_duplicate(nums, k);

        assert!(b);
    }

    /*
        Input: nums = [1,0,1,1], k = 1
        Output: true
     */
    #[test]
    fn example2() {
        let nums = vec![1,0,1,1];
        let k = 1;

        let b = Solution::contains_nearby_duplicate(nums, k);

        assert!(b);
    }

    /*
        Input: nums = [1,2,3,1,2,3], k = 2
        Output: false
     */
    #[test]
    fn example3() {
        let nums = vec![1,2,3,1,2,3];
        let k = 2;

        let b = Solution::contains_nearby_duplicate(nums, k);

        assert!(!b);
    }

    /*
        Input: nums = [1,2,1], k = 0
        Output: false
     */
    #[test]
    fn example4() {
        let nums = vec![1,2,1];
        let k = 0;

        let b = Solution::contains_nearby_duplicate(nums, k);

        assert!(!b);
    }
}