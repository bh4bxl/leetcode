// Created by bh4bxl at 2025/11/03 13:11
// leetgo: 1.4.15
// https://leetcode.com/problems/basic-calculator-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let len = s.len();
        let mut stack = vec![];
        let mut sign = '+';
        let mut num = 0;
        let chars: Vec<char> = s.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if c.is_digit(10) {
                num = num * 10 + (*c as u8 - b'0') as i64;
                if i != len - 1 {
                    continue;
                }
            }
            if *c != ' ' || i == len - 1 {
                match sign {
                    '+' => stack.push(num),
                    '-' => stack.push(-num),
                    '*' => {
                        let last = stack.pop().unwrap();
                        stack.push(last * num);
                    }
                    '/' => {
                        let last = stack.pop().unwrap();
                        stack.push(last / num);
                    }
                    _ => {}
                }
                sign = *c;
                num = 0
            }
        }

        let mut res: i64 = 0;
        for i in stack {
            res += i;
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
