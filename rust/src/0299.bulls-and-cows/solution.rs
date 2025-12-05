// Created by bh4bxl at 2025/12/01 14:57
// leetgo: 1.4.15
// https://leetcode.com/problems/bulls-and-cows/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let s_chars: Vec<char> = secret.chars().collect();
        let g_chars: Vec<char> = guess.chars().collect();
        let (mut a, mut b) = (0, 0);
        let (mut a_cnt, mut b_cnt) = (vec![0; 10], vec![0; 10]);

        for i in 0..s_chars.len() {
            if s_chars[i] == g_chars[i] {
                a += 1;
            } else {
                a_cnt[s_chars[i] as usize - '0' as usize] += 1;
                b_cnt[g_chars[i] as usize - '0' as usize] += 1;
            }
        }
        for i in 0..10 {
            b += a_cnt[i].min(b_cnt[i]);
        }

        format_args!("{a}A{b}B").to_string()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let secret: String = deserialize(&read_line()?)?;
    let guess: String = deserialize(&read_line()?)?;
    let ans: String = Solution::get_hint(secret, guess).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
