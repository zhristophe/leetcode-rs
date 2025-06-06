/*
 * @lc app=leetcode.cn id=3337 lang=rust
 * @lcpr version=30204
 *
 * [3337] 字符串转换后的长度 II
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::length_after_transformations(
            "abcyy".to_string(),
            2,
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2]
        ),
        7
    );
}
// @lcpr-template-end
// @lc code=start
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        // 预计算一次转换
        let mut trans_1 = [[0; 26]; 26];
        for i in 0..26 {
            for j in 1..=nums[i] {
                trans_1[i][(i + j as usize) % 26] += 1;
            }
        }
        // t可达10^9，因此只能二分法
        fn trans_t(t: i32, trans_1: &[[i32; 26]; 26]) -> [[i32; 26]; 26] {
            if t == 1 {
                return trans_1.clone();
            }
            let odd = t % 2 == 1;
            let trans_t_2 = trans_t(t / 2, trans_1);
            let mut ans = [[0; 26]; 26];
            for i in 0..26 {
                for j in 0..26 {
                    for k in 0..26 {
                        ans[i][j] +=
                            ((trans_t_2[i][k] as i64 * trans_t_2[k][j] as i64) % MOD as i64) as i32;
                        ans[i][j] %= MOD;
                    }
                }
            }
            if odd {
                let mut ans2 = [[0; 26]; 26];
                for i in 0..26 {
                    for j in 0..26 {
                        for k in 0..26 {
                            ans2[i][j] +=
                                ((ans[i][k] as i64 * trans_1[k][j] as i64) % MOD as i64) as i32;
                            ans2[i][j] %= MOD;
                        }
                    }
                }
                ans = ans2;
            }

            ans
        }

        let trans = trans_t(t, &trans_1);
        // dbg!(trans_t(1, &trans_1));
        // dbg!(trans_t(2, &trans_1));
        // dbg!(trans_t(4, &trans_1));
        let mut ans = 0;
        for i in s.chars() {
            for j in 0..26 {
                ans += trans[i as usize - 'a' as usize][j];
                ans %= MOD;
            }
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// "abcyy"\n2\n[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2]\n
// @lcpr case=end

// @lcpr case=start
// "azbk"\n1\n[2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2]\n
// @lcpr case=end

 */
