// Created by bh4bxl at 2025/08/08 13:21
// leetgo: 1.4.15
// https://leetcode.com/problems/minimum-window-substring/

use std::usize;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut map = vec![0; 128];

        for i in t.chars() {
            map[i as usize] += 1;
        }

        let (mut left, mut right) = (0, 0);
        let (mut min_l, mut min_r) = (0, usize::MAX);
        let mut min_len = usize::MAX;
        let mut count = t.len();

        while right < chars.len() {
            let right_char = chars[right] as usize;
            map[right_char] -= 1;
            if map[right_char] >= 0 {
                count -= 1;
            }
            while count == 0 {
                let temp_len = right - left + 1;
                if temp_len < min_len {
                    min_l = left;
                    min_r = right;
                    min_len = temp_len;
                }
                let key = chars[left] as usize;
                map[key] += 1;
                if map[key] > 0 {
                    count += 1;
                }
                left += 1;
            }
            right += 1;
        }

        if min_r == usize::MAX {
            String::new()
        } else {
            chars[min_l..=min_r].iter().collect()
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: String = Solution::min_window(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
