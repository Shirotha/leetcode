/*
    Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

    Implement the MinStack class:

    MinStack() initializes the stack object.
    void push(int val) pushes the element val onto the stack.
    void pop() removes the element on the top of the stack.
    int top() gets the top element of the stack.
    int getMin() retrieves the minimum element in the stack.
    You must implement a solution with O(1) time complexity for each function.

    Constraints:

    -2^31 <= val <= 2^31 - 1
    Methods pop, top and getMin operations will always be called on non-empty stacks.
    At most 3 * 10^4 calls will be made to push, pop, top, and getMin.
 */

use std::collections::VecDeque;

struct MinStack {
    data: VecDeque<i32>,
    min: VecDeque<i32>,
}

impl MinStack {

    fn new() -> Self {
        MinStack { data: VecDeque::new() , min: VecDeque::new() }
    }
    
    fn push(&mut self, val: i32) {
        self.data.push_back(val);
        if let Some(min) = self.min.back() {
            if &val <= min { self.min.push_back(val) };
        } else {
            self.min.push_back(val);
        }
    }
    
    fn pop(&mut self) {
        let top = unsafe { self.data.pop_back().unwrap_unchecked() };
        if &top == unsafe { self.min.back().unwrap_unchecked() } {
            self.min.pop_back();
        }
    }
    
    fn top(&self) -> i32 {
        unsafe { *self.data.back().unwrap_unchecked() }
    }
    
    fn get_min(&self) -> i32 {
        unsafe { *self.min.back().unwrap_unchecked() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input
        ["MinStack","push","push","push","getMin","pop","top","getMin"]
        [[],[-2],[0],[-3],[],[],[],[]]

        Output
        [null,null,null,null,-3,null,0,-2]

        Explanation
        MinStack minStack = new MinStack();
        minStack.push(-2);
        minStack.push(0);
        minStack.push(-3);
        minStack.getMin(); // return -3
        minStack.pop();
        minStack.top();    // return 0
        minStack.getMin(); // return -2
     */
    #[test]
    fn example1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}