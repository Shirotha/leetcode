/*
    Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

    Each row must contain the digits 1-9 without repetition.
    Each column must contain the digits 1-9 without repetition.
    Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
    Note:

    A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    Only the filled cells need to be validated according to the mentioned rules.

    Constraints:

    board.length == 9
    board[i].length == 9
    board[i][j] is a digit 1-9 or '.'.
 */

pub struct Solution;

trait Digit {
    fn index(self) -> usize;
}
impl Digit for char {
    #[inline]
    fn index(self) -> usize { self as usize - 0x31 }
}
impl Digit for &char {
    #[inline]
    fn index(self) -> usize { (*self).index() }
}

impl Solution {
    #[inline]
    fn check_row(row: &[char]) -> bool {
        let mut v = [false; 9];
        for c in row.iter().take(9) {
            if c == &'.' { continue; }
            let b = unsafe { v.get_unchecked_mut(c.index()) };
            if *b { return false; } else { *b = true; }
        }
        true
    }
    #[inline]
    fn check_column(grid: &[Vec<char>], c: usize) -> bool {
        let mut v = [false; 9];
        for c in grid.iter().take(9).map( |r| r[c] ) {
            if c == '.' { continue; }
            let b = unsafe { v.get_unchecked_mut(c.index()) };
            if *b { return false; } else { *b = true; }
        }
        true
    }
    #[inline]
    fn check_box(grid: &[Vec<char>], c: usize, r: usize) -> bool {
        let mut v = [false; 9];
        let c = 3*c;
        let r = 3*r;
        for c in grid[r..(r+3)].iter().flat_map( |r| r[c..(c+3)].iter() ) {
            if c == &'.' { continue; }
            let b = unsafe { v.get_unchecked_mut(c.index()) };
            if *b { return false; } else { *b = true; }
        }
        true
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for r in board.iter() {
            if !Self::check_row(r) { return false; }
        }
        for c in 0..9 {
            if !Self::check_column(&board, c) { return false; }
        }
        for c in 0..3 { for r in 0..3 {
            if !Self::check_box(&board, c, r) { return false; }
        } }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: board = 
        [['5','3','.','.','7','.','.','.','.']
        ,['6','.','.','1','9','5','.','.','.']
        ,['.','9','8','.','.','.','.','6','.']
        ,['8','.','.','.','6','.','.','.','3']
        ,['4','.','.','8','.','3','.','.','1']
        ,['7','.','.','.','2','.','.','.','6']
        ,['.','6','.','.','.','.','2','8','.']
        ,['.','.','.','4','1','9','.','.','5']
        ,['.','.','.','.','8','.','.','7','9']]
        Output: true
     */
    #[test]
    fn example1() {
        let board = [
             ['5','3','.','.','7','.','.','.','.']
            ,['6','.','.','1','9','5','.','.','.']
            ,['.','9','8','.','.','.','.','6','.']
            ,['8','.','.','.','6','.','.','.','3']
            ,['4','.','.','8','.','3','.','.','1']
            ,['7','.','.','.','2','.','.','.','6']
            ,['.','6','.','.','.','.','2','8','.']
            ,['.','.','.','4','1','9','.','.','5']
            ,['.','.','.','.','8','.','.','7','9']
        ].map(|r| r.to_vec()).to_vec();

        let v = Solution::is_valid_sudoku(board);

        assert!(v);
    }

        /*
        Input: board = 
        [['8','3','.','.','7','.','.','.','.']
        ,['6','.','.','1','9','5','.','.','.']
        ,['.','9','8','.','.','.','.','6','.']
        ,['8','.','.','.','6','.','.','.','3']
        ,['4','.','.','8','.','3','.','.','1']
        ,['7','.','.','.','2','.','.','.','6']
        ,['.','6','.','.','.','.','2','8','.']
        ,['.','.','.','4','1','9','.','.','5']
        ,['.','.','.','.','8','.','.','7','9']]
        Output: false
        Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
     */
    #[test]
    fn example2() {
        let board = [
             ['8','3','.','.','7','.','.','.','.']
            ,['6','.','.','1','9','5','.','.','.']
            ,['.','9','8','.','.','.','.','6','.']
            ,['8','.','.','.','6','.','.','.','3']
            ,['4','.','.','8','.','3','.','.','1']
            ,['7','.','.','.','2','.','.','.','6']
            ,['.','6','.','.','.','.','2','8','.']
            ,['.','.','.','4','1','9','.','.','5']
            ,['.','.','.','.','8','.','.','7','9']
        ].map(|r| r.to_vec()).to_vec();

        let v = Solution::is_valid_sudoku(board);

        assert!(!v);
    }
}