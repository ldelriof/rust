use std::collections::HashMap;

pub fn stats(nums: &mut Vec<i32>) -> (f32,i32,i32,) {
    nums.sort();
    let middle = nums.len() / 2;
    println!("{:?}", nums);

    let mut map = HashMap::new();
    let mut mode = (0, 0);
    let mut mean = 0.;
    for num in nums.iter_mut() {
        let count = map.entry(*num).or_insert(0);
        *count += 1;

        if *count > mode.1 {
            mode = (*num, *count)
        }

        mean = mean + *num as f32;

    }
    println!("{:?}", mode);

    let mean = mean / nums.len() as f32;
    let median = nums[middle];
    let mode = mode.0;
    println!("mean {:?}, median {:?}, mode {:?}", mean, median, mode);
    (mean, median, mode,)
}


pub fn pig_latin(phrase: &mut str) -> String {
    let vowals = ['a','e','i','o','u'];
    let mut pig_phrase = String::new();

    for word in phrase.split_whitespace() {
        let mut wvec: Vec<char> = word.chars().collect();
        let fchar = wvec[0];
        wvec.push('-');
        if vowals.contains(&fchar) {
            wvec.push('h');
        } else {
            wvec.remove(0);
            wvec.push(fchar);
        }
        let rword: String = wvec.into_iter().collect();
        pig_phrase.push_str(&rword);
        pig_phrase.push_str("ay ");
    }
    pig_phrase
}