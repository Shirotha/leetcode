/*
    Given the root of a binary tree, invert the tree, and return its root.

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
    collections::VecDeque
};

struct DepthFirst {
    stack: VecDeque<Rc<RefCell<TreeNode>>>,
}
impl Iterator for DepthFirst {
    type Item = Rc<RefCell<TreeNode>>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop_back() {
            {
                let node = node.borrow();
                if let Some(right) = &node.right {
                    self.stack.push_back(right.clone());
                }
                if let Some(left) = &node.left {
                    self.stack.push_back(left.clone());
                }
            }
            Some(node)
        } else { None }
    }
}
trait DepthFirstPreWalk {
    fn df_prewalk(&self) -> DepthFirst;
}
impl DepthFirstPreWalk for Option<Rc<RefCell<TreeNode>>> {
    fn df_prewalk(&self) -> DepthFirst {
        let mut queue = VecDeque::new();
        if let Some(root) = self {
            queue.push_back(root.clone());
        }
        DepthFirst { stack: queue }
    }
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        for node in root.df_prewalk() {
            let mut node = node.borrow_mut();
            let tmp = node.left.take();
            node.left = node.right.take();
            node.right = tmp;
        }
        root
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
        Input: root = [4,2,7,1,3,6,9]
        Output: [4,7,2,9,6,3,1]
    */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[4,2,7,1,3,6,9], -1);

        let t = Solution::invert_tree(root);

        let result = slice_to_tree(&[4,7,2,9,6,3,1], -1);

        assert_eq!(t, result);
    }

    /*
        Input: root = [2,1,3]
        Output: [2,3,1]
    */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[2,1,3], -1);

        let t = Solution::invert_tree(root);

        let result = slice_to_tree(&[2,3,1], -1);

        assert_eq!(t, result);
    }

    /*
        Input: root = []
        Output: []
    */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[], -1);

        let t = Solution::invert_tree(root);

        let result = slice_to_tree(&[], -1);

        assert_eq!(t, result);
    }
}