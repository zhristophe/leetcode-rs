/*
 * @lc app=leetcode.cn id=191 lang=rust
 * @lcpr version=30204
 *
 * [191] 位1的个数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::hamming_weight(11), 3);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        n.count_ones() as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// 11\n
// @lcpr case=end

// @lcpr case=start
// 128\n
// @lcpr case=end

// @lcpr case=start
// 2147483645\n
// @lcpr case=end

 */
