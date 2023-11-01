/*
    Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

    Constraints:

    1 <= n <= 8
 */

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 { return vec!["()".to_string()]; }
        let n = n as usize;
        let l = n << 1;
        let mut result = Vec::new();
        result.push(String::with_capacity(l));
        let mut stack = vec![(0, 0, 0, 0); l];
        stack.push((0, 0, 0, 0u8));
        let mut i = 0;
        loop {
            let (j, o, c, b) = stack[i];
            if b & 1 == 0 && o > c {
                let mut s = result[j].clone();
                s.push(')');
                result.push(s);
                stack[i].3 |= 1;
                i += 1;
                stack[i] = (result.len() - 1, o, c + 1, 0);
                continue;
            }
            if b & 2 == 0 {
                let s = &mut result[j];
                s.push('(');
                if o + 1 < n {
                    stack[i].3 |= 2;
                    i += 1;
                    stack[i] = (j, o + 1, c, 0);
                    continue;
                }
                for _ in c..n { s.push(')'); }
            }
            if i == 0 { return result; }
            i -= 1;
        }
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