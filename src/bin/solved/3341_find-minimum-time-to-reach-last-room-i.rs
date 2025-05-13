/*
 * @lc app=leetcode.cn id=3341 lang=rust
 * @lcpr version=30204
 *
 * [3341] 到达最后一个房间的最少时间 I
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_time_to_reach(vec2d![[0, 4], [4, 4]]), 6);
    assert_eq!(Solution::min_time_to_reach(vec2d![[0, 0, 0], [0, 0, 0]]), 3);
    assert_eq!(
        Solution::min_time_to_reach(vec2d![[94, 79, 62, 27, 69, 84], [6, 32, 11, 82, 42, 30]]),
        72
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::BTreeSet;
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        // 想象(0,0)处发大水了，每个节点在time==move_time[i][j]时开闸，
        // 让水可以流至此处
        let m = move_time.len();
        let n = move_time[0].len();
        let mut time = vec![vec![i32::MAX; n]; m]; // time表示到此处的最小时间，对于起点的相邻点，为move_time+1
        time[0][0] = 0;
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
                let min_time = (move_time[i0][j0]).max(t) + 1;
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
// [[0,0,0],[0,0,0]]\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[1,2]]\n
// @lcpr case=end

 */
