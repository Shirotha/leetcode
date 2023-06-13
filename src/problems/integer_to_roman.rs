/*
    Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

    Symbol       Value
    I             1
    V             5
    X             10
    L             50
    C             100
    D             500
    M             1000
    For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

    Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9. 
    X can be placed before L (50) and C (100) to make 40 and 90. 
    C can be placed before D (500) and M (1000) to make 400 and 900.
    Given an integer, convert it to a roman numeral.

    Constraints:

    1 <= num <= 3999
 */

pub struct Solution;

impl Solution {
    fn digit_to_roman(s: &mut String, d: i32, counter: char, corrector: char) {
        if d == 0 { return; }
        if d == 4 { s.push(counter); s.push(corrector); return; }
        for _ in 0..d { s.push(counter); }
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut s = String::new();
        let d = num / 1000;
        Self::digit_to_roman(&mut s, d, 'M', '!');
        let num = num % 1000;
        let d = num / 100;
        if d >= 5 { 
            if d < 9 { s.push('D'); }
            Self::digit_to_roman(&mut s, d - 5, 'C', 'M');
        } else {
            Self::digit_to_roman(&mut s, d, 'C', 'D');
        }
        let num = num % 100;
        let d = num / 10;
        if d >= 5 { 
            if d < 9 { s.push('L'); }
            Self::digit_to_roman(&mut s, d - 5, 'X', 'C');
        } else {
            Self::digit_to_roman(&mut s, d, 'X', 'L');
        }
        let d = num % 10;
        if d >= 5 {
            if d < 9 { s.push('V'); }
            Self::digit_to_roman(&mut s, d - 5, 'I', 'X');
        } else {
            Self::digit_to_roman(&mut s, d, 'I', 'V');
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: num = 3
        Output: "III"
        Explanation: 3 is represented as 3 ones.
     */
    #[test]
    fn example1() {
        let num = 3;

        let s = Solution::int_to_roman(num);

        assert_eq!(s.as_str(), "III");
    }

    /*
        Input: num = 58
        Output: "LVIII"
        Explanation: L = 50, V = 5, III = 3.
     */
    #[test]
    fn example2() {
        let num = 58;

        let s = Solution::int_to_roman(num);

        assert_eq!(s.as_str(), "LVIII");
    }

    /*
        Input: num = 1994
        Output: "MCMXCIV"
        Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
     */
    #[test]
    fn example3() {
        let num = 1994;

        let s = Solution::int_to_roman(num);

        assert_eq!(s.as_str(), "MCMXCIV");
    }
}