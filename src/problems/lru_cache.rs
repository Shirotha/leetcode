/*
    Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.

    Implement the LRUCache class:

    LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
    int get(int key) Return the value of the key if the key exists, otherwise return -1.
    void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
    The functions get and put must each run in O(1) average time complexity.

    Constraints:

    1 <= capacity <= 3000
    0 <= key <= 10^4
    0 <= value <= 10^5
    At most 2 * 10^5 calls will be made to get and put.
 */

use std::{
    collections::HashMap, 
    ptr::null_mut,
    alloc::{ alloc, Layout },
};

struct Node {
    key: i32, value: i32,
    next: *mut Node,
    prev: *mut Node,
}
impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node { key, value, next: null_mut(), prev: null_mut() }
    }
}

struct LRUCache {
    data: HashMap<i32, *mut Node>,
    head: *mut Node,
    capacity: usize,
}

impl LRUCache {

    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let head = unsafe { alloc(Layout::new::<Node>()) as *mut Node };
        unsafe { (*head).next = head; (*head).prev = head; }
        LRUCache { 
            data: HashMap::with_capacity(capacity), 
            head,
            capacity 
        }
    }

    #[inline] fn detech(&mut self, node: *mut Node) { unsafe {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
    } }
    #[inline] fn attach(&mut self, node: *mut Node) { unsafe {
        (*node).next = (*self.head).next;
        (*node).prev = self.head;
        (*self.head).next = node;
        (*(*node).next).prev = node;
    } }
   
    fn get(&mut self, key: i32) -> i32 {
        if let Some(&node) = self.data.get(&key) {
            self.detech(node);
            self.attach(node);
            unsafe { (*node).value }
        } else { -1 }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let node = if let Some(&node) = self.data.get(&key) {
            self.detech(node);
            node
        } else if self.data.len() == self.capacity {
            let node = unsafe { (*self.head).prev };
            self.detech(node);
            self.data.remove(&unsafe { (*node).key });
            self.data.insert(key, node);
            node
        } else {
            let node = unsafe { alloc(Layout::new::<Node>()) as *mut Node };
            self.data.insert(key, node);
            node
        };
        self.attach(node);
        unsafe { (*node).value = value; (*node).key = key; }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input
        ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
        [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
        Output
        [null, null, null, 1, null, -1, null, -1, 3, 4]

        Explanation
        LRUCache lRUCache = new LRUCache(2);
        lRUCache.put(1, 1); // cache is {1=1}
        lRUCache.put(2, 2); // cache is {1=1, 2=2}
        lRUCache.get(1);    // return 1
        lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        lRUCache.get(2);    // returns -1 (not found)
        lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        lRUCache.get(1);    // return -1 (not found)
        lRUCache.get(3);    // return 3
        lRUCache.get(4);    // return 4
     */
    #[test]
    fn example1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}