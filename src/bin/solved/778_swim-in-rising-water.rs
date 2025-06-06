/*
 * @lc app=leetcode.cn id=778 lang=rust
 * @lcpr version=30204
 *
 * [778] 水位上升的泳池中游泳
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::swim_in_water(vec2d![[0, 2], [1, 3]]), 3);
    assert_eq!(
        Solution::swim_in_water(vec2d![
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6]
        ]),
        16
    );
}
// @lcpr-template-end
// @lc code=start
use std::{collections::BinaryHeap, vec};
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        // 优先队列
        #[derive(Eq, Ord)]
        struct Cell {
            height: i32,
            x: usize,
            y: usize,
        }
        impl PartialEq for Cell {
            fn eq(&self, other: &Self) -> bool {
                self.height == other.height
            }
        }
        impl PartialOrd for Cell {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(other.height.cmp(&self.height))
            }
        }
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];
        let mut pq = BinaryHeap::new();
        pq.push(Cell {
            height: grid[0][0],
            x: 0,
            y: 0,
        });
        let mut max_height = 0;
        while let Some(Cell { height, x, y }) = pq.pop() {
            max_height = max_height.max(height);
            let mut nbrs = vec![];
            if x > 0 {
                nbrs.push((x - 1, y));
            }
            if y > 0 {
                nbrs.push((x, y - 1));
            }
            if x < n - 1 {
                nbrs.push((x + 1, y));
            }
            if y < n - 1 {
                nbrs.push((x, y + 1));
            }
            for (nx, ny) in nbrs {
                if (nx, ny) == (n - 1, n - 1) {
                    return max_height.max(grid[nx][ny]);
                }
                if visited[nx][ny] {
                    continue;
                }
                visited[nx][ny] = true;
                pq.push(Cell {
                    height: grid[nx][ny],
                    x: nx,
                    y: ny,
                });
            }
        }

        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[0,2],[1,3]]\n
// @lcpr case=end

// @lcpr case=start
// [[0,1,2,3,4],[24,23,22,21,5],[12,13,14,15,16],[11,17,18,19,20],[10,9,8,7,6]]\n
// @lcpr case=end

 */
