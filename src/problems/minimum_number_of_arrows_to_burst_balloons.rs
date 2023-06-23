/*
    There are some spherical balloons taped onto a flat wall that represents the XY-plane. The balloons are represented as a 2D integer array points where points[i] = [x_start, x_end] denotes a balloon whose horizontal diameter stretches between xstart and xend. You do not know the exact y-coordinates of the balloons.

    Arrows can be shot up directly vertically (in the positive y-direction) from different points along the x-axis. A balloon with x_start and x_end is burst by an arrow shot at x if x_start <= x <= x_end. There is no limit to the number of arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any balloons in its path.

    Given the array points, return the minimum number of arrows that must be shot to burst all balloons.

    Constraints:

    1 <= points.length <= 10^5
    points[i].length == 2
    -2^31 <= x_start < x_end <= 2^31 - 1
 */

pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by( |a, b| a[0].cmp(&b[0]) );
        let mut n = 1;
        let mut end = points[0][1];
        for p in points.into_iter().skip(1) {
            if p[0] > end {
                n += 1;
                end = p[1];
            } else if p[1] < end {
                end = p[1];
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: points = [[10,16],[2,8],[1,6],[7,12]]
        Output: 2
        Explanation: The balloons can be burst by 2 arrows:
        - Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6].
        - Shoot an arrow at x = 11, bursting the balloons [10,16] and [7,12].
     */
    #[test]
    fn example1() {
        let points = vec![
            vec![10,16],
            vec![2, 8],
            vec![1, 6],
            vec![7, 12]
        ];

        let n = Solution::find_min_arrow_shots(points);

        assert_eq!(n, 2);
    }

    /*
        Input: points = [[1,2],[3,4],[5,6],[7,8]]
        Output: 4
        Explanation: One arrow needs to be shot for each balloon for a total of 4 arrows.
     */
    #[test]
    fn example2() {
        let points = vec![
            vec![1,2],
            vec![3,4],
            vec![5,6],
            vec![7,8]
        ];

        let n = Solution::find_min_arrow_shots(points);

        assert_eq!(n, 4);
    }

    /*
        Input: points = [[1,2],[2,3],[3,4],[4,5]]
        Output: 2
        Explanation: The balloons can be burst by 2 arrows:
        - Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3].
        - Shoot an arrow at x = 4, bursting the balloons [3,4] and [4,5].
     */
    #[test]
    fn example3() {
        let points = vec![
            vec![1,2],
            vec![2,3],
            vec![3,4],
            vec![4,5]
        ];

        let n = Solution::find_min_arrow_shots(points);

        assert_eq!(n, 2);
    }

    /*
        Input: points = [[-2147483648,2147483647]]
        Output: 1
     */
    #[test]
    fn example4() {
        let points = vec![
            vec![-2147483648,2147483647]
        ];

        let n = Solution::find_min_arrow_shots(points);

        assert_eq!(n, 1);
    }

    /*
        Input: points = [[3,9],[7,12],[3,8],[6,8],[9,10],[2,9],[0,9],[3,9],[0,6],[2,8]]
        Output: 2
     */
    #[test]
    fn example5() {
        let points = vec![
            vec![3,9],
            vec![7,12],
            vec![3,8],
            vec![6,8],
            vec![9,10],
            vec![2,9],
            vec![0,9],
            vec![3,9],
            vec![0,6],
            vec![2,8]
        ];

        let n = Solution::find_min_arrow_shots(points);

        assert_eq!(n, 2);
    }

    /*
        Input: points = [[9,12],[1,10],[4,11],[8,12],[3,9],[6,9],[6,7]]
        Output: 2
     */
    #[test]
    fn example6() {
        let points = vec![
            vec![9,12],
            vec![1,10],
            vec![4,11],
            vec![8,12],
            vec![3,9],
            vec![6,9],
            vec![6,7]
        ];

        let n = Solution::find_min_arrow_shots(points);

        assert_eq!(n, 2);
    }
}