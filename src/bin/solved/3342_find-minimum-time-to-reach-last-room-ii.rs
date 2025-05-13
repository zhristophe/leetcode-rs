/*
 * @lc app=leetcode.cn id=3342 lang=rust
 * @lcpr version=30204
 *
 * [3342] 到达最后一个房间的最少时间 II
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_time_to_reach(vec2d![[0, 4], [4, 4]]), 7);
    assert_eq!(
        Solution::min_time_to_reach(vec2d![[0, 0, 0, 0], [0, 0, 0, 0]]),
        6
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::BTreeSet;
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        // 类似3341，要记录移动代价
        // 但是无论怎么移动，移到i+j奇数的需要1，偶数需要2
        let m = move_time.len();
        let n = move_time[0].len();
        let mut time = vec![vec![i32::MAX; n]; m]; // time表示到此处的最小时间，对于起点的相邻点，为move_time+1
        time[0][0] = 0;
        time[0][1] = move_time[0][1] + 1;
        time[1][0] = move_time[1][0] + 1;
        let mut bt = BTreeSet::new();
        bt.insert((move_time[1][0] + 1, 1, 0));
        bt.insert((move_time[0][1] + 1, 0, 1));
        while let Some((t, i, j)) = bt.pop_first() {
            // dbg!((t, i, j));
            if i == m - 1 && j == n - 1 {
                return t;
            }
            let mut nbrs = Vec::with_capacity(4);
            if i >= 1 {
                nbrs.push((i - 1, j));
            }
            if i + 1 < m {
                nbrs.push((i + 1, j));
            }
            if j >= 1 {
                nbrs.push((i, j - 1));
            }
            if j + 1 < n {
                nbrs.push((i, j + 1));
            }
            for (i0, j0) in nbrs {
                let cost = if (i0 + j0) % 2 == 1 { 1 } else { 2 };
                let min_time = (move_time[i0][j0]).max(t) + cost;
                if min_time < time[i0][j0] {
                    time[i0][j0] = min_time;
                    bt.insert((min_time, i0, j0));
                }
            }
            // dbg!(&time);
        }

        unreachable!()
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,4],[4,4]]\n
// @lcpr case=end

// @lcpr case=start
// [[0,0,0,0],[0,0,0,0]]\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[1,2]]\n
// @lcpr case=end

 */
