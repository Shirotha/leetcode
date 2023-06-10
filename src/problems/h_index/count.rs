/*
    Given an array of integers citations where citations[i] is the number of citations a researcher received for their ith paper, return the researcher's h-index.

    According to the definition of h-index on Wikipedia: The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.

    Constraints:

    n == citations.length
    1 <= n <= 5000
    0 <= citations[i] <= 1000
 */

pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations.to_vec();
        citations.sort_unstable();
        let mut h = 0;
        let l = citations.len();
        for i in (0..l).rev() {
            if citations[i] as usize >= l - i { h += 1; }
        }
        h
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: citations = [3,0,6,1,5]
        Output: 3
        Explanation: [3,0,6,1,5] means the researcher has 5 papers in total and each of them had received 3, 0, 6, 1, 5 citations respectively.
        Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
     */
    #[test]
    fn example1() {
        let citations = vec![3,0,6,1,5];

        let h = Solution::h_index(citations);

        assert_eq!(h, 3);
    }

    /*
        Input: citations = [1,3,1]
        Output: 1
     */
    #[test]
    fn example2() {
        let citations = vec![1,3,1];

        let h = Solution::h_index(citations);

        assert_eq!(h, 1);
    }
}