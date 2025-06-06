/*
 * @lc app=leetcode.cn id=174 lang=rust
 * @lcpr version=30204
 *
 * [174] 地下城游戏
 */

// @lcpr-template-start
struct Solution;
fn main() {}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let f = vec![vec![0; n]; m];
        // 维护两个量：到此处的最大血量和对应的需要血量

        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[-2,-3,3],[-5,-10,1],[10,30,-5]]\n
// @lcpr case=end

// @lcpr case=start
// [[0]]\n
// @lcpr case=end

 */
