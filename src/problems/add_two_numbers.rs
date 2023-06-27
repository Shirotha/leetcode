/*
    You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

    You may assume the two numbers do not contain any leading zero, except the number 0 itself.

    Constraints:

    The number of nodes in each linked list is in the range [1, 100].
    0 <= Node.val <= 9
    It is guaranteed that the list represents a number that does not have leading zeros.
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub struct Solution;

use std::iter::repeat;

struct Stack {
    head: Option<Box<ListNode>>
}
impl Stack {
    #[inline] fn new() -> Self { Stack { head: None } }
    #[inline] fn push(&mut self, mut node: Box<ListNode>) {
        node.next = self.head.take();
        self.head = Some(node);
    }
    #[inline] fn pop(&mut self) -> Option<Box<ListNode>> {
        self.head.take().map( |mut head| {
            self.head = head.next.take();
            head
        } )
    }
}

pub struct List {
    left: Stack,
    right: Stack,
}
impl List {
    #[inline] fn new() -> Self { List { left: Stack::new(), right: Stack::new() } }
    #[inline] fn move_right(&mut self) -> bool {
        self.right.pop().map( |node| {
            self.left.push(node);
        } ).is_some()
    }
    #[inline] fn push_right(&mut self, val: i32) {
        self.right.push(Box::new(ListNode::new(val)));
    }
}

struct IntoIter(Option<Box<ListNode>>);
impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map( |node| {
            self.0 = node.next;
            node.val
        } )
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = List::new();
        let mut carry = 0;
        for (mut a, mut b) in IntoIter(l1).chain(repeat(-1))
            .zip(IntoIter(l2).chain(repeat(-1))) 
        {
            if a < 0 {
                if b < 0 { break; } else { a = 0; }
            } else if b < 0 { b = 0; }
            let s = a + b + carry;
            carry = s / 10;
            list.push_right(s % 10);
        }
        if carry > 0 { list.push_right(carry) }
        while list.move_right() {}
        list.left.head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn slice_to_list(slice: &[i32]) -> Option<Box<ListNode>> {
        let mut next = None;
        for n in slice.iter().rev() {
            let node = ListNode { val: *n, next };
            next = Some(Box::new(node));
        }
        next
    }

    /*
        Input: l1 = [2,4,3], l2 = [5,6,4]
        Output: [7,0,8]
        Explanation: 342 + 465 = 807.
     */
    #[test]
    fn example1() {
        let l1 = slice_to_list(&[2,4,3]);
        let l2 = slice_to_list(&[5,6,4]);

        let r = Solution::add_two_numbers(l1, l2);

        let result = slice_to_list(&[7,0,8]);

        assert_eq!(r, result);
    }

    /*
        Input: l1 = [0], l2 = [0]
        Output: [0]
     */
    #[test]
    fn example2() {
        let l1 = slice_to_list(&[0]);
        let l2 = slice_to_list(&[0]);

        let r = Solution::add_two_numbers(l1, l2);

        let result = slice_to_list(&[0]);

        assert_eq!(r, result);
    }

    /*
        Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        Output: [8,9,9,9,0,0,0,1]
     */
    #[test]
    fn example3() {
        let l1 = slice_to_list(&[9,9,9,9,9,9,9]);
        let l2 = slice_to_list(&[9,9,9,9]);

        let r = Solution::add_two_numbers(l1, l2);

        let result = slice_to_list(&[8,9,9,9,0,0,0,1]);

        assert_eq!(r, result);
    }
}