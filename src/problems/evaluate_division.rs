/*
    You are given an array of variable pairs equations and an array of real numbers values, where equations[i] = [A_i, B_i] and values[i] represent the equation A_i / B_i = values[i]. Each A_i or B_i is a string that represents a single variable.

    You are also given some queries, where queries[j] = [C_j, D_j] represents the jth query where you must find the answer for C_j / D_j = ?.

    Return the answers to all queries. If a single answer cannot be determined, return -1.0.

    Note: The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.

    Constraints:

    1 <= equations.length <= 20
    equations[i].length == 2
    1 <= A_i.length, B_i.length <= 5
    values.length == equations.length
    0.0 < values[i] <= 20.0
    1 <= queries.length <= 20
    queries[i].length == 2
    1 <= C_j.length, D_j.length <= 5
    A_i, B_i, C_j, D_j consist of lower case English letters and digits.
 */

pub struct Solution;

use std::{
    alloc::{alloc, Layout},
    cmp::Ordering::*, 
    collections::HashMap, 
    ptr,
};

struct Node {
    parent: *mut Node,
    quotient: f64,
}
impl PartialEq for Node {
    #[inline] fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}
impl Node {
    fn forest(count: usize) -> Vec<Self> { unsafe {
        let ptr = alloc(Layout::array::<Self>(count).unwrap()) as *mut Node;
        for i in 0..count {
            let ptr = ptr.add(i);
            (*ptr).parent = ptr;
            (*ptr).quotient = 1.0;
        }
        Vec::from_raw_parts(ptr, count, count)
    } }
    #[inline] fn find(&mut self) -> &mut Node { unsafe {
        if self.parent != (*self.parent).parent {
            let root = (*self.parent).find();
            self.quotient *= (*self.parent).quotient;
            self.parent = root;
        }
        &mut *self.parent
    } }
    #[inline] fn union(&mut self, other: &mut Node, quotient: f64) {
        let root = self.find() as *mut Node;
        let other_root = other.find() as *mut Node;
        let quotient = self.quotient * quotient / other.quotient;
        unsafe { match root.cmp(&other_root) {
            Less => { 
                (*other_root).parent = root; 
                (*other_root).quotient = quotient;
            },
            Greater => { 
                (*root).parent = other_root; 
                (*root).quotient = quotient.recip();
            },
            Equal => (),
        } }
    }
    #[inline] fn path(&mut self, other: &mut Node) -> f64 {
        if self.find() == other.find() { other.quotient / self.quotient }
        else { -1.0 }
    }
}

#[inline] fn get_pair<T>(vec: &[T]) -> (&T, &T) {
    let ptr = vec.as_ptr();
    unsafe { (&*ptr, &*ptr.add(1)) }
}
#[inline] fn get_pair_mut<T>(vec: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    let ptr = vec.as_mut_ptr();
    unsafe { (&mut *ptr.add(i), &mut *ptr.add(j)) }
}

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut names = HashMap::new();
        let mut n = 0;
        for eq in equations.iter() {
            let (a, b) = get_pair(eq);
            names.entry(a).or_insert_with( || { let j = n; n += 1; j } );
            names.entry(b).or_insert_with( || { let j = n; n += 1; j } );
        }
        let mut vars = Node::forest(n);
        for (eq, v) in equations.iter().zip(values.into_iter()) {
            let (a, b) = get_pair(eq);
            let (a, b) = get_pair_mut(&mut vars, names[a], names[b]);
            a.union(b, v);
        }
        queries.iter().map( |eq| {
            let (a, b) = get_pair(eq);
            let a = if let Some(&a) = names.get(&a) { a } else { return -1.0; };
            let b = if let Some(&b) = names.get(&b) { b } else { return -1.0; };
            let (a, b) = get_pair_mut(&mut vars, a, b);
            a.path(b)
        } ).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(r: &[f64], result: &[f64]) {
        for (&a, &b) in r.iter().zip(result.iter()) {
            assert!((a - b).abs() < 1e-5);
        }
    }

    /*
        Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
        Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
        Explanation: 
        Given: a / b = 2.0, b / c = 3.0
        queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
        return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
     */
    #[test]
    fn example1() {
        let equations = vec![vec!["a","b"],vec!["b","c"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();
        let values = vec![2.0,3.0];
        let queries = vec![vec!["a","c"],vec!["b","a"],vec!["a","e"],vec!["a","a"],vec!["x","x"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();

        let r = Solution::calc_equation(equations, values, queries);

        judge(&r, &[6.0,0.5,-1.0,1.0,-1.0]);
    }

    /*
        Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
        Output: [3.75000,0.40000,5.00000,0.20000]
     */
    #[test]
    fn example2() {
        let equations = vec![vec!["a","b"],vec!["b","c"],vec!["bc","cd"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();
        let values = vec![1.5,2.5,5.0];
        let queries = vec![vec!["a","c"],vec!["c","b"],vec!["bc","cd"],vec!["cd","bc"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();

        let r = Solution::calc_equation(equations, values, queries);

        judge(&r, &[3.75,0.4,5.0,0.2]);
    }

    /*
        Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
        Output: [0.50000,2.00000,-1.00000,-1.00000]
     */
    #[test]
    fn example3() {
        let equations = vec![vec!["a","b"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();
        let values = vec![0.5];
        let queries = vec![vec!["a","b"],vec!["b","a"],vec!["a","c"],vec!["x","y"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();

        let r = Solution::calc_equation(equations, values, queries);

        judge(&r, &[0.5,2.0,-1.0,-1.0]);
    }

    /*
        Input: equations = [["a","e"],["b","e"]], values = [4.0,3.0], queries =[["a","b"],["e","e"],["x","x"]]
        Output: [1.33333,1.00000,-1.00000]
     */
    #[test]
    fn example4() {
        let equations = vec![vec!["a","e"],vec!["b","e"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();
        let values = vec![4.0,3.0];
        let queries = vec![vec!["a","b"],vec!["e","e"],vec!["x","x"]].into_iter()
            .map( |eq| eq.into_iter().map(str::to_string).collect() ).collect();

        let r = Solution::calc_equation(equations, values, queries);

        judge(&r, &[1.33333,1.00000,-1.00000]);
    }
}