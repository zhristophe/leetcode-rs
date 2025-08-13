/*
 * @lc app=leetcode.cn id=1617 lang=rust
 * @lcpr version=30204
 *
 * [1617] 统计子树中城市之间最大距离
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::count_subgraphs_for_each_diameter(4, vec2d![[1, 2], [2, 3], [2, 4]]),
        vec![3, 4, 0]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // 枚举所有子树，因为n<15
        let n = n as usize;
        let mut tree = vec![vec![]; n];
        for edge in edges {
            tree[edge[0] as usize - 1].push(edge[1] as usize - 1);
            tree[edge[1] as usize - 1].push(edge[0] as usize - 1);
        }
        // 枚举树，用二进制
        let mut ans = vec![0; n - 1];
        for i in 0..1 << n {
            let mut vs = vec![false; n];
            for j in 0..n {
                vs[j] = (i >> j) & 1 != 0;
            }
            // 判断连通性，并计算距离
            // 随便选取root
            let root = 'root: loop {
                for j in 0..n {
                    if vs[j] {
                        break 'root Some(j);
                    }
                }

                break None;
            };
            let Some(root) = root else {
                continue;
            };
            // 计算深度和最大距离
            fn dfs(rt: usize, g: &Vec<Vec<usize>>, vs: &mut [bool]) -> (usize, usize) {
                let mut dist = 0;
                let mut depth = vec![];
                vs[rt] = false; // 移除根节点，防止回溯
                for &u in &g[rt] {
                    if !vs[u] {
                        continue;
                    }
                    let (u_depth, u_dist) = dfs(u, g, vs);
                    depth.push(u_depth);
                    dist = dist.max(u_dist);
                }
                depth.sort_unstable();
                depth.reverse();
                let new_dist = match depth.len() {
                    0 => 0,
                    1 => depth[0],
                    _ => depth[0] + depth[1],
                };
                dist = dist.max(new_dist);
                let depth = depth.iter().max().unwrap_or(&0);

                (depth + 1, dist)
            }
            let (_, dist) = dfs(root, &tree, &mut vs);
            // 如果有节点没访问，则不连通
            if vs.contains(&true) {
                continue;
            }
            if dist > 0 {
                ans[dist - 1] += 1;
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 4\n[[1,2],[2,3],[2,4]]\n
// @lcpr case=end

// @lcpr case=start
// 2\n[[1,2]]\n
// @lcpr case=end

// @lcpr case=start
// 3\n[[1,2],[2,3]]\n
// @lcpr case=end

 */
