/*
    You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.

    Increment the large integer by one and return the resulting array of digits.

    Constraints:

    1 <= digits.length <= 100
    0 <= digits[i] <= 9
    digits does not contain any leading 0's.
 */

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = true;
        for d in digits.iter_mut().rev() {
            if d == &9 { *d = 0; carry = true; }
            else { *d += 1; carry = false; break; }
        }
        if carry { digits.insert(0, 1); }
        digits
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: digits = [1,2,3]
        Output: [1,2,4]
        Explanation: The array represents the integer 123.
        Incrementing by one gives 123 + 1 = 124.
        Thus, the result should be [1,2,4].
     */
    #[test]
    fn example1() {
        let digits = vec![1,2,3];

        let inc = Solution::plus_one(digits);

        assert_eq!(inc, vec![1,2,4]);
    }

    /*
        Input: digits = [4,3,2,1]
        Output: [4,3,2,2]
        Explanation: The array represents the integer 4321.
        Incrementing by one gives 4321 + 1 = 4322.
        Thus, the result should be [4,3,2,2].
     */
    #[test]
    fn example2() {
        let digits = vec![4,3,2,1];

        let inc = Solution::plus_one(digits);

        assert_eq!(inc, vec![4,3,2,2]);
    }

    /*
        Input: digits = [9]
        Output: [1,0]
        Explanation: The array represents the integer 9.
        Incrementing by one gives 9 + 1 = 10.
        Thus, the result should be [1,0].
     */
    #[test]
    fn example3() {
        let digits = vec![9];

        let inc = Solution::plus_one(digits);

        assert_eq!(inc, vec![1,0]);
    }
}