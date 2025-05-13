/*
 * @lc app=leetcode.cn id=407 lang=rust
 * @lcpr version=30204
 *
 * [407] 接雨水 II
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::trap_rain_water(vec2d![
            [1, 4, 3, 1, 3, 2],
            [3, 2, 1, 3, 2, 4],
            [2, 3, 3, 2, 3, 1]
        ]),
        4
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        // 传播漏水区，边缘一定是漏水区
        let m = height_map.len();
        let n = height_map[0].len();
        let mut water = vec![vec![2 * 10_000; n]; m]; // 水面高度
        let mut queue = Vec::new();
        for j in 0..n {
            queue.push((0, j));
            water[0][j] = height_map[0][j];
            // visited[0][j] = true;
            queue.push((m - 1, j));
            water[m - 1][j] = height_map[m - 1][j];
            // visited[m - 1][j] = true;
        }
        for i in 0..m {
            queue.push((i, 0));
            water[i][0] = height_map[i][0];
            // visited[i][0] = true;
            queue.push((i, n - 1));
            water[i][n - 1] = height_map[i][n - 1];
            // visited[i][n - 1] = true;
        }
        // dbg!(&water);
        while let Some((i, j)) = queue.pop() {
            let cur = water[i as usize][j as usize];
            let i = i as i64;
            let j = j as i64;
            for next in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if 0 > next.0 || next.0 >= m as i64 || 0 > next.1 || next.1 >= n as i64 {
                    continue;
                }
                let (i2, j2) = (next.0 as usize, next.1 as usize);
                let new_water = cur.max(height_map[i2][j2]);
                if new_water < water[i2][j2] {
                    water[i2][j2] = new_water;
                    queue.push((i2, j2));
                }
            }
        }
        // dbg!(&water);
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans += water[i][j] - height_map[i][j];
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,4,3,1,3,2],[3,2,1,3,2,4],[2,3,3,2,3,1]]\n
// @lcpr case=end

// @lcpr case=start
// [[3,3,3,3,3],[3,2,2,2,3],[3,2,1,2,3],[3,2,2,2,3],[3,3,3,3,3]]\n
// @lcpr case=end

 */
