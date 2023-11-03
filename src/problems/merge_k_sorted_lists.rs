/*
    You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.

    Merge all the linked-lists into one sorted linked-list and return it.

    Constraints:

    k == lists.length
    0 <= k <= 10^4
    0 <= lists[i].length <= 500
    -10^4 <= lists[i][j] <= 10^4
    lists[i] is sorted in ascending order.
    The sum of lists[i].length will not exceed 10^4.
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
type List = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<List>) -> List {
        for i in (0..lists.len()).rev() {
            if lists[i].is_none() { lists.swap_remove(i); }
        }
        if lists.is_empty() { return None; }
        if lists.len() == 1 { return lists[0].take(); }
        let mut head = None;
        while lists.len() > 1 {
            let mut val = i32::MAX;
            let mut i = 0;
            for (j, h) in lists.iter().enumerate() {
                let v = unsafe { h.as_ref().unwrap_unchecked().val };
                if v < val { val = v; i = j; }
            }
            let mut node = unsafe { lists[i].take().unwrap_unchecked() };
            match node.next.take() {
                Some(head) => lists[i] = Some(head),
                None => { lists.swap_remove(i); }
            }
            node.next = head; head = Some(node);
        }
        let mut result = lists.swap_remove(0);
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = result; result = Some(node);
        }
        result
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
        Input: lists = [[1,4,5],[1,3,4],[2,6]]
        Output: [1,1,2,3,4,4,5,6]
        Explanation: The linked-lists are:
        [
        1->4->5,
        1->3->4,
        2->6
        ]
        merging them into one sorted list:
        1->1->2->3->4->4->5->6
     */
    #[test]
    fn example1() {
        let lists = vec![
            create_list(&[1,4,5]),
            create_list(&[1,3,4]),
            create_list(&[2,6]),
        ];

        let l = Solution::merge_k_lists(lists);

        judge(l, &[1,1,2,3,4,4,5,6]);
    }

    /*
        Input: lists = []
        Output: []
     */
    #[test]
    fn example2() {
        let lists = vec![];

        let l = Solution::merge_k_lists(lists);

        judge(l, &[]);
    }

    /*
        Input: lists = [[]]
        Output: []
     */
    #[test]
    fn example3() {
        let lists = vec![
            create_list(&[])
        ];

        let l = Solution::merge_k_lists(lists);

        judge(l, &[]);
    }
}