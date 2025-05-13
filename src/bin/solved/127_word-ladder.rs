/*
 * @lc app=leetcode.cn id=127 lang=rust
 * @lcpr version=30204
 *
 * [127] 单词接龙
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec!["hot", "dot", "dog", "lot", "log", "cog"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        5
    );
}
// @lcpr-template-end
// @lc code=start
use std::{collections::VecDeque, i32};
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // 单源最短路
        let mut dis = vec![i32::MAX; word_list.len()];
        let mut q = VecDeque::new();
        let n = word_list.len();
        let Some(root) = word_list.iter().position(|w| w == &end_word) else {
            return 0;
        };
        q.push_back(root);
        dis[root] = 0;
        let get_edge = |a: &str, b: &str| {
            let mut diff = 0;
            for (a, b) in a.chars().zip(b.chars()) {
                if a != b {
                    if diff == 1 {
                        return 2;
                    }
                    diff += 1;
                }
            }
            diff
        };
        while let Some(node) = q.pop_front() {
            for i in 0..n {
                if get_edge(&word_list[node], &word_list[i]) == 1 {
                    if dis[node] + 1 < dis[i] {
                        dis[i] = dis[node] + 1;
                        q.push_back(i);
                    }
                }
            }
        }
        // dbg!(&dis);
        let mut ans = i32::MAX;
        for i in 0..n {
            if dis[i] != i32::MAX {
                let edge = get_edge(&begin_word, &word_list[i]);
                if edge <= 1 {
                    ans = ans.min(dis[i] + edge);
                }
            }
        }
        if ans == i32::MAX {
            return 0;
        }

        ans + 1
    }
}
// @lc code=end

/*
// @lcpr case=start
// "hit"\n"cog"\n["hot","dot","dog","lot","log","cog"]\n
// @lcpr case=end

// @lcpr case=start
// "hit"\n"cog"\n["hot","dot","dog","lot","log"]\n
// @lcpr case=end

 */
