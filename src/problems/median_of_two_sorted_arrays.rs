/*
    Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

    The overall run time complexity should be O(log (m+n)).

    Constraints:

    nums1.length == m
    nums2.length == n
    0 <= m <= 1000
    0 <= n <= 1000
    1 <= m + n <= 2000
    -10^6 <= nums1[i], nums2[i] <= 10^6
 */

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len(); let n = nums2.len();
        let mut k = (m + n - 1) >> 1;
        let mut i0 = 0; let mut j0 = 0;
        while i0 != m && j0 != n && k > 1 {
            let d = (k >> 1).min(m - i0).min(n - j0);
            let i = i0 + d - 1; let j = j0 + d - 1;
            if nums1[i] < nums2[j] { i0 = i + 1; } else { j0 = j + 1; }
            k -= d;
        }
        if (m + n) & 1 == 0 {
            if i0 == m { j0 += k; } else if j0 == n { i0 += k; }
            else if k == 1 { if nums1[i0] < nums2[j0] { i0 += 1; } else { j0 += 1; } }
            let a = if let Some(&a) = nums1.get(i0) {
                if let Some(&b) = nums2.get(j0) {
                    if a < b { i0 += 1; a } else { j0 += 1; b }
                } else { i0 += 1; a }
            } else { let b = nums2[j0]; j0 += 1; b };
            let b = nums1.get(i0)
                .map( |&a| nums2.get(j0).map( |&b| a.min(b) ).unwrap_or(a) )
                .unwrap_or_else( || nums2[j0] );
            (a + b) as f64 / 2.0
        } else if i0 == m { nums2[j0 + k] as f64 }
        else if j0 == n { nums1[i0 + k] as f64 }
        else {
            if k == 1 { if nums1[i0] < nums2[j0] { i0 += 1; } else { j0 += 1; } }
            nums1.get(i0)
                .map( |&a| nums2.get(j0).map( |&b| a.min(b) ).unwrap_or(a) )
                .unwrap_or_else( || nums2[j0] ) as f64
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn judge(m: f64, x: f64) {
        const E: f64 = 1e-10;
        assert!(x - E <= m && m <= x + E);
    }

    /*
        Input: nums1 = [1,3], nums2 = [2]
        Output: 2.00000
        Explanation: merged array = [1,2,3] and median is 2.
     */
    #[test]
    fn example1() {
        let nums1 = vec![1,3];
        let nums2 = vec![2];

        let m = Solution::find_median_sorted_arrays(nums1, nums2);

        judge(m, 2.0);
    }

    /*
        Input: nums1 = [1,2], nums2 = [3,4]
        Output: 2.50000
        Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
     */
    #[test]
    fn example2() {
        let nums1 = vec![1,2];
        let nums2 = vec![3,4];

        let m = Solution::find_median_sorted_arrays(nums1, nums2);

        judge(m, 2.5);
    }
}