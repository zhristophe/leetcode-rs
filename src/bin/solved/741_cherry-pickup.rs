/*
 * @lc app=leetcode.cn id=741 lang=rust
 * @lcpr version=30204
 *
 * [741] 摘樱桃
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::cherry_pickup(vec2d![[0, 1, -1], [1, 0, -1], [1, 1, 1]]),
        5
    );
    assert_eq!(
        Solution::cherry_pickup(vec2d![[1, 1, -1], [1, -1, 1], [-1, 1, 1]]),
        0
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        // 等价于走两次
        // 怎么记录两次的路径？当成两个人同时出发，这样只记录x坐标即可
        let n = grid.len();
        let mut dp = vec![vec![vec![i32::MIN; n]; n]; 2 * n - 1];
        dp[0][0][0] = grid[0][0];
        for k in 1..=n * 2 - 2 {
            let start = if k < n { 0 } else { k - n + 1 };
            let end = n.min(k + 1);
            for x1 in start..end {
                let y1 = k - x1;
                if grid[x1][y1] == -1 {
                    continue;
                }
                for x2 in x1..end {
                    let y2 = k - x2;
                    if grid[x2][y2] == -1 {
                        continue;
                    }
                    let mut res = dp[k - 1][x1][x2];
                    if x1 > 0 {
                        res = res.max(dp[k - 1][x1 - 1][x2]);
                    }
                    if x2 > 0 {
                        res = res.max(dp[k - 1][x1][x2 - 1]);
                    }
                    if x1 > 0 && x2 > 0 {
                        res = res.max(dp[k - 1][x1 - 1][x2 - 1]);
                    }
                    res += grid[x1][y1];
                    if x1 != x2 {
                        res += grid[x2][y2];
                    }
                    dp[k][x1][x2] = res;
                }
            }
        }
        // dbg!(&dp);

        0.max(dp[2 * n - 2][n - 1][n - 1])
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,1,-1],[1,0,-1],[1,1,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,1,-1],[1,-1,1],[-1,1,1]]\n
// @lcpr case=end

 */
