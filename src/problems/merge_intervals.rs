/*
    Given an array of intervals where intervals[i] = [start_i, end_i], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

    Constraints:

    1 <= intervals.length <= 10^4
    intervals[i].length == 2
    0 <= start_i <= end_i <= 10^4
 */

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_unstable_by( |a, b| a[0].cmp(&b[0]) );
        let mut result = Vec::new();
        let mut current = [intervals[0][0], intervals[0][1]];
        for i in intervals.into_iter().skip(1) {
            if i[0] <= current[1] {
                if i[1] > current[1] { current[1] = i[1]; }
            } else {
                result.push(current.to_vec());
                current = [i[0], i[1]];
            }
        }
        result.push(current.to_vec());
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
        Output: [[1,6],[8,10],[15,18]]
        Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
     */
    #[test]
    fn example1() {
        let intervals = vec![
            vec![1, 3],
            vec![2, 6],
            vec![8, 10],
            vec![15,18]
        ];

        let m = Solution::merge(intervals);

        let result = vec![
            vec![1,6],
            vec![8,10],
            vec![15,18]
        ];

        assert_eq!(m, result);
    }

    /*
        Input: intervals = [[1,4],[4,5]]
        Output: [[1,5]]
        Explanation: Intervals [1,4] and [4,5] are considered overlapping.
     */
    #[test]
    fn example2() {
        let intervals = vec![
            vec![1, 4],
            vec![4, 5]
        ];

        let m = Solution::merge(intervals);

        let result = vec![
            vec![1,5]
        ];

        assert_eq!(m, result);
    }

    /*
        Input: intervals = [[1,4],[0,4]]
        Output: [[0,4]]
        Explanation: Intervals [1,4] and [4,5] are considered overlapping.
     */
    #[test]
    fn example3() {
        let intervals = vec![
            vec![1, 4],
            vec![0, 4]
        ];

        let m = Solution::merge(intervals);

        let result = vec![
            vec![0,4]
        ];

        assert_eq!(m, result);
    }
}