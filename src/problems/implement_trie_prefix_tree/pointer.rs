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

use std::alloc::{alloc_zeroed, Layout};

#[inline] fn char_to_index(c: char) -> usize { c as usize - 0x61 }

struct Node {
    children: [*mut Node; 26],
    is_terminus: bool,
}
impl Node {
    #[inline] fn alloc() -> *mut Self {
        unsafe { alloc_zeroed(Layout::new::<Self>()) as *mut _ }
    }
    #[inline] fn get(&self, c: char) -> *mut Self {
        self.children[char_to_index(c)]
    }
    #[inline] fn get_or_add(&mut self, c: char) -> *mut Self {
        let i = char_to_index(c);
        let child = self.children[i];
        if child.is_null() {
            let child = Node::alloc();
            self.children[i] = child;
            child
        } else { child }
    }
    #[inline] fn is_terminus(&self) -> bool { self.is_terminus }
    #[inline] fn set_terminus(&mut self) { self.is_terminus = true; }
}

struct Trie {
    root: *mut Node
}
impl Trie {
    #[inline] fn new() -> Self { Trie { root: Node::alloc() } }
    #[inline] fn insert(&self, word: String) {
        let mut node = self.root;
        for c in word.chars() {
            node = unsafe { (*node).get_or_add(c) };
        }
        unsafe { (*node).set_terminus(); }
    }
    #[inline] fn traverse(&self, word: &str) -> *mut Node {
        let mut node = self.root;
        for c in word.chars() {
            node = unsafe { (*node).get(c) };
            if node.is_null() { break; }
        }
        node
    }
    #[inline] fn search(&self, word: String) -> bool {
        let node = self.traverse(&word);
        !node.is_null() && unsafe { (*node).is_terminus() }
    }
    #[inline] fn starts_with(&self, prefix: String) -> bool {
        !self.traverse(&prefix).is_null()
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        let trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}