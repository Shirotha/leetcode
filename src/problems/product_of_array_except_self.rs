/*
    Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

    The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

    You must write an algorithm that runs in O(n) time and without using the division operation.

    Constraints:

    2 <= nums.length <= 10^5
    -30 <= nums[i] <= 30
    The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
 */

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];
        for i in 1..n {
            result[i] = result[i - 1] * nums[i - 1];
        }
        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [1,2,3,4]
        Output: [24,12,8,6]
     */
    #[test]
    fn example1() {
        let nums = vec![1,2,3,4];

        let result = Solution::product_except_self(nums);

        assert_eq!(result, vec![24,12,8,6]);
    }

    /*
        Input: nums = [-1,1,0,-3,3]
        Output: [0,0,9,0,0]
     */
    #[test]
    fn example2() {
        let nums = vec![-1,1,0,-3,3];

        let result = Solution::product_except_self(nums);

        assert_eq!(result, vec![0,0,9,0,0]);
    }
}