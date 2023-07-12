/*
    There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [a_i, b_i] indicates that you must take course b_i first if you want to take course a_i.

    For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
    Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.

    Constraints:

    1 <= numCourses <= 2000
    0 <= prerequisites.length <= numCourses * (numCourses - 1)
    prerequisites[i].length == 2
    0 <= a_i, b_i < numCourses
    a_i != b_i
    All the pairs [a_i, b_i] are distinct.
 */

pub struct Solution;

use std::collections::{HashSet, VecDeque};

#[inline] fn get_pair(vec: &[i32]) -> (usize, usize) {
    (vec[0] as usize, vec[1] as usize)
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut adj = Vec::new();
        adj.resize_with(num_courses, Vec::new);
        let mut indegree = vec![0; num_courses];
        let mut roots = HashSet::with_capacity(num_courses);
        for node in 0..num_courses { roots.insert(node); }
        for req in prerequisites.iter() {
            let (i, j) = get_pair(req);
            adj[j].push(i);
            indegree[i] += 1;
            roots.remove(&i);
        }
        if roots.is_empty() { return vec![] }
        let mut stack = VecDeque::with_capacity(roots.len());
        for root in roots { stack.push_back(root); }
        let mut order = Vec::with_capacity(num_courses);
        while let Some(node) = stack.pop_back() {
            order.push(node as i32);
            for &n in adj[node].iter() {
                indegree[n] -= 1;
                if indegree[n] == 0 { stack.push_back(n); }
            }
        }
        if order.len() < num_courses { order.clear(); }
        order
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: numCourses = 2, prerequisites = [[1,0]]
        Output: [0,1]
        Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
     */
    #[test]
    fn example1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0]];

        let o = Solution::find_order(num_courses, prerequisites);

        assert_eq!(o, vec![0,1]);
    }

    /*
        Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
        Output: [0,2,1,3]
        Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
        So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].     */
    #[test]
    fn example2() {
        let num_courses = 4;
        let prerequisites = vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]];

        let o = Solution::find_order(num_courses, prerequisites);

        assert!(o == vec![0,1,2,3] || o == vec![0,2,1,3]);
    }

    /*
        Input: numCourses = 1, prerequisites = []
        Output: [0]
     */
    #[test]
    fn example3() {
        let num_courses = 1;
        let prerequisites = vec![];

        let o = Solution::find_order(num_courses, prerequisites);

        assert_eq!(o, vec![0]);
    }
}