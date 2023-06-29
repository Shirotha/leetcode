/*
    Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

    Constraints:

    1 <= preorder.length <= 3000
    inorder.length == preorder.length
    -3000 <= preorder[i], inorder[i] <= 3000
    preorder and inorder consist of unique values.
    Each value of inorder also appears in preorder.
    preorder is guaranteed to be the preorder traversal of the tree.
    inorder is guaranteed to be the inorder traversal of the tree.
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
    fn build_subtree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&val) = preorder.first() {
            let mut index = 0;
            while inorder[index] != val { index += 1; }
            let node = TreeNode {
                val,
                left: Self::build_subtree(&preorder[1..(index+1)], &inorder[0..index]),
                right: Self::build_subtree(&preorder[(index+1)..], &inorder[(index+1)..])
            };
            Some(Rc::new(RefCell::new(node)))
        } else { None }
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_subtree(&preorder, &inorder)
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
        Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
        Output: [3,9,20,null,null,15,7]
     */
    #[test]
    fn example1() {
        let preorder = vec![3,9,20,15,7];
        let inorder = vec![9,3,15,20,7];

        let t = Solution::build_tree(preorder, inorder);

        let result = slice_to_tree(&[3,9,20,-1,-1,15,7], -1);

        assert_eq!(t, result);
    }

    /*
        Input: preorder = [-1], inorder = [-1]
        Output: [-1]
     */
    #[test]
    fn example2() {
        let preorder = vec![-1];
        let inorder = vec![-1];

        let t = Solution::build_tree(preorder, inorder);

        let result = slice_to_tree(&[-1], 0);

        assert_eq!(t, result);
    }
}