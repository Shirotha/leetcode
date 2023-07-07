/*
    Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.

    A region is captured by flipping all 'O's into 'X's in that surrounded region.

    Constraints:

    m == board.length
    n == board[i].length
    1 <= m, n <= 200
    board[i][j] is 'X' or 'O'.
 */

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    #[inline] fn floodfill(
        queue: &mut VecDeque<(usize, usize)>, 
        board: &mut [Vec<char>], 
        w: &usize, h: &usize, i: usize, j: usize)
    {
        if board[i][j] != 'O' { return; }
        board[i][j] = 'B'; 
        queue.push_back((i, j));
        while let Some((i, j)) = queue.pop_front() {
            if i != 0 && board[i - 1][j] == 'O' { 
                board[i - 1][j] = 'B'; 
                queue.push_back((i - 1, j)); 
            }
            if j != 0 && board[i][j - 1] == 'O' { 
                board[i][j - 1] = 'B'; 
                queue.push_back((i, j - 1)); 
            }
            if i != h - 1 && board[i + 1][j] == 'O' { 
                board[i + 1][j] = 'B'; 
                queue.push_back((i + 1, j)); 
            }
            if j != w - 1 && board[i][j + 1] == 'O' { 
                board[i][j + 1] = 'B'; 
                queue.push_back((i, j + 1)); 
            }
        }
    }
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let h = board.len();
        let w = board[0].len();
        let queue = &mut VecDeque::new();
        for i in 0..w {
            Self::floodfill(queue, board, &w, &h, 0, i);
            Self::floodfill(queue, board, &w, &h, h - 1, i);
        }
        for j in 1..(h - 1) {
            Self::floodfill(queue, board, &w, &h, j, 0);
            Self::floodfill(queue, board, &w, &h, j, w - 1);
        }
        for row in board.iter_mut() {
            for x in row.iter_mut() {
                if x == &'B' { *x = 'O'; }
                else if x == &'O' { *x = 'X'; }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn str_to_grid(string: &str) -> Vec<Vec<char>> {
        string.split('\n').map( |line| line.chars().collect() ).collect()
    }

    /*
        Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
        Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
        Explanation: Notice that an 'O' should not be flipped if:
        - It is on the border, or
        - It is adjacent to an 'O' that should not be flipped.
        The bottom 'O' is on the border, so it is not flipped.
        The other three 'O' form a surrounded region, so they are flipped.
     */
    #[test]
    fn example1() {
        let mut board = str_to_grid("\
        XXXX\n\
        XOOX\n\
        XXOX\n\
        XOXX");

        Solution::solve(&mut board);

        let result = str_to_grid("\
        XXXX\n\
        XXXX\n\
        XXXX\n\
        XOXX");

        assert_eq!(board, result);
    }

    /*
        Input: board = [["X"]]
        Output: [["X"]]
     */
    #[test]
    fn example2() {
        let mut board = str_to_grid("X");

        Solution::solve(&mut board);

        let result = str_to_grid("X");

        assert_eq!(board, result);
    }

    /*
        Input: board = [["X","O","X","O","X","O"],["O","X","O","X","O","X"],["X","O","X","O","X","O"],["O","X","O","X","O","X"]]
        Output: [["X","O","X","O","X","O"],["O","X","X","X","X","X"],["X","X","X","X","X","O"],["O","X","O","X","O","X"]]
     */
    #[test]
    fn example3() {
        let mut board = str_to_grid("\
            XOXOXO\n\
            OXOXOX\n\
            XOXOXO\n\
            OXOXOX");

        Solution::solve(&mut board);

        let result = str_to_grid("\
            XOXOXO\n\
            OXXXXX\n\
            XXXXXO\n\
            OXOXOX");

        assert_eq!(board, result);
    }

    /*
        Input: board = [["O","O","O","O","O","O","O","O","X","O","O","O","O","O","X","O","O","O","O","O"],["O","O","O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O","O","O"],["X","O","O","X","O","X","O","O","O","O","X","O","O","X","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","X","X","O"],["O","X","X","O","O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O"],["O","O","O","O","X","O","O","O","O","O","O","X","O","O","O","O","O","X","X","O"],["O","O","O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","O","O","O","O","O","X","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","X","O"],["O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O","O"],["O","O","O","O","X","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","O","O"],["X","O","O","O","O","O","O","O","O","X","X","O","O","O","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","O","O","O","X","O","O","O","O","O","O","O","O"],["O","O","O","O","X","O","O","O","O","O","O","O","O","X","O","O","O","O","O","X"],["O","O","O","O","O","X","O","O","O","O","O","O","O","O","O","X","O","X","O","O"],["O","X","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O"],["O","O","O","O","O","O","O","O","X","X","O","O","O","X","O","O","X","O","O","X"],["O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O","O"]]
        Output: [["X","O","X","O","X","O"],["O","X","X","X","X","X"],["X","X","X","X","X","O"],["O","X","O","X","O","X"]]
     */
    #[test]
    fn example4() {
        let mut board = str_to_grid("\
            OOOOOOOOXOOOOOXOOOOO\n\
            OOOOOOOXOOOOOOOOOOOO\n\
            XOOXOXOOOOXOOXOOOOOO\n\
            OOOOOOOOOOOOOOOOOXXO\n\
            OXXOOOOOOXOOOOOOOOOO\n\
            OOOOXOOOOOOXOOOOOXXO\n\
            OOOOOOOXOOOOOOOOOOOO\n\
            OOOOOOOOOOOOOXOOOOOO\n\
            OOOOOOOOOOOOOOOOOOXO\n\
            OOOOOXOOOOOOOOOOOOOO\n\
            OOOOOOOOXOOOOOOOOOOO\n\
            OOOOXOOOOXOOOOOOOOOO\n\
            OOOOOOOOXOOOOOOOOOOO\n\
            XOOOOOOOOXXOOOOOOOOO\n\
            OOOOOOOOOOOXOOOOOOOO\n\
            OOOOXOOOOOOOOXOOOOOX\n\
            OOOOOXOOOOOOOOOXOXOO\n\
            OXOOOOOOOOOOOOOOOOOO\n\
            OOOOOOOOXXOOOXOOXOOX\n\
            OOOOOOOOOOOOOOOOOOOO");

        Solution::solve(&mut board);

        let result = str_to_grid("\
            OOOOOOOOXOOOOOXOOOOO\n\
            OOOOOOOXOOOOOOOOOOOO\n\
            XOOXOXOOOOXOOXOOOOOO\n\
            OOOOOOOOOOOOOOOOOXXO\n\
            OXXOOOOOOXOOOOOOOOOO\n\
            OOOOXOOOOOOXOOOOOXXO\n\
            OOOOOOOXOOOOOOOOOOOO\n\
            OOOOOOOOOOOOOXOOOOOO\n\
            OOOOOOOOOOOOOOOOOOXO\n\
            OOOOOXOOOOOOOOOOOOOO\n\
            OOOOOOOOXOOOOOOOOOOO\n\
            OOOOXOOOOXOOOOOOOOOO\n\
            OOOOOOOOXOOOOOOOOOOO\n\
            XOOOOOOOOXXOOOOOOOOO\n\
            OOOOOOOOOOOXOOOOOOOO\n\
            OOOOXOOOOOOOOXOOOOOX\n\
            OOOOOXOOOOOOOOOXOXOO\n\
            OXOOOOOOOOOOOOOOOOOO\n\
            OOOOOOOOXXOOOXOOXOOX\n\
            OOOOOOOOOOOOOOOOOOOO");

        assert_eq!(board, result);
    }
}