/*
    Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10-5 of the actual answer will be accepted.

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

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut average = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let mut sum = 0.0;
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                sum += node.val as f64;
                if let Some(left) = node.right.clone() { queue.push_back(left); }
                if let Some(right) = node.left.clone() { queue.push_back(right); }
            }
            average.push(sum / n as f64);
        }
        average
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
        Output: [3.00000,14.50000,11.00000]
        Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
        Hence return [3, 14.5, 11].
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[3,9,20,-1,-1,15,7], -1);

        let a = Solution::average_of_levels(root);

        assert_eq!(a, vec![3.0,14.5,11.0]);
    }

    /*
        Input: root = [3,9,20,15,7]
        Output: [3.00000,14.50000,11.00000]
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[3,9,20,15,7], -1);

        let a = Solution::average_of_levels(root);

        assert_eq!(a, vec![3.0,14.5,11.0]);
    }

    
}