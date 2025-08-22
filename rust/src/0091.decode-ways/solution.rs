// Created by bh4bxl at 2025/08/19 13:31
// leetgo: 1.4.15
// https://leetcode.com/problems/decode-ways/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let len = s.len();
        let mut dp = vec![0; len + 1];

        if chars[0] == '0' {
            return 0;
        }

        if len == 1 {
            return 1;
        }

        dp[len] = 1;

        if *chars.last().unwrap() != '0' {
            dp[len - 1] = 1;
        }

        for i in (0..=len - 2).rev() {
            if chars[i] == '0' {
                continue;
            }
            let sum = (chars[i] as u8 - '0' as u8) * 10 + (chars[i + 1] as u8 - '0' as u8);
            if sum <= 26 {
                dp[i] = dp[i + 1] + dp[i + 2];
            } else {
                dp[i] = dp[i + 1];
            }
        }

        dp[0]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_decodings(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
