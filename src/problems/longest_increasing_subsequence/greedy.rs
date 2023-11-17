/*
    Given an integer array nums, return the length of the longest strictly increasing subsequence.

    Constraints:

    1 <= nums.length <= 2500
    -10^4 <= nums[i] <= 10^4
 */

pub struct Solution;

#[inline] fn find_index(nums: &[i32], num: &i32) -> usize {
    let l = nums.len();
    if l <= 2 { return if &nums[0] >= num { 0 } else { 1 }; }
    let (mut i, mut j) = (0, l - 2);
    while i <= j {
        let p = (i + j) >> 1;
        let (a, b) = (&nums[p], &nums[p + 1]);
        if b <= num { i = p + 1; }
        else if a > num { if p == 0 { return 0; } else { j = p - 1; } }
        else if a == num { return p; }
        else { return p + 1; }
    }
    j
}

impl Solution {
    pub fn length_of_lis(mut nums: Vec<i32>) -> i32 {
        let mut tail = 0;
        for i in 1..nums.len() {
            let (num, last) = (nums[i], nums[tail]);
            if num <= last {
                let i = find_index(&nums[0..=tail], &num);
                nums[i] = num;
            } else { tail += 1; nums[tail] = num; }
        }
        tail as i32 + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [10,9,2,5,3,7,101,18]
        Output: 4
        Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
     */
    #[test]
    fn example1() {
        let nums = vec![10,9,2,5,3,7,101,18];

        let l = Solution::length_of_lis(nums);

        assert_eq!(l, 4);
    }

    /*
        Input: nums = [0,1,0,3,2,3]
        Output: 4
     */
    #[test]
    fn example2() {
        let nums = vec![0,1,0,3,2,3];

        let l = Solution::length_of_lis(nums);

        assert_eq!(l, 4);
    }

    /*
        Input: nums = [7,7,7,7,7,7,7]
        Output: 1
     */
    #[test]
    fn example3() {
        let nums = vec![7,7,7,7,7,7,7];

        let l = Solution::length_of_lis(nums);

        assert_eq!(l, 1);
    }

    /*
        Input: nums = [3,5,6,2,5,4,19,5,6,7,12]
        Output: 6
     */
    #[test]
    fn example4() {
        let nums = vec![3,5,6,2,5,4,19,5,6,7,12];

        let l = Solution::length_of_lis(nums);

        assert_eq!(l, 6);
    }

    /*
        Input: nums = [10,9,2,5,3,4]
        Output: 3
     */
    #[test]
    fn example5() {
        let nums = vec![10,9,2,5,3,4];

        let l = Solution::length_of_lis(nums);

        assert_eq!(l, 3);
    }

    /*
        Input: nums = [4,10,4,3,8,9]
        Output: 3
     */
    #[test]
    fn example6() {
        let nums = vec![4,10,4,3,8,9];

        let l = Solution::length_of_lis(nums);

        assert_eq!(l, 3);
    }
}