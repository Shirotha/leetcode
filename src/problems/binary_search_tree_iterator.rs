/*
    Implement the BSTIterator class that represents an iterator over the in-order traversal of a binary search tree (BST):

    BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
    boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
    int next() Moves the pointer to the right, then returns the number at the pointer.
    Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.

    You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.

    Constraints:

    The number of nodes in the tree is in the range [1, 10^5].
    0 <= Node.val <= 10^6
    At most 105 calls will be made to hasNext, and next.
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

struct BSTIterator {
    stack: VecDeque<Rc<RefCell<TreeNode>>>,
    current: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator { stack: VecDeque::new(), current: root }
    }
    
    fn next(&mut self) -> i32 {
        while let Some(node) = self.current.take() {
            self.current = node.borrow().left.clone();
            self.stack.push_back(node);
        }
        let node = self.stack.pop_back().unwrap();
        let node = node.borrow();
        self.current = node.right.clone();
        node.val
    }
    
    fn has_next(&self) -> bool {
        !self.stack.is_empty() || self.current.is_some()
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
        Input
        ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
        [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
        Output
        [null, 3, 7, true, 9, true, 15, true, 20, false]

        Explanation
        BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
        bSTIterator.next();    // return 3
        bSTIterator.next();    // return 7
        bSTIterator.hasNext(); // return True
        bSTIterator.next();    // return 9
        bSTIterator.hasNext(); // return True
        bSTIterator.next();    // return 15
        bSTIterator.hasNext(); // return True
        bSTIterator.next();    // return 20
        bSTIterator.hasNext(); // return False
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[7,3,15,-1,-1,9,20], -1);
        let mut iter = BSTIterator::new(root);
        assert_eq!(iter.next(), 3);
        assert_eq!(iter.next(), 7);
        assert!(iter.has_next());
        assert_eq!(iter.next(), 9);
        assert!(iter.has_next());
        assert_eq!(iter.next(), 15);
        assert!(iter.has_next());
        assert_eq!(iter.next(), 20);
        assert!(!iter.has_next());
    }
}