/*
    Given a triangle array, return the minimum path sum from top to bottom.

    For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

    Constraints:

    1 <= triangle.length <= 200
    triangle[0].length == 1
    triangle[i].length == triangle[i - 1].length + 1
    -10^4 <= triangle[i][j] <= 10^4
 */

pub struct Solution;

#[inline] fn split(triangle: &mut [Vec<i32>], row: usize) -> (&[i32], &mut [i32]) {
    let (a, b) = triangle.split_at_mut(row);
    (a.last().unwrap(), b.first_mut().unwrap())
}

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let rows = triangle.len();
        for row in 1..rows {
            let (parent, current) = split(&mut triangle, row);
            current[0] += parent[0];
            for i in 1..row {
                let (a, b) = (parent[i - 1], parent[i]);
                current[i] += a.min(b);
            }
            current[row] += parent[row - 1];
        }
        *triangle[rows - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_triangle(mut a: &[i32]) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut i = 0;
        while i < a.len() {
            let (l, r) = a.split_at(i + 1);
            result.push(l.to_vec());
            a = r; i += 1;
        }
        result
    }

    /*
        Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
        Output: 11
        Explanation: The triangle looks like:
           2
          3 4
         6 5 7
        4 1 8 3
        The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
     */
    #[test]
    fn example1() {
        let triangle = create_triangle(&[2,3,4,6,5,7,4,1,8,3]);

        let s = Solution::minimum_total(triangle);

        assert_eq!(s, 11);
    }

    /*
        Input: triangle = [[-10]]
        Output: -10
     */
    #[test]
    fn example2() {
        let triangle = create_triangle(&[-10]);

        let s = Solution::minimum_total(triangle);

        assert_eq!(s, -10);
    }
}