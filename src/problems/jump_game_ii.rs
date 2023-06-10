#![allow(clippy::mut_range_bound)]
/*
    You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

    Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:

    0 <= j <= nums[i] and
    i + j < n
    Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

    Constraints:

    1 <= nums.length <= 10^4
    0 <= nums[i] <= 1000
    It's guaranteed that you can reach nums[n - 1].
 */

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return 0; }
        let mut j = 0;
        let mut n = 1;
        let mut k = nums[0] as usize;
        loop {
            if k >= nums.len() - 1 { return n; }
            for i in (j + 1)..=k {
                if i + nums[i] as usize > k { 
                    j = i;
                    k = j + nums[j] as usize;
                }
            }
            n += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [2,3,1,1,4]
        Output: 2
        Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
     */
    #[test]
    fn example1() {
        let nums = vec![2,3,1,1,4];

        let j = Solution::jump(nums);

        assert_eq!(j, 2);
    }

    /*
        Input: nums = [2,3,0,1,4]
        Output: 2
     */
    #[test]
    fn example2() {
        let nums = vec![2,3,0,1,4];

        let j = Solution::jump(nums);

        assert_eq!(j, 2);
    }
}