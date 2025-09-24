// Created by bh4bxl at 2025/09/15 14:25
// leetgo: 1.4.15
// https://leetcode.com/problems/palindrome-partitioning-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();

        let mut is_palindrome = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            for j in i..n {
                if chars[i] == chars[j] && (j - i <= 2 || is_palindrome[i + 1][j - 1]) {
                    is_palindrome[i][j] = true;
                }
            }
        }

        let mut dp = vec![0; n];
        for i in 0..n {
            if is_palindrome[0][i] {
                dp[i] = 0;
            } else {
                let mut min_cut = i32::MAX;
                for j in 0..i {
                    if is_palindrome[j + 1][i] {
                        min_cut = min_cut.min(dp[j] + 1);
                    }
                }
                dp[i] = min_cut;
            }
        }

        dp[n - 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_cut(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
