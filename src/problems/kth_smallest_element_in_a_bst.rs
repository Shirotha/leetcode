/*
    Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.

    Constraints:

    The number of nodes in the tree is n.
    1 <= k <= n <= 10^4
    0 <= Node.val <= 10^4
 */

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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        root.inorder().take(k as usize).last().unwrap()
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
        Input: root = [3,1,4,null,2], k = 1
        Output: 1
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[3,1,4,-1,2], -1);
        let k = 1;

        let x = Solution::kth_smallest(root, k);

        assert_eq!(x, 1);
    }

    /*
        Input: root = [5,3,6,2,4,null,null,1], k = 3
        Output: 3
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[5,3,6,2,4,-1,-1,1], -1);
        let k = 3;

        let x = Solution::kth_smallest(root, k);

        assert_eq!(x, 3);
    }
}