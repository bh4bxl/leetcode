// Created by bh4bxl at 2025/12/09 21:15
// leetgo: 1.4.15
// https://leetcode.com/problems/maximum-product-of-word-lengths/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut masks = vec![0; n];
        let mut lens = vec![0; n];

        for (i, w) in words.iter().enumerate() {
            let mut mask = 0u32;
            for &b in w.as_bytes() {
                mask |= 1 << (b - b'a');
            }
            masks[i] = mask;
            lens[i] = w.len();
        }

        let mut res = 0;

        for i in 0..n {
            for j in i + 1..n {
                if masks[i] & masks[j] == 0 {
                    res = res.max((lens[i] * lens[j]) as i32);
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let words: Vec<String> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_product(words).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
