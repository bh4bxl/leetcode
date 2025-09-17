// Created by bh4bxl at 2025/09/16 15:09
// leetgo: 1.4.15
// https://leetcode.com/problems/word-break/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut word_len = vec![];
        for word in &word_dict {
            word_len.push(word.len());
        }
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 0..n {
            for l in &word_len {
                if i + l <= n {
                    if word_dict.contains(&s[i..i + l].to_string()) {
                        dp[i + l] |= dp[i];
                    }
                }
            }
        }

        dp[n]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let word_dict: Vec<String> = deserialize(&read_line()?)?;
    let ans: bool = Solution::word_break(s, word_dict).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
