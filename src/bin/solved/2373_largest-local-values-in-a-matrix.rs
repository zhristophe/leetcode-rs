/*
 * @lc app=leetcode.cn id=2373 lang=rust
 * @lcpr version=30204
 *
 * [2373] 矩阵中的局部最大值
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::largest_local(vec2d![
            [9, 9, 8, 1],
            [5, 6, 2, 6],
            [8, 2, 6, 4],
            [6, 2, 2, 2]
        ]),
        vec2d![[9, 9], [8, 6]]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut ans = vec![vec![0; n - 2]; n - 2];
        for i in 0..n - 2 {
            for j in 0..n - 2 {
                for x in i..i + 3 {
                    for y in j..j + 3 {
                        ans[i][j] = ans[i][j].max(grid[x][y]);
                    }
                }
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]\n
// @lcpr case=end

 */
