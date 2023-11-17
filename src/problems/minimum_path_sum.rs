/*
    Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.

    Note: You can only move either down or right at any point in time.

    Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 200
    0 <= grid[i][j] <= 200
 */

pub struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for j in 1..n { grid[0][j] += grid[0][j - 1]; }
        for i in 1..m { grid[i][0] += grid[i - 1][0]; }
        let (mut i0, mut j0) = (1, 1);
        while i0 != m && j0 != n {
            if j0 != m {
                for j in j0..n {
                    let (a, b) = (grid[i0 - 1][j], grid[i0][j - 1]);
                    grid[i0][j] += a.min(b);
                }
                i0 += 1;
            }
            if i0 != n {
                for i in i0..m {
                    let (a, b) = (grid[i - 1][j0], grid[i][j0 - 1]);
                    grid[i][j0] += a.min(b);
                }
                j0 += 1;
            }
        }
        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
        Output: 7
        Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
     */
    #[test]
    fn example1() {
        let grid = vec![
            vec![1,3,1],
            vec![1,5,1],
            vec![4,2,1]];

        let s = Solution::min_path_sum(grid);

        assert_eq!(s, 7);
    }

    /*
        Input: grid = [[1,2,3],[4,5,6]]
        Output: 12
     */
    #[test]
    fn example2() {
        let grid = vec![
            vec![1,2,3],
            vec![4,5,6]];

        let s = Solution::min_path_sum(grid);

        assert_eq!(s, 12);
    }
}