/*
 * @lc app=leetcode id=1061 lang=rust
 *
 * [1061] Lexicographically Smallest Equivalent String
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        struct Union {
            parent: HashMap<char, char>,
        }
        impl Union {
            fn find(&mut self, a: char) -> char {
                let parent = self.parent[&a];
                if parent != a {
                    let root = self.find(parent);
                    self.parent.insert(a, root);
                }
                self.parent[&a]
            }
            fn union(&mut self, a: char, b: char) {
                let ra = self.find(a);
                let rb = self.find(b);
                if ra != rb {
                    self.parent.insert(ra, rb);
                }
            }
        }
        let mut union = Union {
            parent: HashMap::new(),
        };
        for ch in 'a'..='z' {
            union.parent.insert(ch, ch);
        }
        for (a, b) in s1.chars().zip(s2.chars()) {
            union.union(a, b);
        }
        let mut map = HashMap::new();
        for ch in 'a'..='z' {
            let root = union.find(ch);
            let e = map.entry(root).or_insert_with(|| ch);
            *e = ch.min(*e);
        }
        let ans = base_str
            .chars()
            .map(|ch: char| {
                let root = union.find(ch);
                let ch = map.get(&root).unwrap_or(&ch);
                *ch
            })
            .collect();

        ans
    }
}
// @lc code=end

struct Solution;
fn main() {
    assert_eq!(
        Solution::smallest_equivalent_string(
            "parker".to_owned(),
            "morris".to_owned(),
            "parser".to_owned()
        ),
        "makkek"
    );
    assert_eq!(
        Solution::smallest_equivalent_string(
            "dccaccbdafgeabeeghbigbhicggfbhiccfgbechdicbhdcgahi".to_owned(),
            "igfcigeciahdafgegfbeddhgbacaeehcdiehiifgbhhehhccde".to_owned(),
            "sanfbzzwblekirguignnfkpzgqjmjmfokrdfuqbgyflpsfpzbo".to_owned()
        ),
        "sanaazzwalakarauaannakpzaqjmjmaokraauqaayalpsapzao"
    );
}
