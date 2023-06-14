/*
    The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

    P   A   H   N
    A P L S I I G
    Y   I   R
    And then read line by line: "PAHNAPLSIIGYIR"

    Write the code that will take a string and make this conversion given a number of rows:

    1 <= s.length <= 1000
    s consists of English letters (lower-case and upper-case), ',' and '.'.
    1 <= numRows <= 1000
 */

pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 { return s; }
        let num_rows = num_rows as usize;
        let mut result = String::with_capacity(s.len());
        let bytes = s.as_bytes();
        let stride = 2 * num_rows - 2;
        for r in 0..num_rows {
            for i in (0..bytes.len()).step_by(stride) {
                if let Some(&b) = bytes.get(i + r) {
                    result.push(b as char);
                }
                if 0 < r && r < num_rows - 1 { 
                    if let Some(&b) = bytes.get(i + stride - r) {
                        result.push(b as char);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "PAYPALISHIRING", numRows = 3
        Output: "PAHNAPLSIIGYIR"
     */
    #[test]
    fn example1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;

        let result = Solution::convert(s, num_rows);

        assert_eq!(result.as_str(), "PAHNAPLSIIGYIR");
    }

    /*
        Input: s = "PAYPALISHIRING", numRows = 4
        Output: "PINALSIGYAHRPI"
        Explanation:
        P     I    N
        A   L S  I G
        Y A   H R
        P     I
     */
    #[test]
    fn example2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;

        let result = Solution::convert(s, num_rows);

        assert_eq!(result.as_str(), "PINALSIGYAHRPI");
    }

    /*
        Input: s = "A", numRows = 1
        Output: "A"
     */
    #[test]
    fn example3() {
        let s = "A".to_string();
        let num_rows = 1;

        let result = Solution::convert(s, num_rows);

        assert_eq!(result.as_str(), "A");
    }
}