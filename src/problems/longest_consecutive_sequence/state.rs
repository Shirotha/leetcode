/*
Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

You must write an algorithm that runs in O(n) time.

Constraints:

0 <= nums.length <= 10^5
-10^9 <= nums[i] <= 10^9
 */

pub struct Solution;

use std::collections::HashMap;

enum Operation {
    Change(i32),
    Set(i32)
}
impl Operation {
    fn apply(&self, other: i32) -> i32 {
        match self {
            Change(this) => this + other,
            Set(this) => *this,
        }
    }
    fn set(&self) -> i32 {
        match self {
            Set(this) => *this,
            _ => panic!("self is not a Set!"),
        }
    }
}
use Operation::*;

enum State {
    Front(i32),
    Back(i32, i32),
    Gap(i32, i32, i32),
    Middle,
}
use State::*;

struct Sequences {
    state: HashMap<i32, State>,
}
impl Sequences {
    pub fn new() -> Sequences { Sequences { state: HashMap::new() } }
    pub fn push(&mut self, n: i32) -> Option<i32> {
        let length = if let Some(e) = self.state.remove(&n) {
            match e {
                Front(back) => {
                    self.set_front(n - 1, Set(back));
                    let length = self.set_back(back, Change(-1), Change(1));
                    Some(length)
                },
                Back(front, length) => {
                    self.set_front(front, Change(1));
                    let length = self.set_back(n + 1, Set(front), Set(length + 1));
                    Some(length)
                },
                Gap(front, length, back) => {
                    let length = length + 1;
                    let total = self.set_back(back, Change(-length), Change(length));
                    self.set_front(front, Set(back));
                    Some(total)
                },
                Middle => None
            }
        } else {
            self.set_front(n - 1, Set(n + 1));
            self.set_back(n + 1, Set(n - 1), Set(1));
            Some(1)
        };
        self.state.insert(n, Middle);
        length
    }
    fn set_front(&mut self, n: i32, back_op: Operation) {
        self.state.entry(n)
            .and_modify( |e| { *e = match *e {
                Front(back) => Front(back_op.apply(back)),
                Back(front, length) => Gap(front, length, back_op.set()),
                Gap(front, length, back) => Gap(front, length, back_op.apply(back)),
                Middle => Front(back_op.set()),
            } } )
            .or_insert_with( || Front(back_op.set()) );
    }
    fn set_back(&mut self, n: i32, front_op: Operation, length_op: Operation) -> i32 {
        let mut l = 0;
        self.state.entry(n)
            .and_modify( |e| { *e = match *e {
                Front(back) => {
                    l = length_op.set();
                    Gap(front_op.set(), l, back)
                },
                Back(front, length) => {
                    l = length_op.apply(length);
                    Back(front_op.apply(front), l)
                },
                Gap(front, length, back) => {
                    l = length_op.apply(length);
                    Gap(front_op.apply(front), l, back)
                },
                Middle => {
                    l = length_op.set();
                    Back(front_op.set(), l)
                },
            } } )
            .or_insert_with( || {
                l = length_op.set();
                Back(front_op.set(), l)
            } );
        l
    }
}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut sequences = Sequences::new();
        let mut best = 0;
        for n in nums {
            if let Some(l) = sequences.push(n) {
                if l > best { best = l; }
            }
        }
        best
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: nums = [100,4,200,1,3,2]
        Output: 4
        Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
     */
    #[test]
    fn example1() {
        let nums = vec![100,4,200,1,3,2];

        let l = Solution::longest_consecutive(nums);

        assert_eq!(l, 4);
    }

    /*
        Input: nums = [0,3,7,2,5,8,4,6,0,1]
        Output: 9
     */
    #[test]
    fn example2() {
        let nums = vec![0,3,7,2,5,8,4,6,0,1];

        let l = Solution::longest_consecutive(nums);

        assert_eq!(l, 9);
    }
}