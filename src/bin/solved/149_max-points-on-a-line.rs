/*
 * @lc app=leetcode.cn id=149 lang=rust
 * @lcpr version=30204
 *
 * [149] 直线上最多的点数
 */

// @lcpr-template-start
use leetcode_rs::vec2d;
struct Solution;
fn main() {
    assert_eq!(Solution::max_points(vec2d![[1, 1], [2, 2], [3, 3]]), 3);
    assert_eq!(
        Solution::max_points(vec2d![[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]),
        4
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 {
            return points.len() as i32;
        }
        let mut hash = vec![HashMap::new(); points.len()];
        let gcd = |mut a, mut b| {
            // 输入保证a不为0
            while b != 0 {
                let t = a;
                a = b;
                b = t % b;
            }
            a
        };
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                let p = if dx == 0 {
                    (0, 0)
                } else {
                    let q = gcd(dx.abs(), dy.abs());
                    if dx > 0 {
                        (dx / q, dy / q)
                    } else {
                        (-dx / q, -dy / q)
                    }
                };
                *hash[i].entry(p).or_insert(0) += 1;
                *hash[j].entry(p).or_insert(0) += 1;
            }
        }
        1 + hash
            .iter()
            .filter_map(|map| map.values().max())
            .max()
            .unwrap()
    }
    #[allow(dead_code)]
    pub fn max_points_slow(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 {
            return points.len() as i32;
        }
        let mut edges = vec![];
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                let mut edge = (
                    points[i][0] - points[j][0],
                    points[i][1] - points[j][1],
                    i,
                    j,
                );
                if edge.0 < 0 {
                    edge = (-edge.0, -edge.1, i, j);
                } else if edge.0 == 0 {
                    edge = (0, edge.1.abs(), i, j)
                }
                edges.push(edge);
            }
        }
        // edges里可能有相同的！
        edges.sort_by(|p1, p2| (p1.1 * p2.0).cmp(&(p2.1 * p1.0)));
        let mut ans = 0;
        let mut start = 0;
        let mut end = 0;
        macro_rules! cal_max {
            () => {{
                let mut cnt = HashMap::new();
                for e in &edges[start..=end] {
                    *cnt.entry(e.2).or_insert(0) += 1;
                    *cnt.entry(e.3).or_insert(0) += 1;
                }
                // dbg!(&cnt);
                *cnt.values().max().unwrap()
            }};
        }
        for i in 1..edges.len() {
            let e1 = edges[start];
            let e2 = edges[i];
            if e1.0 * e2.1 == e1.1 * e2.0 {
                end = i;
            } else {
                ans = ans.max(cal_max!());
                start = i;
                end = i;
            }
        }
        ans = ans.max(cal_max!());
        // dbg!(ans);
        // (2.0 * ans as f32).sqrt() as i32 + 1
        ans + 1
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[1,1],[2,2],[3,3]]\n
// @lcpr case=end

// @lcpr case=start
// [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]]\n
// @lcpr case=end

 */
