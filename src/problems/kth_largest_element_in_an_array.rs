/*
    Given an integer array nums and an integer k, return the kth largest element in the array.

    Note that it is the kth largest element in the sorted order, not the kth distinct element.

    Can you solve it without sorting?

    Constraints:

    1 <= k <= nums.length <= 10^5
    -10^4 <= nums[i] <= 10^4
 */

pub struct Solution;

use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for &n in nums.iter().take(k) { heap.push(Reverse(n)) }
        for &n in nums.iter().skip(k) {
            if n > heap.peek().unwrap().0 { heap.pop(); heap.push(Reverse(n)); }
        }
        heap.pop().unwrap().0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [3,2,1,5,6,4], k = 2
        Output: 5
     */
    #[test]
    fn example1() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;

        let x = Solution::find_kth_largest(nums, k);

        assert_eq!(x, 5);
    }

    /*
        Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
        Output: 4
     */
    #[test]
    fn example2() {
        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;

        let x = Solution::find_kth_largest(nums, k);

        assert_eq!(x, 4);
    }
}