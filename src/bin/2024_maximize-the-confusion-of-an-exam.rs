/*
 * @lc app=leetcode id=2024 lang=rust
 *
 * [2024] Maximize the Confusion of an Exam
 */

// @lc code=start
use std::vec;
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
            dbg!(&segs);
            // seg: (space, count)
            let mut first = 0;
            let raw_k = k;
            let mut k = k;
            let mut cur_len = segs[0].1;
            let mut max_len = cur_len + k;
            for i in 1..segs.len() {
                let seg = segs[i];
                while k < seg.0 && first < i {
                    let first_seg = segs[first];
                    cur_len -= first_seg.1;
                    k += first_seg.0;
                    first += 1;
                }
                if k >= seg.0 {
                    cur_len += seg.1;
                    k -= seg.0;
                } else {
                    cur_len = seg.1;
                    k = raw_k;
                }
                max_len = max_len.max(cur_len + raw_k);
            }
            max_len
        };

        let max1 = max_len(&seq);
        seq.insert(0, 0);
        let max2 = max_len(&seq);
        dbg!(max1, max2);

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
}
