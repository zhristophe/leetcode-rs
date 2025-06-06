/*
 * @lc app=leetcode.cn id=3068 lang=rust
 * @lcpr version=30204
 *
 * [3068] 最大节点价值之和
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::maximum_value_sum(vec![1, 2, 1], 3, vec2d![[0, 1], [0, 2]]),
        6
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        // 节点翻转可以传递（转两次等于没转），
        // 因而可以任意翻转两个节点，无所谓它们是不是在一个边上
        let _ = edges; // 无所谓，只要之间有路径就行
        let mut diffs = nums.iter().map(|n| (n ^ k) - n).collect::<Vec<_>>();
        diffs.sort_unstable_by(|a, b| b.cmp(a));
        let mut ans = nums.iter().map(|i| *i as i64).sum::<i64>();
        for pair in diffs.chunks(2) {
            match pair {
                [a, b] => {
                    if a + b > 0 {
                        ans += (a + b) as i64;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        ans
    }
    #[allow(dead_code)]
    pub fn maximum_value_sum_slow(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        struct Data {
            nums: Vec<i32>,
            k: i32,
            map: HashMap<i32, Vec<i32>>,
            cache: HashMap<i32, (i64, i64)>,
        }
        impl Data {
            fn run(&mut self, parent: i32, node: i32) {
                // 输入保证这是树，因此无环
                let children = self
                    .map
                    .get(&node)
                    .unwrap()
                    .iter()
                    .filter(|c| **c != parent)
                    .cloned()
                    .collect::<Vec<_>>();
                for child in &children {
                    self.run(node, *child);
                }
                let mut n_change = 0;
                let mut sum = 0;
                let mut min_diff = if children.is_empty() { 0 } else { i64::MAX };
                for child in children {
                    let (v0, v1) = self.cache[&child];
                    if v0 > v1 {
                        sum += v0;
                        min_diff = min_diff.min(v0 - v1);
                    } else {
                        n_change += 1;
                        sum += v1;
                        min_diff = min_diff.min(v1 - v0);
                    }
                }
                let mut v0 = self.nums[node as usize] as i64;
                let mut v1 = v0 ^ self.k as i64;
                match n_change % 2 {
                    0 => {
                        v0 += sum;
                        v1 += sum - min_diff;
                    }
                    1 => {
                        v0 += sum - min_diff;
                        v1 += sum;
                    }
                    _ => unreachable!(),
                }
                self.cache.insert(node, (v0, v1));
            }
        }
        let mut map = HashMap::new();
        for edge in edges {
            let (u, v) = (edge[0], edge[1]);
            map.entry(u).or_insert_with(|| vec![]).push(v);
            map.entry(v).or_insert_with(|| vec![]).push(u);
        }
        // 无向图任意节点均可为根节点
        let mut data = Data {
            nums,
            k,
            map,
            cache: HashMap::new(),
        };
        data.run(0, 0);
        dbg!(&data.cache);
        let v0 = data.cache[&0];
        v0.0.max(v0.1)
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,2,1]\n3\n[[0,1],[0,2]]\n
// @lcpr case=end

// @lcpr case=start
// [2,3]\n7\n[[0,1]]\n
// @lcpr case=end

// @lcpr case=start
// [7,7,7,7,7,7]\n3\n[[0,1],[0,2],[0,3],[0,4],[0,5]]\n
// @lcpr case=end

 */
