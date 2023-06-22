/*
    According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

    The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

    Any live cell with fewer than two live neighbors dies as if caused by under-population.
    Any live cell with two or three live neighbors lives on to the next generation.
    Any live cell with more than three live neighbors dies, as if by over-population.
    Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.

    Constraints:

    m == board.length
    n == board[i].length
    1 <= m, n <= 25
    board[i][j] is 0 or 1.
 */

pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for i in 0..board.len() { for j in 0..board[0].len() {
            let count: usize = board.iter().take(i + 2).skip(i.saturating_sub(1))
                .map( |row| {
                    row.iter().take(j + 2).skip(j.saturating_sub(1))
                        .filter( |&s| s % 2 == 1 ).count()
                }).sum();
            if count == 3 || count == 4 && board[i][j] % 2 == 1 {
                board[i][j] |= 2;
            }
        } }
        board.iter_mut().for_each( |row| {
            row.iter_mut().for_each( |s| *s >>= 1 );    
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
        Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
     */
    #[test]
    fn example1() {
        let mut board  = vec![
            vec![0,1,0],
            vec![0,0,1],
            vec![1,1,1],
            vec![0,0,0]
        ];

        Solution::game_of_life(&mut board);

        let result = vec![
            vec![0,0,0],
            vec![1,0,1],
            vec![0,1,1],
            vec![0,1,0]
        ];

        assert_eq!(board, result);
    }

    /*
        Input: board = [[1,1],[1,0]]
        Output: [[1,1],[1,1]]
     */
    #[test]
    fn example2() {
        let mut board  = vec![
            vec![1,1],
            vec![1,0]
        ];

        Solution::game_of_life(&mut board);

        let result = vec![
            vec![1,1],
            vec![1,1]
        ];

        assert_eq!(board, result);
    }
}