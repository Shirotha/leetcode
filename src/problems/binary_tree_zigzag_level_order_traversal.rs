/*
    Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).

    Constraints:

    The number of nodes in the tree is in the range [0, 2000].
    -100 <= Node.val <= 100
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
    collections::VecDeque,
};

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut order = Vec::new();
        let mut queue = VecDeque::new();
        if let Some(root) = root { queue.push_back(root); } else { return order; }
        let mut zag = true;
        while !queue.is_empty() {
            zag = !zag;
            let n = queue.len();
            let mut level = Vec::with_capacity(n);
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                level.push(node.val);
                if let Some(left) = node.left.clone() { queue.push_back(left); }
                if let Some(right) = node.right.clone() { queue.push_back(right); }
            }
            if zag { level.reverse(); }
            order.push(level);
        }
        order
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
        Input: root = [3,9,20,null,null,15,7]
        Output: [[3],[20,9],[15,7]]
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[3,9,20,-1,-1,15,7], -1);

        let o = Solution::zigzag_level_order(root);

        assert_eq!(o, vec![vec![3],vec![20,9],vec![15,7]]);
    }

    /*
        Input: root = [1]
        Output: [[1]]
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[1], -1);

        let o = Solution::zigzag_level_order(root);

        assert_eq!(o, vec![vec![1]]);
    }

    /*
        Input: root = []
        Output: []
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[], -1);

        let o = Solution::zigzag_level_order(root);

        assert!(o.is_empty());
    }

    /*
        Input: root = [1,2,3,4,null,null,5]
        Output: [[1],[3,2],[4,5]]
     */
    #[test]
    fn example4() {
        let root = slice_to_tree(&[1,2,3,4,-1,-1,5], -1);

        let o = Solution::zigzag_level_order(root);

        assert_eq!(o, vec![vec![1],vec![3,2],vec![4,5]]);
    }
}