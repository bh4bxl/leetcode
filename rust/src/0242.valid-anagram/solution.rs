// Created by bh4bxl at 2025/11/17 12:45
// leetgo: 1.4.15
// https://leetcode.com/problems/valid-anagram/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let (slen, tlen) = (s.len(), t.len());
        if slen != tlen {
            return false;
        }
        let (mut ss, mut ts) = (vec![0; 26], vec![0; 26]);
        let (sc, tc) = (s.into_bytes(), t.into_bytes());

        for i in 0..slen {
            ss[(sc[i] - 'a' as u8) as usize] += 1;
            ts[(tc[i] - 'a' as u8) as usize] += 1;
        }

        for i in 0..26 {
            if ss[i] != ts[i] {
                return false;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::is_anagram(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
