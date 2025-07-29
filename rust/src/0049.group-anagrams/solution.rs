// Created by bh4bxl at 2025/07/28 18:55
// leetgo: 1.4.15
// https://leetcode.com/problems/group-anagrams/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut str_map: std::collections::HashMap<String, Vec<String>>
            = std::collections::HashMap::new();
        for s in strs {
            let mut s_v: Vec<char> = s.chars().collect();
            s_v.sort();
            let key = String::from_iter(s_v);
            let mut str_list = match str_map.get(&key) {
                    Some(sl) => sl.clone(),
                    None => vec![],
                };
            str_list.push(s);
            str_map.insert(key, str_list);
        }

        str_map.into_values().collect()
    }
}

// @lc code=end

fn main() -> Result<()> {
	let strs: Vec<String> = deserialize(&read_line()?)?;
	let ans: Vec<Vec<String>> = Solution::group_anagrams(strs).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
