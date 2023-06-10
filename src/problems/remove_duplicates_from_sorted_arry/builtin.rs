/*
    Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

    Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

    Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
    Return k.

    Constraints:

    1 <= nums.length <= 3 * 10^4
    -100 <= nums[i] <= 100
    nums is sorted in non-decreasing order.
 */

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(nums: &mut Vec<i32>, k: i32, result: &Vec<i32>) {
        let k = k as usize;
        assert_eq!(k, result.len());
        nums.truncate(k);
        assert_eq!(nums, result);
    }

    /*
        Input: nums = [1,1,2]
        Output: 2, nums = [1,2,_]
        Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
        It does not matter what you leave beyond the returned k (hence they are underscores).
     */
    #[test]
    fn example1() {
        let mut nums = vec![1,1,2];

        let k = Solution::remove_duplicates(&mut nums);

        judge(&mut nums, k, &vec![1,2]);
    }

    /*
        Input: nums = [0,0,1,1,1,2,2,3,3,4]
        Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
        Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
        It does not matter what you leave beyond the returned k (hence they are underscores).     */
    #[test]
    fn example2() {
        let mut nums = vec![0,0,1,1,1,2,2,3,3,4];

        let k = Solution::remove_duplicates(&mut nums);

        judge(&mut nums, k, &vec![0,1,2,3,4]);
    }
}