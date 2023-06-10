/*
    Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

    Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

    Return k after placing the final result in the first k slots of nums.

    Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

    Constraints:

    1 <= nums.length <= 3 * 10^4
    -10^4 <= nums[i] <= 10^4
    nums is sorted in non-decreasing order.
 */

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut k = nums.len();
        if k < 3 { return k as i32; }
        let mut i = 2;
        while i < k {
            if nums[i] == nums[i - 1] && nums[i] == nums[i - 2] {
                k -= 1;
                nums.remove(i);
            } else {
                i += 1;
            }
        }
        k as i32
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
        Input: nums = [1,1,1,2,2,3]
        Output: 5, nums = [1,1,2,2,3,_]
        Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
        It does not matter what you leave beyond the returned k (hence they are underscores).
     */
    #[test]
    fn example1() {
        let mut nums = vec![1,1,1,2,2,3];

        let k = Solution::remove_duplicates(&mut nums);

        judge(&mut nums, k, &vec![1,1,2,2,3]);
    }

    /*
        Input: nums = [0,0,1,1,1,1,2,3,3]
        Output: 7, nums = [0,0,1,1,2,3,3,_,_]
        Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
        It does not matter what you leave beyond the returned k (hence they are underscores).     */
    #[test]
    fn example2() {
        let mut nums = vec![0,0,1,1,1,1,2,3,3];

        let k = Solution::remove_duplicates(&mut nums);

        judge(&mut nums, k, &vec![0,0,1,1,2,3,3]);
    }
}