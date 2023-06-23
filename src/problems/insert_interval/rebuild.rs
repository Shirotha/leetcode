/*
    You are given an array of non-overlapping intervals intervals where intervals[i] = [start_i, end_i] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

    Insert newInterval into intervals such that intervals is still sorted in ascending order by start_i and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

    Return intervals after the insertion.

    Constraints:

    0 <= intervals.length <= 10^4
    intervals[i].length == 2
    0 <= start_i <= end_i <= 10^5
    intervals is sorted by start_i in ascending order.
    newInterval.length == 2
    0 <= start <= end <= 10^5
 */

pub struct Solution;

use std::iter::Peekable;

struct PeekingTakeWhile<'a, I: Iterator, P: FnMut(&I::Item) -> bool> {
    iter: &'a mut Peekable<I>,
    predicate: P,
}
impl<I: Iterator, P: FnMut(&I::Item) -> bool> Iterator for PeekingTakeWhile<'_, I, P> {
    type Item = I::Item;
    #[inline] fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_if(&mut self.predicate)
    }
    #[inline] fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}
trait PeekingTakeWhileIterator<I: Iterator>: Iterator {
    fn peeking_take_while<P: FnMut(&I::Item) -> bool>(&mut self, predicate: P) -> PeekingTakeWhile<I, P>;
}
impl<I: Iterator> PeekingTakeWhileIterator<I> for Peekable<I> {
    #[inline] fn peeking_take_while<P: FnMut(&<I as Iterator>::Item) -> bool>(&mut self, predicate: P) -> PeekingTakeWhile<I, P> {
        PeekingTakeWhile { iter: self, predicate }
    }
}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut iter = intervals.into_iter().peekable();
        let mut result: Vec<Vec<i32>> = iter
            .peeking_take_while( |i| i[1] < new_interval[0] )
            .collect();
        let mut overlap = iter
            .peeking_take_while( |i| i[0] <= new_interval[1] );
        if let Some(first) = overlap.next()
        {
            let end = overlap.last()
                .map_or_else( || first[1] , |i| i[1] );
            result.push(vec![
                new_interval[0].min(first[0]), 
                new_interval[1].max(end)
            ]);
        } else {
            result.push(new_interval);
        }
        result.extend(iter);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
        Output: [[1,5],[6,9]]
    */
    #[test]
    fn example1() {
        let intervals = vec![
            vec![1,3],
            vec![6,9]
        ];
        let new_interval = vec![2,5];

        let m = Solution::insert(intervals, new_interval);

        let result = vec![
            vec![1,5],
            vec![6,9]
        ];

        assert_eq!(m, result);
    }

    /*
        Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
        Output: [[1,2],[3,10],[12,16]]
        Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
    */
    #[test]
    fn example2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12,16]
        ];
        let new_interval = vec![4,8];

        let m = Solution::insert(intervals, new_interval);

        let result = vec![
            vec![1, 2],
            vec![3, 10],
            vec![12,16]
        ];

        assert_eq!(m, result);
    }

    /*
        Input: intervals = [[1,5]], newInterval = [2,3]
        Output: [[1,5]]
    */
    #[test]
    fn example3() {
        let intervals = vec![
            vec![1,5]
        ];
        let new_interval = vec![2,3];

        let m = Solution::insert(intervals, new_interval);

        let result = vec![
            vec![1,5]
        ];

        assert_eq!(m, result);
    }

    /*
        Input: intervals = [[1,5]], newInterval = [6,8]
        Output: [[1,5],[6,8]]
    */
    #[test]
    fn example4() {
        let intervals = vec![
            vec![1,5]
        ];
        let new_interval = vec![6,8];

        let m = Solution::insert(intervals, new_interval);

        let result = vec![
            vec![1,5],
            vec![6,8]
        ];

        assert_eq!(m, result);
    }

    /*
        Input: intervals = [[3,5],[12,15]], newInterval = [6,6]
        Output: [[3,5],[6,6],[12,15]]
    */
    #[test]
    fn example5() {
        let intervals = vec![
            vec![3, 5],
            vec![12,15]
        ];
        let new_interval = vec![6,6];

        let m = Solution::insert(intervals, new_interval);

        let result = vec![
            vec![3, 5],
            vec![6, 6],
            vec![12,15]
        ];

        assert_eq!(m, result);
    }
}