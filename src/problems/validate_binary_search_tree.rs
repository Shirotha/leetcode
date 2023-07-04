/*
    Given the root of a binary tree, determine if it is a valid binary search tree (BST).

    A valid BST is defined as follows:

    The left subtree of a node contains only nodes with keys less than the node's key.
    The right subtree of a node contains only nodes with keys greater than the node's key.
    Both the left and right subtrees must also be binary search trees.

    Constraints:

    The number of nodes in the tree is in the range [1, 10^4].
    -2^31 <= Node.val <= 2^31 - 1
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

struct BSTIter {
    stack: VecDeque<Rc<RefCell<TreeNode>>>,
    current: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIter { stack: VecDeque::new(), current: root }
    }
}
impl Iterator for BSTIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() && self.current.is_none() { return None; }
        while let Some(node) = self.current.take() {
            self.current = node.borrow().left.clone();
            self.stack.push_back(node);
        }
        let node = self.stack.pop_back().unwrap();
        let node = node.borrow();
        self.current = node.right.clone();
        Some(node.val)
    }
}
trait IntoBSTIter {
    fn inorder(self) -> BSTIter;
}
impl IntoBSTIter for Option<Rc<RefCell<TreeNode>>> {
    fn inorder(self) -> BSTIter { BSTIter::new(self) }
}

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut last = None;
        for value in root.inorder() {
            if let Some(last) = last { if last >= value { return false; } }
            last = Some(value);
        }
        true
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
        Input: root = [2,1,3]
        Output: true
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[2,1,3], -1);

        let b = Solution::is_valid_bst(root);

        assert!(b);
    }

    /*
        Input: root = [5,1,4,null,null,3,6]
        Output: false
        Explanation: The root node's value is 5 but its right child's value is 4.
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[5,1,4,-1,-1,3,6], -1);

        let b = Solution::is_valid_bst(root);

        assert!(!b);
    }

    /*
        Input: root = [5,4,6,null,null,3,7]
        Output: false
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[5,4,6,-1,-1,3,7], -1);

        let b = Solution::is_valid_bst(root);

        assert!(!b);
    }
}