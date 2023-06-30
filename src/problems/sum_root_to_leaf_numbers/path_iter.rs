/*
    You are given the root of a binary tree containing digits from 0 to 9 only.

    Each root-to-leaf path in the tree represents a number.

    For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
    Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.

    A leaf node is a node with no children.

    Constraints:

    The number of nodes in the tree is in the range [1, 1000].
    0 <= Node.val <= 9
    The depth of the tree will not exceed 10.
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

struct Path {
    current: Option<Rc<RefCell<TreeNode>>>,
    path: usize,
    open: usize,
    len: usize
}
impl Path {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self { Path { current: root, path: 0, open: 0, len: 0 } }
    fn push(&mut self, open: bool) {
        self.open <<= 1;
        if open { self.open |= 1; }
        self.len += 1;
    }
    fn reset(&mut self, root: Option<Rc<RefCell<TreeNode>>>, path: usize) -> Option<usize> {
        if self.open == 0 { return None; }
        let mut n = 0;
        while self.open & 1 == 0 { n += 1; self.open >>= 1; }
        let n = self.len - n - 1;
        self.path = (1 << n) | path & ((1 << n) - 1);
        self.open = 0; self.len = 0;
        self.current = root;
        Some(self.path)
    }
}
impl Iterator for Path {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.clone().map( |node| {
            let node = node.borrow();
            self.current = if node.left.is_some() && self.path & 1 == 0 {
                self.push(node.right.is_some());
                node.left.clone()
            } else {
                self.push(false);
                node.right.clone()
            };
            self.path >>= 1;
            node.val
        } )
    }
}
struct Paths {
    root: Option<Rc<RefCell<TreeNode>>>,
    last: usize,
    path: Path,
}
impl Paths {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Paths { root: root.clone(), last: 0, path: Path::new(root) }
    }
    fn next(&mut self) -> bool {
        if let Some(next) = self.path.reset(self.root.clone(), self.last) {
            self.last = next; true
        } else { false }
    }
}
trait PathsIter {
    fn paths(self) -> Paths;
}
impl PathsIter for Option<Rc<RefCell<TreeNode>>> {
    fn paths(self) -> Paths { Paths::new(self) }
}

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut iter = root.paths();
        let mut sum = 0;
        loop {
            let mut num = 0;
            for val in &mut iter.path {
                num = 10 * num + val;
            }
            sum += num;
            if !iter.next() { break; }
        }
        sum
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
        Input: root = [1,2,3]
        Output: 25
        Explanation:
        The root-to-leaf path 1->2 represents the number 12.
        The root-to-leaf path 1->3 represents the number 13.
        Therefore, sum = 12 + 13 = 25.
    */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[1,2,3], -1);

        let s = Solution::sum_numbers(root);

        assert_eq!(s, 25);
    }

    /*
        Input: root = [4,9,0,5,1]
        Output: 1026
        Explanation:
        The root-to-leaf path 4->9->5 represents the number 495.
        The root-to-leaf path 4->9->1 represents the number 491.
        The root-to-leaf path 4->0 represents the number 40.
        Therefore, sum = 495 + 491 + 40 = 1026.
    */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[4,9,0,5,1], -1);

        let s = Solution::sum_numbers(root);

        assert_eq!(s, 1026);
    }

    /*
        Input: root = [9,0,3,null,null,2,null,3,4,null,null,null,7]
        Output: 102660
    */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[9,0,3,-1,-1,2,-1,3,4,-1,-1,-1,7], -1);

        let s = Solution::sum_numbers(root);

        assert_eq!(s, 102660);
    }
}