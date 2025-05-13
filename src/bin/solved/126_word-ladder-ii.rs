/*
 * @lc app=leetcode.cn id=126 lang=rust
 * @lcpr version=30204
 *
 * [126] 单词接龙 II
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    // assert_eq!(
    //     Solution::find_ladders(
    //         "hit".to_string(),
    //         "cog".to_string(),
    //         vec!["hot", "dot", "dog", "lot", "log", "cog"]
    //             .into_iter()
    //             .map(|s| s.to_string())
    //             .collect()
    //     ),
    //     vec2d![
    //         ["hit", "hot", "dot", "dog", "cog"],
    //         ["hit", "hot", "lot", "log", "cog"]
    //     ]
    // );
    assert_eq!(
        Solution::find_ladders(
            "a".to_string(),
            "c".to_string(),
            vec!["a", "b", "c"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec2d![["a", "c"]]
    );
    assert_eq!(
        Solution::find_ladders(
            "hot".to_string(),
            "dog".to_string(),
            vec!["hot", "dog", "dot"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec2d![["hot", "dot", "dog"]]
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        // 单词的长度<=5，创建通配符
        let words = word_list;
        let n = words.len();
        let mut patterns = HashMap::new();
        for i in 0..n {
            let word = &words[i];
            for j in 0..word.len() {
                let pattern = word[..j].to_string() + "*" + &word[j + 1..];
                patterns.entry(pattern).or_insert(HashSet::new()).insert(i);
            }
        }
        let mut dis = vec![(i32::MAX, vec![]); n];
        let Some(root) = words.iter().position(|w| w == &end_word) else {
            return vec![];
        };
        dis[root] = (0, vec![]);
        let mut queue = VecDeque::new();
        queue.push_back(root);
        while let Some(node) = queue.pop_front() {
            let word = &words[node];
            let d = dis[node].0 + 1;
            for j in 0..word.len() {
                let pattern = word[..j].to_string() + "*" + &word[j + 1..];
                for &k in &patterns[&pattern] {
                    if d < dis[k].0 {
                        dis[k] = (d, vec![node]);
                        queue.push_back(k);
                    } else if d == dis[k].0 {
                        dis[k].1.push(node);
                    }
                }
            }
        }
        let mut nearest_nodes = vec![];
        let mut nearest_dis = i32::MAX;
        for j in 0..begin_word.len() {
            let pattern = begin_word[..j].to_string() + "*" + &begin_word[j + 1..];
            for &k in patterns.get(&pattern).unwrap_or(&HashSet::new()) {
                let dis = if words[k] == begin_word {
                    dis[k].0
                } else {
                    dis[k].0 + 1
                };
                if dis < nearest_dis {
                    nearest_nodes = vec![k];
                    nearest_dis = dis;
                } else if dis == nearest_dis {
                    nearest_nodes.push(k);
                }
            }
        }
        // dbg!(&dis);
        // dbg!(&nearest_nodes);
        // dbg!(nearest_dis);

        fn collect_ans(
            ans: &mut Vec<Vec<String>>,
            mut seq: Vec<String>,
            dis: &Vec<(i32, Vec<usize>)>,
            nodes: Vec<usize>,
            words: &Vec<String>,
            end: usize,
        ) {
            let nodes = HashSet::<usize>::from_iter(nodes.into_iter())
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            if nodes == [end] {
                seq.push(words[end].clone());
                ans.push(seq);
                return;
            }
            for node in nodes {
                let mut seq = seq.clone();
                if seq.last() != Some(&words[node]) {
                    seq.push(words[node].clone());
                }
                collect_ans(ans, seq, dis, dis[node].1.clone(), words, end);
            }
        }

        let mut ans = vec![];
        collect_ans(
            &mut ans,
            vec![begin_word],
            &dis,
            nearest_nodes,
            &words,
            root,
        );

        HashSet::<Vec<String>>::from_iter(ans.into_iter())
            .into_iter()
            .collect()
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
