/*
    The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values.

    For example, for arr = [2,3,4], the median is 3.
    For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
    Implement the MedianFinder class:

    MedianFinder() initializes the MedianFinder object.
    void addNum(int num) adds the integer num from the data stream to the data structure.
    double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.

    Constraints:

    -105 <= num <= 105
    There will be at least one element in the data structure before calling findMedian.
    At most 5 * 104 calls will be made to addNum and findMedian.
 */

use std::{collections::BinaryHeap, cmp::Reverse};

enum Center { Empty, Odd(i32), Even(i32, i32) }
use Center::*;
impl Center { 
    #[inline] fn median(&self) -> f64 {
        match *self { 
            Empty => panic!(),  
            Odd(a) => a as f64,
            Even(a, b) => (a + b) as f64 / 2.0
        }
    }
}

struct MedianFinder {
    low: BinaryHeap<i32>,
    high: BinaryHeap<Reverse<i32>>,
    center: Center,
}
impl MedianFinder {
    fn new() -> Self { MedianFinder { low: BinaryHeap::new(), high: BinaryHeap::new(), center: Empty } }
    fn add_num(&mut self, num: i32) {
        self.center = match self.center {
            Empty => Odd(num),
            Odd(a) => if num <= a {
                self.low.push(num);
                Even(self.low.pop().unwrap(), a)
            } else { 
                self.high.push(Reverse(num));
                Even(a, self.high.pop().unwrap().0)
            },
            Even(a, b) => if num <= a {
                self.low.push(num); self.high.push(Reverse(b));
                Odd(a)
            } else if num >= b {
                self.low.push(a); self.high.push(Reverse(num));
                Odd(b)
            } else {
                self.low.push(a); self.high.push(Reverse(b));
                Odd(num)
            }
        };
    }
    fn find_median(&self) -> f64 { self.center.median() }
}

#[cfg(test)]
mod test {
    use super::*;

    enum Command { AddNum(i32), FindMedian(f64) }
    use Command::*;

    fn judge(cmds: &[Command]) {
        const E: f64 = 1e-5;
        let mut mf = MedianFinder::new();
        for cmd in cmds.iter() {
            match cmd {
                AddNum(num) => mf.add_num(*num),
                FindMedian(med) => { let x = mf.find_median(); assert!(*med - E <= x && x <= *med + E); }
            }
        }
    }

    /*
        Input
        ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
        [[], [1], [2], [], [3], []]
        Output
        [null, null, null, 1.5, null, 2.0]

        Explanation
        MedianFinder medianFinder = new MedianFinder();
        medianFinder.addNum(1);    // arr = [1]
        medianFinder.addNum(2);    // arr = [1, 2]
        medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
        medianFinder.addNum(3);    // arr[1, 2, 3]
        medianFinder.findMedian(); // return 2.0
     */
    #[test]
    fn example1() {
        judge(&[AddNum(1), AddNum(2), FindMedian(1.5), AddNum(3), FindMedian(2.0)]);
    }
}