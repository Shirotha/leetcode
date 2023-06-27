/*
    Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.

    k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.

    You may not alter the values in the list's nodes, only nodes themselves may be changed.

    Constraints:

    The number of nodes in the list is n.
    1 <= k <= n <= 5000
    0 <= Node.val <= 1000
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut input = List::new();
        input.right.head = head;
        let mut output = List::new();
        'outer: loop {
            for _ in 0..k { 
                if !input.move_right() { 
                    while input.move_left() {}
                    output.left.head = input.right.head.take();
                    break 'outer;
                }
            }
            output.left.head = input.left.head.take();
            for _ in 0..k { output.move_left(); }
        }
        while output.move_right() {}
        output.left.head
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
        Output: [2,1,4,3,5]
     */
    #[test]
    fn example1() {
        let head = slice_to_list(&[1,2,3,4,5]);
        let k = 2;

        let l = Solution::reverse_k_group(head, k);

        let result = slice_to_list(&[2,1,4,3,5]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [1,2,3,4,5], k = 3
        Output: [3,2,1,4,5]
     */
    #[test]
    fn example2() {
        let head = slice_to_list(&[1,2,3,4,5]);
        let k = 3;

        let l = Solution::reverse_k_group(head, k);

        let result = slice_to_list(&[3,2,1,4,5]);

        assert_eq!(l, result);
    }
}