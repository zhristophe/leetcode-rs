/*
 * @lc app=leetcode.cn id=2157 lang=rust
 * @lcpr version=30204
 *
 * [2157] 字符串分组
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::group_strings(
            vec!["a", "b", "ab", "cde"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec![2, 3]
    );
    assert_eq!(
        Solution::group_strings(
            vec!["ghnv", "uip", "tenv", "hvepx", "e", "ktc", "byjdt", "ulm", "cae", "ea"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec![8, 3]
    );
    assert_eq!(
        Solution::group_strings(
            vec!["xhg", "kove", "ti", "cs", "itfzx", "m", "nrszq", "suc", "gs"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec![7, 3]
    );
    assert_eq!(
        Solution::group_strings(
            vec!["ab", "ab", "c"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        vec![2, 2]
    );
}
// @lcpr-template-end
// @lc code=start
struct DSU {
    root: Vec<usize>,
    rank: Vec<usize>,
}
impl DSU {
    fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: vec![1; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.root[x] != x {
            self.root[x] = self.find(self.root[x]);
        }
        self.root[x]
    }
    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.root[x] = y;
        } else {
            self.root[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
}
use std::{collections::HashMap, ops::AddAssign};
impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        // 坑点: 这是集合而非字符串
        let raw_words = words;
        let mut words = vec![];
        let mut map = HashMap::new();
        for word in raw_words {
            let word = word
                .as_bytes()
                .iter()
                .fold(0i32, |acc, v| acc | (1 << (v - b'a')));
            let idx = *map.entry(word).or_insert_with(|| words.len());
            if idx >= words.len() {
                words.push((word, 1));
            } else {
                words[idx].1 += 1;
            }
        }
        let mut dsu = DSU::new(words.len());
        for (&word, &index) in &map {
            // dbg!(word);
            for i in 0..26 {
                if word & (1 << i) == 0 {
                    if let Some(&index2) = map.get(&(word | (1 << i))) {
                        // dbg!(words[index].0, words[index2].0);
                        dsu.union(index, index2);
                    }
                    for j in 0..26 {
                        if i != j && word & (1 << j) != 0 {
                            if let Some(&index2) = map.get(&((word | (1 << i)) ^ (1 << j))) {
                                // dbg!(words[index].0, words[index2].0);
                                dsu.union(index, index2);
                            }
                        }
                    }
                } else {
                    if let Some(&index2) = map.get(&(word ^ (1 << i))) {
                        // dbg!(words[index].0, words[index2].0);
                        dsu.union(index, index2);
                    }
                }
            }
        }

        let mut hash = HashMap::new();
        for (_, index) in map {
            hash.entry(dsu.find(index))
                .or_insert(0)
                .add_assign(words[index].1);
        }

        vec![hash.len() as i32, *hash.values().max().unwrap() as i32]
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["a","b","ab","cde"]\n
// @lcpr case=end

// @lcpr case=start
// ["a","ab","abc"]\n
// @lcpr case=end

 */
