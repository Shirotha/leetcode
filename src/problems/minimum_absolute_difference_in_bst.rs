/*
    Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.

    Constraints:

    The number of nodes in the tree is in the range [2, 10^4].
    0 <= Node.val <= 10^5
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
    pub fn get_minimum_difference(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = VecDeque::new();
        let mut last = None;
        let mut best = i32::MAX;
        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root.take() {
                root = node.borrow().left.clone();
                stack.push_back(node);
            }
            let node = stack.pop_back().unwrap();
            let node = node.borrow();
            if let Some(last) = last {
                let d = node.val - last;
                if d < best { best = d; }
            }
            last = Some(node.val);
            root = node.right.clone();
        }
        best
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
        Input: root = [4,2,6,1,3]
        Output: 1
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[4,2,6,1,3], -1);

        let d = Solution::get_minimum_difference(root);

        assert_eq!(d, 1);
    }

    /*
        Input: root = [1,0,48,null,null,12,49]
        Output: 1
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[1,0,48,-1,-1,12,49], -1);

        let d = Solution::get_minimum_difference(root);

        assert_eq!(d, 1);
    }
}