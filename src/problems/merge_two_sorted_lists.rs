/*
    You are given the heads of two sorted linked lists list1 and list2.

    Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

    Return the head of the merged linked list.

    Constraints:

    The number of nodes in both lists is in the range [0, 50].
    -100 <= Node.val <= 100
    Both list1 and list2 are sorted in non-decreasing order.
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut iter1 = IntoIter(list1).peekable();
        let mut iter2 = IntoIter(list2).peekable();
        let mut list = List::new();
        while let Some(a) = iter1.peek() {
            if let Some(b) = iter2.peek() {
                if a < b {
                    list.push_right(*a); iter1.next();
                } else {
                    list.push_right(*b); iter2.next();
                }
            } else { break; }
        }
        for a in iter1 { list.push_right(a); }
        for b in iter2 { list.push_right(b); }
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
        Input: list1 = [1,2,4], list2 = [1,3,4]
        Output: [1,1,2,3,4,4]
     */
    #[test]
    fn example1() {
        let list1 = slice_to_list(&[1,2,4]);
        let list2 = slice_to_list(&[1,3,4]);

        let l = Solution::merge_two_lists(list1, list2);

        let result = slice_to_list(&[1,1,2,3,4,4]);

        assert_eq!(l, result);
    }

    /*
        Input: list1 = [], list2 = []
        Output: []
     */
    #[test]
    fn example2() {
        let list1 = slice_to_list(&[]);
        let list2 = slice_to_list(&[]);

        let l = Solution::merge_two_lists(list1, list2);

        let result = slice_to_list(&[]);

        assert_eq!(l, result);
    }

    /*
        Input: list1 = [], list2 = [0]
        Output: [0]
     */
    #[test]
    fn example3() {
        let list1 = slice_to_list(&[]);
        let list2 = slice_to_list(&[0]);

        let l = Solution::merge_two_lists(list1, list2);

        let result = slice_to_list(&[0]);

        assert_eq!(l, result);
    }
}