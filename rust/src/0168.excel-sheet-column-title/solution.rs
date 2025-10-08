// Created by bh4bxl at 2025/10/08 16:27
// leetgo: 1.4.15
// https://leetcode.com/problems/excel-sheet-column-title/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut res = "".to_string();
        let mut num = column_number;

        while num != 0 {
            let c = (((num - 1) % 26) as u8 + 'A' as u8) as char;
            res.push(c);
            num -= 1;
            num /= 26;
        }

        res.chars().rev().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let column_number: i32 = deserialize(&read_line()?)?;
    let ans: String = Solution::convert_to_title(column_number).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
