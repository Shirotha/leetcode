/*
    Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.

    You should preserve the original relative order of the nodes in each of the two partitions.

    Constraints:

    The number of nodes in the list is in the range [0, 200].
    -100 <= Node.val <= 100
    -200 <= x <= 200
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut input = List::new();
        input.right.head = head;
        let mut output = List::new();
        while input.move_right() {
            if let Some(node) = &input.left.head {
                if node.val < x {
                    output.right.push(input.left.pop().unwrap());
                }
            }
        }
        while input.move_left() {}
        output.left.head = input.right.head.take();
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
        Input: head = [1,4,3,2,5,2], x = 3
        Output: [1,2,2,4,3,5]
     */
    #[test]
    fn example1() {
        let head = slice_to_list(&[1,4,3,2,5,2]);
        let x = 3;

        let l = Solution::partition(head, x);

        let result = slice_to_list(&[1,2,2,4,3,5]);

        assert_eq!(l, result);
    }

    /*
        Input: head = [2,1], x = 2
        Output: [1,2]
     */
    #[test]
    fn example2() {
        let head = slice_to_list(&[2,1]);
        let x = 2;

        let l = Solution::partition(head, x);

        let result = slice_to_list(&[1,2]);

        assert_eq!(l, result);
    }
}