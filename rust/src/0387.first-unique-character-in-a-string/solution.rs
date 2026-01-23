// Created by bh4bxl at 2026/01/20 17:57
// leetgo: 1.4.16
// https://leetcode.com/problems/first-unique-character-in-a-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        // (pos, times)
        let mut c_pos = vec![(0, 0); 26];
        let mut pos = 0;

        for c in s.chars() {
            let idx = c as usize - 'a' as usize;
            c_pos[idx].0 = pos;
            c_pos[idx].1 += 1;
            pos += 1;
        }

        let mut first_char_pos = i32::MAX;

        for (pos, times) in c_pos {
            if times == 1 && pos < first_char_pos {
                first_char_pos = pos;
            }
        }

        if first_char_pos == i32::MAX {
            -1
        } else {
            first_char_pos
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::first_uniq_char(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
