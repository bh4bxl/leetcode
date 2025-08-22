// Created by bh4bxl at 2025/08/20 14:08
// leetgo: 1.4.15
// https://leetcode.com/problems/interleaving-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (l1, l2) = (s1.len(), s2.len());
        if l1 + l2 != s3.len() {
            return false;
        }

        let (chars1, chars2, chars3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());

        let mut dp = vec![vec![false; l2 + 1]; l1 + 1];
        dp[0][0] = true;

        for i in 0..=l1 {
            for j in 0..=l2 {
                if i > 0 {
                    dp[i][j] = dp[i][j] || (dp[i - 1][j] && chars1[i - 1] == chars3[i + j - 1]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j] || (dp[i][j - 1] && chars2[j - 1] == chars3[i + j - 1]);
                }
            }
        }

        dp[l1][l2]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s1: String = deserialize(&read_line()?)?;
    let s2: String = deserialize(&read_line()?)?;
    let s3: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_interleave(s1, s2, s3).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
