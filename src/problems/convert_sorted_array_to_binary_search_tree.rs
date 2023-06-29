/*
    Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.

    Constraints:

    1 <= nums.length <= 10^4
    -10^4 <= nums[i] <= 10^4
    nums is sorted in a strictly increasing order.
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
    fn create_subtree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() { return None; }
        let pivot = nums.len() / 2;
        let mut node = TreeNode::new(nums[pivot]);
        node.left = Self::create_subtree(&nums[0..pivot]);
        node.right = Self::create_subtree(&nums[(pivot+1)..]);
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::create_subtree(&nums)
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
        Input: nums = [-10,-3,0,5,9]
        Output: [0,-3,9,-10,null,5]
        Explanation: [0,-10,5,null,-3,null,9] is also accepted:
     */
    #[test]
    fn example1() {
        let nums = vec![-10,-3,0,5,9];

        let t = Solution::sorted_array_to_bst(nums);

        let result = [
            slice_to_tree(&[0,-3,9,-10,-1,5], -1),
            slice_to_tree(&[0,-10,5,-1,-3,-1,9], -1)
        ];

        assert!(result.contains(&t));
    }

    /*
        Input: nums = [1,3]
        Output: [3,1]
        Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
     */
    #[test]
    fn example2() {
        let nums = vec![1,3];

        let t = Solution::sorted_array_to_bst(nums);

        let result = [
            slice_to_tree(&[1,-1,3], -1),
            slice_to_tree(&[3,1], -1)
        ];

        assert!(result.contains(&t));
    }
}