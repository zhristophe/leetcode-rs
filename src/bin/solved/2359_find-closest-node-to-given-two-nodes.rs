/*
 * @lc app=leetcode.cn id=2359 lang=rust
 * @lcpr version=30204
 *
 * [2359] 找到离给定两个节点最近的节点
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1), 2);
    assert_eq!(Solution::closest_meeting_node(vec![-1, -1], 0, 1), -1);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        // 搜两个点到其它点的最短距离，然后取最小值
        let mut dis = vec![vec![i32::MAX; edges.len()]; 2];
        for (n, i) in [(node1, 0), (node2, 1)] {
            let mut q = std::collections::VecDeque::new();
            q.push_back((0, n));
            while let Some((d, n)) = q.pop_front() {
                if dis[i][n as usize] != i32::MAX {
                    continue;
                }
                dis[i][n as usize] = d;
                if edges[n as usize] != -1 {
                    q.push_back((d + 1, edges[n as usize]));
                }
            }
        }
        let mut ans = (i32::MAX, -1);
        for i in 0..edges.len() {
            let d = dis[0][i].max(dis[1][i]);
            if d < ans.0 {
                ans = (d, i as i32);
            }
        }

        ans.1
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,2,3,-1]\n0\n1\n
// @lcpr case=end

// @lcpr case=start
// [1,2,-1]\n0\n2\n
// @lcpr case=end

 */
