/*
 * @lc app=leetcode.cn id=1482 lang=rust
 * @lcpr version=30204
 *
 * [1482] 制作 m 束花所需的最少天数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        // 二分搜索
        use std::collections::BTreeSet;
        let n = bloom_day.len();
        let days = BTreeSet::from_iter(bloom_day.iter().cloned());
        let days = days.into_iter().collect::<Vec<_>>();
        let mut lo = 0;
        let mut hi = days.len() - 1;
        let able = |d: usize| {
            let mut ans = 0;
            let mut cnt = 0;
            for i in 0..n {
                if bloom_day[i] <= days[d] {
                    cnt += 1;
                    if cnt == k {
                        ans += 1;
                        cnt = 0;
                    }
                } else {
                    cnt = 0;
                }
            }
            ans >= m
        };
        while lo < hi {
            let mid = (lo + hi) / 2;
            if able(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if able(lo) {
            days[lo]
        } else {
            -1
        }
    }
}
// @lc code=end

/*
// @lcpr case=start
// [1,10,3,10,2]\n3\n1\n
// @lcpr case=end

// @lcpr case=start
// [1,10,3,10,2]\n3\n2\n
// @lcpr case=end

// @lcpr case=start
// [7,7,7,7,12,7,7]\n2\n3\n
// @lcpr case=end

// @lcpr case=start
// [1000000000,1000000000]\n1\n1\n
// @lcpr case=end

// @lcpr case=start
// [1,10,2,9,3,8,4,7,5,6]\n4\n2\n
// @lcpr case=end

 */
