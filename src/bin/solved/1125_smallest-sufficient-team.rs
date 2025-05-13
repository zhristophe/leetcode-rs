/*
 * @lc app=leetcode.cn id=1125 lang=rust
 * @lcpr version=30204
 *
 * [1125] 最小的必要团队
 */

use leetcode_rs::vec2d;

// @lcpr-template-start
struct Solution;
fn main() {
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec!["java", "nodejs", "reactjs"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            vec2d![["java"], ["nodejs"], ["nodejs", "reactjs"]]
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_string()).collect())
                .collect(),
        ),
        vec![0, 2]
    );
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec!["algorithms", "math", "java", "reactjs", "csharp", "aws"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            vec2d![
                ["algorithms", "math", "java"],
                ["algorithms", "math", "reactjs"],
                ["java", "csharp", "aws"],
                ["reactjs", "csharp"],
                ["csharp", "math"],
                ["aws", "java"]
            ]
            .into_iter()
            .map(|v| v.into_iter().map(|s| s.to_string()).collect())
            .collect(),
        ),
        vec![1, 2]
    );
}
// @lcpr-template-end
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let mut skill_map = HashMap::new();
        for (usize, skill) in req_skills.iter().enumerate() {
            skill_map.insert(skill, usize);
        }
        struct Data {
            people: Vec<i32>,
            cache: HashMap<i32, (i32, usize)>,
        }
        impl Data {
            fn run(&mut self, req_skills: i32) -> i32 {
                if req_skills == 0 {
                    return 0;
                }
                if let Some(v) = self.cache.get(&req_skills) {
                    return v.0;
                }
                let mut ans = (i32::MAX, 0);
                for (index, person) in self.people.clone().into_iter().enumerate() {
                    let new_skills = req_skills & person;
                    if new_skills == 0 {
                        continue;
                    }
                    let new_req_skills = req_skills & !new_skills;
                    let new_num = self.run(new_req_skills);
                    if new_num + 1 < ans.0 {
                        ans = (new_num + 1, index);
                    }
                }
                self.cache.insert(req_skills, ans);
                ans.0
            }
        }
        let people = people
            .into_iter()
            .map(|skills| {
                skills
                    .into_iter()
                    .fold(0i32, |acc, skill| acc | 1 << skill_map[&skill])
            })
            .collect::<Vec<_>>();
        let mut data = Data {
            people,
            cache: HashMap::new(),
        };
        let mut req_skills = (1 << req_skills.len()) - 1;
        data.run(req_skills);

        let mut ans = vec![];
        while req_skills != 0 {
            let &(_, index) = data.cache.get(&req_skills).unwrap();
            ans.push(index as i32);
            req_skills &= !data.people[index];
        }

        ans
    }
}
// @lc code=end

/*
// @lcpr case=start
// ["java","nodejs","reactjs"]\n[["java"],["nodejs"],["nodejs","reactjs"]]\n
// @lcpr case=end

// @lcpr case=start
// ["algorithms","math","java","reactjs","csharp","aws"]\n[["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]]\n
// @lcpr case=end

 */
