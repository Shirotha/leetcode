/*
    Given a string path, which is an absolute path (starting with a slash '/') to a file or directory in a Unix-style file system, convert it to the simplified canonical path.

    In a Unix-style file system, a period '.' refers to the current directory, a double period '..' refers to the directory up a level, and any multiple consecutive slashes (i.e. '//') are treated as a single slash '/'. For this problem, any other format of periods such as '...' are treated as file/directory names.

    The canonical path should have the following format:

    The path starts with a single slash '/'.
    Any two directories are separated by a single slash '/'.
    The path does not end with a trailing '/'.
    The path only contains the directories on the path from the root directory to the target file or directory (i.e., no period '.' or double period '..')
    Return the simplified canonical path.

    Constraints:

    1 <= path.length <= 3000
    path consists of English letters, digits, period '.', slash '/' or '_'.
    path is a valid absolute Unix path.
 */

pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = VecDeque::new();
        path.split('/').filter( |s| !s.is_empty() && s != &"." )
            .for_each( |s| if s == ".." { stack.pop_back(); } else { stack.push_back(s); } );
        let mut path = stack.make_contiguous().join("/");
        path.insert(0, '/');
        path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: path = "/home/"
        Output: "/home"
        Explanation: Note that there is no trailing slash after the last directory name.
     */
    #[test]
    fn example1() {
        let path = "/home/".to_string();

        let c = Solution::simplify_path(path);

        assert_eq!(c, "/home".to_string());
    }

    /*
        Input: path = "/../"
        Output: "/"
        Explanation: Going one level up from the root directory is a no-op, as the root level is the highest level you can go.     
     */
    #[test]
    fn example2() {
        let path = "/../".to_string();

        let c = Solution::simplify_path(path);

        assert_eq!(c, "/".to_string());
    }

    /*
        Input: path = "/home//foo/"
        Output: "/home/foo"
        Explanation: In the canonical path, multiple consecutive slashes are replaced by a single one.
     */
    #[test]
    fn example3() {
        let path = "/home//foo/".to_string();

        let c = Solution::simplify_path(path);

        assert_eq!(c, "/home/foo".to_string());
    }
}