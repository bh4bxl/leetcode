// Created by bh4bxl at 2025/12/23 13:37
// leetgo: 1.4.15
// https://leetcode.com/problems/reverse-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: Vec<char> = deserialize(&read_line()?)?;
    Solution::reverse_string(s);
    let ans: Vec<char> = s.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
