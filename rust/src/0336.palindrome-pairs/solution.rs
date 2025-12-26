// Created by bh4bxl at 2025/12/22 17:50
// leetgo: 1.4.15
// https://leetcode.com/problems/palindrome-pairs/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut res = vec![];

        // Map word -> index
        for (i, w) in words.iter().enumerate() {
            map.insert(w.clone(), i as i32);
        }

        for (i, w) in words.iter().enumerate() {
            let len = w.len();

            for cut in 0..=len {
                let left = &w[..cut];
                let right = &w[cut..];

                // Case 1: left is palindrome, find reverse(right)
                if Self::is_palindrome(left) {
                    let rev_right: String = right.chars().rev().collect();
                    if let Some(&j) = map.get(&rev_right) {
                        if j != i as i32 {
                            res.push(vec![j, i as i32]);
                        }
                    }
                }

                // Case 2: right is palindrome (cut != len to avoid duplicates)
                if cut != len && Self::is_palindrome(right) {
                    let rev_left: String = left.chars().rev().collect();
                    if let Some(&j) = map.get(&rev_left) {
                        if j != i as i32 {
                            res.push(vec![i as i32, j]);
                        }
                    }
                }
            }
        }

        res
    }

    fn is_palindrome(s: &str) -> bool {
        let bytes = s.as_bytes();
        let (mut l, mut r) = (0, bytes.len().saturating_sub(1));

        while l < r {
            if bytes[l] != bytes[r] {
                return false;
            }

            l += 1;
            r -= 1;
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let words: Vec<String> = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::palindrome_pairs(words).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
