/*
 * @lc app=leetcode.cn id=1857 lang=rust
 * @lcpr version=30204
 *
 * [1857] 有向图中最大颜色值
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::largest_path_value(
            "abaca".to_string(),
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
        ),
        3
    )
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        // 利用拓扑排序，同时获取环路信息和最大颜色值
        let c = colors.as_bytes();
        let n = c.len();
        let mut d = vec![0; n]; // 出度。用入度是等价的
        let mut pred = HashMap::new();
        let mut succ = HashMap::new();
        for edge in &edges {
            d[edge[0] as usize] += 1;
            pred.entry(edge[1]).or_insert(vec![]).push(edge[0]);
            succ.entry(edge[0]).or_insert(vec![]).push(edge[1]);
        }
        let mut q = std::collections::VecDeque::new(); // 队列
        let c = colors.as_bytes();
        let mut colors = vec![[0; 26]; n]; // 每个点出发的最大颜色值
        let mut cnt = 0; // 处理过的节点数

        for (v, &d) in d.iter().enumerate() {
            if d == 0 {
                q.push_back(v as i32);
            }
        }

        while let Some(u) = q.pop_front() {
            let mut color = [0; 26];
            if let Some(pred) = pred.get(&u) {
                for &v in pred {
                    d[v as usize] -= 1; // 出度-1
                    if d[v as usize] == 0 {
                        q.push_back(v);
                    }
                }
            }
            if let Some(succ) = succ.get(&u) {
                for &v in succ {
                    for i in 0..26 {
                        if colors[v as usize][i] > color[i] {
                            color[i] = colors[v as usize][i];
                        }
                    }
                }
            }
            color[c[u as usize] as usize - b'a' as usize] += 1;
            colors[u as usize] = color;
            // dbg!((u, &color[..3]));
            cnt += 1; // 处理一个
        }

        if cnt != n {
            -1
        } else {
            colors
                .into_iter()
                .map(|c| c.into_iter())
                .flatten()
                .max()
                .unwrap()
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abaca"\n[[0,1],[0,2],[2,3],[3,4]]\n
// @lcpr case=end

// @lcpr case=start
// "a"\n[[0,0]]\n
// @lcpr case=end

 */
