// Created by bh4bxl at 2026/01/27 17:49
// leetgo: 1.4.16
// https://leetcode.com/problems/utf-8-validation/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut remaining = 0;

        for byte in data {
            let b = byte as u8;

            if remaining == 0 {
                if b >> 7 == 0b0 {
                    continue;
                } else if b >> 5 == 0b110 {
                    remaining = 1;
                } else if b >> 4 == 0b1110 {
                    remaining = 2;
                } else if b >> 3 == 0b11110 {
                    remaining = 3;
                } else {
                    return false;
                }
            } else {
                if b >> 6 == 0b10 {
                    remaining -= 1;
                } else {
                    return false;
                }
            }
        }

        return remaining == 0;
    }
}

// @lc code=end

fn main() -> Result<()> {
    let data: Vec<i32> = deserialize(&read_line()?)?;
    let ans: bool = Solution::valid_utf8(data).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
