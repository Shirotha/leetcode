/*
Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

Constraints:

1 <= nums.length <= 6
-10 <= nums[i] <= 10
All the integers of nums are unique.
 */

pub struct Solution;

struct PermutationsIter {
    data: Vec<i32>,
    state: Vec<u8>,
}
impl PermutationsIter {
    fn new(data: Vec<i32>) -> Self {
        let state = vec![0; data.len()];
        PermutationsIter { data, state }
    }
}
impl Iterator for PermutationsIter {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state[0] == 0 {
            self.state[0] = 1;
            return Some(self.data.clone());
        }
        let mut i = 1;
        while i < self.state.len() {
            if (self.state[i] as usize) < i {
                if i % 2 == 0 {
                    self.data.swap(0, i);
                } else {
                    self.data.swap(self.state[i] as usize, i);
                }
                self.state[i] += 1;
                return Some(self.data.clone());
            } else {
                self.state[i] = 0;
                i += 1;
            }
        }
        None
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        PermutationsIter::new(nums).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(ps: Vec<Vec<i32>>, result: &[&[i32]]) {
        assert_eq!(ps.len(), result.len());
        for p in ps {
            assert!(result.contains(&p.as_slice()));
        }
    }

    /*
        Input: nums = [1,2,3]
        Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
     */
    #[test]
    fn example1() {
        let nums = vec![1,2,3];

        let ps = Solution::permute(nums);

        judge(ps, &[&[1,2,3],&[1,3,2],&[2,1,3],&[2,3,1],&[3,1,2],&[3,2,1]])
    }

    /*
        Input: nums = [0,1]
        Output: [[0,1],[1,0]]
     */
    #[test]
    fn example2() {
        let nums = vec![0,1];

        let ps = Solution::permute(nums);

        judge(ps, &[&[0,1],&[1,0]])
    }

    /*
        Input: nums = [1]
        Output: [[1]]
     */
    #[test]
    fn example3() {
        let nums = vec![1];

        let ps = Solution::permute(nums);

        judge(ps, &[&[1]])
    }
}