/*
 * @lc app=leetcode.cn id=3372 lang=rust
 * @lcpr version=30204
 *
 * [3372] 连接两棵树后最大目标节点数目 I
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_target_nodes(
            vec2d![[0, 1], [0, 2], [2, 3], [2, 4]],
            vec2d![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]],
            2
        ),
        vec![9, 7, 9, 8, 8]
    );
        assert_eq!(
        Solution::max_target_nodes(
            vec2d![[0, 1], [0, 2], [2, 3], [2, 4]],
            vec2d![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]],
            0
        ),
        vec![1, 1, 1, 1, 1]
    );
     assert_eq!(
        Solution::max_target_nodes(
            vec2d![[0, 1], [0, 2], [2, 3], [2, 4]],
            vec2d![[0, 1], [0, 2], [0, 3], [2, 7], [1, 4], [4, 5], [4, 6]],
            1
        ),
        vec![4, 3, 5, 3, 3]
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // 在两个树中各自预计算，然后开搜
        // 但是有贪心法：始终由i连接t2中最大到达数的点，是最大的。证明略
        let m = edges1.len() + 1;
        let n = edges2.len() + 1;
        let mut t1 = vec![vec![]; m];
        let mut t2 = vec![vec![]; n];
        for (t, edges) in [(&mut t1, edges1), (&mut t2, edges2)] {
            for edge in edges {
                let [u, v] = [edge[0] as usize, edge[1] as usize];
                t[u].push(v);
                t[v].push(u);
            }
        }
        // 计算t2每个节点出发到这棵树的k步以内的数量
        let k = k as usize;
        let mut max2 = 0;
        if k > 0 {
            for i in 0..n {
                let mut q = VecDeque::new();
                let mut seen = vec![false; n];
                q.push_back((i, 0));
                seen[i] = true;
                let mut x = 0;
                while let Some((u, d)) = q.pop_front() {
                    x += 1;
                    if d == k - 1 {
                        x += q.len();
                        break;
                    }
                    for &v in &t2[u] {
                        if !seen[v] {
                            q.push_back((v, d + 1));
                            seen[v] = true;
                        }
                    }
                }
                max2 = max2.max(x);
            }
        }

        // 搜索t1
        let mut ans = vec![0; m];
        for i in 0..m {
            let mut q = VecDeque::new();
            let mut seen = vec![false; m];
            q.push_back((i, 0));
            seen[i] = true;
            let mut x = 0;
            while let Some((u, d)) = q.pop_front() {
                x += 1;
                if d == k {
                    x += q.len();
                    break;
                }
                for &v in &t1[u] {
                    if !seen[v] {
                        seen[v] = true;
                        q.push_back((v, d + 1));
                    }
                }
            }
            ans[i] = (x + max2) as i32;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,1],[0,2],[2,3],[2,4]]\n[[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]\n2\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[0,2],[0,3],[0,4]]\n[[0,1],[1,2],[2,3]]\n1\n
// @lcpr case=end

 */
