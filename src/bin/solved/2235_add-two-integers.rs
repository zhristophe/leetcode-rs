/*
 * @lc app=leetcode id=2235 lang=rust
 *
 * [2235] Add Two Integers
 */

// @lc code=start
impl Solution {
    // Your memory usage beats 63.54 % of rust submissions (2.1 MB)
    // WHY???
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::sum(12, 5), 17);
}
