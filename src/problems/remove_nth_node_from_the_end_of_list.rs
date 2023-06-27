/*
    Given the head of a linked list, remove the nth node from the end of the list and return its head.

    Constraints:

    The number of nodes in the list is sz.
    1 <= sz <= 30
    0 <= Node.val <= 100
    1 <= n <= sz
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
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut list = List::new();
        list.right.head = head;
        while list.move_right() {}
        for _ in 0..n { list.move_left(); }
        list.pop_right();
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
        Input: head = [1,2,3,4,5], n = 2
        Output: [1,2,3,5]
     */
    #[test]
    fn example1() {
        let head = slice_to_list(&[1,2,3,4,5]);
        let n = 2;

        let l = Solution::remove_nth_from_end(head, n);

        let result = slice_to_list(&[1,2,3,5]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [1], n = 1
        Output: []
     */
    #[test]
    fn example2() {
        let head = slice_to_list(&[1]);
        let n = 1;

        let l = Solution::remove_nth_from_end(head, n);

        let result = slice_to_list(&[]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [1,2], n = 1
        Output: [1]
     */
    #[test]
    fn example3() {
        let head = slice_to_list(&[1,2]);
        let n = 1;

        let l = Solution::remove_nth_from_end(head, n);

        let result = slice_to_list(&[1]);

        assert_eq!(l, result);
    }
}