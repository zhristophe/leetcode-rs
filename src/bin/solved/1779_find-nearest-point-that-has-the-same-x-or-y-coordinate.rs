/*
 * @lc app=leetcode.cn id=1779 lang=rust
 * @lcpr version=30204
 *
 * [1779] 找到最近的有相同 X 或 Y 坐标的点
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::nearest_valid_point(3, 4, vec2d![[1, 2], [3, 1], [2, 4], [2, 3], [4, 4]]),
        2
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut ans = (i32::MAX, -1);
        for (i, p) in points.into_iter().enumerate() {
            if x == p[0] || y == p[1] {
                let d = (x - p[0]).abs() + (y - p[1]).abs();
                if d == 0 {
                    return i as i32;
                } else if d < ans.0 {
                    ans = (d, i as i32);
                }
            }
        }
        ans.1
    }
}
// @lc code=end

/*
// @lcpr case=start
// 3\n4\n[[1,2],[3,1],[2,4],[2,3],[4,4]]\n
// @lcpr case=end

// @lcpr case=start
// 3\n4\n[[3,4]]\n
// @lcpr case=end

// @lcpr case=start
// 3\n4\n[[2,3]]\n
// @lcpr case=end

 */
