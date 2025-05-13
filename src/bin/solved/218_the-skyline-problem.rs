/*
 * @lc app=leetcode.cn id=218 lang=rust
 * @lcpr version=30204
 *
 * [218] 天际线问题
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::get_skyline(vec2d![
            [2, 9, 10],
            [3, 7, 15],
            [5, 12, 12],
            [15, 20, 10],
            [19, 24, 8]
        ]),
        vec2d![
            [2, 10],
            [3, 15],
            [7, 12],
            [12, 0],
            [15, 10],
            [20, 8],
            [24, 0]
        ]
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::BinaryHeap;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // building已经有序！（按左边）
        let n = buildings.len();
        let mut edges = buildings
            .iter()
            .map(|b| b.iter().take(2))
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        edges.sort();
        let mut heap = BinaryHeap::new();
        let mut ans: Vec<Vec<i32>> = vec![];

        let mut j = 0;
        for i in 0..edges.len() {
            while j < n && buildings[j][0] <= edges[i] {
                heap.push((buildings[j][2], buildings[j][1]));
                j += 1;
            }
            while heap.peek().map(|e| e.1 <= edges[i]) == Some(true) {
                heap.pop();
            }
            let maxh = heap.peek().map(|e| e.0).unwrap_or(0);
            if ans.last().map(|e| e[1]) != Some(maxh) {
                ans.push(vec![edges[i], maxh]);
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[2,9,10],[3,7,15],[5,12,12],[15,20,10],[19,24,8]]\n
// @lcpr case=end

// @lcpr case=start
// [[0,2,3],[2,5,3]]\n
// @lcpr case=end

 */
