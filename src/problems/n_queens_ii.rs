/*
    The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

    Given an integer n, return the number of distinct solutions to the n-queens puzzle.

    Constraints:

    1 <= n <= 9
 */

pub struct Solution;

fn place_queen(board: &[u16], n: u8, row: usize, col: u8, result: &mut [u16]) -> Option<u8> {
    let queen = col + (board[row] >> col).trailing_ones() as u8;
    if queen >= n { return None; }
    let middle = 1u16 << queen;
    let mut left = middle;
    let mut right = middle;
    for (orig, r) in board.iter().zip(result.iter_mut()).skip(row + 1) {
        left <<= 1; right >>= 1;
        *r = orig | left | middle | right;
    }
    Some(queen)
}
#[inline] fn clear_board(board: &mut [u16]) {
    for row in board.iter_mut() { *row = 0; }
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n == 1 { return 1; }
        let mut s = 0;
        let n = n as usize;
        let mut stack = vec![(0u8, vec![0u16; n]); n];
        let n = n as u8;
        let mask = u16::MAX >> n << n;
        let mut row = 0;
        loop {
            if row as u8 == n - 1 {
                let (_, board) = stack.last_mut().unwrap();
                s += (board.last().unwrap() | mask).count_zeros() as i32;
                clear_board(board);
                row -= 1;
            }
            let (current, next) = stack.split_at_mut(row + 1);
            let (col, board) = current.last_mut().unwrap();
            let next = &mut next.first_mut().unwrap().1;
            if let Some(queen) = place_queen(board, n, row, *col, next.as_mut_slice()) {
                *col = queen + 1; row += 1;
            } else if row == 0 { return s; }
            else { *col = 0; clear_board(board); row -= 1; }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: n = 4
        Output: 2
        Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
     */
    #[test]
    fn example1() {
        let n = 4;

        let s = Solution::total_n_queens(n);

        assert_eq!(s, 2);
    }

    /*
        Input: n = 1
        Output: 1
     */
    #[test]
    fn example2() {
        let n = 1;

        let s = Solution::total_n_queens(n);

        assert_eq!(s, 1);
    }
}