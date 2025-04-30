/*
 * @lc app=leetcode id=898 lang=rust
 *
 * [898] Bitwise ORs of Subarrays
 */

// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut ans = HashSet::<i32>::new();
        let mut cur = HashSet::new();
        cur.insert(0);
        for x in arr {
            let mut next = HashSet::new();
            for &y in cur.iter() {
                next.insert(x | y);
            }
            next.insert(x);
            cur = next;
            ans.extend(cur.iter());
        }
        ans.len() as i32
    }
}
// @lc code=end

struct Solution;
fn main() {
    // assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 1, 2]), 3);
    assert_eq!(Solution::subarray_bitwise_o_rs(vec![1, 2, 4]), 6);
}
