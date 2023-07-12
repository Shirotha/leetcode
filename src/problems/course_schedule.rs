/*
    There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [a_i, b_i] indicates that you must take course b_i first if you want to take course a_i.

    For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
    Return true if you can finish all courses. Otherwise, return false.

    Constraints:

    1 <= numCourses <= 2000
    0 <= prerequisites.length <= 5000
    prerequisites[i].length == 2
    0 <= a_i, b_i < numCourses
    All the pairs prerequisites[i] are unique.
 */

pub struct Solution;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Color { White, Gray, Black }
use Color::*;

#[inline] fn get_pair(vec: &[i32]) -> (usize, usize) {
    (vec[0] as usize, vec[1] as usize)
}

impl Solution {
    fn has_cycle(node: usize, adj: &[Vec<usize>], colors: &mut [Color]) -> bool {
        colors[node] = Gray;
        for &n in adj[node].iter() {
            match colors[n] {
                White => {
                    colors[n] = Gray;
                    if Self::has_cycle(n, adj, colors) { return true; }
                },
                Gray => return true,
                Black => (),
            }
        }
        colors[node] = Black;
        false
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut adj = Vec::new();
        adj.resize_with(num_courses, Vec::new);
        for req in prerequisites.iter() {
            let (i, j) = get_pair(req);
            if i == j { return false; }
            adj[j].push(i);
        }
        let mut colors = vec![White; num_courses];
        !(0..num_courses).any( |root| Self::has_cycle(root, &adj, &mut colors) )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: numCourses = 2, prerequisites = [[1,0]]
        Output: true
        Explanation: There are a total of 2 courses to take. 
        To take course 1 you should have finished course 0. So it is possible.
     */
    #[test]
    fn example1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0]];

        let b = Solution::can_finish(num_courses, prerequisites);

        assert!(b);
    }

    /*
        Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
        Output: false
        Explanation: There are a total of 2 courses to take. 
        To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.     */
    #[test]
    fn example2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0],vec![0,1]];

        let b = Solution::can_finish(num_courses, prerequisites);

        assert!(!b);
    }

    /*
        Input: numCourses = 2, prerequisites = [[0,1]]
        Output: true
     */
    #[test]
    fn example3() {
        let num_courses = 2;
        let prerequisites = vec![vec![0,1]];

        let b = Solution::can_finish(num_courses, prerequisites);

        assert!(b);
    }
}