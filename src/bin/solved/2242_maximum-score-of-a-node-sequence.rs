/*
 * @lc app=leetcode.cn id=2242 lang=rust
 * @lcpr version=30204
 *
 * [2242] 节点序列的最大得分
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::maximum_score(
            vec![5, 2, 9, 8, 4],
            vec2d![[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]]
        ),
        24
    );
    assert_eq!(
        Solution::maximum_score(
            vec![9, 20, 6, 4, 11, 12],
            vec2d![[0, 3], [5, 3], [2, 4], [1, 3]]
        ),
        -1
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();
        for edge in &edges {
            graph.entry(edge[0]).or_insert(vec![]).push(edge[1]);
            graph.entry(edge[1]).or_insert(vec![]).push(edge[0]);
        }
        for node in graph.keys().cloned().collect::<Vec<_>>() {
            graph.entry(node).and_modify(|n| {
                n.sort_by_key(|n| -scores[*n as usize]);
                n.truncate(3);
            });
        }
        let mut ans = -1;
        for edge in edges {
            let (b, c) = (edge[0], edge[1]);
            for &a in &graph[&b] {
                for &d in &graph[&c] {
                    if a != c && a != d && b != d {
                        ans = ans.max(
                            scores[a as usize]
                                + scores[b as usize]
                                + scores[c as usize]
                                + scores[d as usize],
                        )
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
// [5,2,9,8,4]\n[[0,1],[1,2],[2,3],[0,2],[1,3],[2,4]]\n
// @lcpr case=end

// @lcpr case=start
// [9,20,6,4,11,12]\n[[0,3],[5,3],[2,4],[1,3]]\n
// @lcpr case=end

 */
