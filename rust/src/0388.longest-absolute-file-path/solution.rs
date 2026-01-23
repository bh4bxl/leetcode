// Created by bh4bxl at 2026/01/20 18:08
// leetgo: 1.4.16
// https://leetcode.com/problems/longest-absolute-file-path/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut max_len = 0;
        // stack[depth] = total length
        let mut stack = vec![];

        for line in input.split('\n') {
            let depth = line.chars().take_while(|&c| c == '\t').count();
            let name = &line[depth..];
            let len = name.len();

            // Ensure stack size
            if depth == stack.len() {
                stack.push(0);
            }

            let cur_len = if depth == 0 {
                len
            } else {
                stack[depth - 1] + 1 + len
            };

            stack[depth] = cur_len;

            // Check if file
            if name.contains('.') {
                max_len = max_len.max(cur_len);
            }
        }

        max_len as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let input: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::length_longest_path(input).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
