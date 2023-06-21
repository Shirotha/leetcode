/*
    Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

    You must do it in place.

    Constraints:

    m == matrix.length
    n == matrix[0].length
    1 <= m, n <= 200
    -2^31 <= matrix[i][j] <= 2^31 - 1
 */

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let r = matrix.len();
        let c = matrix[0].len();
        let mut key = (r, c);
        for i in 0..r { for j in 0..c {
            if i == key.0 || j == key.1 || matrix[i][j] != 0 { continue; }
            if key.0 == r { 
                key = (i, j);
                matrix[i][j] = 1;
            } else {
                matrix[i][key.1] = 0;
                matrix[key.0][j] = 0;
            }
        } }
        if key.0 == r { return; }
        for row in matrix.iter_mut() {
            if row[key.1] == 0 { row.fill(0); }
        }
        for j in 0..c {
            if matrix[key.0][j] == 0 {
                matrix.iter_mut().for_each( |row| row[j] = 0 );
            }
        }
        matrix[key.0].fill(0);
        matrix.iter_mut().for_each( |row| row[key.1] = 0 );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
        Output: [[1,0,1],[0,0,0],[1,0,1]]
     */
    #[test]
    fn example1() {
        let mut matrix = vec![
            vec![1,1,1],
            vec![1,0,1],
            vec![1,1,1]
        ];

        Solution::set_zeroes(&mut matrix);

        let result = vec![
            [1,0,1],
            [0,0,0],
            [1,0,1]
        ];

        assert_eq!(matrix, result);
    }

    /*
        Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
        Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
     */
    #[test]
    fn example2() {
        let mut matrix = vec![
            vec![0,1,2,0],
            vec![3,4,5,2],
            vec![1,3,1,5]
        ];

        Solution::set_zeroes(&mut matrix);

        let result = vec![
            vec![0,0,0,0],
            vec![0,4,5,0],
            vec![0,3,1,0]
        ];

        assert_eq!(matrix, result);
    }
}