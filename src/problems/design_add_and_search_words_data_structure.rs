/*
    Design a data structure that supports adding new words and finding if a string matches any previously added string.

    Implement the WordDictionary class:

    WordDictionary() Initializes the object.
    void addWord(word) Adds word to the data structure, it can be matched later.
    bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.

    Constraints:

    1 <= word.length <= 25
    word in addWord consists of lowercase English letters.
    word in search consist of '.' or lowercase English letters.
    There will be at most 2 dots in word for search queries.
    At most 10^4 calls will be made to addWord and search.
 */

#[inline] fn char_to_index(c: char) -> u8 { c as u8 - 0x61 }

struct WordDictionary {
    data: Vec<usize>,
    free: Vec<usize>,
    root: usize,
}
const CHILDREN: usize = usize::MAX >> 1;
const TERMINUS: usize = 1 << (usize::BITS - 1);
const NULL: usize = 0;
impl WordDictionary {
    #[inline] fn new() -> Self { WordDictionary { data: vec![0, 0], free: vec![NULL; 27], root: 1 } }
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
    #[inline] fn relink(&mut self, node: usize, chr: char, child: usize) {
        let children = self.data[node] & CHILDREN;
        let bit = 1 << char_to_index(chr);
        let child_ptr = node + 1 + (children & (bit - 1)).count_ones() as usize;
        self.data[child_ptr] = child;
    }
    #[inline] fn get(&self, node: usize, chr: char) -> Option<usize> {
        let children = self.data[node] & CHILDREN;
        let bit = 1 << char_to_index(chr);
        if children & bit == 0 { return None; }
        let child_ptr = node + 1 + (children & (bit - 1)).count_ones() as usize;
        Some(self.data[child_ptr])
    }
    #[inline] fn get_or_add(&mut self, node: usize, chr: char) -> (usize, usize) {
        let children = self.data[node] & CHILDREN;
        let bit = 1 << char_to_index(chr);
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
    #[inline] fn traverse(&self, mut node: usize, word: &str) -> Result<Option<usize>,(usize, usize)> {
        for (i, c) in word.chars().enumerate() {
            if c == '.' { return Err((node, i + 1)) }
            if let Some(next) = self.get(node, c) {
                node = next;
            } else { return Ok(None); }
        }
        Ok(Some(node))
    }
    #[inline] fn add_word(&mut self, word: String) {
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
    #[inline] fn local_search(&self, node: usize, frag: &str) -> bool {
        match self.traverse(node, frag) {
            Err((node, i)) => {
                let frag = &frag[i..];
                for c in 'a'..='z' {
                    if let Some(child) = self.get(node, c) {
                        if self.local_search(child, frag) { return true; }
                    }
                }
                false
            },
            Ok(Some(node)) => self.is_terminus(node),
            Ok(None) => false,
        }
    }
    #[inline] fn search(&self, word: String) -> bool {
        self.local_search(self.root, &word)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::MaybeUninit;

    fn command(cmd: &str, arg: &str, result: bool) {
        static mut DICT: MaybeUninit<WordDictionary> = MaybeUninit::uninit();
        match cmd {
            "WordDictionary" => unsafe { DICT.write(WordDictionary::new()); },
            "addWord" => unsafe {
                let dict = DICT.assume_init_mut();
                dict.add_word(arg.to_string());
            },
            "search" => unsafe {
                let dict = DICT.assume_init_ref();
                assert_eq!(dict.search(arg.to_string()), result);
            },
            _ => panic!("unknown command")
        }
    }

    /*
        Input
        ["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
        [[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
        Output
        [null,null,null,null,false,true,true,true]

        Explanation
        WordDictionary wordDictionary = new WordDictionary();
        wordDictionary.addWord("bad");
        wordDictionary.addWord("dad");
        wordDictionary.addWord("mad");
        wordDictionary.search("pad"); // return False
        wordDictionary.search("bad"); // return True
        wordDictionary.search(".ad"); // return True
        wordDictionary.search("b.."); // return True
     */
    #[test]
    fn example1() {
        let cmds = vec!["WordDictionary","addWord","addWord","addWord","search","search","search","search"];
        let args = vec!["","bad","dad","mad","pad","bad",".ad","b.."];
        let results = vec![false,false,false,false,false,true,true,true];
        for ((cmd, arg), result) in cmds.into_iter().zip(args).zip(results) {
            command(cmd, arg, result);
        }
    }
}