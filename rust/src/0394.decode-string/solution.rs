// Created by bh4bxl at 2026/01/27 17:57
// leetgo: 1.4.16
// https://leetcode.com/problems/decode-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut cnt_stack = vec![];
        let mut str_stack = vec![];
        let mut cur = String::new();
        let mut num = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + (c as i32 - '0' as i32);
            } else if c == '[' {
                cnt_stack.push(num);
                str_stack.push(cur);
                num = 0;
                cur = String::new();
            } else if c == ']' {
                let repeat = cnt_stack.pop().unwrap();
                let mut prev = str_stack.pop().unwrap();
                for _ in 0..repeat {
                    prev.push_str(&cur);
                }
                cur = prev
            } else {
                cur.push(c);
            }
        }

        cur
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::decode_string(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
