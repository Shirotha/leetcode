/*
    You are given two integer arrays nums1 and nums2 sorted in non-decreasing order and an integer k.

    Define a pair (u, v) which consists of one element from the first array and one element from the second array.

    Return the k pairs (u1, v1), (u2, v2), ..., (uk, vk) with the smallest sums.

    Constraints:

    1 <= nums1.length, nums2.length <= 10^5
    -10^9 <= nums1[i], nums2[i] <= 10^9
    nums1 and nums2 both are sorted in non-decreasing order.
    1 <= k <= 10^4
 */

pub struct Solution;

use std::collections::{BinaryHeap, HashSet};

#[derive(Eq)]
struct Item(usize, usize, i32);
impl Ord for Item { fn cmp(&self, other: &Self) -> std::cmp::Ordering { other.2.cmp(&self.2) } }
impl PartialOrd for Item { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.2 == other.2 } }

#[inline(always)] fn hash(i: usize, j: usize) -> usize { i | (j << 16) }

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let (n, a, b) = (k as usize, nums1.len() - 1, nums2.len() - 1);
        let mut result = Vec::with_capacity(n);
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::with_capacity(n);
        heap.push(Item(0, 0, nums1[0] + nums2[0]));
        while let Some(Item(i, j, _)) = heap.pop() {
            result.push(vec![nums1[i], nums2[j]]); k -= 1;
            if k == 0 { break; }
            if i != a {
                let l = hash(i + 1, j);
                if !set.contains(&l) { set.insert(l); heap.push(Item(i + 1, j, nums1[i + 1] + nums2[j])); }
            }
            if j != b {
                let r = hash(i, j + 1);
                if !set.contains(&r) { set.insert(r); heap.push(Item(i, j + 1, nums1[i] + nums2[j + 1])); }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(mut ps:Vec<Vec<i32>>, xs: &[[i32; 2]]) {
        assert_eq!(ps.len(), xs.len());
        ps.sort_unstable_by_key( |ab| ab[0] + ab[1] );
        for (p, x) in ps.iter().zip(xs.iter()) {
            assert_eq!(p[0].min(p[1]), x[0]);
            assert_eq!(p[0].max(p[1]), x[1]);
        }
    }

    /*
        Input: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
        Output: [[1,2],[1,4],[1,6]]
        Explanation: The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
     */
    #[test]
    fn example1() {
        let nums1 = vec![1,7,11];
        let nums2 = vec![2,4,6];
        let k = 3;

        let p = Solution::k_smallest_pairs(nums1, nums2, k);

        judge(p, &[[1,2],[1,4],[1,6]]);
    }

    /*
        Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
        Output: [[1,1],[1,1]]
        Explanation: The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
     */
    #[test]
    fn example2() {
        let nums1 = vec![1,1,2];
        let nums2 = vec![1,2,3];
        let k = 2;

        let p = Solution::k_smallest_pairs(nums1, nums2, k);

        judge(p, &[[1,1],[1,1]]);
    }
    /*
        Input: nums1 = [1,2], nums2 = [3], k = 3
        Output: [[1,3],[2,3]]
        Explanation: All possible pairs are returned from the sequence: [1,3],[2,3]
     */

    #[test]
    fn example3() {
        let nums1 = vec![1,2];
        let nums2 = vec![3];
        let k = 3;

        let p = Solution::k_smallest_pairs(nums1, nums2, k);

        judge(p, &[[1,3],[2,3]]);
    }
}