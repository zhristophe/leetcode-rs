/*
 * @lc app=leetcode.cn id=3356 lang=rust
 * @lcpr version=30204
 *
 * [3356] 零数组变换 II
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::min_zero_array(vec![2, 0, 2], vec2d![[0, 2, 1], [0, 2, 1], [1, 1, 3]]),
        2
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        // 和1类似，但是要二分查找。使用差分数组
        let ok = |k: usize| {
            let mut cnt = vec![0; nums.len() + 1];
            for query in &queries[..k] {
                let (i, j, x) = (query[0] as usize, query[1] as usize, query[2]);
                cnt[i] += x;
                cnt[j + 1] -= x;
            }
            let mut tot = 0;
            for (i, &num) in nums.iter().enumerate() {
                tot += cnt[i];
                if tot < num {
                    return false;
                }
            }
            true
        };
        let (mut l, mut r) = (0, queries.len());
        while l < r {
            let m = (l + r) / 2;
            if ok(m) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if !ok(r) {
            return -1;
        }

        r as i32
    }
    #[allow(dead_code)]
    pub fn min_zero_array_slow(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        // 和1类似，但是要二分查找
        let ok = |k: usize| {
            let mut beg = HashMap::new();
            let mut end = HashMap::new();
            for query in &queries[..k] {
                let (l, r, k) = (query[0], query[1], query[2]);
                *beg.entry(l).or_insert(0) += k;
                *end.entry(r).or_insert(0) += k;
            }
            let mut tot = 0;
            for (i, &num) in nums.iter().enumerate() {
                tot += beg.get(&(i as i32)).unwrap_or(&0);
                if tot < num {
                    return false;
                }
                tot -= end.get(&(i as i32)).unwrap_or(&0);
            }
            true
        };
        let (mut l, mut r) = (0, queries.len());
        while l < r {
            let m = (l + r) / 2;
            if ok(m) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if !ok(r) {
            return -1;
        }

        r as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// [2,0,2]\n[[0,2,1],[0,2,1],[1,1,3]]\n
// @lcpr case=end

// @lcpr case=start
// [4,3,2,1]\n[[1,3,2],[0,2,1]]\n
// @lcpr case=end

 */
