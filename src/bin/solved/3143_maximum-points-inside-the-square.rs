/*
 * @lc app=leetcode.cn id=3143 lang=rust
 * @lcpr version=30204
 *
 * [3143] 正方形中的最多点数
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::max_points_inside_square(
            vec2d![[2, 2], [-1, -2], [-4, 4], [-3, 1], [3, -3]],
            "abdca".to_string()
        ),
        2
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let n = points.len();
        let s = s.bytes().map(|b| b - b'a').collect::<Vec<_>>();
        let mut pts = points
            .into_iter()
            .enumerate()
            .map(|(i, p)| (p[0].abs().max(p[1].abs()), s[i] as usize))
            .collect::<Vec<_>>();
        pts.sort();

        let mut visited = [false; 26];
        let mut ans = 0;
        let mut i = 0;
        let mut a = 0; // 边长
        loop {
            while i < n && pts[i].0 <= a {
                if visited[pts[i].1] {
                    return ans;
                }
                visited[pts[i].1] = true;
                i += 1;
            }
            ans = i as i32;
            if i == n {
                return ans;
            }
            a = pts[i].0;
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [[2,2],[-1,-2],[-4,4],[-3,1],[3,-3]]\n"abdca"\n
// @lcpr case=end

// @lcpr case=start
// [[1,1],[-2,-2],[-2,2]]\n"abb"\n
// @lcpr case=end

// @lcpr case=start
// [[1,1],[-1,-1],[2,-2]]\n"ccd"\n
// @lcpr case=end

 */
