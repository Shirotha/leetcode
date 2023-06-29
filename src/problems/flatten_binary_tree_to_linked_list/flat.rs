/*
    Given the root of a binary tree, flatten the tree into a "linked list":

    The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
    The "linked list" should be in the same order as a pre-order traversal of the binary tree.

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
};

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut node = (*root).clone();
        while let Some(node_) = node.clone() {
            let mut current = node_.borrow_mut();
            if let Some(left) = current.left.clone() {
                let mut pre = left.clone();
                while let Some(right) = {
                    let node = pre.borrow();
                    node.right.clone()
                } { pre = right; }
                pre.borrow_mut().right = current.right.take();
                current.right = current.left.take();
            }
            node = current.right.clone();
        }
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

    fn judge(mut root: Option<Rc<RefCell<TreeNode>>>, list: &[i32]) {
        for &value in list {
            if let Some(root_) = root {
                let node = root_.borrow();
                assert_eq!(node.val, value);
                assert!(node.left.is_none());
                root = node.right.clone();
            } else { panic!("too short!") }
        }
        assert!(root.is_none());
    }

    /*
        Input: root = [1,2,5,3,4,null,6]
        Output: [1,null,2,null,3,null,4,null,5,null,6]
    */
    #[test]
    fn example1() {
        let mut root = slice_to_tree(&[1,2,5,3,4,-1,6], -1);

        Solution::flatten(&mut root);

        judge(root, &[1,2,3,4,5,6]);
    }

    /*
        Input: root = []
        Output: []
    */
    #[test]
    fn example2() {
        let mut root = slice_to_tree(&[], -1);

        Solution::flatten(&mut root);

        judge(root, &[]);
    }

    /*
        Input: root = [0]
        Output: [0]
    */
    #[test]
    fn example3() {
        let mut root = slice_to_tree(&[0], -1);

        Solution::flatten(&mut root);

        judge(root, &[0]);
    }
}