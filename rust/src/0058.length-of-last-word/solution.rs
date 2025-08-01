// Created by bh4bxl at 2025/07/31 15:04
// leetgo: 1.4.15
// https://leetcode.com/problems/length-of-last-word/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut last = 0;
        let mut curr = 0;

        for c in s.chars() {
            match c {
                ' ' => {
                    if curr != 0 {
                        last = curr;
                    }
                    curr = 0;
                }
                _ => curr += 1,
            }
        }
        if curr != 0 {
            last = curr
        }

        last
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::length_of_last_word(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
