/*
 * @lc app=leetcode id=3495 lang=rust
 *
 * [3495] Minimum Operations to Make Array Elements Zero
 */

use leetcode_rs::vec2d;

// @lc code=start
impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ans = 0;
        for query in queries {
            let (l, r) = (query[0], query[1]);
            let mut n = 0;
            while (1 << 2 * n) < l {
                n += 1;
            }
            if l == r {
                ans += n as i64;
                continue;
            }
            let mut l = l;
            let r = r + 1;
            let mut cnt = 0;
            loop {
                let thr = r.min(1 << 2 * n);
                cnt += (thr - l) as i64 * n as i64;
                l = thr;
                n += 1;
                if l == r {
                    break;
                }
            }
            ans += (cnt + 1) / 2;
        }

        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::min_operations(vec2d![[1, 2], [2, 4]]), 3);
    assert_eq!(Solution::min_operations(vec2d![[2, 6]]), 4);
    assert_eq!(Solution::min_operations(vec2d![[0, 5]]), 4);
}
