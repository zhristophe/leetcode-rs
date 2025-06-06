/*
 * @lc app=leetcode.cn id=273 lang=rust
 * @lcpr version=30204
 *
 * [273] 整数转换英文表示
 */

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(Solution::number_to_words(1), "One".to_string());
    assert_eq!(Solution::number_to_words(10), "Ten".to_string());
    assert_eq!(Solution::number_to_words(20), "Twenty".to_string());
    assert_eq!(
        Solution::number_to_words(101),
        "One Hundred One".to_string()
    );
    assert_eq!(
        Solution::number_to_words(10_101),
        "Ten Thousand One Hundred One".to_string()
    );
    assert_eq!(
        Solution::number_to_words(123),
        "One Hundred Twenty Three".to_string()
    );
    assert_eq!(
        Solution::number_to_words(12345),
        "Twelve Thousand Three Hundred Forty Five".to_string()
    );
    assert_eq!(
        Solution::number_to_words(1234567),
        "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
    );
}
// @lcpr-template-end
// @lc code=start
const NUMS0: &[&str] = &[
    "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];
const NUMS1: &[&str] = &[
    "Ten",
    "Eleven",
    "Twelve",
    "Thirteen",
    "Fourteen",
    "Fifteen",
    "Sixteen",
    "Seventeen",
    "Eighteen",
    "Nineteen",
];
const NUMS2: &[&str] = &[
    "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        // 好逆天的题目。。。
        // 好在不用写and。此外i32最大约21亿，billion足够
        if num == 0 {
            return "Zero".to_string();
        }
        let billion = 1_000_000_000;
        let million = 1_000_000;
        let thousand = 1_000;
        let mut ans = vec![];
        macro_rules! space {
            ($a:expr) => {
                if ($a.len() > 0) {
                    $a.push(b' ');
                }
            };
        }
        fn trans(num: i32) -> Vec<u8> {
            if num == 0 {
                return vec![];
            }
            let mut ans = vec![];
            let [d0, d1, d2] = [num / 100, (num / 10) % 10, num % 10];
            if d0 > 0 {
                ans.extend_from_slice(NUMS0[d0 as usize].as_bytes());
                ans.extend_from_slice(b" Hundred");
            }
            if d1 >= 2 {
                space!(ans);
                ans.extend_from_slice(NUMS2[d1 as usize].as_bytes());
                if d2 > 0 {
                    space!(ans);
                    ans.extend_from_slice(NUMS0[d2 as usize].as_bytes());
                }
            } else if d1 == 1 {
                space!(ans);
                ans.extend_from_slice(NUMS1[d2 as usize].as_bytes());
            } else if d2 > 0 {
                space!(ans);
                ans.extend_from_slice(NUMS0[d2 as usize].as_bytes());
            }

            ans
        }
        let x = num / billion;
        if x > 0 {
            ans.extend_from_slice(&trans(x));
            ans.extend_from_slice(b" Billion");
        }
        let x = (num % billion) / million;
        if x > 0 {
            space!(ans);
            ans.extend_from_slice(&trans(x));
            ans.extend_from_slice(b" Million");
        }
        let x = (num % million) / thousand;
        if x > 0 {
            space!(ans);
            ans.extend_from_slice(&trans(x));
            ans.extend_from_slice(b" Thousand");
        }
        let x = num % thousand;
        if x > 0 {
            space!(ans);
            ans.extend_from_slice(&trans(x));
        }

        unsafe { String::from_utf8_unchecked(ans) }
    }
}
// @lc code=end

/*
// @lcpr case=start
// 123\n
// @lcpr case=end

// @lcpr case=start
// 12345\n
// @lcpr case=end

// @lcpr case=start
// 1234567\n
// @lcpr case=end

 */
