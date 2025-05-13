/*
 * @lc app=leetcode.cn id=2024 lang=rust
 * @lcpr version=30204
 *
 * [2024] 考试的最大困扰度
 */

// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut seq = vec![0];
        for ch in answer_key.chars() {
            if (ch == 'F') == (seq.len() % 2 == 1) {
                seq.push(0);
            }
            *seq.last_mut().unwrap() += 1;
        }
        // dbg!(&seq);
        let max_len = |seq: &[i32]| {
            let segs = seq
                .chunks(2)
                .map(|chunk| match chunk {
                    [a, b] => (*a, *b),
                    [a] => (*a, 0),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            // dbg!(&segs);
            // seg: (count, space)
            let mut first = 0;
            let raw_k = k;
            let mut k = k;
            let mut cur_len = 0;
            let mut max_len = 0;
            for i in 0..segs.len() {
                while k < 0 && first < i {
                    let first_seg = segs[first];
                    cur_len -= first_seg.0;
                    k += first_seg.1;
                    first += 1;
                }
                let seg = segs[i];
                if k < 0 {
                    cur_len = seg.0;
                    k = raw_k - seg.1;
                } else {
                    cur_len += seg.0;
                    k -= seg.1;
                }
                max_len = max_len.max(cur_len + raw_k);
            }
            max_len
        };

        let max1 = max_len(&seq);
        seq.insert(0, 0);
        let max2 = max_len(&seq);
        // dbg!(max1, max2);

        (answer_key.len() as i32).min(max1.max(max2))
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(Solution::max_consecutive_answers("TTFF".to_owned(), 2), 4);
    assert_eq!(Solution::max_consecutive_answers("TFFT".to_owned(), 1), 3);
    assert_eq!(
        Solution::max_consecutive_answers("TTTTTFTFFT".to_owned(), 2),
        8
    );
    assert_eq!(
        Solution::max_consecutive_answers("FFFTTFTTFT".to_owned(), 3),
        8
    );
}

/*
// @lcpr case=start
// "TTFF"\n2\n
// @lcpr case=end

// @lcpr case=start
// "TFFT"\n1\n
// @lcpr case=end

// @lcpr case=start
// "TTFTTFTT"\n1\n
// @lcpr case=end

 */
