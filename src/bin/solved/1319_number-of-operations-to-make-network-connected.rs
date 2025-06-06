/*
 * @lc app=leetcode.cn id=1319 lang=rust
 * @lcpr version=30204
 *
 * [1319] 连通网络的操作次数
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::make_connected(4, vec2d![[0, 1], [0, 2], [1, 2]]),
        1
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        if (connections.len() as i32) < n - 1 {
            return -1;
        }
        // 并查集或者搜索，有几组就是几
        let mut parent = vec![0; n as usize];
        for i in 0..n as usize {
            parent[i] = i;
        }
        fn root(x: usize, parent: &mut [usize]) -> usize {
            if parent[x] != x {
                parent[x] = root(parent[x], parent);
            }
            parent[x]
        }
        fn union(x: usize, y: usize, parent: &mut [usize]) {
            parent[root(x, parent)] = root(y, parent);
        }
        for connect in connections {
            let (x, y) = (connect[0] as usize, connect[1] as usize);
            union(x, y, &mut parent);
        }
        let mut seen = vec![false; n as usize];
        let mut ans = 0;
        for i in 0..n {
            let root = root(i as usize, &mut parent);
            if !seen[root] {
                seen[root] = true;
                ans += 1;
            }
        }

        ans - 1
    }
}
// @lc code=end

/*
// @lcpr case=start
// 4\n[[0,1],[0,2],[1,2]]\n
// @lcpr case=end

// @lcpr case=start
// 6\n[[0,1],[0,2],[0,3],[1,2],[1,3]]\n
// @lcpr case=end

// @lcpr case=start
// 6\n[[0,1],[0,2],[0,3],[1,2]]\n
// @lcpr case=end

// @lcpr case=start
// 5\n[[0,1],[0,2],[3,4],[2,3]]\n
// @lcpr case=end

 */
