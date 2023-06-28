/*
    Given the root of a binary tree, return its maximum depth.

    A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

    Constraints:

    The number of nodes in the tree is in the range [0, 10^4].
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
    collections::VecDeque
};

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let mut levels = 0;
                let mut queue = VecDeque::new();
                queue.push_back(root);
                while !queue.is_empty() {
                    levels += 1;
                    for _ in 0..queue.len() {
                        let node = queue.pop_front().unwrap();
                        let node = node.borrow();
                        if let Some(left) = &node.left {
                            queue.push_back(left.clone());
                        }
                        if let Some(right) = &node.right {
                            queue.push_back(right.clone());
                        }
                    }
                }
                levels
            },
            None => 0
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
        Input: root = [3,9,20,null,null,15,7]
        Output: 3
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[
                 3,
              9,   20,
            -1,-1,15,7
        ], -1);
        let d = Solution::max_depth(root);

        assert_eq!(d, 3);
    }

    /*
        Input: root = [1,null,2]
        Output: 2
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[
              1,
            -1,2
        ], -1);

        let d = Solution::max_depth(root);

        assert_eq!(d, 2);
    }

    /*
        Input: root = [1,2,null,3,null,4,null,5]
        Output: 5
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[
                                  1,
                       2,                     -1,
                 3,         -1,
             4,    -1,
            5,
        ], -1);

        let d = Solution::max_depth(root);

        assert_eq!(d, 5);
    }
}