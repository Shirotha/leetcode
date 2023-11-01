/*
    Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

    Constraints:

    1 <= n <= 8
 */

pub struct Solution;

use std::str::from_utf8_unchecked;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 { return vec!["()".into()]; }
        let n = n as usize;
        let l = n << 1;
        let mut m = 1;
        let mut str = "()".repeat((3usize.pow(n as u32) + 3) * l / 12);
        let a = unsafe { str.as_bytes_mut() };
        for i in 1..n {
            let s = i << 1;
            let o0 = m * l;
            for o in (0..o0).step_by(l) {
                a.copy_within(o..(o + s), o0 + o + 1);
            }
            if i != 1 {
                a.copy_within(l..(o0 - 2), (o0 + 1) << 1);
            }
            m += (m << 1) - 1;
        }
        a.chunks(l).map( |v| {
            unsafe { from_utf8_unchecked(v) }.to_owned()
        } ).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(r: &[String], s: &[&str]) {
        assert_eq!(r.len(), s.len());
        for x in s.iter() {
            assert!(r.contains(&(*x).into()));
        }
    }

    /*
        Input: n = 3
        Output: ["((()))","(()())","(())()","()(())","()()()"]
     */
    #[test]
    fn example1() {
        let n = 3;

        let r = Solution::generate_parenthesis(n);

        judge(&r, &["((()))","(()())","(())()","()(())","()()()"]);
    }

    /*
        Input: n = 1
        Output: ["()"]
     */
    #[test]
    fn example2() {
        let n = 1;

        let r = Solution::generate_parenthesis(n);

        judge(&r, &["()"]);
    }

    #[test]
    fn example3() {
        let n = 4;

        let r = Solution::generate_parenthesis(n);
        // FIXME: case (())(()) is missing, but ()(())() is occuring twice
        judge(&r, &["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"]);
    }
}