/*
    Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

    Constraints:

    The number of nodes in the tree is in the range [0, 100].
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
    pub fn right_side_view(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = Vec::new();
        let mut stack = VecDeque::new();
        let mut last = None;
        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root.take() {
                root = node.borrow().left.clone();
                stack.push_back(node);
            }
            let node = stack.back().unwrap();
            let node = node.borrow();
            if node.right.is_some() && last != node.right {
                root = node.right.clone();
            } else {
                let d = stack.len();
                if view.len() < d { view.resize(d, 0); }
                view[d - 1] = node.val;
                drop(node);
                last = stack.pop_back().clone();
            }
        }
        view
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
        Input: root = [1,2,3,null,5,null,4]
        Output: [1,3,4]
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[1,2,3,-1,5,-1,4], -1);

        let r = Solution::right_side_view(root);

        assert_eq!(r, vec![1,3,4]);
    }

    /*
        Input: root = [1,null,3]
        Output: [1,3]
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[1,-1,3], -1);

        let r = Solution::right_side_view(root);

        assert_eq!(r, vec![1,3]);
    }

    /*
        Input: root = []
        Output: []
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[], -1);

        let r = Solution::right_side_view(root);

        assert_eq!(r, vec![]);
    }
}