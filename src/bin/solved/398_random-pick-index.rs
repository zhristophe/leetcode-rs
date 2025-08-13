/*
 * @lc app=leetcode.cn id=398 lang=rust
 * @lcpr version=30204
 *
 * [398] 随机数索引
 */

// @lcpr-template-start
fn main() {
    let input = vec![1, 2, 3, 3, 3];
    let mut obj = Solution::new(input.clone());
    let ret_1 = obj.pick(3);
    assert_eq!(input[ret_1 as usize], 3);
}
// @lcpr-template-end
// @lc code=start
// use rand::Rng;
use std::collections::HashMap;
struct Solution {
    hash: HashMap<i32, Vec<i32>>,
    seed: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut hash = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            hash.entry(*num).or_insert(vec![]).push(index as i32);
        }
        Self { hash, seed: 0 }
    }

    fn pick(&mut self, target: i32) -> i32 {
        self.seed += 1;
        let indices = self.hash.get(&target).unwrap();
        indices[self.seed % indices.len()]
    }
}

// /**
//  * Your Solution object will be instantiated and called as such:
//  * let obj = Solution::new(nums);
//  * let ret_1: i32 = obj.pick(target);
//  */
// @lc code=end
