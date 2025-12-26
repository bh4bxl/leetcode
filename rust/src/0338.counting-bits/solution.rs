// Created by bh4bxl at 2025/12/22 21:46
// leetgo: 1.4.15
// https://leetcode.com/problems/counting-bits/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0; n + 1];

        for i in 1..=n {
            if i % 2 == 0 {
                res[i] = res[i / 2];
            } else {
                res[i] = res[i / 2] + 1;
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::count_bits(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
