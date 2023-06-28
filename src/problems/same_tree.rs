/*
    Given the roots of two binary trees p and q, write a function to check if they are the same or not.

    Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.

    Constraints:

    The number of nodes in both trees is in the range [0, 100].
    -10^4 <= Node.val <= 10^4
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

struct BreadthFirst {
    queue: VecDeque<Option<Rc<RefCell<TreeNode>>>>,
}
impl Iterator for BreadthFirst {
    type Item = Option<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            if let Some(node) = node {
                let node = node.borrow();
                self.queue.push_back(node.left.clone());
                self.queue.push_back(node.right.clone());
                Some(Some(node.val))
            } else { Some(None) }
        } else { None }
    }
}
trait BreadthFirstIter {
    fn bf_iter(&self) -> BreadthFirst;
}
impl BreadthFirstIter for Option<Rc<RefCell<TreeNode>>> {
    fn bf_iter(&self) -> BreadthFirst {
        let mut queue = VecDeque::new();
        queue.push_back(self.clone());
        BreadthFirst { queue }
    }
}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        p.bf_iter().eq(q.bf_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        Input: p = [1,2,3], q = [1,2,3]
        Output: true
     */
    #[test]
    fn example1() {
        let p = slice_to_tree(&[1,2,3], -1);
        let q = slice_to_tree(&[1,2,3], -1);

        let b = Solution::is_same_tree(p, q);

        assert!(b);
    }

    /*
        Input: p = [1,2], q = [1,null,2]
        Output: false
     */
    #[test]
    fn example2() {
        let p = slice_to_tree(&[1,2], -1);
        let q = slice_to_tree(&[1,-1,2], -1);

        let b = Solution::is_same_tree(p, q);

        assert!(!b);
    }

    /*
        Input: p = [1,2,1], q = [1,1,2]
        Output: false
     */
    #[test]
    fn example3() {
        let p = slice_to_tree(&[1,2,1], -1);
        let q = slice_to_tree(&[1,1,2], -1);

        let b = Solution::is_same_tree(p, q);

        assert!(!b);
    }
}