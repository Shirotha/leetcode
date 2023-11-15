/*
    Given an array of points where points[i] = [x_i, y_i] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.

    Constraints:

    1 <= points.length <= 300
    points[i].length == 2
    -10^4 <= x_i, y_i <= 10^4
    All the points are unique.
 */

pub struct Solution;

use std::collections::{HashMap, hash_map::Entry::*};

#[inline] fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let tmp = a;
        a = b; b = tmp % b;
    }
    a
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 { return points.len() as i32; }
        let mut max = 0;
        let mut lines = HashMap::new();
        for p in points.iter() {
            for q in points.iter() {
                if p == q { continue; }
                let (a, b) = (p[1] - q[1], p[0] - q[0]);
                let x = gcd(a.unsigned_abs(), b.unsigned_abs()) as i32;
                match lines.entry((a/x, b/x)) {
                    Occupied(mut e) => *e.get_mut() += 1,
                    Vacant(e) => { e.insert(2); },
                }
            }
            max = max.max(*lines.values().max().unwrap());
            lines.clear();
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: points = [[1,1],[2,2],[3,3]]
        Output: 3
     */
    #[test]
    fn example1() {
        let points = vec![
            vec![1,1],
            vec![2,2],
            vec![3,3]];

        let n = Solution::max_points(points);

        assert_eq!(n, 3);
    }

    /*
        Input: points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]
        Output: 4
     */
    #[test]
    fn example2() {
        let points = vec![
            vec![1,1],
            vec![3,2],
            vec![5,3],
            vec![4,1],
            vec![2,3],
            vec![1,4]];

        let n = Solution::max_points(points);

        assert_eq!(n, 4);
    }
}