// Created by bh4bxl at 2026/01/27 18:06
// leetgo: 1.4.16
// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        fn helper(s: &str, k: i32) -> i32 {
            if s.len() < k as usize {
                return 0;
            }

            let mut freq = [0; 26];
            for c in s.bytes() {
                freq[(c - b'a') as usize] += 1;
            }

            for (i, c) in s.bytes().enumerate() {
                if freq[(c - b'a') as usize] < k {
                    let left = helper(&s[..i], k);
                    let right = helper(&s[i + 1..], k);
                    return left.max(right);
                }
            }
            s.len() as i32
        }

        helper(&s, k)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let k: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::longest_substring(s, k).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
