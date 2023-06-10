/*
    Implement the RandomizedSet class:

    RandomizedSet() Initializes the RandomizedSet object.
    bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
    bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
    int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
    You must implement the functions of the class such that each function works in average O(1) time complexity.

    Constraints:

    -2^31 <= val <= 2^31 - 1
    At most 2 * 10^5 calls will be made to insert, remove, and getRandom.
    There will be at least one element in the data structure when getRandom is called.
 */

use std::collections::{HashMap, hash_map::Entry};
use rand::{thread_rng, seq::SliceRandom};

struct RandomizedSet {
    lookup: HashMap<i32, usize>,
    items: Vec<i32>
}

impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet { lookup: HashMap::new(), items: Vec::new() }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let Entry::Vacant(e) = self.lookup.entry(val) {
            e.insert(self.items.len());
            self.items.push(val);
            return true;
        }
        false
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.lookup.remove(&val) {
            self.items.swap_remove(i);
            if i == self.items.len() { return true; }
            self.lookup.insert(self.items[i], i);
            return true;
        }
        false
    }
    
    fn get_random(&self) -> i32 {
        *self.items.choose(&mut thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge_random(set: &RandomizedSet, values: &Vec<i32>, n: usize) {
        assert_eq!(&set.items, values);
        let mut counts: HashMap<i32, usize> = HashMap::with_capacity(values.len());
        for _ in 0..n {
            let x = set.get_random();
            *counts.entry(x).or_insert(0) += 1;
        }
        let n = n as f32;
        let exp = n / (values.len() as f32);
        let var = n / 20.0;
        for &s in counts.values() {
            let s = s as f32;
            assert!(exp - var <= s && s <= exp + var);
        }
    }

    /*
        Input
        ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
        [[], [1], [2], [2], [], [1], [2], []]
        Output
        [null, true, false, true, 2, true, false, 2]

        Explanation
        RandomizedSet randomizedSet = new RandomizedSet();
        randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
        randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
        randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
        randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
        randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
        randomizedSet.insert(2); // 2 was already in the set, so return false.
        randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
     */
    #[test]
    fn example1() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));
        judge_random(&set, &vec![1,2], 1000);
        assert!(set.remove(1));
        assert!(!set.insert(2));
        judge_random(&set, &vec![2], 100);
    }

    /*
        Input
        ["RandomizedSet","insert","insert","remove","insert","remove","getRandom"]
        [[],[0],[1],[0],[2],[1],[]]
        Output
        [null,true,true,true,true,true,2]
     */
    #[test]
    fn example2() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(0));
        assert!(set.insert(1));
        assert!(set.remove(0));
        assert!(set.insert(2));
        assert!(set.remove(1));
        judge_random(&set, &vec![2], 100);
    }
}