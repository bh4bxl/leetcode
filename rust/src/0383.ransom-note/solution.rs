// Created by bh4bxl at 2026/01/19 20:25
// leetgo: 1.4.16
// https://leetcode.com/problems/ransom-note/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        fn str2vec(str: &String) -> Vec<i32> {
            let mut res = vec![0; 26];

            for c in str.as_bytes() {
                res[(c - 'a' as u8) as usize] += 1;
            }

            res
        }

        let note = str2vec(&ransom_note);
        let maga = str2vec(&magazine);

        for i in 0..26 {
            if maga[i] < note[i] {
                return false;
            }
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ransom_note: String = deserialize(&read_line()?)?;
    let magazine: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::can_construct(ransom_note, magazine).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
