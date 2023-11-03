/*
    Given an integer array nums, find the subarray with the largest sum, and return its sum.

    Constraints:

    1 <= nums.length <= 10^5
    -10^4 <= nums[i] <= 10^4
 */

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut best = i32::MIN;
        let mut left = 0;
        let mut cost;
        let mut right;
        let mut i = 0;
        while i < l {
            cost = 0;
            while i < l && nums[i] <= 0 {
                if nums[i] > best { best = nums[i]; }
                cost += nums[i]; i += 1;
            }
            if i == l { return best; }
            right = 0;
            while i < l && nums[i] >= 0 { right += nums[i]; i += 1; }
            left = right.max(left + cost + right);
            if left > best { best = left; }
        }
        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
        Output: 6
        Explanation: The subarray [4,-1,2,1] has the largest sum 6.
     */
    #[test]
    fn example1() {
        let nums = vec![-2,1,-3,4,-1,2,1,-5,4];

        let s = Solution::max_sub_array(nums);

        assert_eq!(s, 6);
    }

    /*
        Input: nums = [1]
        Output: 1
        Explanation: The subarray [1] has the largest sum 1.
     */
    #[test]
    fn example2() {
        let nums = vec![1];

        let s = Solution::max_sub_array(nums);

        assert_eq!(s, 1);
    }

    /*
        Input: nums = [5,4,-1,7,8]
        Output: 23
        Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
     */
    #[test]
    fn example3() {
        let nums = vec![5,4,-1,7,8];

        let s = Solution::max_sub_array(nums);

        assert_eq!(s, 23);
    }
}