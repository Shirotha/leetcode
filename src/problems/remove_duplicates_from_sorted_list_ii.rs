/*
Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.

Constraints:

The number of nodes in the list is in the range [0, 300].
-100 <= Node.val <= 100
The list is guaranteed to be sorted in ascending order.
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
    #[inline] fn move_left(&mut self) -> bool {
        self.left.pop().map( |node| {
            self.right.push(node);
        } ).is_some()
    }
    #[inline] fn pop_right(&mut self) -> Option<i32> {
        self.right.pop().map( |node| node.val )
    }
    #[inline] fn pop_left(&mut self) -> Option<i32> {
        self.left.pop().map( |node| node.val )
    }
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = List::new();
        list.right.head = head;
        let mut dup;
        while list.move_right() {
            dup = false;
            if let Some(node) = &list.left.head {
                let val = node.val;
                while let Some(node) = &list.right.head {
                    if node.val == val { dup = true; list.pop_right(); continue; }
                    break;
                }
            }
            if dup { list.pop_left(); }
        }
        while list.move_left() {}
        list.right.head
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
        Input: head = [1,2,3,3,4,4,5]
        Output: [1,2,5]
     */
    #[test]
    fn example1() {
        let head = slice_to_list(&[1,2,3,3,4,4,5]);

        let l = Solution::delete_duplicates(head);

        let result = slice_to_list(&[1,2,5]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [1,1,1,2,3]
        Output: [2,3]
     */
    #[test]
    fn example2() {
        let head = slice_to_list(&[1,1,1,2,3]);

        let l = Solution::delete_duplicates(head);

        let result = slice_to_list(&[2,3]);

        assert_eq!(l, result);
    }
}