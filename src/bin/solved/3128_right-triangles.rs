/*
 * @lc app=leetcode.cn id=3128 lang=rust
 * @lcpr version=30204
 *
 * [3128] 直角三角形
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::number_of_right_triangles(vec2d![[0, 1, 0], [0, 1, 1], [0, 1, 0]]),
        2
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let r = grid.len();
        let c = grid[0].len();
        let mut rows = vec![0; r];
        let mut cols = vec![0; c];
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }
        let mut ans = 0;
        for i in 0..r {
            for j in 0..c {
                if grid[i][j] == 1 {
                    ans += (rows[i] - 1) * (cols[j] - 1);
                }
            }
        }
        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,1,0],[0,1,1],[0,1,0]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,0,0,0],[0,1,0,1],[1,0,0,0]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,0,1],[1,0,0],[1,0,0]]\n
// @lcpr case=end

 */
