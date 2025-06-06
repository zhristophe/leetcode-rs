/*
 * @lc app=leetcode.cn id=3403 lang=rust
 * @lcpr version=30204
 *
 * [3403] 从盒子中找出字典序最大的字符串 I
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::answer_string("bbba".to_string(), 2), "bbb");
    assert_eq!(Solution::answer_string("abc".to_string(), 3), "c");
    assert_eq!(Solution::answer_string("dbca".to_string(), 2), "dbc");
    assert_eq!(Solution::answer_string("dadc".to_string(), 2), "dc");
    assert_eq!(Solution::answer_string("gggg".to_string(), 4), "g");
    assert_eq!(Solution::answer_string("abcd".to_string(), 1), "abcd");
}
// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        // 前缀相同的字符串更长的字典序更大，贪心法即可
        // 坑点：分给一个人时，只有一种分法
        if num_friends == 1 {
            return word;
        }
        let word = word.as_bytes();
        let n = word.len();
        let mut maxc = 0;
        let mut cand = vec![];
        for i in 0..n {
            if word[i] > maxc {
                maxc = word[i];
                cand.clear();
                cand.push(i);
            } else if word[i] == maxc {
                cand.push(i);
            }
        }
        // dbg!(&cand);
        let m = n - num_friends as usize + 1;
        let mut ans = vec![word[cand[0]]];
        for cand in cand {
            for j in cand + 1..n.min(cand + m) {
                if let Some(&old) = ans.get(j - cand) {
                    if old > word[j] {
                        break;
                    } else if old == word[j] {
                        continue;
                    } else {
                        ans.resize(j - cand + 1, b'_');
                        ans[j - cand] = word[j];
                    }
                } else {
                    ans.push(word[j]);
                }
            }
            // dbg!(unsafe { String::from_utf8_unchecked(ans.clone()) });
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
// @lc code=end

/*
// @lcpr case=start
// "dbca"\n2\n
// @lcpr case=end

// @lcpr case=start
// "gggg"\n4\n
// @lcpr case=end

 */
