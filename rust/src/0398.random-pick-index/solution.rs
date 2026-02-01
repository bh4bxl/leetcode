// Created by bh4bxl at 2026/01/28 17:57
// leetgo: 1.4.16
// https://leetcode.com/problems/random-pick-index/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use rand::Rng;
use std::collections::HashMap;

struct Solution {
    map: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();

        for (i, &v) in nums.iter().enumerate() {
            map.entry(v).or_insert(Vec::new()).push(i as i32);
        }
        Self { map }
    }

    fn pick(&self, target: i32) -> i32 {
        let indices = self.map.get(&target).unwrap();
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..indices.len());
        indices[idx]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let nums: Vec<i32> = deserialize(&constructor_params[0])?;
    let nums_size: i32 = deserialize(&constructor_params[1])?;
    #[allow(unused_mut)]
    let mut obj = Solution::new(nums, nums_size);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "pick" => {
                let method_params = split_array(&params[i])?;
                let target: i32 = deserialize(&method_params[0])?;
                let ans: i32 = obj.pick(target).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
