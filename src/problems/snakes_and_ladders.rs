/*
    You are given an n x n integer matrix board where the cells are labeled from 1 to n^2 in a Boustrophedon style starting from the bottom left of the board (i.e. board[n - 1][0]) and alternating direction each row.

    You start on square 1 of the board. In each move, starting from square curr, do the following:

    Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n^2)].
    This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
    If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
    The game ends when you reach the square n^2.
    A board square on row r and column c has a snake or ladder if board[r][c] != -1. The destination of that snake or ladder is board[r][c]. Squares 1 and n^2 do not have a snake or ladder.

    Note that you only take a snake or ladder at most once per move. If the destination to a snake or ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.

    For example, suppose the board is [[-1,4],[-1,3]], and on the first move, your destination square is 2. You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
    Return the least number of moves required to reach the square n^2. If it is not possible to reach the square, return -1.

    Constraints:

    n == board.length == board[i].length
    2 <= n <= 20
    board[i][j] is either -1 or in the range [1, n^2].
    The squares labeled 1 and n^2 do not have any ladders or snakes.
 */

pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let goal = n * n - 1;
        let mut open = VecDeque::new();
        let mut closed = HashSet::new();
        open.push_back(0);
        closed.insert(0);
        let mut g = 0;
        while !open.is_empty() {
            g += 1;
            for _ in 0..open.len() {
                let current = open.pop_front().unwrap();
                let mut first = true;
                for i in (1..=6).rev() {
                    let child = current + i;
                    if child >= goal { return g; }
                    let r = child / n;
                    let c = if r & 1 == 0 { child % n } else { n - child % n - 1 };
                    let value = board[n - r - 1][c];
                    if value == -1 { if first { first = false; } else { continue; } }
                    let target = match value {
                        -1 => child,
                        x => {
                            let target = x as usize - 1;
                            if target == goal { return g; }
                            target
                        },
                    };
                    if !closed.insert(target) { continue; }
                    open.push_back(target);
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
        Output: 4
        Explanation: 
        In the beginning, you start at square 1 (at row 5, column 0).
        You decide to move to square 2 and must take the ladder to square 15.
        You then decide to move to square 17 and must take the snake to square 13.
        You then decide to move to square 14 and must take the ladder to square 35.
        You then decide to move to square 36, ending the game.
        This is the lowest possible number of moves to reach the last square, so return 4.
     */
    #[test]
    fn example1() {
        let board = vec![
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,35,-1,-1,13,-1],
            vec![-1,-1,-1,-1,-1,-1],
            vec![-1,15,-1,-1,-1,-1]
        ];

        let n = Solution::snakes_and_ladders(board);

        assert_eq!(n, 4);
    }

    /*
        Input: board = [[-1,-1],[-1,3]]
        Output: 1
     */
    #[test]
    fn example2() {
        let board = vec![
            vec![-1,-1],
            vec![-1, 3],
        ];

        let n = Solution::snakes_and_ladders(board);

        assert_eq!(n, 1);
    }
}