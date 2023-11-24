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

use std::mem::{swap, ManuallyDrop};
use std::ptr::{read, copy_nonoverlapping};

#[inline(always)] fn parent(i: usize) -> usize { (i - 1) >> 1 }
#[inline(always)] fn grandparent(i: usize) -> usize { parent(parent(i)) }
#[inline(always)] fn child1(i: usize) -> usize { (i << 1) + 1 }
#[inline(always)] fn child2(i: usize) -> usize { (i << 1) + 2 }
#[inline(always)] fn grandchild1(i: usize) -> usize { child1(child1(i)) }
#[inline(always)] fn grandchild2(i: usize) -> usize { child2(child1(i)) }
#[inline(always)] fn grandchild3(i: usize) -> usize { child1(child2(i)) }
#[inline(always)] fn grandchild4(i: usize) -> usize { child2(child2(i)) }
#[inline(always)] fn has_parent(i: usize) -> bool { i > 0 }
#[inline(always)] fn has_grandparent(i: usize) -> bool { i > 2 }
#[inline(always)] fn is_min_level(i: usize) -> bool { (i + 1).leading_zeros() & 1 == 1 }

enum Generation { Same, Parent, Grandparent }
use Generation::*;

struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    elt: ManuallyDrop<T>,
    pos: usize,
}
impl<'a, T: PartialOrd> Hole<'a, T> {
    #[inline] fn new(data: &'a mut [T], pos: usize) -> Self {
        let elt = unsafe { read(&data[pos]) };
        Hole { data, elt: ManuallyDrop::new(elt), pos }
    }
    #[inline] fn move_to(&mut self, index: usize) {
        let index_ptr = &self.data[index] as *const _;
        let hole_ptr = &mut self.data[self.pos];
        unsafe { copy_nonoverlapping(index_ptr, hole_ptr, 1); }
        self.pos = index;
    }
    #[inline] fn swap_with_parent(&mut self) {
        swap(&mut self.data[parent(self.pos)], &mut self.elt);
    }
    #[inline] fn index_of_best_child_or_grandchild<F>(&self, len: usize, f: F) -> (usize, Generation)
        where F: Fn(&T, &T) -> bool
    {
        let p = self.pos;
        let (mut pos, mut depth, mut element) = (p, Same, &*self.elt);
        let mut check = |i, gen| {
            if i >= len { return false; }
            if f(&self.data[i], element) { pos = i; depth = gen; element = &self.data[i]; }
            true
        };
        let _ = check(child1(p), Parent)
             && check(child2(p), Parent)
             && check(grandchild1(p), Grandparent)
             && check(grandchild2(p), Grandparent)
             && check(grandchild3(p), Grandparent)
             && check(grandchild4(p), Grandparent);
        (pos, depth)
    }
    #[inline] fn index_of_largest_child_or_grandchild(&self, len: usize) -> (usize, Generation) {
        self.index_of_best_child_or_grandchild(len, |a, b| a > b)
    }
    #[inline] fn index_of_smallest_child_or_grandchild(&self, len: usize) -> (usize, Generation) {
        self.index_of_best_child_or_grandchild(len, |a, b| a < b)
    }
    #[inline] fn bubble_up(&mut self) {
        let p = self.pos;
        if is_min_level(p) {
            if has_parent(p) && *self.elt > self.data[parent(p)] {
                self.move_to(parent(p));
                self.bubble_up_max();
            } else { self.bubble_up_min(); }
        } else if has_parent(p) && *self.elt < self.data[parent(p)] {
            self.move_to(parent(p));
            self.bubble_up_min();
        } else { self.bubble_up_max(); }
    }
    #[inline] fn bubble_up_min(&mut self) {
        while has_grandparent(self.pos) && *self.elt < self.data[grandparent(self.pos)] {
            self.move_to(grandparent(self.pos));
        }
    }
    #[inline] fn bubble_up_max(&mut self) {
        while has_grandparent(self.pos) && *self.elt > self.data[grandparent(self.pos)] {
            self.move_to(grandparent(self.pos));
        }
    }
    #[inline] fn trickle_down_min(&mut self) {
        let len = self.data.len();
        loop {
            let (m, gen) = self.index_of_smallest_child_or_grandchild(len);
            match gen {
                Grandparent => {
                    self.move_to(m);
                    if *self.elt > self.data[parent(self.pos)] { self.swap_with_parent(); }
                },
                Parent => { self.move_to(m); return; },
                Same => return,
            }
        }
    }
    #[inline] fn trickle_down_max(&mut self) {
        let len = self.data.len();
        loop {
            let (m, gen) = self.index_of_largest_child_or_grandchild(len);
            match gen {
                Grandparent => {
                    self.move_to(m);
                    if *self.elt < self.data[parent(self.pos)] { self.swap_with_parent(); }
                },
                Parent => { self.move_to(m); return; },
                Same => return,
            }
        }
    }
}
impl<'a, T> Drop for Hole<'a, T> {
    fn drop(&mut self) { unsafe { copy_nonoverlapping(&*self.elt, &mut self.data[self.pos], 1); } }
}

struct MinMaxHeap<T>(Vec<T>);
impl<T: PartialOrd> MinMaxHeap<T> {
    #[inline] fn with_capacity(capacity: usize) -> Self { MinMaxHeap(Vec::with_capacity(capacity)) }
    #[inline] fn len(&self) -> usize { self.0.len() }
    #[inline] fn peek_min(&self) -> Option<&T> { self.0.first() }
    #[inline] fn pop_max(&mut self) -> Option<T> {
        let i = match self.0.len() {
            0 => return None, 1 => 0, 2 => 1,
            _ => if self.0[1] > self.0[2] { 1 } else { 2 }
        };
        let mut x = self.0.pop().unwrap();
        if i < self.0.len() {
            swap(&mut x, &mut self.0[i]);
            Hole::new(&mut self.0, i).trickle_down_max();
        }
        Some(x)
    }
    #[inline] fn replace_min(&mut self, mut value: T) -> Option<T> {
        if self.0.is_empty() {
            self.insert(value);
            return None;
        }
        swap(&mut value, &mut self.0[0]);
        Hole::new(&mut self.0, 0).trickle_down_min();
        Some(value)
    }
    #[inline] fn insert(&mut self, value: T) {
        let i = self.0.len();
        self.0.push(value);
        Hole::new(&mut self.0, i).bubble_up();
    }
}
impl<T> IntoIterator for MinMaxHeap<T> {
    type Item = T; type IntoIter = std::vec::IntoIter<Self::Item>;
    #[inline] fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}
#[derive(Eq)]
struct Item(i32, i32, i32);
impl Item { fn into_vec(self) -> Vec<i32> { vec![self.0, self.1] } }
impl Ord for Item { fn cmp(&self, other: &Self) -> std::cmp::Ordering { other.2.cmp(&self.2) } }
impl PartialOrd for Item { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) } }
impl PartialEq for Item { fn eq(&self, other: &Self) -> bool { self.2 == other.2 } }

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap: MinMaxHeap<Item> = MinMaxHeap::with_capacity(k);
        for a in nums1 {
            if heap.len() == k && heap.peek_min().unwrap().2 < a + nums2[0] { break; }
            for &b in &nums2 {
                let s = a + b;
                if heap.len() < k { heap.insert(Item(a, b, s)); }
                else if heap.peek_min().unwrap().2 > s { heap.replace_min(Item(a, b, s)); }
            } 
        }
        heap.into_iter().map(Item::into_vec).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(mut ps:Vec<Vec<i32>>, xs: &[[i32; 2]]) {
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