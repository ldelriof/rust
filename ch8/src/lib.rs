mod company;
mod pig_latin;
mod stats;

use std::collections::HashMap;

pub fn stats(nums: &mut Vec<i32>) -> (f32, i32, i32) {
    stats::stats::calc(&mut *nums)
}

pub fn pig_latin(phrase: &mut str) -> String {
    pig_latin::pig_latin::phrase(&mut *phrase)
}

pub fn parse_command(command: &str, departments: &mut HashMap<String, Vec<String>>) {
    company::company::parse_command(command, &mut *departments)
}
