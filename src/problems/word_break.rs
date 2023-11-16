/*
    Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.

    Note that the same word in the dictionary may be reused multiple times in the segmentation.

    Constraints:

    1 <= s.length <= 300
    1 <= wordDict.length <= 1000
    1 <= wordDict[i].length <= 20
    s and wordDict[i] consist of only lowercase English letters.
    All the strings of wordDict are unique.
 */

pub struct Solution;

use std::collections::VecDeque;

#[inline] fn ceil_div(a: usize, b: usize) -> usize { (a + b - 1) / b }
#[inline] fn index_mask(i: usize) -> (usize, usize) { (i / usize::BITS as usize, 1 << (i % usize::BITS as usize)) }

struct BitSet(Vec<usize>);
impl BitSet {
    #[inline] fn new(len: usize) -> Self { BitSet(vec![0; ceil_div(len, usize::BITS as usize)]) }
    #[inline] fn insert(&mut self, i: usize) -> bool {
        let (i, b) = index_mask(i);
        if self.0[i] & b == 0 {
            self.0[i] |= b;
            true
        } else { false }
    }
}

struct Trie {
    data: Vec<usize>,
    free: Vec<usize>,
    root: usize,
}
const A: u8 = 0x61;
const CHILDREN: usize = usize::MAX >> 1;
const TERMINUS: usize = 1 << (usize::BITS - 1);
const NULL: usize = 0;
impl Trie {
    #[inline] fn new() -> Self { Trie { data: vec![0, 0], free: vec![NULL; 27], root: 1 } }
    #[inline] fn alloc(&mut self, size: usize) -> usize {
        let free = self.free[size - 1];
        if free != 0 {
            self.free[size - 1] = self.data[free];
            free
        } else {
            let id = self.data.len();
            self.data.resize(id + size, 0);
            id
        }
    }
    #[inline] fn dealloc(&mut self, node: usize, size: usize) {
        if node + size == self.data.len() {
            self.data.truncate(node);
        } else {
            self.data[node] = self.free[size - 1];
            self.free[size - 1] = node;
        }
    }
    #[inline] fn widen(&mut self, node: usize, size: usize, child_ptr: usize) -> (usize, usize) {
        let new_node = self.alloc(size + 1);
        let new_child_ptr = new_node + child_ptr - node;
        self.data.copy_within(node..child_ptr, new_node);
        if child_ptr < node + size {
            self.data.copy_within(child_ptr..(node+size), new_child_ptr+1);
        }
        self.dealloc(node, size);
        (new_node, new_child_ptr)
    }
    #[inline] fn relink(&mut self, node: usize, chr: u8, child: usize) {
        let children = self.data[node] & CHILDREN;
        let bit = 1 << (chr - A);
        let child_ptr = node + 1 + (children & (bit - 1)).count_ones() as usize;
        self.data[child_ptr] = child;
    }
    #[inline] fn get(&self, node: usize, chr: u8) -> Option<usize> {
        let children = self.data[node] & CHILDREN;
        let bit = 1 << (chr - A);
        if children & bit == 0 { return None; }
        let child_ptr = node + 1 + (children & (bit - 1)).count_ones() as usize;
        Some(self.data[child_ptr])
    }
    #[inline] fn get_or_add(&mut self, node: usize, chr: u8) -> (usize, usize) {
        let children = self.data[node] & CHILDREN;
        let bit = 1 << (chr - A);
        let child_ptr = node + 1 + (children & (bit - 1)).count_ones() as usize;
        if children & bit == 0 {
            let (node, child_ptr) = self.widen(node, children.count_ones() as usize + 1, child_ptr);
            self.data[node] |= bit;
            let child = self.alloc(1);
            self.data[child_ptr] = child;
            (node, child)
        } else { (node, self.data[child_ptr]) }
    }
    #[inline] fn is_terminus(&self, node: usize) -> bool  { self.data[node] & TERMINUS != 0 }
    #[inline] fn set_terminus(&mut self, node: usize) { self.data[node] |= TERMINUS; }
    #[inline] fn insert(&mut self, word: String) {
        let word = word.into_bytes();
        let mut node = self.root;
        let mut parent = NULL;
        let mut last_chr = 0;
        for c in word {
            let (new_node, child) = self.get_or_add(node, c);
            if new_node != node {
                if parent == NULL {
                    self.root = new_node;
                } else {
                    self.relink(parent, last_chr, new_node);
                }
            }
            parent = new_node; last_chr = c;
            node = child;
        }
        self.set_terminus(node);
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut trie = Trie::new();
        for word in word_dict { trie.insert(word); }
        let mut stack = VecDeque::new();
        let (mut i, n, mut node) = (0, s.len(), trie.root);
        let mut closed = BitSet::new(n);
        loop {
            node = if let Some(next) = s.get(i).and_then( |&chr| trie.get(node, chr) ) {
                i += 1;
                if trie.is_terminus(next) {
                    if i == n { return true; }
                    if closed.insert(i) { stack.push_back(i); }
                }
                next
            } else if let Some(j) = stack.pop_back() {
                i = j; trie.root
            } else { return false; };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: s = "leetcode", wordDict = ["leet","code"]
        Output: true
        Explanation: Return true because "leetcode" can be segmented as "leet code".
     */
    #[test]
    fn example1() {
        let s = "leetcode".to_string();
        let word_dict = ["leet","code"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(b);
    }

    /*
        Input: s = "applepenapple", wordDict = ["apple","pen"]
        Output: true
        Explanation: Return true because "applepenapple" can be segmented as "apple pen apple".
        Note that you are allowed to reuse a dictionary word.
     */
    #[test]
    fn example2() {
        let s = "applepenapple".to_string();
        let word_dict = ["apple","pen"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(b);
    }

    /*
        Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
        Output: false
     */
    #[test]
    fn example3() {
        let s = "catsandog".to_string();
        let word_dict = ["cats","dog","sand","and","cat"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(!b);
    }

    /*
        Input: s = "aaaaaaa", wordDict = ["aaaa","aa"]
        Output: false
     */
    #[test]
    fn example4() {
        let s = "aaaaaaa".to_string();
        let word_dict = ["aaaa","aa"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(!b);
    }

    /*
        Input: s = "bb", wordDict = ["a","b","bbb","bbbb"]
        Output: true
     */
    #[test]
    fn example5() {
        let s = "bb".to_string();
        let word_dict = ["a","b","bbb","bbbb"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(b);
    }

    /*
        Input: s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", wordDict = ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
        Output: false
     */
    #[test]
    fn example6() {
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
        let word_dict = ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(!b);
    }

    /*
        Input: s = "abcdefg", wordDict = ["abcde","cdef","fg","bcde","a"]
        Output: true
     */
    #[test]
    fn example7() {
        let s = "abcdefg".to_string();
        let word_dict = ["abcde","cdef","fg","bcde","a"]
            .map(str::to_string).to_vec();

        let b = Solution::word_break(s, word_dict);

        assert!(b);
    }
}