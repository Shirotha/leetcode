/*
Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

You must write an algorithm that runs in O(n) time.

Constraints:

0 <= nums.length <= 10^5
-10^9 <= nums[i] <= 10^9
 */

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut length = HashMap::new();
        let mut best = 0;
        for n in nums {
            if length.contains_key(&n) { continue; }
            let left = *length.get(&(n - 1)).unwrap_or(&0);
            let right = *length.get(&(n + 1)).unwrap_or(&0);
            let len = left + 1 + right;
            length.insert(n, len);
            length.insert(n - left, len);
            length.insert(n + right, len);
            if len > best { best = len; }
        }
        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [100,4,200,1,3,2]
        Output: 4
        Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
    */
    #[test]
    fn example1() {
        let nums = vec![100,4,200,1,3,2];

        let l = Solution::longest_consecutive(nums);

        assert_eq!(l, 4);
    }

    /*
        Input: nums = [0,3,7,2,5,8,4,6,0,1]
        Output: 9
    */
    #[test]
    fn example2() {
        let nums = vec![0,3,7,2,5,8,4,6,0,1];

        let l = Solution::longest_consecutive(nums);

        assert_eq!(l, 9);
    }
}