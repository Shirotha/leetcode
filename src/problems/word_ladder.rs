/*
    A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s_1 -> s_2 -> ... -> s_k such that:

    Every adjacent pair of words differs by a single letter.
    Every s_i for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
    sk == endWord
    Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.

    Constraints:

    1 <= beginWord.length <= 10
    endWord.length == beginWord.length
    1 <= wordList.length <= 5000
    wordList[i].length == beginWord.length
    beginWord, endWord, and wordList[i] consist of lowercase English letters.
    beginWord != endWord
    All the words in wordList are unique.
 */

pub struct Solution;

use std::{
    collections::{HashSet, VecDeque, HashMap},
    iter::once,
};

fn encode_word(word: &str) -> u64 {
    let mut x = 0;
    for (i, &c) in word.as_bytes().iter().enumerate() {
        x |= (c as u64 - 0x61) << (i * 6);
    }
    x
}
fn is_adjacent(a: u64, b: u64) -> bool {
    let mut diff = a ^ b;
    if diff == 0 { return false; }
    if diff & 0x3fffffff == 0 { diff >>= 30; }
    if diff & 0x3ffff == 0 { diff >>= 18; }
    if diff & 0x3f == 0 { diff >>= 6; }
    if diff & 0x3f == 0 { diff >>= 6; }
    diff < 64
}

struct BreadthFirstSearcher {
    open: VecDeque<u64>,
    closed: HashSet<u64>,
    g: u16,
}
impl BreadthFirstSearcher {
    fn new(start: u64) -> Self {
        let mut open = VecDeque::new();
        let mut closed = HashSet::new();
        open.push_back(start);
        closed.insert(start);
        BreadthFirstSearcher { open, closed, g: 0 }
    }
    fn step(&mut self, adj: &HashMap<u64, Vec<u64>>, target: &BreadthFirstSearcher) -> Option<i32> {
        self.g += 1;
        for _ in 0..self.open.len() {
            let current = self.open.pop_front().unwrap();
            for next in adj[&current].iter() {
                if !self.closed.insert(*next) { continue; }
                if target.visited(next) { return Some((self.g + target.g + 1) as i32) }
                self.open.push_back(*next);
            }
        }
        if self.open.is_empty() { Some(0) } else { None }
    }
    fn visited(&self, node: &u64) -> bool { self.closed.contains(node) }
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let begin_word = encode_word(&begin_word);
        let end_word = encode_word(&end_word);
        let word_list: Vec<u64> = word_list.into_iter()
            .map( |gene| encode_word(&gene) )
            .chain(once(begin_word)).collect();
        let mut adj = HashMap::new();
        for i in 0..(word_list.len() - 1) {
            let a = word_list[i];
            for &b in word_list.iter().skip(i + 1) {
                if is_adjacent(a, b) {
                    adj.entry(a)
                        .and_modify( |n: &mut Vec<u64>| n.push(b) )
                        .or_insert_with( || vec![b] );
                    adj.entry(b)
                        .and_modify( |n: &mut Vec<u64>| n.push(a) )
                        .or_insert_with( || vec![a] );
                }
            }
        }
        if !adj.contains_key(&end_word) { return 0; }
        let mut front = BreadthFirstSearcher::new(begin_word);
        let mut back = BreadthFirstSearcher::new(end_word);
        loop {
            if let Some(result) = front.step(&adj, &back) { return result; }
            if let Some(result) = back.step(&adj, &front) { return result; }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
        Output: 5
        Explanation: One shortest transformation sequence is "hit" -> "hot" -> "dot" -> "dog" -> cog", which is 5 words long.
     */
    #[test]
    fn example1() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot","dot","dog","lot","log","cog"].into_iter().map(str::to_string).collect();

        let n = Solution::ladder_length(begin_word, end_word, word_list);

        assert_eq!(n, 5);
    }

    /*
        Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
        Output: 0
        Explanation: The endWord "cog" is not in wordList, therefore there is no valid transformation sequence.
     */
    #[test]
    fn example2() {
        let begin_word = "hit".to_string();
        let end_word = "cog".to_string();
        let word_list = ["hot","dot","dog","lot","log"].into_iter().map(str::to_string).collect();

        let n = Solution::ladder_length(begin_word, end_word, word_list);

        assert_eq!(n, 0);
    }
}