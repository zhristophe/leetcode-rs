/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - n)) {
                return vec![j, i as i32];
            }
            map.insert(n, i as i32);
        }

        vec![]
    }
    #[allow(dead_code)]
    pub fn two_sum_slow(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
