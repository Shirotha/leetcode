/*
    Given an integer array nums, return the length of the longest strictly increasing subsequence.

    Constraints:

    1 <= nums.length <= 2500
    -10^4 <= nums[i] <= 10^4
 */

pub struct Solution;

use std::collections::{HashMap, hash_map::Entry::*};

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut front = HashMap::new();
        for num in nums {
            let count = front.iter()
                .filter_map( |(&count, &tail)| if tail < num { Some(count) } else { None } )
                .max().unwrap_or(0) + 1;
            match front.entry(count) {
                Occupied(mut e) => if e.get() > &num { e.insert(num); },
                Vacant(e) => { e.insert(num); },
            }
        }
        front.into_keys().max().unwrap()
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
}