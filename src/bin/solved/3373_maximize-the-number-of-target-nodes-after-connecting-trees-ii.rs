/*
 * @lc app=leetcode.cn id=3373 lang=rust
 * @lcpr version=30204
 *
 * [3373] 连接两棵树后最大目标节点数目 II
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_target_nodes(
            vec2d![[0, 1], [0, 2], [2, 3], [2, 4]],
            vec2d![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]]
        ),
        vec![8, 7, 7, 8, 8]
    )
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        // 类似1。但是树可以被分成同余系，系中每组的到达点完全一样
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        // 转邻接表
        let [t1, t2] = [edges1, edges2].map(|edges| {
            let n = edges.len() + 1;
            let mut t = vec![vec![]; n];
            for edge in edges {
                let [a, b] = [edge[0] as usize, edge[1] as usize];
                t[a].push(b);
                t[b].push(a);
            }
            t
        });
        // 寻找t2中到达点
        let mut seen = vec![false; m];
        let mut q = std::collections::VecDeque::new();
        macro_rules! push {
            ($seen:expr, $q:expr, $v:expr, $t:expr) => {
                if !$seen[$v] {
                    $seen[$v] = true;
                    $q.push_back(($v, $t));
                }
            };
        }
        push!(seen, q, 0, 0);
        let mut cnt = 0;
        while let Some((u, t)) = q.pop_front() {
            if t % 2 == 0 {
                cnt += 1;
            }
            for &v in &t2[u] {
                push!(seen, q, v, t + 1);
            }
        }
        // t1永远可以连接最大的同余系
        let maxv = cnt.max(m as i32 - cnt);
        // 搜索t1，找到同余系
        let mut ans = vec![0i32; n];
        let mut seen = vec![false; n];
        let mut q = std::collections::VecDeque::new();
        push!(seen, q, 0, 0);
        let mut cnt = [0, 0];
        while let Some((u, t)) = q.pop_front() {
            ans[u] = t % 2; // 建立同余系
            cnt[t as usize % 2] += 1;
            for &v in &t1[u] {
                push!(seen, q, v, t + 1);
            }
        }
        for i in 0..n {
            ans[i] = cnt[ans[i] as usize] + maxv;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,1],[0,2],[2,3],[2,4]]\n[[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[0,2],[0,3],[0,4]]\n[[0,1],[1,2],[2,3]]\n
// @lcpr case=end

 */
