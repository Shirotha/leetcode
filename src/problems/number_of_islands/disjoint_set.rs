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

use std::{
    alloc::{alloc, Layout},
    cmp::Ordering::*, 
    collections::HashSet,
};

struct Node {
    parent: *mut Node,
}
impl Node {
    fn forest(count: usize) -> Vec<Self> { unsafe {
        let ptr = alloc(Layout::array::<Self>(count).unwrap()) as *mut Node;
        for i in 0..count {
            let ptr = ptr.add(i);
            (*ptr).parent = ptr;
        }
        Vec::from_raw_parts(ptr, count, count)
    } }
    fn find(&mut self) -> &mut Node { unsafe {
        if self.parent != (*self.parent).parent {
            self.parent = (*self.parent).find() as *mut _;
        }
        &mut *self.parent
    } }
    fn union(&mut self, other: &mut Node) {
        let node = self.find() as *mut Node;
        let other = other.find() as *mut Node;
        unsafe { match node.cmp(&other) {
            Less => (*other).parent = node,
            Greater => (*node).parent = other,
            Equal => (),
        } }
    }
}

impl Solution {
    #[inline] fn get_neighbourhood(forest: &mut [Node], i: usize, j: usize, w: usize, h: usize) -> [Option<&mut Node>; 3] { unsafe {
        if i >= h || j >= w { return [None, None, None]; }
        let ij = i * w + j;
        let this = forest.get_unchecked_mut(ij) as *mut _;
        let right = if j + 1 >= w { None }
            else { Some(forest.get_unchecked_mut(ij + 1) as *mut _) };
        let bottom = if i + 1 >= h { None }
            else { Some(forest.get_unchecked_mut(ij + w) as *mut _) };
        [Some(&mut *this), right.map( |r| &mut *r ), bottom.map( |b| &mut *b )]
    } }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let h = grid.len();
        let w = grid[0].len();
        let mut forest = Node::forest(w * h);
        let mut islands = HashSet::new();
        for (i, row) in grid.iter().enumerate() { 
            for (j, &x) in row.iter().enumerate() {
                if x == '0' { continue; }
                let [this, right, bottom] = Self::get_neighbourhood(&mut forest, i, j, w, h);
                let this = this.unwrap();
                let right = right.map( |right| if row[j + 1] == '1' { 
                    this.union(right); true } 
                    else { false } ).unwrap_or(false);
                let bottom = bottom.map( |bottom| if grid[i + 1][j] == '1' { 
                    this.union(bottom); true } 
                    else { false } ).unwrap_or(false);
                if !(right || bottom) {
                    islands.insert(this.find() as *mut _);
                }
            } 
        }
        let mut islands: Vec<*mut Node> = islands.into_iter()
            .map( |ptr: *mut Node| unsafe { (*ptr).find() as *mut _ } ).collect();
        islands.sort_unstable(); islands.dedup(); islands.len() as i32
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
            0111110111");

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

    /*
        Input: grid = [
        ["1","1","1","1","1","0","1","1","1","1","1","1","1","1","1","0","1","0","1","1"],
        ["0","1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","1","0"],
        ["1","0","1","1","1","0","0","1","1","0","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","0","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","0","0","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","0","1","1","1","1","1","1","0","1","1","1","0","1","1","1","0","1","1","1"],
        ["0","1","1","1","1","1","1","1","1","1","1","1","0","1","1","0","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","0","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["0","1","1","1","1","1","1","1","0","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","0","1","1","1","1","1","1","1","0","1","1","1","1","1","1"],
        ["1","0","1","1","1","1","1","0","1","1","1","0","1","1","1","1","0","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","1","1","0"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","0","1","1","1","1","0","0"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"],
        ["1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1","1"]
        ]
        Output: 1
        
    */
    #[test]
    fn example8() {
        let grid = str_to_grid("\
            11111011111111101011\n\
            01111111111110111110\n\
            10111001101111111111\n\
            11110111111111111111\n\
            10011111111111111111\n\
            10111111011101110111\n\
            01111111111101101111\n\
            11111111111101111011\n\
            11111111110111111111\n\
            11111111111111111111\n\
            01111111011111111111\n\
            11111111111111111111\n\
            11111111111111111111\n\
            11111011111110111111\n\
            10111110111011110111\n\
            11111111111101111110\n\
            11111111111110111100\n\
            11111111111111111111\n\
            11111111111111111111\n\
            11111111111111111111");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 1);
    }

    /*
        Input: grid = [
        ["1","0","1","1","0","0","1","0","1","1","1","1","0","1","0","1","1","1","1","0"],
		["0","1","0","0","1","0","1","0","1","1","1","1","1","1","0","1","1","0","1","1"],
		["1","0","0","1","0","1","0","1","0","1","1","0","1","1","1","0","0","1","1","0"],
		["0","1","1","0","0","1","1","0","1","1","1","1","0","0","1","0","0","0","1","1"],
		["1","1","0","1","0","0","1","0","0","0","1","0","1","0","1","1","1","0","1","1"],
		["0","0","0","0","1","0","1","1","0","0","1","0","0","1","0","1","1","1","1","0"],
		["1","0","1","1","1","1","0","1","1","0","1","1","0","1","1","1","0","0","1","0"],
		["0","1","1","0","0","0","1","0","0","1","0","1","1","1","0","0","1","1","0","1"],
		["0","0","0","0","1","1","0","1","0","0","1","1","0","1","0","0","1","0","1","0"],
		["0","0","1","1","1","0","1","0","1","0","1","1","1","0","1","1","1","1","1","0"],
		["1","0","1","0","1","1","1","0","1","1","1","0","1","0","1","0","1","0","1","1"],
		["0","0","1","1","1","1","0","1","1","1","0","1","0","0","0","1","1","1","0","1"],
		["1","1","1","0","0","0","0","0","1","1","0","1","1","1","0","1","1","1","1","0"],
		["0","0","1","1","1","0","0","1","0","0","1","1","1","1","1","1","0","1","1","0"],
		["0","0","0","1","1","0","0","0","0","1","1","0","1","0","0","1","1","1","1","1"],
		["0","1","1","1","0","1","0","0","1","1","1","1","1","0","1","1","1","0","0","1"],
		["0","0","0","0","1","1","1","1","0","0","0","0","1","0","0","0","0","1","1","0"],
		["1","1","1","1","1","1","1","1","1","1","0","1","1","0","1","1","1","1","1","1"],
		["0","1","0","0","1","0","0","1","1","1","1","1","1","0","1","0","1","1","1","1"],
		["0","0","1","1","1","1","1","0","0","0","1","1","1","1","1","1","0","1","1","0"]
        ]
        Output: 23
    */
    #[test]
    fn example9() {
        let grid = str_to_grid("\
            10110010111101011110\n\
            01001010111111011011\n\
            10010101011011100110\n\
            01100110111100100011\n\
            11010010001010111011\n\
            00001011001001011110\n\
            10111101101101110010\n\
            01100010010111001101\n\
            00001101001101001010\n\
            00111010101110111110\n\
            10101110111010101011\n\
            00111101110100011101\n\
            11100000110111011110\n\
            00111001001111110110\n\
            00011000011010011111\n\
            01110100111110111001\n\
            00001111000010000110\n\
            11111111110110111111\n\
            01001001111110101111\n\
            00111110001111110110");

        let n = Solution::num_islands(grid);

        assert_eq!(n, 23);
    }
}