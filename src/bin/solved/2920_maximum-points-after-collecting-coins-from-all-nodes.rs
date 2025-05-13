/*
 * @lc app=leetcode.cn id=2920 lang=rust
 * @lcpr version=30204
 *
 * [2920] 收集所有金币可获得的最大积分
 */

// @lcpr-template-start
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        #[derive(Default)]
        struct Data {
            cache: HashMap<(i32, i32), i32>,
            tree: HashMap<i32, Vec<i32>>,
            coins: Vec<i32>,
            k: i32,
            max_coins: i32,
        }
        impl Data {
            fn new(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> Self {
                let mut data = Self {
                    coins,
                    k,
                    ..Default::default()
                };
                data.build_tree(&edges);
                data.max_coins = *data.coins.iter().max().unwrap();
                data
            }
            fn build_tree(&mut self, edges: &Vec<Vec<i32>>) {
                for edge in edges {
                    self.tree.entry(edge[0]).or_insert(vec![]).push(edge[1]);
                    self.tree.entry(edge[1]).or_insert(vec![]).push(edge[0]);
                }
            }
            fn search(&mut self, node: i32, parent: i32, d: i32) -> i32 {
                // dbg!(node, d);
                if self.max_coins >> d == 0 {
                    return 0;
                }
                if let Some(v) = self.cache.get(&(node, d)) {
                    return *v;
                }
                let coin = self.coins[node as usize] >> d;
                let ans1 = coin - self.k;
                let ans2 = coin >> 1;
                macro_rules! sub_max {
                    ($d:expr) => {{
                        let mut ans = 0;
                        for child in self.tree.get(&node).unwrap().clone() {
                            if child != parent {
                                ans += self.search(child, node, $d);
                            }
                        }
                        ans
                    }};
                }
                let ans = if ans1 >= ans2 {
                    ans1 + sub_max!(d)
                } else {
                    (ans1 + sub_max!(d)).max(ans2 + sub_max!(d + 1))
                };
                self.cache.insert((node, d), ans);
                ans
            }
        }
        let mut data = Data::new(edges, coins, k);
        data.search(0, 0, 0)
    }
}
// @lc code=end
struct Solution;
fn main() {
    assert_eq!(
        Solution::maximum_points(
            leetcode_rs::vec2d![[0, 1], [1, 2], [2, 3]],
            vec![10, 10, 3, 3],
            5
        ),
        11
    );
    assert_eq!(
        Solution::maximum_points(leetcode_rs::vec2d![[0, 1], [0, 2]], vec![8, 4, 4], 0),
        16
    );
    assert_eq!(
        Solution::maximum_points(
            leetcode_rs::vec2d![[0, 1], [0, 2], [3, 2], [0, 4]],
            vec![5, 6, 8, 7, 4],
            7
        ),
        8
    );
    let n = 100_000;
    let mut edges = vec![];
    for i in 0..n - 1 {
        let i = i as i32;
        edges.push(vec![i, i + 1]);
    }
    let coins = vec![10_000; n];
    dbg!(Solution::maximum_points(edges, coins, 0));
}
/*
// @lcpr case=start
// [[0,1],[1,2],[2,3]]\n[10,10,3,3]\n5\n
// @lcpr case=end

// @lcpr case=start
// [[0,1],[0,2]]\n[8,4,4]\n0\n
// @lcpr case=end

 */
