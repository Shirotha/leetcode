/*
    Given an m x n grid of characters board and a string word, return true if word exists in the grid.

    The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

    Constraints:

    m == board.length
    n = board[i].length
    1 <= m, n <= 6
    1 <= word.length <= 15
    board and word consists of only lowercase and uppercase English letters.
 */

pub struct Solution;

use std::collections::{VecDeque, HashSet};

#[inline] fn step(i: usize, j: usize, d0: &mut u8, imax: usize, jmax: usize) -> Option<(usize, usize)> {
    let d = *d0;
    *d0 += 1;
    match d {
        0 if i != 0 => Some((i - 1, j)),
        1 if j != 0 => Some((i, j - 1)),
        2 if j != jmax => Some((i, j + 1)),
        3 if i != imax => Some((i + 1, j)),
        _ => None,
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let l0 = word.len();
        if l0 == 1 {
            let c = word.chars().next().unwrap();
            return board.iter().any( |row| row.contains(&c) );
        }
        let word = word.as_bytes();
        let imax = board.len() - 1;
        let jmax = board[0].len() - 1;
        let mut stack = VecDeque::with_capacity(l0);
        let mut visited = HashSet::new();
        let mut l;
        for (i0, row) in board.iter().enumerate() {
            for (j0, root) in row.iter().enumerate() {
                if *root as u8 == word[0] {
                    stack.push_back((i0, j0, 0));
                    visited.insert((i0, j0));
                    l = 1;
                    'path: while let Some((i, j, d)) = stack.back_mut() {
                        let (i, j) = (*i, *j);
                        while *d < 4 {
                            if let Some((i, j)) = step(i, j, d, imax, jmax) {
                                if !visited.contains(&(i, j)) && board[i][j] as u8 == word[l] {
                                    l += 1;
                                    if l == l0 { return true; }
                                    stack.push_back((i, j, 0));
                                    visited.insert((i, j));
                                    continue 'path;
                                }
                            }
                        }
                        visited.remove(&(i, j));
                        stack.pop_back();
                        l -= 1;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
        Output: true
     */
    #[test]
    fn example1() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']];
        let word = "ABCCED".to_string();

        let b = Solution::exist(board, word);

        assert!(b);
    }

    /*
        Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
        Output: true
     */
    #[test]
    fn example2() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']];
        let word = "SEE".to_string();

        let b = Solution::exist(board, word);

        assert!(b);
    }

    /*
        Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
        Output: false
     */
    #[test]
    fn example3() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']];
        let word = "ABCB".to_string();

        let b = Solution::exist(board, word);

        assert!(!b);
    }

    /*
        Input: board = [["A","B","C","E"],["S","F","E","S"],["A","D","E","E"]], word = "ABCESEEEFS"
        Output: true
     */
    #[test]
    fn example4() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','E','S'],
            vec!['A','D','E','E']];
        let word = "ABCESEEEFS".to_string();

        let b = Solution::exist(board, word);

        assert!(b);
    }
}