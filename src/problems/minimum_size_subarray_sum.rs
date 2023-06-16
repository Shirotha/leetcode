/*
    Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.

    Constraints:

    1 <= target <= 109
    1 <= nums.length <= 105
    1 <= nums[i] <= 104
 */

pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;
        let mut i = 0;
        let mut j = 0;
        let mut s = nums[0];
        let mut best = usize::MAX;
        loop { if s < target {
            if j == n { break; }
            j += 1; s += nums[j];
        } else {
            let d = j - i + 1;
            if d < best { best = d; }
            s -= nums[i]; i += 1;
        } }
        if best <= n + 1 { best as i32 } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: target = 7, nums = [2,3,1,2,4,3]
        Output: 2
        Explanation: The subarray [4,3] has the minimal length under the problem constraint.
     */
    #[test]
    fn example1() {
        let target = 7;
        let nums = vec![2,3,1,2,4,3];

        let s = Solution::min_sub_array_len(target, nums);

        assert_eq!(s, 2);
    }

    /*
        Input: target = 4, nums = [1,4,4]
        Output: 1
     */
    #[test]
    fn example2() {
        let target = 4;
        let nums = vec![1,4,4];

        let s = Solution::min_sub_array_len(target, nums);

        assert_eq!(s, 1);
    }

    /*
        Input: target = 11, nums = [1,1,1,1,1,1,1,1]
        Output: 0
     */
    #[test]
    fn example3() {
        let target = 11;
        let nums = vec![1,1,1,1,1,1,1,1];

        let s = Solution::min_sub_array_len(target, nums);

        assert_eq!(s, 0);
    }

    /*
        Input: target = 15, nums = [1,2,3,4,5]
        Output: 5
     */
    #[test]
    fn example4() {
        let target = 15;
        let nums = vec![1,2,3,4,5];

        let s = Solution::min_sub_array_len(target, nums);

        assert_eq!(s, 5);
    }
}