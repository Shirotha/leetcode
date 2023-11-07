/*
    There is an integer array nums sorted in ascending order (with distinct values).

    Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

    Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

    You must write an algorithm with O(log n) runtime complexity.

    Constraints:

    1 <= nums.length <= 5000
    -10^4 <= nums[i] <= 10^4
    All values of nums are unique.
    nums is an ascending array that is possibly rotated.
    -10^4 <= target <= 10^4
 */

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len() - 1;
        let (i, j) = {
            let a = nums[0];
            let b = nums[n];
            if b < a {
                let mut i = 0;
                let mut j = n;
                while i <= j {
                    let p = (i + j) >> 1;
                    if nums[p + 1] > a { i = p + 1; }
                    else if nums[p] < a { j = p - 1; }
                    else { i = p; break; }
                }
                if target < a { (i + 1, n) } else { (0, i) }
            } else { (0, n) }
        };
        nums[i..=j].binary_search(&target)
            .map( |j| (i + j) as i32 )
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [4,5,6,7,0,1,2], target = 0
        Output: 4
     */
    #[test]
    fn example1() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 0;

        let i = Solution::search(nums, target);

        assert_eq!(i, 4);
    }

    /*
        Input: nums = [4,5,6,7,0,1,2], target = 3
        Output: -1
     */
    #[test]
    fn example2() {
        let nums = vec![4,5,6,7,0,1,2];
        let target = 3;

        let i = Solution::search(nums, target);

        assert_eq!(i, -1);
    }

    /*
        Input: nums = [1], target = 0
        Output: -1
     */
    #[test]
    fn example3() {
        let nums = vec![1];
        let target = 0;

        let i = Solution::search(nums, target);

        assert_eq!(i, -1);
    }
}