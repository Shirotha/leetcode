/*
    Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).

    Constraints:

    The number of nodes in the tree is in the range [1, 1000].
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

enum Direction {
    Forwards,
    Backwards,
}
use Direction::*;

struct BreadthFirst {
    queue: VecDeque<Option<Rc<RefCell<TreeNode>>>>,
    direction: Direction
}
impl Iterator for BreadthFirst {
    type Item = Option<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            if let Some(node) = node {
                let node = node.borrow();
                match self.direction {
                    Forwards => {
                        self.queue.push_back(node.left.clone());
                        self.queue.push_back(node.right.clone());
                    },
                    Backwards => {
                        self.queue.push_back(node.right.clone());
                        self.queue.push_back(node.left.clone());
                    }
                }
                Some(Some(node.val))
            } else { Some(None) }
        } else { None }
    }
}
trait BreadthFirstIter {
    fn bf_iter(&self, direction: Direction) -> BreadthFirst;
}
impl BreadthFirstIter for Option<Rc<RefCell<TreeNode>>> {
    fn bf_iter(&self, direction: Direction) -> BreadthFirst {
        let mut queue = VecDeque::new();
        queue.push_back(self.clone());
        BreadthFirst { queue, direction }
    }
}

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let root = root.borrow();
            root.left.bf_iter(Forwards).eq(root.right.bf_iter(Backwards))
        } else { true }
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
        Input: root = [1,2,2,3,4,4,3]
        Output: true
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[1,2,2,3,4,4,3], -1);

        let b = Solution::is_symmetric(root);

        assert!(b);
    }

    /*
        Input: root = [1,2,2,null,3,null,3]
        Output: false
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[1,2,2,-1,3,-1,3], -1);

        let b = Solution::is_symmetric(root);

        assert!(!b);
    }
}