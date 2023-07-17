/*
    A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.

    Implement the Trie class:

    Trie() Initializes the trie object.
    void insert(String word) Inserts the string word into the trie.
    boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
    boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.

    Constraints:

    1 <= word.length, prefix.length <= 2000
    word and prefix consist only of lowercase English letters.
    At most 3 * 10^4 calls in total will be made to insert, search, and startsWith.
 */

#[inline] fn char_to_index(c: char) -> u8 { c as u8 - 0x61 }

struct Trie {
    data: Vec<usize>,
    free: Vec<usize>,
    root: usize,
}
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
    #[inline] fn traverse(&self, word: &str) -> Option<usize> {
        let mut node = self.root;
        for c in word.chars() {
            if let Some(next) = self.get(node, c) {
                node = next;
            } else { return None; }
        }
        Some(node)
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
    #[inline] fn search(&self, word: String) -> bool {
        self.traverse(&word).map_or(false, |node| self.is_terminus(node) )
    }
    #[inline] fn starts_with(&self, prefix: String) -> bool {
        self.traverse(&prefix).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::MaybeUninit;

    fn command(cmd: &str, arg: &str, result: bool) {
        static mut TRIE: MaybeUninit<Trie> = MaybeUninit::uninit();
        match cmd {
            "Trie" => unsafe { TRIE.write(Trie::new()); },
            "insert" => unsafe {
                let trie = TRIE.assume_init_mut();
                trie.insert(arg.to_string());
            },
            "search" => unsafe {
                let trie = TRIE.assume_init_ref();
                assert_eq!(trie.search(arg.to_string()), result);
            },
            "startsWith" => unsafe {
                let trie = TRIE.assume_init_ref();
                assert_eq!(trie.starts_with(arg.to_string()), result);
            }
            _ => panic!("unknown command")
        }
    }

    /*
        Input
        ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
        [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
        Output
        [null, null, true, false, true, null, true]

        Explanation
        Trie trie = new Trie();
        trie.insert("apple");
        trie.search("apple");   // return True
        trie.search("app");     // return False
        trie.startsWith("app"); // return True
        trie.insert("app");
        trie.search("app");     // return True
    */
    #[test]
    fn example1() {
        let cmds = vec!["Trie", "insert", "search", "search", "startsWith", "insert", "search"];
        let args = vec!["", "apple", "apple", "app", "app", "app", "app"];
        let results = vec![false, false, true, false, true, false, true];
        for ((cmd, arg), result) in cmds.into_iter().zip(args).zip(results) {
            command(cmd, arg, result);
        }
    }

    /*
        Input
        ["Trie","insert","insert","insert","insert","insert","insert","search","search","search","search","search","search","search","search","search","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith"]
        [[],["app"],["apple"],["beer"],["add"],["jam"],["rental"],["apps"],["app"],["ad"],["applepie"],["rest"],["jan"],["rent"],["beer"],["jam"],["apps"],["app"],["ad"],["applepie"],["rest"],["jan"],["rent"],["beer"],["jam"]]
        Output
        [null,null,null,null,null,null,null,false,true,false,false,false,false,false,true,true,false,true,true,false,false,false,true,true,true]
    */
    #[test]
    fn example2() {
        let cmds = vec!["Trie","insert","insert","insert","insert","insert","insert","search","search","search","search","search","search","search","search","search","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith","startsWith"];
        let args = vec!["","app","apple","beer","add","jam","rental","apps","app","ad","applepie","rest","jan","rent","beer","jam","apps","app","ad","applepie","rest","jan","rent","beer","jam"];
        let results = vec![false,false,false,false,false,false,false,false,true,false,false,false,false,false,true,true,false,true,true,false,false,false,true,true,true];
        for ((cmd, arg), result) in cmds.into_iter().zip(args).zip(results) {
            command(cmd, arg, result);
        }
    }
}