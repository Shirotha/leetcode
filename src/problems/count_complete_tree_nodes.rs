/*
    Given the root of a complete binary tree, return the number of the nodes in the tree.

    According to Wikipedia, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2h nodes inclusive at the last level h.

    Design an algorithm that runs in less than O(n) time complexity.

    Constraints:

    The number of nodes in the tree is in the range [0, 5 * 10^4].
    0 <= Node.val <= 5 * 10^4
    The tree is guaranteed to be complete.
 */

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub struct Solution;

use std::{
    rc::Rc,
    cell::RefCell,
};

impl Solution {
    fn reverse_bits(v: u32, n: i32) -> u32 {
        let v = ((v >> 1) & 0x55555555) | ((v & 0x55555555) << 1);
        let v = ((v >> 2) & 0x33333333) | ((v & 0x33333333) << 2);
        let v = ((v >> 4) & 0x0F0F0F0F) | ((v & 0x0F0F0F0F) << 4);
        let v = ((v >> 8) & 0x00FF00FF) | ((v & 0x00FF00FF) << 8);
        let v = ( v >> 16             ) | ( v               << 16);
        v >> (32 - n)
    }
    fn measure_path(mut root: Option<Rc<RefCell<TreeNode>>>, mut path: u32) -> i32 {
        let mut d = 0;
        while let Some(node) = root {
            d += 1;
            let node = node.borrow();
            if node.left.is_some() && path & 1 == 0 {
                root = node.left.clone();
            } else { root = node.right.clone(); }
            path >>= 1;
        }
        d
    }
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let d = Self::measure_path(root.clone(), 0);
        if d < 2 { return d; }
        let n = 1 << (d - 1);
        let mut i = 0;
        let mut j = n;
        while i < j {
            let p = (i + j) / 2;
            let path = Self::reverse_bits(p, d - 1);
            if Self::measure_path(root.clone(), path) == d {
                i = p + 1;
            } else {
                j = p;
            }
        }
        (n - 1 + i) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    fn slice_to_tree(slice: &[i32], null: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if slice.is_empty() { return None; }
        let mut queue = VecDeque::new();
        let mut index = 0;
        let root = Rc::new(RefCell::new(TreeNode::new(slice[index])));
        queue.push_back(root.clone());
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let mut node = node.borrow_mut();
            index += 1;
            if index >= slice.len() { break; }
            {
                let left = slice[index];
                if left != null {
                    let left = Rc::new(RefCell::new(TreeNode::new(left)));
                    queue.push_back(left.clone());
                    node.left = Some(left);
                }
            }
            index += 1;
            if index >= slice.len() { break; }
            {
                let right = slice[index];
                if right != null {
                    let right = Rc::new(RefCell::new(TreeNode::new(right)));
                    queue.push_back(right.clone());
                    node.right = Some(right);
                }
            }
        }
        Some(root)
    }

    /*
        Input: root = [1,2,3,4,5,6]
        Output: 6
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[1,2,3,4,5,6], -1);

        let n = Solution::count_nodes(root);

        assert_eq!(n, 6);
    }

    /*
        Input: root = []
        Output: 0
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[], -1);

        let n = Solution::count_nodes(root);

        assert_eq!(n, 0);
    }

    /*
        Input: root = [1]
        Output: 1
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[1], -1);

        let n = Solution::count_nodes(root);

        assert_eq!(n, 1);
    }

    /*
        Input: root = [1,2,3]
        Output: 3
     */
    #[test]
    fn example4() {
        let root = slice_to_tree(&[1,2,3], -1);

        let n = Solution::count_nodes(root);

        assert_eq!(n, 3);
    }

    /*
        Input: root = [1,2,3,4,5]
        Output: 5
     */
    #[test]
    fn example5() {
        let root = slice_to_tree(&[1,2,3,4,5], -1);

        let n = Solution::count_nodes(root);

        assert_eq!(n, 5);
    }
}