pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut common = strs.first().unwrap().as_str();

        for word in strs.iter() {
            if common.is_empty() {
                break;
            }

            for (i, (w, c)) in common
                .chars()
                .map(Some)
                .zip(word.chars().map(Some).chain(std::iter::repeat(None)))
                .enumerate()
            {
                if w.is_none() || c.is_none() || w != c {
                    common = &common[..i];
                    break;
                }
            }
        }

        common.to_owned()
    }
}

fn main() {
    let common = Solution::longest_common_prefix(vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ]);
    assert_eq!(common, "fl");

    let common = Solution::longest_common_prefix(vec![String::from("ab"), String::from("a")]);
    assert_eq!(common, "a");
}
