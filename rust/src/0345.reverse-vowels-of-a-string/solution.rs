// Created by bh4bxl at 2025/12/23 20:11
// leetgo: 1.4.15
// https://leetcode.com/problems/reverse-vowels-of-a-string/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels = vec!['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
        let mut vowels_pos = vec![];
        let mut chars: Vec<char> = s.chars().collect();

        for (i, c) in chars.iter().enumerate() {
            if vowels.contains(&c) {
                vowels_pos.push(i);
            }
        }

        let len = vowels_pos.len();

        for i in 0..len / 2 {
            chars.swap(vowels_pos[i], vowels_pos[len - i - 1]);
        }

        String::from_iter(chars)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::reverse_vowels(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
