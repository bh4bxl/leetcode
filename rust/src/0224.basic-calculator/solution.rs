// Created by bh4bxl at 2025/11/03 10:51
// leetgo: 1.4.15
// https://leetcode.com/problems/basic-calculator/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut sign = 1;
        let mut res: i64 = 0;
        let mut stack = vec![];
        let len = s.len();
        let mut i = 0;

        while i < len {
            let c = chars[i];
            if c.is_digit(10) {
                let mut num = (c as u8 - b'0') as i64;
                while i + 1 < len && chars[i + 1].is_digit(10) {
                    num = num * 10 + (chars[i + 1] as u8 - b'0') as i64;
                    i += 1;
                }
                res += num * sign;
            } else if c == '+' {
                sign = 1;
            } else if c == '-' {
                sign = -1;
            } else if c == '(' {
                stack.push(res);
                stack.push(sign);
                res = 0;
                sign = 1;
            } else if c == ')' {
                sign = stack.pop().unwrap();
                res = stack.pop().unwrap() + sign * res;
            }
            i += 1;
        }

        res as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::calculate(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
