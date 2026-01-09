// Created by bh4bxl at 2026/01/05 20:25
// leetgo: 1.4.16
// https://leetcode.com/problems/count-numbers-with-unique-digits/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }

        let mut res = 10;
        let mut cnt = 9;
        for i in 2..=n {
            cnt *= 11 - i;
            res += cnt;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::count_numbers_with_unique_digits(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
