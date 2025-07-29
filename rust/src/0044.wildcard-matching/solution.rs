// Created by bh4bxl at 2025/07/28 15:21
// leetgo: 1.4.15
// https://leetcode.com/problems/wildcard-matching/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; 2];

        dp[s.len() % 2][p.len()] = true;

        for i in (0..s.len() + 1).rev() {
            for j in (0..p.len() + 1).rev() {
                if i == s.len() && j == p.len() {
                    continue;
                }
                let first_match =
                    i < s.len() && j < p.len() &&
                    (p.chars().nth(j).unwrap() == s.chars().nth(i).unwrap() ||
                     p.chars().nth(j).unwrap() == '?' ||
                     p.chars().nth(j).unwrap() == '*');
                if j < p.len() && p.chars().nth(j).unwrap() == '*' {
                    dp[i % 2][j] = dp[i % 2][j + 1] || first_match && dp[(i + 1) % 2][j];
                } else {
                    dp[i % 2][j] = first_match && dp[(i + 1) % 2][j + 1];
                }
            }
        }

        dp[0][0]
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let p: String = deserialize(&read_line()?)?;
	let ans: bool = Solution::is_match(s, p).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
