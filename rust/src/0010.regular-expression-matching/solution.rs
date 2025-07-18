// Created by bh4bxl at 2025/07/13 13:01
// leetgo: 1.4.14
// https://leetcode.com/problems/regular-expression-matching/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_len = s.len();
        let p_len = p.len();
        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];

        dp[0][0] = true;

        for i in 0..s_len + 1 {
            for j in 1..p_len + 1 {
                if p.chars().nth(j - 1).unwrap() == '*' {
                    dp[i][j] =
                        dp[i][j - 2] ||
                        (i > 0 &&
                        (s.chars().nth(i - 1).unwrap() ==
                         p.chars().nth(j - 2).unwrap() ||
                         p.chars().nth(j - 2).unwrap() == '.') &&
                        dp[i - 1][j]);
                } else {
                    dp[i][j] = i > 0 && dp[i - 1][j - 1] &&
                        (s.chars().nth(i - 1).unwrap() ==
                         p.chars().nth(j - 1).unwrap() ||
                         p.chars().nth(j - 1).unwrap() == '.');
                }
            }
        }

        return dp[s_len][p_len];
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
