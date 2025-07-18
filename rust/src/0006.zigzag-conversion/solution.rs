// Created by bh4bxl at 2025/07/12 10:08
// leetgo: 1.4.14
// https://leetcode.com/problems/zigzag-conversion/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let mut ret = String::new();
        let cycle_len = 2 * num_rows as usize - 2;
        let str_len = s.len();

        for i in 0..num_rows as usize {
            let mut j = 0;
            while j + i < str_len {
                ret.push(s.chars().nth(j + i as usize).unwrap());
                if i != 0 && i != num_rows as usize - 1 && j + cycle_len - i < str_len {
                    ret.push(s.chars().nth(j + cycle_len - i).unwrap());
                }
                j += cycle_len;
            }
        }

        ret
    }
}

// @lc code=end

fn main() -> Result<()> {
	let s: String = deserialize(&read_line()?)?;
	let num_rows: i32 = deserialize(&read_line()?)?;
	let ans: String = Solution::convert(s, num_rows).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
