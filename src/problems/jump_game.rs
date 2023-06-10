/*
    You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

    Return true if you can reach the last index, or false otherwise.

    Constraints:

    1 <= nums.length <= 10^4
    0 <= nums[i] <= 10^5
 */

pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut j = nums.len() - 1;
        for i in (0..j).rev() {
            if i + nums[i] as usize >= j { j = i; }
        }
        j == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [2,3,1,1,4]
        Output: true
        Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
     */
    #[test]
    fn example1() {
        let nums = vec![2,3,1,1,4];

        let j = Solution::can_jump(nums);

        assert!(j);
    }

    /*
        Input: nums = [3,2,1,0,4]
        Output: false
        Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
     */
    #[test]
    fn example2() {
        let nums = vec![3,2,1,0,4];

        let j = Solution::can_jump(nums);

        assert!(!j);
    }
}