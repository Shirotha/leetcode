/*
    A peak element is an element that is strictly greater than its neighbors.

    Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.

    You may imagine that nums[-1] = nums[n] = -∞. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.

    You must write an algorithm that runs in O(log n) time.

    Constraints:

    1 <= nums.length <= 1000
    -2^31 <= nums[i] <= 2^31 - 1
    nums[i] != nums[i + 1] for all valid i.
 */

pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = nums.len() - 1;
        if j == 0 { return 0; }
        loop {
            if i + 1 == j { return if nums[i] < nums[j] { j as i32 } else { i as i32 } }
            let p = (i + j) >> 1;
            if nums[p] < nums[p + 1] { i = p; }
            else { j = p; }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(p: i32, peaks: &[i32]) {
        assert!(peaks.contains(&p));
    }

    /*
        Input: nums = [1,2,3,1]
        Output: 2
        Explanation: 3 is a peak element and your function should return the index number 2.
     */
    #[test]
    fn example1() {
        let nums = vec![1,2,3,1];

        let p = Solution::find_peak_element(nums);

        judge(p, &[2]);
    }

    /*
        Input: nums = [1,2,1,3,5,6,4]
        Output: 5
        Explanation: Your function can return either index number 1 where the peak element is 2, or index number 5 where the peak element is 6.
     */
    #[test]
    fn example2() {
        let nums = vec![1,2,1,3,5,6,4];

        let p = Solution::find_peak_element(nums);

        judge(p, &[1,5]);
    }
}