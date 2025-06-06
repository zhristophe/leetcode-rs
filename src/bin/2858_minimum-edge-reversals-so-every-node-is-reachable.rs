/*
 * @lc app=leetcode.cn id=2858 lang=rust
 * @lcpr version=30204
 *
 * [2858] 可以到达每一个节点的最少边反转次数
 */

// @lcpr-template-start
struct Solution;
fn main() {}
// @lcpr-template-end
// @lc code=start
use std::collections::{HashMap, VecDeque};
impl Solution {
    pub fn min_edge_reversals(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // 输入保证是一颗树
        // 这意味着每个节点都有唯一到根的路径
        // 因为我们维护到子节点的边反转次数即可

        // 创建树，记录边的方向，向下为真。
        #[derive(Clone)]
        struct Node {
            parent: usize,
            direction: bool,
            children: Vec<usize>,
        }
        let mut tree = vec![
            Node {
                parent: 0,
                direction: false,
                children: vec![]
            };
            n as usize
        ];
        // 无向图可以让任意节点为根
        let mut nbrs = vec![vec![]; n as usize];
        for edge in &edges {
            nbrs[edge[0] as usize].push(edge[1] as usize);
            nbrs[edge[1] as usize].push(edge[0] as usize);
        }
        fn build_tree(node: usize, parent: usize, tree: &mut Vec<Node>, nbrs: &Vec<Vec<usize>>) {
            tree[node].parent = parent;
            for &nbr in &nbrs[node] {
                if nbr != parent {
                    tree[node].children.push(nbr);
                    build_tree(nbr, node, tree, nbrs);
                }
            }
        }
        build_tree(0, 0, &mut tree, &nbrs);
        // 确定方向
        for edge in edges {
            for (u, v) in [(edge[0], edge[1]), (edge[1], edge[0])] {
                if tree[v as usize].parent == u as usize {
                    tree[v as usize].direction = true;
                    break;
                }
            }
        }
        // 正式计算，使用拓扑排序
        // 记录两个方向
        let mut f = vec![0; n as usize]; // 表示节点到其各个子树的翻转数
        let mut q = VecDeque::new();
        let mut cnt = vec![0; n as usize];
        for i in 0..n as usize {
            cnt[i] = tree[i].children.len();
            if cnt[i] == 0 {
                q.push_back(i);
            }
        }
        while let Some(u) = q.pop_front() {
            for &v in &tree[u].children {
                if tree[v].direction == false {
                    f[u] += 1;
                }
                f[u] += f[v];
            }
            // 减少父节点的出度计数
            let cnt = &mut cnt[tree[u].parent];
            *cnt -= 1;
            if *cnt == 0 {
                q.push_back(tree[u].parent);
            }
        }
        // 得出最终答案
        // 为了O(n)的复杂度，需要记录中间结果

        let mut ans = vec![0; n as usize];
        for i in 0..n {}

        todo!()
    }
}
// @lc code=end

/*
// @lcpr case=start
// 4\n[[2,0],[2,1],[1,3]]\n
// @lcpr case=end

// @lcpr case=start
// 3\n[[1,2],[2,0]]\n
// @lcpr case=end

 */
