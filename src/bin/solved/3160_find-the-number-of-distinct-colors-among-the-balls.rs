/*
 * @lc app=leetcode.cn id=3160 lang=rust
 * @lcpr version=30204
 *
 * [3160] 所有球里面不同颜色的数目
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::query_results(4, vec2d![[1, 4], [2, 5], [1, 3], [3, 4]]),
        vec![1, 2, 2, 3]
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let _ = limit;
        let mut balls = std::collections::HashMap::new();
        let mut cnt = std::collections::HashMap::new();
        let mut n_color = 0;
        let mut ans = vec![0; queries.len()];
        for (index, query) in queries.into_iter().enumerate() {
            let (x, y) = (query[0] as usize, query[1]);
            if let Some(x) = balls.get(&x) {
                let color = cnt.entry(*x).or_insert(0);
                *color -= 1;
                if *color == 0 {
                    n_color -= 1;
                }
            }
            balls.insert(x, y);
            let color = cnt.entry(y).or_insert(0);
            if *color == 0 {
                n_color += 1;
            }
            *color += 1;

            ans[index] = n_color;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// 4\n[[1,4],[2,5],[1,3],[3,4]]\n
// @lcpr case=end

// @lcpr case=start
// 4\n[[0,1],[1,2],[2,2],[3,4],[4,5]]\n
// @lcpr case=end

 */
