// Created by bh4bxl at 2025/10/15 20:25
// leetgo: 1.4.15
// https://leetcode.com/problems/reverse-bits/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut res = 0;

        for i in 0..32 {
            if (n >> i) & 0x01 == 0x01 {
                res |= 1 << (31 - i);
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::reverse_bits(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
