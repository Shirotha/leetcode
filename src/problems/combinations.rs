/*
    Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].

    You may return the answer in any order.

    Constraints:

    1 <= n <= 20
    1 <= k <= n
 */

pub struct Solution;

struct CombinationsIter {
    max: i32,
    state: Vec<i32>,
}
impl CombinationsIter {
    fn new(max: i32, dims: i32) -> Self {
        let mut state: Vec<i32> = ((max - dims + 1)..=max).rev().collect();
        state[dims as usize - 1] = 0;
        CombinationsIter { max, state }
    }
}
impl<'a> Iterator for &'a mut CombinationsIter {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        let mut imax = self.max;
        let len = self.state.len();
        while self.state[i] == imax {
            i += 1; imax -= 1;
            if i == len { return None; }
        }
        self.state[i] += 1;
        for j in (0..i).rev() {
            self.state[j] = self.state[j + 1] + 1;
        }
        Some(self.state.clone())
    }
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        CombinationsIter::new(n, k).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(cs: Vec<Vec<i32>>, result: &[&[i32]]) {
        assert_eq!(cs.len(), result.len());
        for mut c in cs {
            let f = result.contains(&c.as_slice());
            if !f {
                c.reverse();
                assert!(result.contains(&c.as_slice()));
            }
        }
    }

    /*
        Input: n = 4, k = 2
        Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
        Explanation: There are 4 choose 2 = 6 total combinations.
        Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
     */
    #[test]
    fn example1() {
        let n = 4;
        let k = 2;

        let cs = Solution::combine(n, k);

        judge(cs, &[&[1,2],&[1,3],&[1,4],&[2,3],&[2,4],&[3,4]]);
    }

    /*
        Input: n = 1, k = 1
        Output: [[1]]
        Explanation: There is 1 choose 1 = 1 total combination.
     */
    #[test]
    fn example2() {
        let n = 1;
        let k = 1;

        let cs = Solution::combine(n, k);

        assert_eq!(cs.len(), 1);
        assert_eq!(cs[0], [1]);
    }

    #[test]
    fn example3() {
        let n = 4;
        let k = 3;

        let cs = Solution::combine(n, k);

        judge(cs, &[&[1,2,3],&[1,2,4],&[1,3,4],&[2,3,4]]);
    }
}