/*
 * @lc app=leetcode.cn id=458 lang=rust
 * @lcpr version=30204
 *
 * [458] 可怜的小猪
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
    assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
    assert_eq!(Solution::poor_pigs(5, 15, 15), 3);
    assert_eq!(Solution::poor_pigs(8, 15, 15), 3);
    assert_eq!(Solution::poor_pigs(9, 15, 15), 4);
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        // 传统只有一轮时，`k`个猪可以试`1 << k`个桶
        // 这是因为一个猪可以提供一个比特的信息量
        // 在这个问题中，一个猪可以提供`log(turns+1)`比特信息量
        // `turns`为轮数
        let turns = minutes_to_test / minutes_to_die;
        let mut l = 0;
        let mut r = buckets;
        while l < r {
            let mid = (l + r) / 2;
            let mut acc = 1;
            for _ in 0..mid {
                acc *= turns + 1;
                if acc >= buckets {
                    break;
                }
            }
            // dbg!((l, r, mid, buckets));
            if acc >= buckets {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1000\n15\n60\n
// @lcpr case=end

// @lcpr case=start
// 4\n15\n15\n
// @lcpr case=end

// @lcpr case=start
// 4\n15\n30\n
// @lcpr case=end

 */
