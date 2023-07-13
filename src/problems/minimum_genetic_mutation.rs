/*
    A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.

    Suppose we need to investigate a mutation from a gene string startGene to a gene string endGene where one mutation is defined as one single character changed in the gene string.

    For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
    There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a valid gene string.

    Given the two gene strings startGene and endGene and the gene bank bank, return the minimum number of mutations needed to mutate from startGene to endGene. If there is no such a mutation, return -1.

    Note that the starting point is assumed to be valid, so it might not be included in the bank.

    Constraints:

    0 <= bank.length <= 10
    startGene.length == endGene.length == bank[i].length == 8
    startGene, endGene, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].
 */

pub struct Solution;

use std::{
    collections::{HashSet, VecDeque, HashMap},
    iter::once,
};

fn encode_gene(gene: &str) -> u16 {
    let mut x = 0;
    for (i, c) in gene.chars().enumerate() {
        x |= match c {
            'A' => 0b00,
            'C' => 0b01,
            'G' => 0b10,
            'T' => 0b11,
            _ => panic!("invalid gene")
        } << (i << 1);
    }
    x
}
fn is_valid_mutation(a: u16, b: u16) -> bool {
    let mut diff = a ^ b;
    if diff == 0 { return false; }
    if diff & 0xff == 0 { diff >>= 8; }
    if diff & 0xf == 0 { diff >>= 4; }
    if diff & 0b11 == 0 { diff >>= 2; }
    diff < 4
}

struct BreadthFirstSearcher {
    open: VecDeque<u16>,
    closed: HashSet<u16>,
    g: u8,
}
impl BreadthFirstSearcher {
    fn new(start: u16) -> Self {
        let mut open = VecDeque::new();
        let mut closed = HashSet::new();
        open.push_back(start);
        closed.insert(start);
        BreadthFirstSearcher { open, closed, g: 0 }
    }
    fn step(&mut self, adj: &HashMap<u16, Vec<u16>>, target: &BreadthFirstSearcher) -> Option<i32> {
        self.g += 1;
        for _ in 0..self.open.len() {
            let current = self.open.pop_front().unwrap();
            for next in adj[&current].iter() {
                if !self.closed.insert(*next) { continue; }
                if target.visited(next) { return Some((self.g + target.g) as i32) }
                self.open.push_back(*next);
            }
        }
        if self.open.is_empty() { Some(-1) } else { None }
    }
    fn visited(&self, node: &u16) -> bool { self.closed.contains(node) }
}

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let start_gene = encode_gene(&start_gene);
        let end_gene = encode_gene(&end_gene);
        let bank: Vec<u16> = bank.into_iter()
            .map( |gene| encode_gene(&gene) )
            .chain(once(start_gene)).collect();
        let mut adj = HashMap::new();
        for i in 0..(bank.len() - 1) {
            let a = bank[i];
            for &b in bank.iter().skip(i + 1) {
                if is_valid_mutation(a, b) {
                    adj.entry(a)
                        .and_modify( |n: &mut Vec<u16>| n.push(b) )
                        .or_insert_with( || vec![b] );
                    adj.entry(b)
                        .and_modify( |n: &mut Vec<u16>| n.push(a) )
                        .or_insert_with( || vec![a] );
                }
            }
        }
        if !adj.contains_key(&end_gene) { return -1; }
        let mut front = BreadthFirstSearcher::new(start_gene);
        let mut back = BreadthFirstSearcher::new(end_gene);
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
        Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
        Output: 1
     */
    #[test]
    fn example1() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AACCGGTA".to_string();
        let bank = ["AACCGGTA"].into_iter().map(str::to_string).collect();

        let n = Solution::min_mutation(start_gene, end_gene, bank);

        assert_eq!(n, 1);
    }

    /*
        Input: startGene = "AACCGGTT", endGene = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
        Output: 2
     */
    #[test]
    fn example2() {
        let start_gene = "AACCGGTT".to_string();
        let end_gene = "AAACGGTA".to_string();
        let bank = ["AACCGGTA","AACCGCTA","AAACGGTA"].into_iter().map(str::to_string).collect();

        let n = Solution::min_mutation(start_gene, end_gene, bank);

        assert_eq!(n, 2);
    }
}