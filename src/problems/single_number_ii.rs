/*
    Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.

    You must implement a solution with a linear runtime complexity and use only constant extra space.

    Constraints:

    1 <= nums.length <= 3 * 10^4
    -2^31 <= nums[i] <= 2^31 - 1
    Each element in nums appears exactly three times except for one element which appears once.
 */

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut one = 0; let mut two = 0;
        for n in nums {
            two = (two ^ one & n) & !(two & n);
            one ^= n & !two;
        }
        one
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [2,2,3,2]
        Output: 3
     */
    #[test]
    fn example1() {
        let nums = vec![2,2,3,2];

        let n = Solution::single_number(nums);

        assert_eq!(n, 3);
    }

    /*
        Input: nums = [0,1,0,1,0,1,99]
        Output: 99
     */
    #[test]
    fn example2() {
        let nums = vec![0,1,0,1,0,1,99];

        let n = Solution::single_number(nums);

        assert_eq!(n, 99);
    }
}