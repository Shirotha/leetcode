/*
    Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

    Constraints:

    n == height.length
    1 <= n <= 2 * 10^4
    0 <= height[i] <= 10^5
 */

pub struct Solution;

impl Solution {
    fn pass<'a, I : Iterator<Item = &'a i32>>(iter: I) -> (i32, usize) {
        let mut total = 0;
        let mut current = 0;
        let mut peak_i = 0;
        let mut peak_h = 0;
        for (i, &h) in iter.enumerate() {
            let diff = peak_h - h;
            if diff.is_positive() {
                current += diff;
            } else {
                total += current;
                current = 0;
                peak_i = i;
                peak_h = h;
            }
        }
        (total, peak_i)
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let (left, i) = Self::pass(height.iter());
        let (right, _) = Self::pass(height.iter().rev().take(height.len() - i));
        left + right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
        Output: 6
        Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
     */
    #[test]
    fn example1() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];

        let w = Solution::trap(height);

        assert_eq!(w, 6);
    }

    /*
        Input: height = [4,2,0,3,2,5]
        Output: 9
     */
    #[test]
    fn example2() {
        let height = vec![4,2,0,3,2,5];

        let w = Solution::trap(height);

        assert_eq!(w, 9);
    }

    /*
        Input: height = [2,0,2]
        Output: 2
     */
    #[test]
    fn example3() {
        let height = vec![2,0,2];

        let w = Solution::trap(height);

        assert_eq!(w, 2);
    }
}