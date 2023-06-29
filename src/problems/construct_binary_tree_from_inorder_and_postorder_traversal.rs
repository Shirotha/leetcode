/*
    Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.

    Constraints:

    1 <= inorder.length <= 3000
    postorder.length == inorder.length
    -3000 <= inorder[i], postorder[i] <= 3000
    inorder and postorder consist of unique values.
    Each value of postorder also appears in inorder.
    inorder is guaranteed to be the inorder traversal of the tree.
    postorder is guaranteed to be the postorder traversal of the tree.
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
    fn build_subtree(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&val) = postorder.last() {
            let mut index = 0;
            while inorder[index] != val { index += 1; }
            let node = TreeNode {
                val,
                left: Self::build_subtree(&inorder[0..index], &postorder[0..index]),
                right: Self::build_subtree(&inorder[(index+1)..], &postorder[index..(postorder.len()-1)])
            };
            Some(Rc::new(RefCell::new(node)))
        } else { None }
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_subtree(&inorder, &postorder)
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
    Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
    Output: [3,9,20,null,null,15,7]
    */
    #[test]
    fn example1() {
        let inorder = vec![9,3,15,20,7];
        let postorder = vec![9,15,7,20,3];

        let t = Solution::build_tree(inorder, postorder);

        let result = slice_to_tree(&[3,9,20,-1,-1,15,7], -1);

        assert_eq!(t, result);
    }

    /*
        Input: inorder = [-1], postorder = [-1]
        Output: [-1]
    */
    #[test]
    fn example2() {
        let inorder = vec![-1];
        let postorder = vec![-1];

        let t = Solution::build_tree(inorder, postorder);

        let result = slice_to_tree(&[-1], 0);

        assert_eq!(t, result);
    }
}