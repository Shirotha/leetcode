#![allow(clippy::needless_range_loop)]
/*
    Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.

    Constraints:

    m == matrix.length
    n == matrix[i].length
    1 <= m, n <= 300
    matrix[i][j] is '0' or '1'.
 */

pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        #[inline] fn to_int(chr: char) -> u16 { (chr as u8 - b'0') as u16 }
        let (w, h) = (matrix[0].len(), matrix.len());
        let mut raw = vec![0; w << 1];
        let (prev, curr) = raw.split_at_mut(w);
        let mut best = 0;
        for (&chr, side) in matrix[0].iter().zip(curr.iter_mut()) {
            *side = to_int(chr);
            if side == &1 { best = 1; }
        }
        prev.copy_from_slice(curr);
        for i in 1..h {
            let sq = to_int(matrix[i][0]);
            curr[0] = sq;
            if sq > best { best = sq; }
            for j in 1..w {
                if matrix[i][j] == '0' { curr[j] = 0; continue; }
                let sq = 1 + curr[j - 1].min(prev[j]).min(prev[j - 1]);
                curr[j] = sq;
                if sq > best { best = sq; }
            }
            prev.copy_from_slice(curr);
        }
        let best = best as i32;
        best * best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_matrix(s: &str, w: usize) -> Vec<Vec<char>> {
        let s = s.as_bytes();
        s.chunks_exact(w)
            .map( |row| row.iter().map( |&b| b as char ).collect() )
            .collect()
    }

    /*
        Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
        Output: 4
     */
    #[test]
    fn example1() {
        let matrix = create_matrix("10100101111111110010", 5);

        let s = Solution::maximal_square(matrix);

        assert_eq!(s, 4);
    }

    /*
        Input: matrix = [["0","1"],["1","0"]]
        Output: 1
     */
    #[test]
    fn example2() {
        let matrix = create_matrix("0110", 2);

        let s = Solution::maximal_square(matrix);

        assert_eq!(s, 1);
    }

    /*
        Input: matrix = [["0"]]
        Output: 0
     */
    #[test]
    fn example3() {
        let matrix = create_matrix("0", 1);

        let s = Solution::maximal_square(matrix);

        assert_eq!(s, 0);
    }
}