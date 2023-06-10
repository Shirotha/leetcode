/*
    Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

    Constraints:

    1 <= nums.length <= 10^5
    -2^31 <= nums[i] <= 2^31 - 1
    0 <= k <= 10^5
 */

pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        if k == 0 { return; }
        nums.rotate_right(k);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [1,2,3,4,5,6,7], k = 3
        Output: [5,6,7,1,2,3,4]
        Explanation:
        rotate 1 steps to the right: [7,1,2,3,4,5,6]
        rotate 2 steps to the right: [6,7,1,2,3,4,5]
        rotate 3 steps to the right: [5,6,7,1,2,3,4]
     */
    #[test]
    fn example1() {
        let mut nums = vec![1,2,3,4,5,6,7];
        let k = 3;

        Solution::rotate(&mut nums, k);

        assert_eq!(nums, vec![5,6,7,1,2,3,4]);
    }

    /*
        Input: nums = [-1,-100,3,99], k = 2
        Output: [3,99,-1,-100]
        Explanation: 
        rotate 1 steps to the right: [99,-1,-100,3]
        rotate 2 steps to the right: [3,99,-1,-100]
     */
    #[test]
    fn example2() {
        let mut nums = vec![-1,-100,3,99];
        let k = 2;

        Solution::rotate(&mut nums, k);

        assert_eq!(nums, vec![3,99,-1,-100]);
    }
}