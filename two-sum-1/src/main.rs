use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());

        for (idx, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&other) => return vec![other, idx as i32],
                None => {
                    map.insert(target - num, idx as i32);
                }
            }
        }

        vec![]
    }
}

fn main() {
    let out = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(out, vec![0, 1]);
}
