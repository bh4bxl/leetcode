// Created by bh4bxl at 2025/08/04 16:12
// leetgo: 1.4.15
// https://leetcode.com/problems/plus-one/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; digits.len() + 1];

        res[digits.len()] = (digits.last().unwrap() + 1) % 10;
        res[digits.len() - 1] = (digits.last().unwrap() + 1) / 10;
        for i in (0..digits.len() - 1).rev() {
            let sum = res[i + 1] + digits[i];
            res[i + 1] = sum % 10;
            res[i] = sum / 10;
        }

        if res[0] == 0 {
            res.remove(0);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let digits: Vec<i32> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::plus_one(digits).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
