// Created by bh4bxl at 2025/09/11 12:58
// leetgo: 1.4.15
// https://leetcode.com/problems/valid-palindrome/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut ns = String::new();

        for c in s.chars() {
            if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') {
                ns.push(c);
            }
        }

        let ls = ns.to_lowercase();

        let len = ls.len();
        let chars: Vec<u8> = ls.into_bytes();

        for i in 0..len / 2 {
            if chars[i] != chars[len - 1 - i] {
                return false;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_palindrome(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
