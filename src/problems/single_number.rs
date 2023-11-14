/*
    Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

    You must implement a solution with a linear runtime complexity and use only constant extra space.

    Constraints:

    1 <= nums.length <= 3 * 10^4
    -3 * 10^4 <= nums[i] <= 3 * 10^4
    Each element in the array appears twice except for one element which appears only once.
 */

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |x, n| x ^ n )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [2,2,1]
        Output: 1
     */
    #[test]
    fn example1() {
        let nums = vec![2,2,1];

        let n = Solution::single_number(nums);

        assert_eq!(n, 1);
    }

    /*
        Input: nums = [4,1,2,1,2]
        Output: 4
     */
    #[test]
    fn example2() {
        let nums = vec![4,1,2,1,2];

        let n = Solution::single_number(nums);

        assert_eq!(n, 4);
    }

    /*
        Input: nums = [1]
        Output: 1
     */
    #[test]
    fn example3() {
        let nums = vec![1];

        let n = Solution::single_number(nums);

        assert_eq!(n, 1);
    }
}