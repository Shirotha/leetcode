/*
    You are given a sorted unique integer array nums.

    A range [a,b] is the set of all integers from a to b (inclusive).

    Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.

    Each range [a,b] in the list should be output as:

    "a->b" if a != b
    "a" if a == b

    Constraints:

    0 <= nums.length <= 20
    -2^31 <= nums[i] <= 2^31 - 1
    All the values of nums are unique.
    nums is sorted in ascending order.
 */

pub struct Solution;

trait PushRange<T> {
    fn push_range(&mut self, start: T, end: T);
}
impl PushRange<i32> for Vec<String> {
    fn push_range(&mut self, start: i32, end: i32) {
        if start == end {
            self.push(start.to_string());
        } else {
            self.push(format!("{}->{}", start, end));
        }
    }
}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        if nums.is_empty() { return result; }
        let mut start = nums[0];
        let mut end = start;
        for n in nums.into_iter().skip(1) {
            if n > end + 1 {
                result.push_range(start, end);
                start = n;
            }
            end = n;
        }
        result.push_range(start, end);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [0,1,2,4,5,7]
        Output: ["0->2","4->5","7"]
        Explanation: The ranges are:
        [0,2] --> "0->2"
        [4,5] --> "4->5"
        [7,7] --> "7"
     */
    #[test]
    fn example1() {
        let nums = vec![0,1,2,4,5,7];

        let r = Solution::summary_ranges(nums);

        let result: Vec<String> = vec!["0->2","4->5","7"]
            .into_iter().map(str::to_string).collect();

        assert_eq!(r, result);
    }

    /*
        Input: nums = [0,2,3,4,6,8,9]
        Output: ["0","2->4","6","8->9"]
        Explanation: The ranges are:
        [0,0] --> "0"
        [2,4] --> "2->4"
        [6,6] --> "6"
        [8,9] --> "8->9"
     */
    #[test]
    fn example2() {
        let nums = vec![0,2,3,4,6,8,9];

        let r = Solution::summary_ranges(nums);

        let result: Vec<String> = vec!["0","2->4","6","8->9"]
            .into_iter().map(str::to_string).collect();

        assert_eq!(r, result);
    }
}