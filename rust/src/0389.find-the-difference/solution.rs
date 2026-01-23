// Created by bh4bxl at 2026/01/20 18:26
// leetgo: 1.4.16
// https://leetcode.com/problems/find-the-difference/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        fn char_count(s: &String) -> Vec<i32> {
            let mut res = vec![0; 26];
            for c in s.chars() {
                res[c as usize - 'a' as usize] += 1;
            }

            res
        }
        let s_cnt = char_count(&s);
        let t_cnt = char_count(&t);

        for i in 0u8..26 {
            if s_cnt[i as usize] != t_cnt[i as usize] {
                return (i + 'a' as u8) as char;
            }
        }

        return 0 as char;
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: char = Solution::find_the_difference(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
