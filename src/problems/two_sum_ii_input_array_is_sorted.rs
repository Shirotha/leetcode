/*
    Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 < numbers.length.

    Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

    The tests are generated such that there is exactly one solution. You may not use the same element twice.

    Your solution must use only constant extra space.

    Constraints:

    2 <= numbers.length <= 3 * 10^4
    -1000 <= numbers[i] <= 1000
    numbers is sorted in non-decreasing order.
    -1000 <= target <= 1000
    The tests are generated such that there is exactly one solution.
 */

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &n) in numbers.iter().enumerate() {
            let m = target - n;
            if n == m { return vec![i as i32 + 1, i as i32 + 2]; }
            if let Ok(j) = numbers[(i + 1)..].binary_search(&m) {
                return vec![i as i32 + 1, (i + j + 2) as i32];
            }
        }
        panic!("no solution!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: numbers = [2,7,11,15], target = 9
        Output: [1,2]
        Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
     */
    #[test]
    fn example1() {
        let numbers = vec![2,7,11,15];
        let target = 9;

        let ij = Solution::two_sum(numbers, target);

        assert_eq!(ij, vec![1,2]);
    }

    /*
        Input: numbers = [2,3,4], target = 6
        Output: [1,3]
        Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
     */
    #[test]
    fn example2() {
        let numbers = vec![2,3,4];
        let target = 6;

        let ij = Solution::two_sum(numbers, target);

        assert_eq!(ij, vec![1,3]);
    }

    /*
        Input: numbers = [-1,0], target = -1
        Output: [1,2]
        Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
     */
    #[test]
    fn example3() {
        let numbers = vec![-1,0];
        let target = -1;

        let ij = Solution::two_sum(numbers, target);

        assert_eq!(ij, vec![1,2]);
    }
}