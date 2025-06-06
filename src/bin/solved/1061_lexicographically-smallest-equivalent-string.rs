/*
 * @lc app=leetcode.cn id=1061 lang=rust
 * @lcpr version=30204
 *
 * [1061] 按字典序排列最小的等效字符串
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::smallest_equivalent_string(
            "parker".to_string(),
            "morris".to_string(),
            "parser".to_string()
        ),
        "makkek"
    );
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        // 经典并查集
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut uf = (0..26).map(|i| i as usize).collect::<Vec<_>>();
        fn root(x: usize, uf: &mut [usize]) -> usize {
            if uf[x] != x {
                uf[x] = root(uf[x], uf);
            }
            uf[x]
        }
        let mut union = |x: usize, y: usize| {
            let x = root(x, &mut uf);
            let y = root(y, &mut uf);
            if x != y {
                uf[x.max(y)] = x.min(y);
            }
        };
        for (x, y) in s1.iter().zip(s2.iter()) {
            let x = x - b'a';
            let y = y - b'a';
            union(x as usize, y as usize);
        }
        let ans = base_str
            .as_bytes()
            .into_iter()
            .map(|b| root(*b as usize - b'a' as usize, &mut uf) as u8 + b'a')
            .collect::<Vec<_>>();

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "parker"\n"morris"\n"parser"\n
// @lcpr case=end

// @lcpr case=start
// "hello"\n"world"\n"hold"\n
// @lcpr case=end

// @lcpr case=start
// "leetcode"\n"programs"\n"sourcecode"\n
// @lcpr case=end

 */
