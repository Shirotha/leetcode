/*
    You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

    Find two lines that together with the x-axis form a container, such that the container contains the most water.

    Return the maximum amount of water a container can store.

    Notice that you may not slant the container.

    Constraints:

    n == height.length
    2 <= n <= 10^5
    0 <= height[i] <= 10^4
 */

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max = 0;
        while i < j {
            let hi = height[i];
            let hj = height[j];
            let a = if hi <= hj {
                i += 1; hi
            } else {
                j -= 1; hj
            } * ((j + 1 - i) as i32);
            if a > max { max = a; }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: height = [1,8,6,2,5,4,8,3,7]
        Output: 49
        Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
     */
    #[test]
    fn example1() {
        let height = vec![1,8,6,2,5,4,8,3,7];

        let a = Solution::max_area(height);

        assert_eq!(a, 49);
    }

    /*
        Input: height = [1,1]
        Output: 1
     */
    #[test]
    fn example2() {
        let height = vec![1,1];

        let a = Solution::max_area(height);

        assert_eq!(a, 1);
    }
}