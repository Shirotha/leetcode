/*
    RandomizedCollection is a data structure that contains a collection of numbers, possibly duplicates (i.e., a multiset). It should support inserting and removing specific elements and also reporting a random element.

    Implement the RandomizedCollection class:

    RandomizedCollection() Initializes the empty RandomizedCollection object.
    bool insert(int val) Inserts an item val into the multiset, even if the item is already present. Returns true if the item is not present, false otherwise.
    bool remove(int val) Removes an item val from the multiset if present. Returns true if the item is present, false otherwise. Note that if val has multiple occurrences in the multiset, we only remove one of them.
    int getRandom() Returns a random element from the current multiset of elements. The probability of each element being returned is linearly related to the number of the same values the multiset contains.
    You must implement the functions of the class such that each function works on average O(1) time complexity.

    Note: The test cases are generated such that getRandom will only be called if there is at least one item in the RandomizedCollection.

    Constraints:

    -231 <= val <= 231 - 1
    At most 2 * 10^5 calls in total will be made to insert, remove, and getRandom.
    There will be at least one element in the data structure when getRandom is called.
 */

use std::collections::{HashMap, hash_map::Entry};
use rand::{thread_rng, seq::SliceRandom};

struct RandomizedCollection {
    lookup: HashMap<i32, Vec<usize>>,
    items: Vec<i32>,
}

impl RandomizedCollection {

    fn new() -> Self {
        RandomizedCollection { lookup: HashMap::new(), items: Vec::new() }
    }

    fn insert(&mut self, val: i32) -> bool {
        match self.lookup.entry(val) {
            Entry::Vacant(e) => {
                let v = e.insert(Vec::new());
                v.push(self.items.len());
                self.items.push(val);
                true
            },
            Entry::Occupied(ref mut e) => {
                e.get_mut().push(self.items.len());
                self.items.push(val);
                false
            }
        }    
    }
    
    fn remove(&mut self, val: i32) -> bool {
        let mut i = usize::MAX;
        let mut result = false;
        if let Entry::Occupied(mut e) = self.lookup.entry(val) {
            let v = e.get_mut();
            i = v.pop().unwrap();
            self.items.swap_remove(i);
            if v.is_empty() { e.remove(); }
            result = true;
        }
        let j = self.items.len();
        if i < j {
            if let Entry::Occupied(mut e) =  self.lookup.entry(self.items[i]) {
                let v = e.get_mut();
                v.retain(|x| *x != j);
                v.push(i);
            }
        }
        result
    }

    fn get_random(&self) -> i32 {
        *self.items.choose(&mut thread_rng()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge_random(col: &RandomizedCollection, values: &mut Vec<i32>, _n: usize) {
        let mut items = col.items.to_vec();
        items.sort_unstable();
        values.sort_unstable();
        assert_eq!(&items, values);
        /*
        let mut counts: HashMap<i32, usize> = HashMap::with_capacity(values.len());
        for _ in 0..n {
            let x = col.get_random();
            *counts.entry(x).or_insert(0) += 1;
        }
        let n = n as f32;
        let exp = n / (values.len() as f32);
        let var = n / 20.0;
        for &s in counts.values() {
            let s = s as f32;
            assert!(exp - var <= s && s <= exp + var);
        }
         */
    }

    /*
        Input
        ["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
        [[], [1], [1], [2], [], [1], []]
        Output
        [null, true, false, true, 2, true, 1]

        Explanation
        RandomizedCollection randomizedCollection = new RandomizedCollection();
        randomizedCollection.insert(1);   // return true since the collection does not contain 1.
                                        // Inserts 1 into the collection.
        randomizedCollection.insert(1);   // return false since the collection contains 1.
                                        // Inserts another 1 into the collection. Collection now contains [1,1].
        randomizedCollection.insert(2);   // return true since the collection does not contain 2.
                                        // Inserts 2 into the collection. Collection now contains [1,1,2].
        randomizedCollection.getRandom(); // getRandom should:
                                        // - return 1 with probability 2/3, or
                                        // - return 2 with probability 1/3.
        randomizedCollection.remove(1);   // return true since the collection contains 1.
                                        // Removes 1 from the collection. Collection now contains [1,2].
        randomizedCollection.getRandom(); // getRandom should return 1 or 2, both equally likely.
     */
    #[test]
    fn example1() {
        let mut col = RandomizedCollection::new();
        assert!(col.insert(1));
        assert!(!col.insert(1));
        assert!(col.insert(2));
        judge_random(&col, &mut vec![1,1,2], 1000);
        assert!(col.remove(1));
        judge_random(&col, &mut vec![1,2], 1000);
    }

    /*
        Input
        ["RandomizedCollection","insert","remove","insert"]
        [[],[1],[1],[1]]
        Output
        [null,true,true,true]
     */
    #[test]
    fn example2() {
        let mut col = RandomizedCollection::new();
        assert!(col.insert(1));
        assert!(col.remove(1));
        assert!(col.insert(1));
    }

    /*
        Input
        ["RandomizedCollection","insert","remove","insert","getRandom","remove","insert","getRandom"]
        [[],[1],[2],[2],[],[1],[2],[]]
        Output
        [null,true,false,true,2,true,false,2]
     */
    #[test]
    fn example3() {
        let mut col = RandomizedCollection::new();
        assert!(col.insert(1));
        assert!(!col.remove(2));
        assert!(col.insert(2));
        judge_random(&col, &mut vec![1,2], 1000);
        assert!(col.remove(1));
        assert!(!col.insert(2));
        judge_random(&col, &mut vec![2,2], 100);
    }
}