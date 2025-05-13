/*
 * @lc app=leetcode.cn id=1232 lang=rust
 * @lcpr version=30204
 *
 * [1232] 缀点成线
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::check_straight_line(vec2d![[1, 1], [2, 2], [3, 4], [4, 5], [5, 6], [7, 7]]),
        false
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 2 {
            return true;
        }
        let (x1, y1) = (
            coordinates[0][0] - coordinates[1][0],
            coordinates[0][1] - coordinates[1][1],
        );
        for i in 2..coordinates.len() {
            let (x2, y2) = (
                coordinates[i - 1][0] - coordinates[i][0],
                coordinates[i - 1][1] - coordinates[i][1],
            );
            if x1 * y2 != x2 * y1 {
                return false;
            }
        }
        true
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]]\n
// @lcpr case=end

 */
