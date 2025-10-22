// Created by bh4bxl at 2025/10/20 10:39
// leetgo: 1.4.15
// https://leetcode.com/problems/number-of-1-bits/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            if n >> i & 1 == 1 {
                res += 1;
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::hamming_weight(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
