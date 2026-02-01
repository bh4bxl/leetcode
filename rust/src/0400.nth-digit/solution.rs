// Created by bh4bxl at 2026/01/29 17:24
// leetgo: 1.4.16
// https://leetcode.com/problems/nth-digit/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let mut digit_len = 1;
        let mut cnt = 9;
        let mut start = 1;

        // Find digit length
        while n > digit_len * cnt {
            n -= digit_len * cnt;
            digit_len += 1;
            cnt *= 10;
            start *= 10;
        }

        // Find number
        let num = start + (n - 1) / digit_len;

        // Find digit
        let idx = ((n - 1) % digit_len) as usize;
        num.to_string()
            .chars()
            .nth(idx)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::find_nth_digit(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
