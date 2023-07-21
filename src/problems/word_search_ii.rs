/*
    Given an m x n board of characters and a list of strings words, return all words on the board.

    Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

    Constraints:

    m == board.length
    n == board[i].length
    1 <= m, n <= 12
    board[i][j] is a lowercase English letter.
    1 <= words.length <= 3 * 10^4
    1 <= words[i].length <= 10
    words[i] consists of lowercase English letters.
    All the strings of words are unique.
 */

pub struct Solution;

const WORD_LENGTH: usize = 10;

#[inline] fn char_to_index(c: char) -> u32 { c as u32 - 0x61 }
#[inline] fn index_to_char(i: u8) -> char { (i + 0x61) as char }

struct BitField<const OFFSET: u32, const MASK: usize>;
struct Bit;
impl Bit {
    #[inline] fn from<const O: u32, const M: usize>(_field: BitField<O, M>, value: usize) -> usize {
        (value << O) & M
    }
    #[inline] fn get<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>) -> usize {
        (this & M) >> O
    }
    #[inline] fn set_unchecked<const O: u32, const M: usize>(this: &mut usize, _field: BitField<O, M>, value: usize) {
        *this |= value << O;
    }
    #[inline] fn set<const O: u32, const M: usize>(this: &mut usize, _field: BitField<O, M>, value: usize) {
        *this ^= (Self::get(*this, _field) ^ value) << O;
    }
    #[inline] fn is_zero<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>) -> bool {
        this & M == 0
    }
    #[inline] fn clear<const O: u32, const M: usize>(this: &mut usize, _field: BitField<O, M>) {
        *this &= !M
    }
    #[inline] fn is_set<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>, bit: u32) -> bool {
        (this >> (O + bit)) & 1 != 0
    }
    #[inline] fn set_bit<const O: u32, const M: usize>(this: &mut usize, _field: BitField<O, M>, bit: u32) {
        *this |= 1 << (O + bit);
    }
    #[inline] fn index<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>, bit: u32) -> u32 {
        ((this >> O) & ((1 << bit) - 1)).count_ones()
    }
    #[inline] fn count<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>) -> u32 {
        ((this & M) >> O).count_ones()
    }
    #[inline] fn first<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>) -> u32 {
        (this >> O).trailing_zeros()
    }
    #[inline] fn next<const O: u32, const M: usize>(this: usize, _field: BitField<O, M>, current: u32) -> u32 {
        (this >> (O + current)).trailing_zeros()
    }
}
struct Head;
impl Head {
    const CHILDREN: BitField<0,  0x0000000003ffffff> = BitField{};
    const KNOT:     BitField<26, 0x7ffffffffc000000> = BitField{};
    const TERMINUS: BitField<63, 0x8000000000000000> = BitField{};
}
struct Tail;
impl Tail {
    const POINTER:    BitField<0,  0x7fffffffffffffff> = BitField{};
    const CONTRACTED: BitField<63, 0x8000000000000000> = BitField{};
    fn contraction(knot: usize) -> usize {
        Bit::from(Self::POINTER, knot) | Bit::from(Self::CONTRACTED, 1)
    }
}
struct ContractState;
impl ContractState {
    const NODE:   BitField<0,  0x007fffffffffffff> = BitField{};
    const BRIDGE: BitField<55, 0x0780000000000000> = BitField{};
    const CHAR:   BitField<59, 0xf800000000000000> = BitField{};
    fn from(node: usize, bridge: usize, char: usize) -> usize {
        Bit::from(Self::NODE, node) 
            | Bit::from(Self::BRIDGE, bridge)
            | Bit::from(Self::CHAR, char)
    }
}

enum PeekAction<T> {
    Push(T),
    Next,
    Pop,
}
use PeekAction::*;
struct PeekResult<T, R>(PeekAction<T>, Option<R>);
impl<T, R> PeekResult<T, R> {
    fn found(action: PeekAction<T>, result: R) -> Self {
        PeekResult(action, Some(result))
    }
    fn skip(action: PeekAction<T>) -> Self {
        PeekResult(action, None)
    }
}
struct PathIter<T> {
    stack: VecDeque<T>,
}
impl<T> PathIter<T> {
    fn new(root: T) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(root);
        PathIter { stack }
    }
    fn next<P, R>(&mut self, peek: P) -> Result<Option<R>,()> where 
        P: FnOnce(&mut T) -> PeekResult<T, R>
    {
        if let Some(state) = self.stack.back_mut() {
            let PeekResult(action, result) = peek(state);
            match action {
                Push(next) => self.stack.push_back(next),
                Next => (),
                Pop => { self.stack.pop_back(); },
            }
            Ok(result)
        } else { Err(()) }
    }
}

/* TODO: implement PathIter with backtracking
 * - assumed to start in the middle (at a knot)
 * - iterate like normal until possible result is found
 * - use front of stack to backtrack (confirm result)
 * - iterate rest (treat all results as confirmed)
*/

struct Knot {
    node: usize,
    value: char,
    prefix: String,
}
struct Trie {
    data: Vec<usize>,
    free: Vec<usize>,
    root: usize,
}
enum Edge {
    Regular(usize),
    Contraction(usize),
}
use std::collections::VecDeque;

use Edge::*;
impl Edge {
    fn new(child: usize) -> Self {
        if Bit::is_zero(child, Tail::CONTRACTED) {
            Regular(Bit::from(Tail::POINTER, child))
        } else {
            Contraction(Bit::from(Tail::POINTER, child))
        }
    }
}
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
    #[inline] fn is_terminus(&self, node: usize) -> bool { !Bit::is_zero(self.data[node], Head::TERMINUS) }
    #[inline] fn pointer(&self, node: usize, index: u32) -> usize {
        let index = Bit::index(self.data[node], Head::CHILDREN, index);
        node + 1 + index as usize
    }
    #[inline] fn pointer_iter(&self, node: usize, current: u32) -> (usize, u32) {
        let head = self.data[node];
        let index = Bit::index(head, Head::CHILDREN, current);
        let next = Bit::next(head, Head::CHILDREN, current);
        (node + 1 + index as usize, next)
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
    #[inline] fn relink(&mut self, node: usize, chr: char, child: usize) {
        let index = Bit::index(self.data[node], Head::CHILDREN, char_to_index(chr));
        let child_ptr = node + 1 + index as usize;
        self.data[child_ptr] = child;
    }
    #[inline] fn get_or_add(&mut self, node: usize, chr: char) -> (usize, usize) {
        let head = self.data[node];
        let bit = char_to_index(chr);
        let index = Bit::index(head, Head::CHILDREN, bit);
        let child_ptr = node + 1 + index as usize;
        if Bit::is_set(head, Head::CHILDREN, bit) {
            let size = Bit::count(head, Head::CHILDREN) as usize + 1;
            let (node, child_ptr) = self.widen(node, size, child_ptr);
            Bit::set_bit(&mut self.data[node], Head::CHILDREN, bit);
            let child = self.alloc(1);
            self.data[child_ptr] = child;
            (node, child)
        } else { (node, self.data[child_ptr]) }
    }
    #[inline] fn set_terminus(&mut self, node: usize) {
        Bit::set_unchecked(&mut self.data[node], Head::TERMINUS, 1);
    }
    #[inline] fn insert(&mut self, word: String) {
        let mut node = self.root;
        let mut parent = NULL;
        let mut last_chr = '\0';
        for c in word.chars() {
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

    #[inline] fn traverse_line(&self, mut node: usize) -> (Option<usize>, u32) {
        let mut n = 0;
        let mut d;
        loop {
            d = Bit::count(self.data[node], Head::CHILDREN);
            if self.is_terminus(node) { break; }
            if d != 1 { break; }
            node = self.data[node + 1];
            n += 1;
        }
        if d == 0 { (None, 0) } else { (Some(node), n) }
    }
    #[inline] fn create_bridge(&self, bridge: &mut String, mut node: usize, knot: usize) -> char {
        while node != knot {
            let i = Bit::first(self.data[node], Head::CHILDREN);
            bridge.push(index_to_char(i as u8));
            node = self.data[node + 1];
        }
        bridge.pop().unwrap()
    }
    fn contract(&mut self, min_length: u32) -> Vec<Knot> {
        let mut result = vec![
            Knot { node: self.root, value: '\0', prefix: String::new() }
        ];
        let mut knot_index = 0;
        let mut iter = {
            let first = Bit::first(self.data[self.root], Head::CHILDREN);
            let state = ContractState::from(self.root, 0, first as usize);
            PathIter::new(state)
        };
        let mut prefix = String::new();
        while let Ok(path) = iter.next( |state| {
            let i = Bit::get(*state, ContractState::CHAR) as u32;
            if i >= 26 {
                let len = Bit::get(*state, ContractState::BRIDGE);
                for _ in 0..len { prefix.pop(); }
                return PeekResult::skip(Pop);
            }
            let node = Bit::get(*state, ContractState::NODE);
            let (child_ptr, j) = self.pointer_iter(node, i);
            Bit::set(state, ContractState::CHAR, j as usize);
            let child = self.data[child_ptr];
            let (knot, len) = self.traverse_line(child);
            if let Some(knot) = knot {
                prefix.push(index_to_char(i as u8));
                let value = self.create_bridge(&mut prefix, child, knot);
                let action = Push(ContractState::from(knot, len as usize, 0));
                if len >= min_length {
                    knot_index += 1;
                    result.push(Knot { node: knot, value, prefix: prefix.clone() });
                    Bit::set(&mut self.data[knot], Head::KNOT, knot_index);
                    self.data[child_ptr] = Tail::contraction(knot_index);
                    PeekResult::found(action, (knot, value))
                } else { PeekResult::skip(action) }
            } else { PeekResult::skip(Next) }
        } ) {
            if let Some((knot, value)) = path {
                result.push(Knot { node: knot, value, prefix: prefix.clone() });
            }
        }
        result
    }

    #[inline] fn narrow(&mut self, node: usize, child: usize) {
        let mut children = Bit::get(self.data[node], Head::CHILDREN);
        let n = children.count_ones() as usize;
        let last = node + n;
        let mut child_ptr = node;
        for i in 0..n {
            child_ptr += 1;
            if self.data[child_ptr] == child {
                let mut bit = 0;
                let mut i = i + 1;
                while i > 0 {
                    if children & 1 == 1 { i -= 1; }
                    children >>= 1;
                    bit += 1;
                }
                self.data[node] ^= 1 << (bit - 1);
                break; 
            }
        }
        if node == last { return; }
        self.data.copy_within((child_ptr+1)..=last, child_ptr);
    }
    #[inline] fn get(&self, node: usize, chr: char) -> Option<Edge> {
        let child_ptr = self.pointer(node, char_to_index(chr));
        let child = self.data[child_ptr];
        if child == NULL { None } else { Some(Edge::new(child)) }
    }
    #[inline] fn is_leaf(&self, node: usize) -> bool {
        Bit::is_zero(self.data[node], Head::CHILDREN)
    }
    #[inline] fn clear_terminus(&mut self, node: usize) -> bool {
        if self.is_terminus(node) { 
            Bit::clear(&mut self.data[node], Head::TERMINUS); true 
        } else { false }
    }
    #[inline] fn remove(&mut self, parent: usize, child: usize) {
        let knot = Bit::get(child, Head::KNOT);
        if knot == 0 {
            self.narrow(parent, child);
        } else {
            self.narrow(parent, Tail::contraction(knot));
        }
    }
}

// TODO: v use new types for everything v

const PARENT_OFFSET: u32 = usize::BITS - 10;
const PARENT: usize = 0xff << PARENT_OFFSET;
const DIR_OFFSET: u32 = usize::BITS - 3;
const DIR: usize = 0x7 << DIR_OFFSET;
const NODE: usize = usize::MAX >> 11;
#[inline] fn get_neighbour(board: &[Vec<char>], ij: usize, dir: usize, w: usize) -> Option<(usize, char)> {
    let i = ij / w;
    let j = ij % w;
    match dir {
        0 => if j > 0 { board.get(i).and_then( |row| row.get(j - 1) ).map( |c| (ij - 1, *c) ) } else { None },
        1 => if i > 0 { board.get(i - 1).and_then( |row| row.get(j) ).map( |c| (ij - w, *c) ) } else { None },
        2 => board.get(i).and_then( |row| row.get(j + 1) ).map( |c| (ij + 1, *c) ),
        3 => board.get(i + 1).and_then( |row| row.get(j) ).map( |c| (ij + w, *c) ),
        _ => panic!("invalid direction")
    }
}
enum State {
    Explore,
    Confirm(usize),
}
use State::*;
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let h = board.len();
        let w = board[0].len();
        let mut result = Vec::new();
        let mut builder = String::new();
        let mut trie = Trie::new();
        for word in words { trie.insert(word); }
        let knots = trie.contract(2);
        let mut stack = vec![0; w * h];
        let mut state = Explore;
        for knot in knots.iter().rev() {
            let prefix = knot.prefix.as_bytes();
            for root_y in 0..h { for root_x in 0..w {
                let root_chr = board[root_y][root_x];
                let mut node = knot.node;
                let mut confirmed = node == trie.root;
                if confirmed {
                    match trie.get(node, root_chr) {
                        Some(Regular(knot)) => node = knot,
                        Some(Contraction(knot)) => {
                            trie.remove(node, knot);
                            continue;
                        },
                        _ => continue,
                    }
                } else if root_chr != knot.value { continue; }
                builder.clear();
                builder.push(root_chr);
                let root = root_y * w + root_x;
                let mut ij = root;
                stack[ij] = node;
                loop {
                     // FIXME: example6: OOB
                    let s: usize = stack[ij];
                    let dir = match &mut state {
                        Confirm(terminus) if ij == knot.node => {
                            let dir = (*terminus & DIR) >> DIR_OFFSET;
                            *terminus ^= ((dir + 1) ^ dir) << DIR_OFFSET;
                            dir
                        },
                        _ => {
                            let dir = (s & DIR) >> DIR_OFFSET;
                            stack[ij] ^= ((dir + 1) ^ dir) << DIR_OFFSET;
                            dir
                        },
                    };
                    if dir >= 4 {
                        stack[ij] = 0;
                        match state {
                            Explore => {
                                if ij == root { break; }
                                ij = (s & PARENT) >> PARENT_OFFSET;
                                let parent = stack[ij] & NODE;
                                if trie.is_leaf(node) { trie.remove(parent, node); }
                                node = parent;
                                builder.pop();
                            },
                            Confirm(_) => {
                                if ij == knot.node {
                                    ij = s & NODE;
                                    loop {
                                        let s = stack[ij];
                                        stack[ij] = 0;
                                        if ij == root { break; }
                                        ij = s & NODE;
                                    }
                                    break;
                                }
                                ij = s % NODE;
                                node += 1;
                            }
                        }
                        continue;
                    }
                    if let Some((next_ij, chr)) = get_neighbour(&board, ij, dir, w) {
                        if stack[next_ij] != 0 { continue; }
                        match state {
                            Explore => {
                                match trie.get(node, chr) {
                                    Some(Regular(next_node)) => {
                                        builder.push(chr);
                                        let s = ij << PARENT_OFFSET | next_node & NODE;
                                        if trie.is_terminus(next_node) {
                                            if confirmed {
                                                if trie.clear_terminus(next_node) { result.push(builder.clone()); }
                                                if trie.is_leaf(next_node) { 
                                                    trie.remove(node, next_node);
                                                    builder.pop();
                                                    continue;
                                                }
                                            } else {
                                                state = Confirm(next_node | (next_ij << PARENT_OFFSET));
                                                if trie.is_leaf(next_node) {
                                                    stack[next_ij] = s & (4 << DIR_OFFSET);
                                                } else {
                                                    stack[next_ij] = s;
                                                }
                                                ij = root; node = prefix.len() - 1;
                                                continue;
                                            }
                                        }
                                        stack[next_ij] = s;
                                        ij = next_ij; node = next_node;
                                    },
                                    Some(Contraction(next_node)) => {
                                        trie.remove(node, next_node);
                                    },
                                    None => (),
                                }
                            },
                            Confirm(terminus) => {
                                if chr as u8 == prefix[node] {
                                    if node == 0 {
                                        while ij != root {
                                            let s = stack[ij];
                                            stack[ij] = 0;
                                            ij = s & NODE;
                                        }
                                        confirmed = true;
                                        state = Explore;
                                        ij = (terminus & PARENT) >> PARENT_OFFSET;
                                        // FIXME: stack at ij is 0
                                        node = terminus & NODE;
                                        if trie.clear_terminus(node) {
                                            let total = knot.prefix.clone() + &builder;
                                            result.push(total); 
                                        }
                                    } else { 
                                        node -= 1; 
                                        stack[next_ij] = ij;
                                        ij = next_ij;
                                    }
                                }
                            }
                        }
                    }
                }
            } }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(ws: Vec<String>, result: &[&str]) {
        assert_eq!(ws.len(), result.len());
        for w in ws {
            assert!(result.contains(&w.as_str()));
        }
    }

    /*
        Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
        Output: ["eat","oath"]
     */
    #[test]
    fn example1() {
        let board = vec![
            vec!['o','a','a','n'],
            vec!['e','t','a','e'],
            vec!['i','h','k','r'],
            vec!['i','f','l','v']];
        let words = ["oath","pea","eat","rain"].into_iter().map(str::to_string).collect();

        let ws = Solution::find_words(board, words);

        judge(ws, &["eat","oath"]);
    }

    /*
        Input: board = [["o","a","b","n"],["o","t","a","e"],["a","h","k","r"],["a","f","l","v"]], words = ["oa","oaa"]
        Output: ["oa","oaa"]
     */
    #[test]
    fn example2() {
        let board = vec![
            vec!['o','a','b','n'],
            vec!['o','t','a','e'],
            vec!['a','h','k','r'],
            vec!['a','f','l','v']];
        let words = ["oa","oaa"].into_iter().map(str::to_string).collect();

        let ws = Solution::find_words(board, words);

        judge(ws, &["oa","oaa"]);
    }

    /*
        Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain","hklf", "hf"]
        Output: ["oath","eat","hklf","hf"]
     */
    #[test]
    fn example3() {
        let board = vec![
            vec!['o','a','a','n'],
            vec!['e','t','a','e'],
            vec!['i','h','k','r'],
            vec!['i','f','l','v']];
        let words = ["oath","pea","eat","rain","hklf", "hf"].into_iter().map(str::to_string).collect();

        let ws = Solution::find_words(board, words);

        judge(ws, &["oath","eat","hklf","hf"]);
    }

    #[test]
    fn example4() {
        let board = vec![
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['a','a','a','a','a','a','a','a','a','a','a','a']];
        let words = ["lllllll","fffffff","ssss","s","rr","xxxx","ttt","eee","ppppppp","iiiiiiiii","xxxxxxxxxx","pppppp","xxxxxx","yy","jj","ccc","zzz","ffffffff","r","mmmmmmmmm","tttttttt","mm","ttttt","qqqqqqqqqq","z","aaaaaaaa","nnnnnnnnn","v","g","ddddddd","eeeeeeeee","aaaaaaa","ee","n","kkkkkkkkk","ff","qq","vvvvv","kkkk","e","nnn","ooo","kkkkk","o","ooooooo","jjj","lll","ssssssss","mmmm","qqqqq","gggggg","rrrrrrrrrr","iiii","bbbbbbbbb","aaaaaa","hhhh","qqq","zzzzzzzzz","xxxxxxxxx","ww","iiiiiii","pp","vvvvvvvvvv","eeeee","nnnnnnn","nnnnnn","nn","nnnnnnnn","wwwwwwww","vvvvvvvv","fffffffff","aaa","p","ddd","ppppppppp","fffff","aaaaaaaaa","oooooooo","jjjj","xxx","zz","hhhhh","uuuuu","f","ddddddddd","zzzzzz","cccccc","kkkkkk","bbbbbbbb","hhhhhhhhhh","uuuuuuu","cccccccccc","jjjjj","gg","ppp","ccccccccc","rrrrrr","c","cccccccc","yyyyy","uuuu","jjjjjjjj","bb","hhh","l","u","yyyyyy","vvv","mmm","ffffff","eeeeeee","qqqqqqq","zzzzzzzzzz","ggg","zzzzzzz","dddddddddd","jjjjjjj","bbbbb","ttttttt","dddddddd","wwwwwww","vvvvvv","iii","ttttttttt","ggggggg","xx","oooooo","cc","rrrr","qqqq","sssssss","oooo","lllllllll","ii","tttttttttt","uuuuuu","kkkkkkkk","wwwwwwwwww","pppppppppp","uuuuuuuu","yyyyyyy","cccc","ggggg","ddddd","llllllllll","tttt","pppppppp","rrrrrrr","nnnn","x","yyy","iiiiiiiiii","iiiiii","llll","nnnnnnnnnn","aaaaaaaaaa","eeeeeeeeee","m","uuu","rrrrrrrr","h","b","vvvvvvv","ll","vv","mmmmmmm","zzzzz","uu","ccccccc","xxxxxxx","ss","eeeeeeee","llllllll","eeee","y","ppppp","qqqqqq","mmmmmm","gggg","yyyyyyyyy","jjjjjj","rrrrr","a","bbbb","ssssss","sss","ooooo","ffffffffff","kkk","xxxxxxxx","wwwwwwwww","w","iiiiiiii","ffff","dddddd","bbbbbb","uuuuuuuuu","kkkkkkk","gggggggggg","qqqqqqqq","vvvvvvvvv","bbbbbbbbbb","nnnnn","tt","wwww","iiiii","hhhhhhh","zzzzzzzz","ssssssssss","j","fff","bbbbbbb","aaaa","mmmmmmmmmm","jjjjjjjjjj","sssss","yyyyyyyy","hh","q","rrrrrrrrr","mmmmmmmm","wwwww","www","rrr","lllll","uuuuuuuuuu","oo","jjjjjjjjj","dddd","pppp","hhhhhhhhh","kk","gggggggg","xxxxx","vvvv","d","qqqqqqqqq","dd","ggggggggg","t","yyyy","bbb","yyyyyyyyyy","tttttt","ccccc","aa","eeeeee","llllll","kkkkkkkkkk","sssssssss","i","hhhhhh","oooooooooo","wwwwww","ooooooooo","zzzz","k","hhhhhhhh","aaaaa","mmmmm"]
            .into_iter().map(str::to_string).collect();

        let ws = Solution::find_words(board, words);

        judge(ws, &["aaaaaaaa","aaaaaaa","aaaaaa","aaa","aaaaaaaaa","aaaaaaaaaa","a","aaaa","aa","aaaaa"]);
    }

    #[test]
    fn example5() {
        let board = vec![
            vec!['o','a','a','n'],
            vec!['e','t','a','e'],
            vec!['i','h','k','r'],
            vec!['i','f','l','v']];
        let words = ["oath","pea","eat","rain","oathi","oathk","oathf","oate","oathii","oathfi","oathfii"]
        .into_iter().map(str::to_string).collect();

        let ws = Solution::find_words(board, words);

        judge(ws, &["oath","oathk","oathf","oathfi","oathfii","oathi","oathii","oate","eat"]);
    }

    #[test]
    fn example6() {
        let board = vec![
            vec!['m','b','c','d','e','f','g','h','i','j','k','l'],
            vec!['n','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['o','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['p','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['q','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['r','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['s','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['t','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['u','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['v','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['w','a','a','a','a','a','a','a','a','a','a','a'],
            vec!['x','y','z','a','a','a','a','a','a','a','a','a']];
        let words = ["aaaaaaaaaa","aaaaaaaaab","aaaaaaaaac","aaaaaaaaad","aaaaaaaaae","aaaaaaaaaf","aaaaaaaaag","aaaaaaaaah","aaaaaaaaai","aaaaaaaaaj","aaaaaaaaak","aaaaaaaaal","aaaaaaaaam","aaaaaaaaan","aaaaaaaaao","aaaaaaaaap","aaaaaaaaaq","aaaaaaaaar","aaaaaaaaas","aaaaaaaaat","aaaaaaaaau","aaaaaaaaav","aaaaaaaaaw","aaaaaaaaax","aaaaaaaaay","aaaaaaaaaz","aaaaaaaaba","aaaaaaaabb","aaaaaaaabc","aaaaaaaabd","aaaaaaaabe","aaaaaaaabf","aaaaaaaabg","aaaaaaaabh","aaaaaaaabi","aaaaaaaabj","aaaaaaaabk","aaaaaaaabl","aaaaaaaabm","aaaaaaaabn","aaaaaaaabo","aaaaaaaabp","aaaaaaaabq","aaaaaaaabr","aaaaaaaabs","aaaaaaaabt","aaaaaaaabu","aaaaaaaabv","aaaaaaaabw","aaaaaaaabx","aaaaaaaaby","aaaaaaaabz","aaaaaaaaca","aaaaaaaacb","aaaaaaaacc","aaaaaaaacd","aaaaaaaace","aaaaaaaacf","aaaaaaaacg","aaaaaaaach","aaaaaaaaci","aaaaaaaacj","aaaaaaaack","aaaaaaaacl","aaaaaaaacm","aaaaaaaacn","aaaaaaaaco","aaaaaaaacp","aaaaaaaacq","aaaaaaaacr","aaaaaaaacs","aaaaaaaact","aaaaaaaacu","aaaaaaaacv","aaaaaaaacw","aaaaaaaacx","aaaaaaaacy","aaaaaaaacz","aaaaaaaada","aaaaaaaadb","aaaaaaaadc","aaaaaaaadd","aaaaaaaade","aaaaaaaadf","aaaaaaaadg","aaaaaaaadh","aaaaaaaadi","aaaaaaaadj","aaaaaaaadk","aaaaaaaadl","aaaaaaaadm","aaaaaaaadn","aaaaaaaado","aaaaaaaadp","aaaaaaaadq","aaaaaaaadr","aaaaaaaads","aaaaaaaadt","aaaaaaaadu","aaaaaaaadv","aaaaaaaadw","aaaaaaaadx","aaaaaaaady","aaaaaaaadz","aaaaaaaaea","aaaaaaaaeb","aaaaaaaaec","aaaaaaaaed","aaaaaaaaee","aaaaaaaaef","aaaaaaaaeg","aaaaaaaaeh","aaaaaaaaei","aaaaaaaaej","aaaaaaaaek","aaaaaaaael","aaaaaaaaem","aaaaaaaaen","aaaaaaaaeo","aaaaaaaaep","aaaaaaaaeq","aaaaaaaaer","aaaaaaaaes","aaaaaaaaet","aaaaaaaaeu","aaaaaaaaev","aaaaaaaaew","aaaaaaaaex","aaaaaaaaey","aaaaaaaaez","aaaaaaaafa","aaaaaaaafb","aaaaaaaafc","aaaaaaaafd","aaaaaaaafe","aaaaaaaaff","aaaaaaaafg","aaaaaaaafh","aaaaaaaafi","aaaaaaaafj","aaaaaaaafk","aaaaaaaafl","aaaaaaaafm","aaaaaaaafn","aaaaaaaafo","aaaaaaaafp","aaaaaaaafq","aaaaaaaafr","aaaaaaaafs","aaaaaaaaft","aaaaaaaafu","aaaaaaaafv","aaaaaaaafw","aaaaaaaafx","aaaaaaaafy","aaaaaaaafz","aaaaaaaaga","aaaaaaaagb","aaaaaaaagc","aaaaaaaagd","aaaaaaaage","aaaaaaaagf","aaaaaaaagg","aaaaaaaagh","aaaaaaaagi","aaaaaaaagj","aaaaaaaagk","aaaaaaaagl","aaaaaaaagm","aaaaaaaagn","aaaaaaaago","aaaaaaaagp","aaaaaaaagq","aaaaaaaagr","aaaaaaaags","aaaaaaaagt","aaaaaaaagu","aaaaaaaagv","aaaaaaaagw","aaaaaaaagx","aaaaaaaagy","aaaaaaaagz","aaaaaaaaha","aaaaaaaahb","aaaaaaaahc","aaaaaaaahd","aaaaaaaahe","aaaaaaaahf","aaaaaaaahg","aaaaaaaahh","aaaaaaaahi","aaaaaaaahj","aaaaaaaahk","aaaaaaaahl","aaaaaaaahm","aaaaaaaahn","aaaaaaaaho","aaaaaaaahp","aaaaaaaahq","aaaaaaaahr","aaaaaaaahs","aaaaaaaaht","aaaaaaaahu","aaaaaaaahv","aaaaaaaahw","aaaaaaaahx","aaaaaaaahy","aaaaaaaahz","aaaaaaaaia","aaaaaaaaib","aaaaaaaaic","aaaaaaaaid","aaaaaaaaie","aaaaaaaaif","aaaaaaaaig","aaaaaaaaih","aaaaaaaaii","aaaaaaaaij","aaaaaaaaik","aaaaaaaail","aaaaaaaaim","aaaaaaaain","aaaaaaaaio","aaaaaaaaip","aaaaaaaaiq","aaaaaaaair","aaaaaaaais","aaaaaaaait","aaaaaaaaiu","aaaaaaaaiv","aaaaaaaaiw","aaaaaaaaix","aaaaaaaaiy","aaaaaaaaiz","aaaaaaaaja","aaaaaaaajb","aaaaaaaajc","aaaaaaaajd","aaaaaaaaje","aaaaaaaajf","aaaaaaaajg","aaaaaaaajh","aaaaaaaaji","aaaaaaaajj","aaaaaaaajk","aaaaaaaajl","aaaaaaaajm","aaaaaaaajn","aaaaaaaajo","aaaaaaaajp","aaaaaaaajq","aaaaaaaajr","aaaaaaaajs","aaaaaaaajt","aaaaaaaaju","aaaaaaaajv","aaaaaaaajw","aaaaaaaajx","aaaaaaaajy","aaaaaaaajz","aaaaaaaaka","aaaaaaaakb","aaaaaaaakc","aaaaaaaakd","aaaaaaaake","aaaaaaaakf","aaaaaaaakg","aaaaaaaakh","aaaaaaaaki","aaaaaaaakj","aaaaaaaakk","aaaaaaaakl","aaaaaaaakm","aaaaaaaakn","aaaaaaaako","aaaaaaaakp","aaaaaaaakq","aaaaaaaakr","aaaaaaaaks","aaaaaaaakt","aaaaaaaaku","aaaaaaaakv","aaaaaaaakw","aaaaaaaakx","aaaaaaaaky","aaaaaaaakz","aaaaaaaala","aaaaaaaalb","aaaaaaaalc","aaaaaaaald","aaaaaaaale","aaaaaaaalf","aaaaaaaalg","aaaaaaaalh","aaaaaaaali","aaaaaaaalj","aaaaaaaalk","aaaaaaaall","aaaaaaaalm","aaaaaaaaln","aaaaaaaalo","aaaaaaaalp","aaaaaaaalq","aaaaaaaalr","aaaaaaaals","aaaaaaaalt","aaaaaaaalu","aaaaaaaalv","aaaaaaaalw","aaaaaaaalx","aaaaaaaaly","aaaaaaaalz","aaaaaaaama","aaaaaaaamb","aaaaaaaamc","aaaaaaaamd","aaaaaaaame","aaaaaaaamf","aaaaaaaamg","aaaaaaaamh","aaaaaaaami","aaaaaaaamj","aaaaaaaamk","aaaaaaaaml","aaaaaaaamm","aaaaaaaamn","aaaaaaaamo","aaaaaaaamp","aaaaaaaamq","aaaaaaaamr","aaaaaaaams","aaaaaaaamt","aaaaaaaamu","aaaaaaaamv","aaaaaaaamw","aaaaaaaamx","aaaaaaaamy","aaaaaaaamz","aaaaaaaana","aaaaaaaanb","aaaaaaaanc","aaaaaaaand","aaaaaaaane","aaaaaaaanf","aaaaaaaang","aaaaaaaanh","aaaaaaaani","aaaaaaaanj","aaaaaaaank","aaaaaaaanl","aaaaaaaanm","aaaaaaaann","aaaaaaaano","aaaaaaaanp","aaaaaaaanq","aaaaaaaanr","aaaaaaaans","aaaaaaaant","aaaaaaaanu","aaaaaaaanv","aaaaaaaanw","aaaaaaaanx","aaaaaaaany","aaaaaaaanz","aaaaaaaaoa","aaaaaaaaob","aaaaaaaaoc","aaaaaaaaod","aaaaaaaaoe","aaaaaaaaof","aaaaaaaaog","aaaaaaaaoh","aaaaaaaaoi","aaaaaaaaoj","aaaaaaaaok","aaaaaaaaol","aaaaaaaaom","aaaaaaaaon","aaaaaaaaoo","aaaaaaaaop","aaaaaaaaoq","aaaaaaaaor","aaaaaaaaos","aaaaaaaaot","aaaaaaaaou","aaaaaaaaov","aaaaaaaaow","aaaaaaaaox","aaaaaaaaoy","aaaaaaaaoz","aaaaaaaapa","aaaaaaaapb","aaaaaaaapc","aaaaaaaapd","aaaaaaaape","aaaaaaaapf","aaaaaaaapg","aaaaaaaaph","aaaaaaaapi","aaaaaaaapj","aaaaaaaapk","aaaaaaaapl","aaaaaaaapm","aaaaaaaapn","aaaaaaaapo","aaaaaaaapp","aaaaaaaapq","aaaaaaaapr","aaaaaaaaps","aaaaaaaapt","aaaaaaaapu","aaaaaaaapv","aaaaaaaapw","aaaaaaaapx","aaaaaaaapy","aaaaaaaapz","aaaaaaaaqa","aaaaaaaaqb","aaaaaaaaqc","aaaaaaaaqd","aaaaaaaaqe","aaaaaaaaqf","aaaaaaaaqg","aaaaaaaaqh","aaaaaaaaqi","aaaaaaaaqj","aaaaaaaaqk","aaaaaaaaql","aaaaaaaaqm","aaaaaaaaqn","aaaaaaaaqo","aaaaaaaaqp","aaaaaaaaqq","aaaaaaaaqr","aaaaaaaaqs","aaaaaaaaqt","aaaaaaaaqu","aaaaaaaaqv","aaaaaaaaqw","aaaaaaaaqx","aaaaaaaaqy","aaaaaaaaqz","aaaaaaaara","aaaaaaaarb","aaaaaaaarc","aaaaaaaard","aaaaaaaare","aaaaaaaarf","aaaaaaaarg","aaaaaaaarh","aaaaaaaari","aaaaaaaarj","aaaaaaaark","aaaaaaaarl","aaaaaaaarm","aaaaaaaarn","aaaaaaaaro","aaaaaaaarp","aaaaaaaarq","aaaaaaaarr","aaaaaaaars","aaaaaaaart","aaaaaaaaru","aaaaaaaarv","aaaaaaaarw","aaaaaaaarx","aaaaaaaary","aaaaaaaarz","aaaaaaaasa","aaaaaaaasb","aaaaaaaasc","aaaaaaaasd","aaaaaaaase","aaaaaaaasf","aaaaaaaasg","aaaaaaaash","aaaaaaaasi","aaaaaaaasj","aaaaaaaask","aaaaaaaasl","aaaaaaaasm","aaaaaaaasn","aaaaaaaaso","aaaaaaaasp","aaaaaaaasq","aaaaaaaasr","aaaaaaaass","aaaaaaaast","aaaaaaaasu","aaaaaaaasv","aaaaaaaasw","aaaaaaaasx","aaaaaaaasy","aaaaaaaasz","aaaaaaaata","aaaaaaaatb","aaaaaaaatc","aaaaaaaatd","aaaaaaaate","aaaaaaaatf","aaaaaaaatg","aaaaaaaath","aaaaaaaati","aaaaaaaatj","aaaaaaaatk","aaaaaaaatl","aaaaaaaatm","aaaaaaaatn","aaaaaaaato","aaaaaaaatp","aaaaaaaatq","aaaaaaaatr","aaaaaaaats","aaaaaaaatt","aaaaaaaatu","aaaaaaaatv","aaaaaaaatw","aaaaaaaatx","aaaaaaaaty","aaaaaaaatz","aaaaaaaaua","aaaaaaaaub","aaaaaaaauc","aaaaaaaaud","aaaaaaaaue","aaaaaaaauf","aaaaaaaaug","aaaaaaaauh","aaaaaaaaui","aaaaaaaauj","aaaaaaaauk","aaaaaaaaul","aaaaaaaaum","aaaaaaaaun","aaaaaaaauo","aaaaaaaaup","aaaaaaaauq","aaaaaaaaur","aaaaaaaaus","aaaaaaaaut","aaaaaaaauu","aaaaaaaauv","aaaaaaaauw","aaaaaaaaux","aaaaaaaauy","aaaaaaaauz","aaaaaaaava","aaaaaaaavb","aaaaaaaavc","aaaaaaaavd","aaaaaaaave","aaaaaaaavf","aaaaaaaavg","aaaaaaaavh","aaaaaaaavi","aaaaaaaavj","aaaaaaaavk","aaaaaaaavl","aaaaaaaavm","aaaaaaaavn","aaaaaaaavo","aaaaaaaavp","aaaaaaaavq","aaaaaaaavr","aaaaaaaavs","aaaaaaaavt","aaaaaaaavu","aaaaaaaavv","aaaaaaaavw","aaaaaaaavx","aaaaaaaavy","aaaaaaaavz","aaaaaaaawa","aaaaaaaawb","aaaaaaaawc","aaaaaaaawd","aaaaaaaawe","aaaaaaaawf","aaaaaaaawg","aaaaaaaawh","aaaaaaaawi","aaaaaaaawj","aaaaaaaawk","aaaaaaaawl","aaaaaaaawm","aaaaaaaawn","aaaaaaaawo","aaaaaaaawp","aaaaaaaawq","aaaaaaaawr","aaaaaaaaws","aaaaaaaawt","aaaaaaaawu","aaaaaaaawv","aaaaaaaaww","aaaaaaaawx","aaaaaaaawy","aaaaaaaawz","aaaaaaaaxa","aaaaaaaaxb","aaaaaaaaxc","aaaaaaaaxd","aaaaaaaaxe","aaaaaaaaxf","aaaaaaaaxg","aaaaaaaaxh","aaaaaaaaxi","aaaaaaaaxj","aaaaaaaaxk","aaaaaaaaxl","aaaaaaaaxm","aaaaaaaaxn","aaaaaaaaxo","aaaaaaaaxp","aaaaaaaaxq","aaaaaaaaxr","aaaaaaaaxs","aaaaaaaaxt","aaaaaaaaxu","aaaaaaaaxv","aaaaaaaaxw","aaaaaaaaxx","aaaaaaaaxy","aaaaaaaaxz","aaaaaaaaya","aaaaaaaayb","aaaaaaaayc","aaaaaaaayd","aaaaaaaaye","aaaaaaaayf","aaaaaaaayg","aaaaaaaayh","aaaaaaaayi","aaaaaaaayj","aaaaaaaayk","aaaaaaaayl","aaaaaaaaym","aaaaaaaayn","aaaaaaaayo","aaaaaaaayp","aaaaaaaayq","aaaaaaaayr","aaaaaaaays","aaaaaaaayt","aaaaaaaayu","aaaaaaaayv","aaaaaaaayw","aaaaaaaayx","aaaaaaaayy","aaaaaaaayz","aaaaaaaaza","aaaaaaaazb","aaaaaaaazc","aaaaaaaazd","aaaaaaaaze","aaaaaaaazf","aaaaaaaazg","aaaaaaaazh","aaaaaaaazi","aaaaaaaazj","aaaaaaaazk","aaaaaaaazl","aaaaaaaazm","aaaaaaaazn","aaaaaaaazo","aaaaaaaazp","aaaaaaaazq","aaaaaaaazr","aaaaaaaazs","aaaaaaaazt","aaaaaaaazu","aaaaaaaazv","aaaaaaaazw","aaaaaaaazx","aaaaaaaazy","aaaaaaaazz"]
        .into_iter().map(str::to_string).collect();

        let ws = Solution::find_words(board, words);

        judge(ws, &["oath","oathk","oathf","oathfi","oathfii","oathi","oathii","oate","eat"]);
    }
}