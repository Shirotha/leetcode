#![allow(clippy::question_mark)]
/*
    Given the head of a linked list, return the list after sorting it in ascending order.

    Constraints:

    The number of nodes in the list is in the range [0, 5 * 10^4].
    -10^5 <= Node.val <= 10^5
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
impl ListNode {
    #[inline] fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}


pub struct Solution;

type Node = Box<ListNode>;
type List = Option<Node>;

#[inline] fn steal(dest: List, mut src: Node) -> (List, List) {
    let tmp = src.next.take();
    src.next = dest;
    (Some(src), tmp)
}

impl Solution {
    pub fn sort_list(mut head: List) -> List {
        let mut a = None;
        let mut b = None;
        while let Some(node) = head {
            (a, head) = steal(a, node);
            (a, b) = (b, a);
        }
        if a.is_none() { return b; }
        a = Self::sort_list(a);
        b = Self::sort_list(b);
        while let Some(node_a) = a {
            if let Some(node_b) = b {
                if node_b.val < node_a.val {
                    (head, b) = steal(head, node_b);
                    a = Some(node_a);
                } else {
                    (head, a) = steal(head, node_a);
                    b = Some(node_b);
                }
            } else {(head, a) = steal(head, node_a); }
        }
        while let Some(node) = b { (head, b) = steal(head, node); }
        while let Some(node) = head { (a, head) = steal(a, node); }
        a
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_list(xs: &[i32]) -> List {
        if xs.is_empty() { return None; }
        let mut node = ListNode::new(*xs.last().unwrap());
        for x in xs.iter().rev().skip(1) {
            let mut prev = ListNode::new(*x);
            prev.next = Some(Box::new(node));
            node = prev;
        }
        Some(Box::new(node))
    }

    fn judge(mut node: List, xs: &[i32]) {
        for x in xs.iter() {
            if let Some(current) = node {
                assert_eq!(current.val, *x);
                node = current.next;
            }
        }
        assert!(node.is_none());
    }

    /*
        Input: head = [4,2,1,3]
        Output: [1,2,3,4]
     */
    #[test]
    fn example1() {
        let head = create_list(&[4,2,1,3]);

        let s = Solution::sort_list(head);

        judge(s, &[1,2,3,4]);
    }

    /*
        Input: head = [-1,5,3,4,0]
        Output: [-1,0,3,4,5]
     */
    #[test]
    fn example2() {
        let head = create_list(&[-1,5,3,4,0]);

        let s = Solution::sort_list(head);

        judge(s, &[-1,0,3,4,5]);
    }

    /*
        Input: head = []
        Output: []
     */
    #[test]
    fn example3() {
        let head = create_list(&[]);

        let s = Solution::sort_list(head);

        judge(s, &[]);
    }
}