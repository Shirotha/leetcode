/*
    You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

    An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.

    Return the number of possible unique paths that the robot can take to reach the bottom-right corner.

    The testcases are generated so that the answer will be less than or equal to 2 * 10^9.

    Constraints:

    m == obstacleGrid.length
    n == obstacleGrid[i].length
    1 <= m, n <= 100
    obstacleGrid[i][j] is 0 or 1.
 */

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 { return 0; }
        obstacle_grid[0].iter_mut()
            .take_while( |x| **x == 0 )
            .for_each( |x| *x = -1 );
        obstacle_grid.iter_mut().skip(1)
            .take_while( |row| row[0] == 0 )
            .for_each( |row| row[0] = -1 );
        let (mut i0, mut j0) = (1, 1);
        while i0 != m && j0 != n {
            if j0 != m {
                for j in j0..n {
                    if obstacle_grid[i0][j] == 1 { continue; }
                    let (a, b) = (obstacle_grid[i0 - 1][j], obstacle_grid[i0][j - 1]);
                    obstacle_grid[i0][j] += a.min(0) + b.min(0);
                }
                i0 += 1;
            }
            if i0 != n {
                for i in i0..m {
                    if obstacle_grid[i][j0] == 1 { continue; }
                    let (a, b) = (obstacle_grid[i - 1][j0], obstacle_grid[i][j0 - 1]);
                    obstacle_grid[i][j0] += a.min(0) + b.min(0);
                }
                j0 += 1;
            }
        }
        -obstacle_grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
        Output: 2
        Explanation: There is one obstacle in the middle of the 3x3 grid above.
        There are two ways to reach the bottom-right corner:
        1. Right -> Right -> Down -> Down
        2. Down -> Down -> Right -> Right
     */
    #[test]
    fn example1() {
        let obstacle_grid = vec![
            vec![0,0,0],
            vec![0,1,0],
            vec![0,0,0]];

        let n = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(n, 2);
    }

    /*
        Input: obstacleGrid = [[0,1],[0,0]]
        Output: 1
     */
    #[test]
    fn example2() {
        let obstacle_grid = vec![
            vec![0,1],
            vec![0,0]];

        let n = Solution::unique_paths_with_obstacles(obstacle_grid);

        assert_eq!(n, 1);
    }
}