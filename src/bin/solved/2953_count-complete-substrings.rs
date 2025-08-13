/*
 * @lc app=leetcode.cn id=2953 lang=rust
 * @lcpr version=30204
 *
 * [2953] 统计完全子字符串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    // assert_eq!(Solution::count_complete_substrings("aa".to_string(), 1), 2);
    // assert_eq!(Solution::count_complete_substrings("aa".to_string(), 2), 1);
    // assert_eq!(Solution::count_complete_substrings("aa".to_string(), 3), 0);
    // assert_eq!(
    //     Solution::count_complete_substrings("igigee".to_string(), 2),
    //     3
    // );
    // let mut input = String::new();
    // for _ in (0..100_000).step_by(2) {
    //     input.push('a');
    //     input.push('d');
    // }
    // assert_eq!(Solution::count_complete_substrings(input, 2), 0);
    let mut input = String::new();
    // for _ in 0..1_000 {
    //     for _ in 0..5 {
    for k in 0..20 {
        input.push((b'a' + k) as char);
    }
    //     }
    // }
    let _ = dbg!(Solution::count_complete_substrings(input, 2));
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        // 先根据相邻字符的差距划分为子段，然后只需要考虑字符数量
        // 双指针每个位置，找到出现0次和k次的位置，26个字母都来一次
        // 复杂度O(n)

        fn solve(word: &[u8], k: i32) -> i32 {
            let n = word.len();
            if n < k as usize {
                return 0;
            }
            let mut f = vec![vec![0; 26]; n];
            for i in 0..n {
                if i != 0 {
                    f[i] = f[i - 1].clone();
                }
                let j = word[i] as usize - 'a' as usize;
                f[i][j] += 1;
            }
            let mut ps = vec![[0, 0]; 26];
            for i in 0..n {
                let mut regions = vec![(i, n)];
                for j in 0..26 {
                    let cnt = if i == 0 { 0 } else { f[i - 1][j] };
                    for (p_idx, tgt) in [(0, cnt), (1, cnt + k)] {
                        let mut p = ps[j][p_idx];
                        // 找到tgt的位置
                        while p < n && f[p][j] < tgt {
                            p += 1;
                        }
                        if p == n || f[p][j] != tgt {
                            continue;
                        }
                        // 找到第一个大于tgt的位置
                        let mut q = p;
                        while q < n && f[q][j] == tgt {
                            q += 1;
                        }
                        // [p, q)均为tgt
                        // 维护指针
                        ps[j][p_idx] = q;
                    }
                }
            }

            use std::collections::BTreeSet;
            let n = word.len();
            if n < k as usize {
                return 0;
            }
            let mut f = vec![vec![0; 26]; n];
            for i in 0..n {
                if i != 0 {
                    f[i] = f[i - 1].clone();
                }
                let j = word[i] as usize - 'a' as usize;
                f[i][j] += 1;
            }
            let mut bt = vec![BTreeSet::new(); 26];
            for i in 0..n {
                for j in 0..26 {
                    bt[j].insert((f[i][j], i));
                }
            }
            fn merge(region: BTreeSet<(usize, usize)>) -> BTreeSet<(usize, usize)> {
                let mut region2 = BTreeSet::new();
                let mut iter = region.into_iter();
                let Some(mut tmp) = iter.next() else {
                    return region2;
                };
                while let Some((l, r)) = iter.next() {
                    if l <= tmp.1 + 1 {
                        tmp.1 = r;
                    } else {
                        region2.insert(tmp);
                        tmp = (l, r);
                    }
                }
                region2.insert(tmp);
                region2
            }
            let mut ans = 0;
            for i in 0..n {
                let mut region = BTreeSet::new();
                region.insert((0usize, n - 1));
                // 查找所有字母出现0次或者k次的子段，求交
                for j in 0..26 {
                    let cnt = if i == 0 { 0 } else { f[i - 1][j] };
                    let mut region2 = BTreeSet::new();
                    for tgt in [cnt, cnt + k] {
                        let mut x = bt[j].range((tgt, i)..=(tgt, n));
                        let Some(l) = x.clone().next() else {
                            continue;
                        };
                        let l = l.1;
                        let Some(r) = x.next_back() else {
                            continue;
                        };
                        let r = r.1;
                        for region in &region {
                            let x = (l.max(region.0), r.min(region.1));
                            if x.0 > x.1 {
                                continue;
                            }
                            region2.insert(x);
                        }
                    }
                    region = merge(region2);
                    if region.is_empty() {
                        break;
                    }
                }
                for x in &region {
                    ans += x.1 - x.0 + 1;
                }
            }
            ans as i32
        }
        let word = word.as_bytes();
        let n = word.len();
        let mut i = 0;
        let mut ans = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && (word[j] as i32 - word[j - 1] as i32).abs() <= 2 {
                j += 1;
            }
            ans += solve(&word[i..j], k);
            i = j;
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// "igigee"\n2\n
// @lcpr case=end

// @lcpr case=start
// "aaabbbccc"\n3\n
// @lcpr case=end

 */
