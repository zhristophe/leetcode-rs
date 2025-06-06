/*
 * @lc app=leetcode.cn id=871 lang=rust
 * @lcpr version=30204
 *
 * [871] 最低加油次数
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
    assert_eq!(Solution::min_refuel_stops(100, 1, vec2d![[10, 100]]), -1);
    assert_eq!(
        Solution::min_refuel_stops(100, 10, vec2d![[10, 60], [20, 30], [30, 30], [60, 40]]),
        2
    );
    assert_eq!(
        Solution::min_refuel_stops(100, 10, vec2d![[10, 20], [20, 10], [30, 30], [60, 40]]),
        3
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        // 动态规划：计算到达加油站i、加油j次下的最大油量
        // 注意油量可能溢出
        let n = stations.len();
        let mut f = vec![-1i64; n + 1];
        f[0] = start_fuel as i64;
        for i in 0..n {
            if f[i] < stations[i][0] as i64 {
                return -1;
            }
            f[i + 1] = f[i] + stations[i][1] as i64;
            for j in (0..=i).rev() {
                if f[j] < stations[i][0] as i64 {
                    break;
                }
                f[j + 1] = f[j + 1].max(f[j] + stations[i][1] as i64);
            }

            // dbg!((&stations[i], &f));
        }
        if f[n] < target as i64 {
            return -1;
        }
        for j in (0..n).rev() {
            if f[j] < target as i64 {
                return j as i32 + 1;
            }
        }

        0
    }
}
// @lc code=end

/*
// @lcpr case=start
// 1\n1\n[]\n
// @lcpr case=end

// @lcpr case=start
// 100\n1\n[[10,100]]\n
// @lcpr case=end

// @lcpr case=start
// 100\n10\n[[10,60],[20,30],[30,30],[60,40]]\n
// @lcpr case=end

 */
