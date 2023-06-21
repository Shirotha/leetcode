/*
    Given an m x n matrix, return all elements of the matrix in spiral order.

    Constraints:

    m == matrix.length
    n == matrix[i].length
    1 <= m, n <= 10
    -100 <= matrix[i][j] <= 100
 */

pub struct Solution;

use std::cmp::Ordering::*;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let r = matrix.len();
        let c = matrix[0].len();
        let b = r.min(c)/2;
        let mut order = Vec::with_capacity(r*c);
        for i in 0..b {
            let j = i << 1 | 1;
            let top = matrix[i].iter()
                .skip(i).take(c-j);
            let right = matrix.iter()
                .skip(i).take(r-j)
                .map( |row| &row[c-i-1] );
            let bottom = matrix[r-i-1].iter().rev()
                .skip(i).take(c-j);
            let left = matrix.iter().rev()
                .skip(i).take(r-j)
                .map( |row| &row[i] );
            top.chain(right).chain(bottom).chain(left)
                .for_each( |x| order.push(*x) );
        }
        match r.cmp(&c) {
            Less if r % 2 == 1 => matrix[r/2].iter()
                .skip(b).take(c-(b<<1))
                .for_each( |x| order.push(*x) ),
            Greater if c % 2 == 1 => matrix.iter()
                .skip(b).take(r-(b<<1))
                .for_each( |x| order.push(x[b])),
            Equal if r % 2 == 1 => order.push(matrix[b][b]),
            _ => ()
        }
        order
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
        Output: [1,2,3,6,9,8,7,4,5]
     */
    #[test]
    fn example1() {
        let matrix = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9]
        ];

        let o = Solution::spiral_order(matrix);

        assert_eq!(o, vec![1,2,3,6,9,8,7,4,5]);
    }

    /*
        Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
        Output: [1,2,3,4,8,12,11,10,9,5,6,7]
     */
    #[test]
    fn example2() {
        let matrix = vec![
            vec![ 1, 2, 3, 4],
            vec![ 5, 6, 7, 8],
            vec![ 9,10,11,12]
        ];

        let o = Solution::spiral_order(matrix);

        assert_eq!(o, vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }
}