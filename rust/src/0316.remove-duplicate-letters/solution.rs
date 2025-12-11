// Created by bh4bxl at 2025/12/09 18:05
// leetgo: 1.4.15
// https://leetcode.com/problems/remove-duplicate-letters/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let bytes = s.as_bytes();
        let mut last = vec![0; 26];

        for i in 0..bytes.len() {
            last[(bytes[i] - b'a') as usize] = i;
        }

        let mut stack: Vec<u8> = vec![];
        let mut used = [false; 26];

        for (i, c) in bytes.iter().enumerate() {
            let idx = (c - b'a') as usize;

            if !used[idx] {
                while let Some(&top) = stack.last() {
                    let top_idx = (top - b'a') as usize;
                    if top > *c && last[top_idx] > i {
                        used[top_idx] = false;
                        stack.pop();
                    } else {
                        break;
                    }
                }

                stack.push(*c);
                used[idx] = true;
            }
        }

        String::from_utf8(stack).unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::remove_duplicate_letters(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
