/*
Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.

The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

Constraints:

1 <= candidates.length <= 30
2 <= candidates[i] <= 40
All elements of candidates are distinct.
1 <= target <= 40
 */

pub struct Solution;

use std::collections::VecDeque;

enum PeekAction<T> {
    Push(T),
    Next,
    Pop,
}
use PeekAction::*;
struct PeekResult<T, R>(PeekAction<T>, Option<R>);
impl<T, R> PeekResult<T, R> {
    fn found(action: PeekAction<T>, result: R) -> Self {
        PeekResult(action, Some(result))
    }
    fn skip(action: PeekAction<T>) -> Self {
        PeekResult(action, None)
    }
}
struct PathIter<T> {
    stack: VecDeque<T>,
}
impl<T> PathIter<T> {
    fn new(root: T) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(root);
        PathIter { stack }
    }
    fn forward<P, R>(&mut self, peek: P) -> Result<Option<R>,()> where 
        P: FnOnce(&mut T) -> PeekResult<T, R>
    {
        if let Some(state) = self.stack.back_mut() {
            let PeekResult(action, result) = peek(state);
            match action {
                Push(next) => self.stack.push_back(next),
                Next => (),
                Pop => { self.stack.pop_back(); },
            }
            Ok(result)
        } else { Err(()) }
    }
}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = candidates.len() as u8;
        let mut result = Vec::new();
        let mut builder = Vec::new();
        let mut sum = 0;
        let mut iter = PathIter::new((0u8, 0u8));
        while let Ok(path) = iter.forward( |(index, count)| {
            if *index >= len { return PeekResult::skip(Pop); }
            let num = candidates[*index as usize];
            if count != &0 {
                *count -= 1;
                sum -= num;
                builder.pop();
                return if count == &0 {
                    *index += 1;
                    PeekResult::skip(Next)
                } else { PeekResult::skip(Push((*index + 1, 0))) }
            }
            let n = (target - sum) / num;
            if n == 0 {
                *index += 1;
                return PeekResult::skip(Next);
            }
            for _ in 0..n { builder.push(num); }
            *count += n as u8;
            sum += num * n;
            if sum == target { PeekResult::found(Next, ()) } 
            else { PeekResult::skip(Push((*index + 1, 0))) }
        } ) {
            if path.is_some() { result.push(builder.clone()); }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(cs: Vec<Vec<i32>>, result: &[&[i32]]) {
        assert_eq!(cs.len(), result.len());
        for mut c in cs {
            c.sort_unstable();
            assert!(result.contains(&c.as_slice()));
        }
    }

    /*
        Input: candidates = [2,3,6,7], target = 7
        Output: [[2,2,3],[7]]
        Explanation:
        2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
        7 is a candidate, and 7 = 7.
        These are the only two combinations.
     */
    #[test]
    fn example1() {
        let candidates = vec![2,3,6,7];
        let target = 7;

        let cs = Solution::combination_sum(candidates, target);

        judge(cs, &[&[2,2,3],&[7]]);
    }

    /*
        Input: candidates = [2,3,5], target = 8
        Output: [[2,2,2,2],[2,3,3],[3,5]]
     */
    #[test]
    fn example2() {
        let candidates = vec![2,3,5];
        let target = 8;

        let cs = Solution::combination_sum(candidates, target);

        judge(cs, &[&[2,2,2,2],&[2,3,3],&[3,5]]);
    }

    /*
        Input: candidates = [2], target = 1
        Output: []
     */
    #[test]
    fn example3() {
        let candidates = vec![2];
        let target = 1;

        let cs = Solution::combination_sum(candidates, target);

        judge(cs, &[]);
    }
}