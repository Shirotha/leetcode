/*
    You are given an m x n integer matrix matrix with the following two properties:

    Each row is sorted in non-decreasing order.
    The first integer of each row is greater than the last integer of the previous row.
    Given an integer target, return true if target is in matrix or false otherwise.

    You must write a solution in O(log(m * n)) time complexity.

    Constraints:

    m == matrix.length
    n == matrix[i].length
    1 <= m, n <= 100
    -10^4 <= matrix[i][j], target <= 10^4
 */

pub struct Solution;

use std::cmp::Ordering::{self, *};

#[inline] fn range_compare(min: i32, max: i32, target: i32) -> Ordering {
    if target < min { return Greater; }
    if target > max { return Less; }
    Equal
}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix[0].len() - 1;
        if let Ok(i) = matrix.binary_search_by( |row| range_compare(row[0], row[n], target) ) {
            matrix[i].binary_search(&target).is_ok()
        } else { false }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
        Output: true
     */
    #[test]
    fn example1() {
        let matrix = vec![
            vec![1,3,5,7],
            vec![10,11,16,20],
            vec![23,30,34,60]];
        let target = 3;

        let b = Solution::search_matrix(matrix, target);

        assert!(b)
    }

    /*
        Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
        Output: false
     */
    #[test]
    fn example2() {
        let matrix = vec![
            vec![1,3,5,7],
            vec![10,11,16,20],
            vec![23,30,34,60]];
        let target = 13;

        let b = Solution::search_matrix(matrix, target);

        assert!(!b)
    }
}