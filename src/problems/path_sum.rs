/*
    Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.

    A leaf is a node with no children.

    Constraints:

    The number of nodes in the tree is in the range [0, 5000].
    -1000 <= Node.val <= 1000
    -1000 <= targetSum <= 1000
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            let target_sum = target_sum - root.val;
            if root.left.is_none() && root.right.is_none() { return target_sum == 0; }
            Self::has_path_sum(root.left.clone(), target_sum) 
                || Self::has_path_sum(root.right.clone(), target_sum)
        } else { false }
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
        Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
        Output: true
        Explanation: The root-to-leaf path with the target sum is shown.
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[5,4,8,11,-1,13,4,7,2,-1,-1,-1,1], -1);
        let target_sum = 22;

        let b = Solution::has_path_sum(root, target_sum);

        assert!(b);
    }

    /*
        Input: root = [1,2,3], targetSum = 5
        Output: false
        Explanation: There two root-to-leaf paths in the tree:
        (1 --> 2): The sum is 3.
        (1 --> 3): The sum is 4.
        There is no root-to-leaf path with sum = 5.
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[1,2,3], -1);
        let target_sum = 5;

        let b = Solution::has_path_sum(root, target_sum);

        assert!(!b);
    }

    /*
        Input: root = [], targetSum = 0
        Output: false
        Explanation: Since the tree is empty, there are no root-to-leaf paths.
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[], -1);
        let target_sum = 0;

        let b = Solution::has_path_sum(root, target_sum);

        assert!(!b);
    }
}