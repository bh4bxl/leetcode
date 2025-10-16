// Created by bh4bxl at 2025/10/14 12:15
// leetgo: 1.4.15
// https://leetcode.com/problems/excel-sheet-column-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let chars: Vec<char> = column_title.chars().collect();
        let mut res = 0;

        for c in chars {
            res *= 26;
            res += (c as u8 - 'A' as u8) as i32 + 1;
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let column_title: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::title_to_number(column_title).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
