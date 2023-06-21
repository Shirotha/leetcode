/*
    You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).

    You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.

    Constraints:

    n == matrix.length == matrix[i].length
    1 <= n <= 20
    -1000 <= matrix[i][j] <= 1000
 */

pub struct Solution;

trait Swap2 {
    type Index;
    fn swap(self, i: Self::Index, j: Self::Index);
}
impl Swap2 for &mut Vec<Vec<i32>> {
    type Index = (usize, usize);
    #[inline]
    fn swap(self, i: Self::Index, j: Self::Index) {
        let tmp = self[i.0][i.1];
        self[i.0][i.1] = self[j.0][j.1];
        self[j.0][j.1] = tmp;
    }
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for l in 0..(n/2) {
            let r = n-l-1;
            for i in 0..(r-l) {
                let tmp = (l, l + i);
                matrix.swap(tmp, (l + i, r));
                matrix.swap(tmp, (r, r - i));
                matrix.swap(tmp, (r - i, l));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
        Output: [[7,4,1],[8,5,2],[9,6,3]]
     */
    #[test]
    fn example1() {
        let mut matrix = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];

        Solution::rotate(&mut matrix);

        let result = vec![
            vec![7,4,1],
            vec![8,5,2],
            vec![9,6,3]
        ];

        assert_eq!(matrix, result);
    }

    /*
        Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
        Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
     */
    #[test]
    fn example2() {
        let mut matrix = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16]
        ];

        Solution::rotate(&mut matrix);

        let result = vec![
            vec![15,13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7,10,11]
        ];

        assert_eq!(matrix, result);
    }
}