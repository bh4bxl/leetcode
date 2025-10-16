// Created by bh4bxl at 2025/10/14 20:39
// leetgo: 1.4.15
// https://leetcode.com/problems/repeated-dna-sequences/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        let mut res = vec![];
        let len = s.len();
        if len < 10 {
            return res;
        }
        for i in 0..=(len - 10) {
            let sub = &s[i..i + 10];
            if !seen.insert(sub) {
                if !res.contains(&sub.to_string()) {
                    res.push(sub.to_string());
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::find_repeated_dna_sequences(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
