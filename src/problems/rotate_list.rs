/*
    Given the head of a linked list, rotate the list to the right by k places.

    Constraints:

    The number of nodes in the list is in the range [0, 500].
    -100 <= Node.val <= 100
    0 <= k <= 2 * 10^9
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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut list = List::new();
        list.right.head = head;
        let mut n = 0;
        while list.move_right() { n += 1; }
        if n <= 1 { return list.left.head; }
        for _ in 0..(k % n) { list.move_left(); }
        {
            let mut tmp = List::new();
            tmp.right.head = list.right.head.take();
            while tmp.move_right() {}
            while list.move_left() {}
            list.left.head = tmp.left.head.take();
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
        Input: head = [1,2,3,4,5], k = 2
        Output: [4,5,1,2,3]
     */
    #[test]
    fn example1() {
        let head = slice_to_list(&[1,2,3,4,5]);
        let k = 2;

        let l = Solution::rotate_right(head, k);

        let result = slice_to_list(&[4,5,1,2,3]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [0,1,2], k = 4
        Output: [2,0,1]
     */
    #[test]
    fn example2() {
        let head = slice_to_list(&[0,1,2]);
        let k = 4;

        let l = Solution::rotate_right(head, k);

        let result = slice_to_list(&[2,0,1]);

        assert_eq!(l, result);
    }
}