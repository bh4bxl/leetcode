// Created by bh4bxl at 2025/08/19 20:37
// leetgo: 1.4.15
// https://leetcode.com/problems/restore-ip-addresses/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut res = vec![];
        let mut parts = vec![];

        Self::backtrack(&s, 0, &mut parts, &mut res);

        res
    }

    fn backtrack(s: &str, start: usize, parts: &mut Vec<String>, result: &mut Vec<String>) {
        if parts.len() == 4 {
            if start == s.len() {
                result.push(parts.join("."));
            }
            return;
        }

        for len in 1..=3 {
            if start + len > s.len() {
                break;
            }

            let part = &s[start..start + len];

            if (part.len() > 1 && part.starts_with('0')) || part.parse::<u32>().unwrap() > 255 {
                continue;
            }

            parts.push(part.to_string());
            Self::backtrack(s, start + len, parts, result);
            parts.pop();
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::restore_ip_addresses(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
