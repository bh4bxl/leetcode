// Created by bh4bxl at 2025/11/10 22:30
// leetgo: 1.4.15
// https://leetcode.com/problems/number-of-digit-one/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut cnt = 0;
        let mut i = 1_i64;

        let n = n as i64;

        while i <= n {
            let divider = i * 10;
            cnt += (n / divider) * i + (n % divider - i + 1).max(0).min(i);
            i *= 10;
        }

        cnt as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::count_digit_one(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
