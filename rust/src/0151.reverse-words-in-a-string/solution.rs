// Created by bh4bxl at 2025/09/24 15:12
// leetgo: 1.4.15
// https://leetcode.com/problems/reverse-words-in-a-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<&str> = s.split(' ').collect();
        let mut sentence = String::new();

        while !&words.is_empty() {
            if let Some(w) = words.pop() {
                if w.is_empty() {
                    continue;
                }
                sentence.push(' ');
                sentence.push_str(w);
            }
        }

        sentence[1..].to_string()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::reverse_words(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
