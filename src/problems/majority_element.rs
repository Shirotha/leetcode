/*
    Given an array nums of size n, return the majority element.

    The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

    Constraints:

    n == nums.length
    1 <= n <= 5 * 10^4
    -10^9 <= nums[i] <= 10^9
 */

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut c = 0;
        for num in nums.iter() {
            if c == 0 { m = *num; }
            if m == *num {
                c += 1;
            } else {
                c -= 1;
            }
        }
        m
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [3,2,3]
        Output: 3
     */
    #[test]
    fn example1() {
        let nums = vec![3,2,3];

        let m = Solution::majority_element(nums);

        assert_eq!(m, 3);
    }

    /*
        Input: nums = [2,2,1,1,1,2,2]
        Output: 2     */
    #[test]
    fn example2() {
        let nums = vec![2,2,1,1,1,2,2];

        let m = Solution::majority_element(nums);

        assert_eq!(m, 2);
    }
}