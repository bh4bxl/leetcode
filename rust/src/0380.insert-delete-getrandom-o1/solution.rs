// Created by bh4bxl at 2026/01/14 17:51
// leetgo: 1.4.16
// https://leetcode.com/problems/insert-delete-getrandom-o1/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            nums: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }

        self.nums.push(val);
        self.map.insert(val, self.nums.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last = *self.nums.last().unwrap();
            self.nums[idx] = last;
            self.map.insert(last, idx);

            self.nums.pop();
            self.map.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.nums.len());
        self.nums[idx]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = RandomizedSet::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "insert" => {
                let method_params = split_array(&params[i])?;
                let val: i32 = deserialize(&method_params[0])?;
                let ans: bool = obj.insert(val).into();
                output.push(serialize(ans)?);
            }
            "remove" => {
                let method_params = split_array(&params[i])?;
                let val: i32 = deserialize(&method_params[0])?;
                let ans: bool = obj.remove(val).into();
                output.push(serialize(ans)?);
            }
            "getRandom" => {
                let ans: i32 = obj.get_random().into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
