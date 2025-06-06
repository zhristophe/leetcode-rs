/*
 * @lc app=leetcode.cn id=765 lang=rust
 * @lcpr version=30204
 *
 * [765] 情侣牵手
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
    assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        // 循环群
        let n = row.len() / 2;
        struct UnionFind {
            p: Vec<usize>,
        }
        impl UnionFind {
            fn root(&mut self, x: usize) -> usize {
                if self.p[x] != x {
                    self.p[x] = self.root(self.p[x]);
                }
                self.p[x]
            }
            fn union(&mut self, x: usize, y: usize) {
                let x = self.root(x);
                let y = self.root(y);
                self.p[x] = y;
            }
        }
        let mut uf = UnionFind {
            p: (0..n).collect(),
        };
        for i in 0..n {
            uf.union((row[2 * i] / 2) as usize, (row[2 * i + 1] / 2) as usize);
        }
        let mut cnt = vec![0; n];
        for i in 0..n {
            cnt[uf.root(i)] += 1;
        }

        cnt.iter()
            .fold(0, |acc, v| if *v > 0 { acc + v - 1 } else { acc })
    }
}
// @lc code=end

/*
// @lcpr case=start
// [0,2,1,3]\n
// @lcpr case=end

// @lcpr case=start
// [3,2,0,1]\n
// @lcpr case=end

 */
