/*
Given a circular integer array nums of length n, return the maximum possible sum of a non-empty subarray of nums.

A circular array means the end of the array connects to the beginning of the array. Formally, the next element of nums[i] is nums[(i + 1) % n] and the previous element of nums[i] is nums[(i - 1 + n) % n].

A subarray may only include each element of the fixed buffer nums at most once. Formally, for a subarray nums[i], nums[i + 1], ..., nums[j], there does not exist i <= k_1, k_2 <= j with k_1 % n == k_2 % n.

Constraints:

n == nums.length
1 <= n <= 3 * 10^4
-3 * 10^4 <= nums[i] <= 3 * 10^4
 */

pub struct Solution;

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut cmin = 0;
        let mut cmax = 0;
        for n in nums {
            sum += n;
            cmin = n.min(cmin + n);
            if cmin < min { min = cmin; }
            cmax = n.max(cmax + n);
            if cmax > max { max = cmax; }
        }
        if sum == min { max } else { max.max(sum - min) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [1,-2,3,-2]
        Output: 3
        Explanation: Subarray [3] has maximum sum 3.
     */
    #[test]
    fn example1() {
        let nums = vec![1,-2,3,-2];

        let s = Solution::max_subarray_sum_circular(nums);

        assert_eq!(s, 3);
    }

    /*
        Input: nums = [5,-3,5]
        Output: 10
        Explanation: Subarray [5,5] has maximum sum 5 + 5 = 10.
     */
    #[test]
    fn example2() {
        let nums = vec![5,-3,5];

        let s = Solution::max_subarray_sum_circular(nums);

        assert_eq!(s, 10);
    }

    /*
        Input: nums = [-3,-2,-3]
        Output: -2
        Explanation: Subarray [-2] has maximum sum -2.
     */
    #[test]
    fn example3() {
        let nums = vec![-3,-2,-3];

        let s = Solution::max_subarray_sum_circular(nums);

        assert_eq!(s, -2);
    }
}