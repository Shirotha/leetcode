/*
    A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.

    The path sum of a path is the sum of the node's values in the path.

    Given the root of a binary tree, return the maximum path sum of any non-empty path.

    Constraints:

    The number of nodes in the tree is in the range [1, 3 * 10^4].
    -1000 <= Node.val <= 1000
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

struct Sum {
    local: i32,
    extensible: i32,
}
impl Sum {
    fn new(value: i32) -> Self { Sum { local: value, extensible: value } }
    fn chain(&self, knot: i32) -> Sum {
        let extensible = if self.extensible > 0 { self.extensible + knot } else { knot };
        Sum { local: extensible.max(self.local), extensible }
    }
    fn merge(&self, other: &Sum, knot: i32) -> Sum {
        let inner = self.extensible.max(other.extensible);
        let extensible = if inner > 0 { inner + knot } else { knot };
        let local = self.local.max(other.local);
        let cross = self.extensible + knot + other.extensible;
        Sum { local: extensible.max(local).max(inner).max(cross), extensible }
    }
    fn try_merge(left: Option<Sum>, right: Option<Sum>, knot: i32) -> Sum {
        if let Some(left) = left {
            right.map_or_else( || left.chain(knot) , |right| left.merge(&right, knot) )
        } else { right.map_or_else( || Sum::new(knot) , |right| right.chain(knot) ) }
    }
    fn max(&self) -> i32 { self.local.max(self.extensible) }
}

impl Solution {
    fn sum(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Sum> {
        if let Some(root) = root {
            let root = root.borrow();
            let left = Self::sum(root.left.clone());
            let right = Self::sum(root.right.clone());
            Some(Sum::try_merge(left, right, root.val))
        } else { None }
    }
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum(root).unwrap().max()
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
        Output: 6
        Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
     */
    #[test]
    fn example1() {
        let root = slice_to_tree(&[1,2,3], -1);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, 6);
    }

    /*
        Input: root = [-10,9,20,null,null,15,7]
        Output: 42
        Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
     */
    #[test]
    fn example2() {
        let root = slice_to_tree(&[-10,9,20,-1,-1,15,7], -1);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, 42);
    }

    /*
        Input: root = [-3]
        Output: -3
     */
    #[test]
    fn example3() {
        let root = slice_to_tree(&[-3], -1);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, -3);
    }

    /*
        Input: root = [-2,-1]
        Output: -1
     */
    #[test]
    fn example4() {
        let root = slice_to_tree(&[-2,-1], 0);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, -1);
    }

    /*
        Input: root = [2,1]
        Output: 3
     */
    #[test]
    fn example5() {
        let root = slice_to_tree(&[2,1], 0);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, 3);
    }

    /*
        Input: root = [2,-1]
        Output: 2
     */
    #[test]
    fn example6() {
        let root = slice_to_tree(&[2,-1], 0);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, 2);
    }

    /*
        Input: root = [3,-2,-1]
        Output: 3
     */
    #[test]
    fn example7() {
        let root = slice_to_tree(&[3,-2,-1], 0);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, 3);
    }

    /*
        Input: root = [-2,6,null,0,-6]
        Output: 6
     */
    #[test]
    fn example8() {
        let root = slice_to_tree(&[-2,6,-1,0,-6], -1);

        let s = Solution::max_path_sum(root);

        assert_eq!(s, 6);
    }
}