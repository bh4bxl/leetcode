// Created by bh4bxl at 2025/09/16 16:15
// leetgo: 1.4.15
// https://leetcode.com/problems/word-break-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let n = s.len();
        let mut dp = vec![vec![]; n + 1];
        dp[n] = vec!["".to_string()];

        for i in (0..n).rev() {
            let mut sentences = vec![];
            for j in i + 1..=n {
                let word = s[i..j].to_string();
                if word_dict.contains(&word) {
                    for sub in &dp[j] {
                        if sub.is_empty() {
                            sentences.push(word.clone());
                        } else {
                            sentences.push(format!("{} {}", word, sub));
                        }
                    }
                }
            }
            dp[i] = sentences;
        }

        dp[0].clone()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let word_dict: Vec<String> = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::word_break(s, word_dict).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
