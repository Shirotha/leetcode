/*
    Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

    An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

    Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 300
    grid[i][j] is '0' or '1'.
 */

pub struct Solution;

use std::cmp::{Ordering, Ordering::*};

enum NarrowAction { Delay, Narrow, Remove, Split(Island) }
use NarrowAction::*;

enum WidenAction { Widen, Merge, Remap(usize, usize) }
use WidenAction::*;

struct Island {
    id: usize,
    start: usize,
    end: usize,
}
impl Island {
    #[inline] fn new(start: usize) -> Self { 
        static mut ID: usize = 0;
        let id = unsafe { ID += 1; ID };
        Island { id, start, end: start } 
    }
    #[inline] fn contains(&self, i: usize) -> Ordering {
        if i < self.start { Less }
        else if i > self.end { Greater }
        else { Equal }
    }
    #[inline] fn try_narrow(&mut self, i: usize) -> NarrowAction {
        if i == self.start { return self.try_delay() }
        let end = self.end;
        if self.start == end { return Remove; }
        self.end = i - 1;
        if end > i {
            Split(Island { id: self.id, start: i + 1, end })
        } else { Narrow }
    }
    #[inline] fn try_widen(&mut self, next: Option<&Island>) -> WidenAction {
        self.end += 1;
        if let Some(next) = next {
            if self.end == next.start {
                self.end = next.end;
                if self.id != next.id {
                    Remap(self.id, next.id)
                } else { Merge }
            } else { Widen }
        } else { Widen }
    }
    #[inline] fn try_delay(&mut self) -> NarrowAction {
        if self.start < self.end {
            self.start += 1;
            Delay
        } else { Remove }
    }
}

impl Solution {
    #[inline] fn pair_mut<T>(slice: &mut [T], i: usize) -> (Option<&mut T>, Option<&mut T>) {
        match (i + 1).cmp(&slice.len()) {
            Less => { unsafe {
                let left = slice.get_unchecked_mut(i) as *mut _;
                let right = slice.get_unchecked_mut(i + 1) as *mut _;
                (Some(&mut *left), Some(&mut *right))
            } }
            Equal => (slice.last_mut(), None),
            Greater => (None, None)
        }
    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut n = 0;
        let mut islands: Vec<Island> = Vec::new();
        for row in grid {
            let mut index = 0;
            for (i, x) in row.into_iter().enumerate() {
                let (mut current, mut next) = Self::pair_mut(&mut islands, index);
                match x {
                    '0' => {
                        if let Some(island) = &mut current {
                            match island.contains(i) {
                                Less => (),
                                Equal => match island.try_narrow(i) {
                                    Delay => (),
                                    Narrow => index += 1,
                                    Remove => { islands.remove(index); },
                                    Split(new) => {
                                        index += 1;
                                        islands.insert(index, new)
                                    }
                                },
                                Greater => {
                                    index += 1;
                                    if let Some(next) = &mut next {
                                        if next.contains(i).is_eq() && matches!(next.try_delay(), Remove) { 
                                            islands.remove(index); 
                                        }
                                    }
                                },
                            }
                        }
                    },
                    '1' => {
                        if let Some(island) = &mut current {
                            match island.contains(i) {
                                Less => {
                                    islands.insert(index, Island::new(i));
                                    n += 1;
                                },
                                Equal => (),
                                Greater => match island.try_widen(next.as_deref()) {
                                    Widen => (),
                                    Merge => { islands.remove(index + 1); },
                                    Remap(from, to) => {
                                        islands.remove(index + 1);
                                        n -= 1;
                                        islands.iter_mut()
                                            .filter( |island| island.id == from )
                                            .for_each( |island| island.id = to );
                                    }
                                },
                            }
                        } else {
                            islands.push(Island::new(i));
                            n += 1;
                        }
                    },
                    _ => panic!("unknown tile")
                }
            }
        }
        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn str_to_grid(string: &str) -> Vec<Vec<char>> {
        string.split('\n').map( |line| line.chars().collect() ).collect()
    }

    /*
        Input: grid = [
        ["1","1","1","1","0"],
        ["1","1","0","1","0"],
        ["1","1","0","0","0"],
        ["0","0","0","0","0"]
        ]
        Output: 1
     */
    #[test]
    fn example1() {
        let grid = str_to_grid("\
            11110\n\
            11010\n\
            11000\n\
            00000");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","1","0","0","0"],
        ["1","1","0","0","0"],
        ["0","0","1","0","0"],
        ["0","0","0","1","1"]
        ]
        Output: 3
     */
    #[test]
    fn example2() {
        let grid = str_to_grid("\
            11000\n\
            11000\n\
            00100\n\
            00011");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 3);
    }

    /*
        Input: grid = [
        ["1","1","1","1","0"],
        ["1","1","0","1","0"],
        ["1","0","1","0","0"],
        ["0","0","0","0","0"]
        ]
        Output: 2
     */
    #[test]
    fn example3() {
        let grid = str_to_grid("\
            11110\n\
            11010\n\
            10100\n\
            00000");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 2);
    }

    /*
        Input: grid = [
        ["1","1","1"],
        ["0","1","0"],
        ["1","1","1"]
        ]
        Output: 1
     */
    #[test]
    fn example4() {
        let grid = str_to_grid("\
            111\n\
            010\n\
            111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","1","1"],
        ["1","0","1"],
        ["1","1","1"]
        ]
        Output: 1
     */
    #[test]
    fn example5() {
        let grid = str_to_grid("\
            111\n\
            101\n\
            111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","1","1","1","1","0","1","1","1","1"],
        ["0","1","1","0","1","1","1","0","1","1"],
        ["1","0","1","0","1","1","0","1","0","1"],
        ["1","0","1","1","0","1","1","1","1","1"],
        ["1","1","0","0","1","1","1","1","1","1"],
        ["1","1","0","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","0","1"],
        ["0","1","1","0","1","1","1","1","1","0"],
        ["1","1","0","1","1","0","1","1","1","1"],
        ["0","1","1","1","1","1","0","1","1","1"]
        ]
        Output: 1
     */
    #[test]
    fn example6() {
        let grid = str_to_grid("\
            1111101111\n\
            0110111011\n\
            1010110101\n\
            1011011111\n\
            1100111111\n\
            1101111111\n\
            1111111101\n\
            0110111110\n\
            1101101111\n\
            0111110111\n\
            ");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["0","1","0"],
        ["1","0","1"],
        ["0","1","0"]
        ]
        Output: 1
     */
    #[test]
    fn example7() {
        let grid = str_to_grid("\
            010\n\
            101\n\
            010");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 4);
    }
}