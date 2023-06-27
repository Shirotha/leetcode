/*
    Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.

    Constraints:

    The number of nodes in the list is n.
    1 <= n <= 500
    -500 <= Node.val <= 500
    1 <= left <= right <= n
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
}

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right { return head; }
        let mut list = List::new();
        list.right.head = head;
        for _ in 1..left { list.move_right(); }
        {
            let mut tmp = List::new();
            tmp.right.head = list.right.head.take();
            for _ in left..=right { tmp.move_right(); }
            list.right.head = tmp.left.head.take();
            while list.move_right() {}
            list.right.head = tmp.right.head.take();
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
        Input: head = [1,2,3,4,5], left = 2, right = 4
        Output: [1,4,3,2,5]
     */
    #[test]
    fn example1() {
        let head = slice_to_list(&[1,2,3,4,5]);
        let left = 2;
        let right = 4;

        let l = Solution::reverse_between(head, left, right);

        let result = slice_to_list(&[1,4,3,2,5]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [5], left = 1, right = 1
        Output: [5]
     */
    #[test]
    fn example2() {
        let head = slice_to_list(&[5]);
        let left = 1;
        let right = 1;

        let l = Solution::reverse_between(head, left, right);

        let result = slice_to_list(&[5]);

        assert_eq!(l, result);
    }
}