/*
 * @lc app=leetcode.cn id=2266 lang=rust
 * @lcpr version=30204
 *
 * [2266] 统计打字方案数
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::count_texts("22233".to_string()), 8);
    assert_eq!(
        Solution::count_texts("222222222222222222222222222222222222".to_string()),
        82876089
    );
}
// @lcpr-template-end
// @lc code=start
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        // 含有n个字母的按键被按m次，有多少种情况？记忆化搜索
        let cnt = [0, 0, 3, 3, 3, 3, 3, 4, 3, 4];
        let mut memo = std::collections::HashMap::new();
        fn f(n: i32, m: i32, memo: &mut std::collections::HashMap<(i32, i32), i32>) -> i32 {
            if m <= 1 {
                return 1;
            }
            if let Some(ans) = memo.get(&(n, m)) {
                return *ans;
            }
            let mut ans = 0;
            for i in 1..=n.min(m) {
                ans += f(n, m - i, memo);
                ans %= 1_000_000_007;
            }
            memo.insert((n, m), ans);

            ans
        }
        let mut ans = 1i64;
        let keys = pressed_keys.as_bytes();
        let mut i = 0;
        while i < keys.len() {
            let mut j = i + 1;
            while j < keys.len() && keys[j] == keys[i] {
                j += 1;
            }
            ans *= f(
                cnt[keys[i] as usize - b'0' as usize],
                (j - i) as i32,
                &mut memo,
            ) as i64;
            ans %= MOD as i64;
            i = j;
        }

        ans as i32
    }
}
// @lc code=end

/*
// @lcpr case=start
// "22233"\n
// @lcpr case=end

// @lcpr case=start
// "222222222222222222222222222222222222"\n
// @lcpr case=end

 */
