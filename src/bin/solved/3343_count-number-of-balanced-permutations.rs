/*
 * @lc app=leetcode.cn id=3343 lang=rust
 * @lcpr version=30204
 *
 * [3343] 统计平衡排列的数目
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::count_balanced_permutations("123".to_string()), 2);
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        // 求和为一半、长度也为一半的排列数
        // 依次计算0-9分配后的排列数量
        let mut cnt = [0u8; 10];
        num.bytes().for_each(|b| cnt[(b - b'0') as usize] += 1);
        let mut sum = 0;
        for (i, n) in cnt.iter().enumerate() {
            sum += (*n as usize) * i;
        }
        if sum % 2 == 1 {
            return 0;
        }
        // 预计算组合数
        let cnt_sum = cnt.iter().fold(0, |acc, v| acc + *v as usize);
        let len = (cnt_sum + 1) / 2 + 1;
        let mut comb = vec![vec![0; len]; len];
        for i in 0..len {
            comb[i][i] = 1;
            comb[i][0] = 1;
        }
        for i in 0..len {
            for j in 1..i {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
            }
        }
        fn search(
            tgt: usize,     // 目标和
            num: usize,     // 当前数字
            pos: usize,     // 偶数位剩下的位置数量
            pos_odd: usize, // 奇数位剩下的位置数量。用于简化计算，不作为键
            memo: &mut HashMap<(usize, usize, usize), i32>,
            cnt: &[u8; 10],
            comb: &Vec<Vec<i32>>,
        ) -> i32 {
            if num == 10 {
                if tgt == 0 {
                    return 1;
                } else {
                    return 0;
                }
            }
            if let Some(res) = memo.get(&(tgt, pos, num)) {
                return *res;
            }
            let mut ans = 0;
            for k in 0..=cnt[num] as usize {
                if k > pos {
                    break;
                }
                let k_odd = cnt[num] as usize - k;
                if k_odd > pos_odd {
                    continue;
                }
                if k * num > tgt {
                    break;
                }
                let mut tmp = 1i64;
                let n1 = comb[pos][k] as i64;
                let n2 = comb[pos_odd][k_odd] as i64;
                let n3 = search(
                    tgt - k * num,
                    num + 1,
                    pos - k,
                    pos_odd - k_odd,
                    memo,
                    cnt,
                    comb,
                ) as i64;
                for n in [n1, n2, n3] {
                    tmp *= n;
                    tmp %= MOD as i64;
                }
                ans += tmp as i32;
                ans %= MOD;
            }

            memo.insert((tgt, pos, num), ans);
            ans
        }

        search(
            sum / 2,
            0,
            cnt_sum / 2,
            (cnt_sum + 1) / 2,
            &mut HashMap::new(),
            &cnt,
            &comb,
        )
    }
}
// @lc code=end

/*
// @lcpr case=start
// "123"\n
// @lcpr case=end

// @lcpr case=start
// "112"\n
// @lcpr case=end

// @lcpr case=start
// "12345"\n
// @lcpr case=end

 */
