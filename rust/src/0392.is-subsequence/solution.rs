// Created by bh4bxl at 2026/01/27 17:00
// leetgo: 1.4.16
// https://leetcode.com/problems/is-subsequence/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (s_len, t_len) = (s.len(), t.len());
        if s_len > t_len {
            return false;
        } else if s_len == 0 {
            return true;
        }
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        let (mut idx_s, mut idx_t) = (0, 0);

        while idx_s < s_len && idx_t < t_len {
            if s_chars[idx_s] == t_chars[idx_t] {
                idx_s += 1;
                idx_t += 1;
            } else {
                idx_t += 1;
            }
        }

        idx_s == s_len
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_subsequence(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
