/*
    Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

    According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”

    Constraints:

    The number of nodes in the tree is in the range [2, 10^5].
    -10^9 <= Node.val <= 10^9
    All Node.val are unique.
    p != q
    p and q will exist in the tree.
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
    pub fn lowest_common_ancestor(mut root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut p = p.unwrap();
        let q = q.unwrap();
        let mut stack = VecDeque::new();
        let mut ancestor = None;
        let mut d = 0;
        loop {
            while let Some(node) = root.take() {
                root = node.borrow().left.clone();
                stack.push_back(node);
            }
            let node = stack.pop_back().unwrap();
            if ancestor.is_some() {
                if stack.len() < d {
                    d -= 1; if d == 0 { return Some(node); }
                    ancestor = Some(node.clone());
                }
                if node == p { return ancestor; }
            } else if node == p {
                ancestor = Some(node.clone());
                p = q.clone();
                d = stack.len();
            } else if node == q {
                ancestor = Some(node.clone());
                d = stack.len();
            }
            root = node.borrow().right.clone();
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

    /*
        Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
        Output: 3
        Explanation: The LCA of nodes 5 and 1 is 3.
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[3,5,1,6,2,0,8,-1,-1,7,4], -1);
        let p = root.clone().unwrap().borrow().left.clone();
        let q = root.clone().unwrap().borrow().right.clone();

        let a = Solution::lowest_common_ancestor(root.clone(), p, q);

        assert_eq!(a, root);
    }

    /*
        Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
        Output: 5
        Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[3,5,1,6,2,0,8,-1,-1,7,4], -1);
        let p = root.clone().unwrap().borrow().left.clone();
        let q = p.clone().unwrap().borrow().right.clone().unwrap().borrow().right.clone();

        let a = Solution::lowest_common_ancestor(root, p.clone(), q);

        assert_eq!(a, p);
    }

    /*
        Input: root = [1,2], p = 1, q = 2
        Output: 1
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[1,2], -1);
        let p = root.clone();
        let q = p.clone().unwrap().borrow().left.clone();

        let a = Solution::lowest_common_ancestor(root.clone(), p, q);

        assert_eq!(a, root);
    }
}