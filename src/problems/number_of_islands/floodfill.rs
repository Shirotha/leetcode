/*
    Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

    An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

    Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 300
    grid[i][j] is '0' or '1'.
 */

pub struct Solution;

use std::collections::VecDeque;

type Deque<'a> = &'a mut VecDeque<(usize, usize)>;

impl Solution {
    #[inline] fn try_push(grid: &mut [Vec<char>], fill: Deque, explore: Deque, i: usize, j: usize) -> char {
        if let Some(tile) = grid.get_mut(i).and_then( |row| row.get_mut(j) ) {
            let result = *tile;
            match tile {
                '0' => { explore.push_back((i, j)); *tile = '~'; },
                '1' => { fill.push_back((i, j)); *tile = '.'; },
                _ => (),
            }
            result
        } else { '\0' }
    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut c = 0;
        let fill = &mut VecDeque::new();
        let explore = &mut VecDeque::new();
        if Self::try_push(&mut grid, fill, explore, 0, 0) == '0' { c -= 1; }
        while !fill.is_empty() || !explore.is_empty() {
            c += 1;
            while let Some((i, j)) = fill.pop_front() {
                if j > 0 { Self::try_push(&mut grid, fill, explore, i, j - 1); }
                Self::try_push(&mut grid, fill, explore, i, j + 1);
                if i > 0 { Self::try_push(&mut grid, fill, explore, i - 1, j); }
                Self::try_push(&mut grid, fill, explore, i + 1, j);
            }
            while let Some((i, j)) = explore.pop_back() {
                if Self::try_push(&mut grid, fill, explore, i + 1, j) == '1' { break; }
                if i > 0 && Self::try_push(&mut grid, fill, explore, i - 1, j) == '1' { break; }
                if Self::try_push(&mut grid, fill, explore, i, j + 1) == '1' { break; }
                if j > 0 && Self::try_push(&mut grid, fill, explore, i, j - 1) == '1' { break; }
            }
        }
        c
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn str_to_grid(string: &str) -> Vec<Vec<char>> {
        string.split('\n').map( |line| line.chars().collect() ).collect()
    }

    /*
        Input: grid = [
        ["1","1","1","1","0"],
        ["1","1","0","1","0"],
        ["1","1","0","0","0"],
        ["0","0","0","0","0"]
        ]
        Output: 1
    */
    #[test]
    fn example1() {
        let grid = str_to_grid("\
            11110\n\
            11010\n\
            11000\n\
            00000");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","1","0","0","0"],
        ["1","1","0","0","0"],
        ["0","0","1","0","0"],
        ["0","0","0","1","1"]
        ]
        Output: 3
    */
    #[test]
    fn example2() {
        let grid = str_to_grid("\
            11000\n\
            11000\n\
            00100\n\
            00011");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 3);
    }

    /*
        Input: grid = [
        ["1","1","1","1","0"],
        ["1","1","0","1","0"],
        ["1","0","1","0","0"],
        ["0","0","0","0","0"]
        ]
        Output: 2
    */
    #[test]
    fn example3() {
        let grid = str_to_grid("\
            11110\n\
            11010\n\
            10100\n\
            00000");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 2);
    }

    /*
        Input: grid = [
        ["1","1","1"],
        ["0","1","0"],
        ["1","1","1"]
        ]
        Output: 1
    */
    #[test]
    fn example4() {
        let grid = str_to_grid("\
            111\n\
            010\n\
            111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","1","1"],
        ["1","0","1"],
        ["1","1","1"]
        ]
        Output: 1
    */
    #[test]
    fn example5() {
        let grid = str_to_grid("\
            111\n\
            101\n\
            111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","1","1","1","1","0","1","1","1","1"],
        ["0","1","1","0","1","1","1","0","1","1"],
        ["1","0","1","0","1","1","0","1","0","1"],
        ["1","0","1","1","0","1","1","1","1","1"],
        ["1","1","0","0","1","1","1","1","1","1"],
        ["1","1","0","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","0","1"],
        ["0","1","1","0","1","1","1","1","1","0"],
        ["1","1","0","1","1","0","1","1","1","1"],
        ["0","1","1","1","1","1","0","1","1","1"]
        ]
        Output: 1
    */
    #[test]
    fn example6() {
        let grid = str_to_grid("\
            1111101111\n\
            0110111011\n\
            1010110101\n\
            1011011111\n\
            1100111111\n\
            1101111111\n\
            1111111101\n\
            0110111110\n\
            1101101111\n\
            0111110111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["0","1","0"],
        ["1","0","1"],
        ["0","1","0"]
        ]
        Output: 1
    */
    #[test]
    fn example7() {
        let grid = str_to_grid("\
            010\n\
            101\n\
            010");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 4);
    }

    /*
        Input: grid = [
        ["1","1","1","1","1","0","1","1","1","1","1","1","1","1","1","0","1","0","1","1"],
        ["0","1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","1","0"],
        ["1","0","1","1","1","0","0","1","1","0","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","0","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","0","0","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","0","1","1","1","1","1","1","0","1","1","1","0","1","1","1","0","1","1","1"],
        ["0","1","1","1","1","1","1","1","1","1","1","1","0","1","1","0","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","0","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["0","1","1","1","1","1","1","1","0","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","0","1","1","1","1","1","1","1","0","1","1","1","1","1","1"],
        ["1","0","1","1","1","1","1","0","1","1","1","0","1","1","1","1","0","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","1","1","0"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","0","0"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"]
        ]
        Output: 1
        
    */
    #[test]
    fn example8() {
        let grid = str_to_grid("\
            11111011111111101011\n\
            01111111111110111110\n\
            10111001101111111111\n\
            11110111111111111111\n\
            10011111111111111111\n\
            10111111011101110111\n\
            01111111111101101111\n\
            11111111111101111011\n\
            11111111110111111111\n\
            11111111111111111111\n\
            01111111011111111111\n\
            11111111111111111111\n\
            11111111111111111111\n\
            11111011111110111111\n\
            10111110111011110111\n\
            11111111111101111110\n\
            11111111111110111100\n\
            11111111111111111111\n\
            11111111111111111111\n\
            11111111111111111111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }
}