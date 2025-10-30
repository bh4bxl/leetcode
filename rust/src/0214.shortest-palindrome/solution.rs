// Created by bh4bxl at 2025/10/27 14:37
// leetgo: 1.4.15
// https://leetcode.com/problems/shortest-palindrome/

use std::char;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut ss = s.clone();
        ss.push('#');
        let sr: String = s.chars().rev().collect();
        ss.push_str(&sr);
        let mut next = vec![0; ss.len()];
        let mut j: i32 = -1;
        let ss_chars: Vec<char> = ss.chars().collect();

        next[0] = -1;
        for i in 1..ss.len() {
            while j > -1 && ss_chars[i] != ss_chars[(j + 1) as usize] {
                j = next[j as usize];
            }
            if ss_chars[i] == ss_chars[(j + 1) as usize] {
                j += 1;
            }
            next[i] = j;
        }

        let sub: &str = &s[next[ss.len() - 1] as usize + 1..];
        let mut res: String = sub.chars().rev().collect();
        res.push_str(&s[..]);
        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: String = Solution::shortest_palindrome(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
