// Created by bh4bxl at 2025/08/04 14:37
// leetgo: 1.4.15
// https://leetcode.com/problems/valid-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut chars: Vec<char> = s.chars().collect();
        let mut idx = 0;

        chars.push(0 as char);

        while chars[idx] == ' ' {
            idx += 1;
        }

        if chars[idx] == '-' || chars[idx] == '+' {
            idx += 1;
        }

        let (mut number, mut point) = (0, 0);
        while chars[idx] <= '9' && chars[idx] >= '0' || chars[idx] == '.' {
            if chars[idx] == '.' {
                point += 1;
            } else {
                number += 1;
            }
            idx += 1;
        }
        if point > 1 || number < 1 {
            return false;
        }

        if chars[idx] == 'e' || chars[idx] == 'E' {
            idx += 1;
            if chars[idx] == '+' || chars[idx] == '-' {
                idx += 1;
            }
            number = 0;
            while chars[idx] <= '9' && chars[idx] >= '0' {
                idx += 1;
                number += 1;
            }
            if number < 1 {
                return false;
            }
        }

        while chars[idx] == ' ' {
            idx += 1;
        }

        chars[idx] as u8 == 0
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_number(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
