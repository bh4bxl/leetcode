// Created by bh4bxl at 2025/11/27 20:21
// leetgo: 1.4.15
// https://leetcode.com/problems/word-pattern/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let chars: Vec<char> = pattern.chars().collect();
        let words: Vec<&str> = s.split(' ').collect();
        if chars.len() != words.len() {
            return false;
        }

        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut vals = vec![];

        for i in 0..chars.len() {
            if !map.contains_key(&chars[i]) && !vals.contains(&words[i]) {
                map.insert(chars[i], words[i]);
                vals.push(words[i]);
            } else if !map.contains_key(&chars[i]) && vals.contains(&words[i]) {
                return false;
            } else {
                if words[i] != *map.get(&chars[i]).unwrap() {
                    return false;
                }
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let pattern: String = deserialize(&read_line()?)?;
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::word_pattern(pattern, s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
