use std::collections::HashMap;
pub fn calc(num_list: &mut Vec<i32>) -> (f32, i32, i32) {
    num_list.sort();
    let middle = num_list.len() / 2;
    println!("{:?}", num_list);

    let mut map = HashMap::new();
    let mut mode = (0, 0);
    let mut mean = 0.;
    for num in num_list.iter_mut() {
        let count = map.entry(*num).or_insert(0);
        *count += 1;

        if *count > mode.1 {
            mode = (*num, *count)
        }

        mean = mean + *num as f32;
    }
    println!("{:?}", mode);

    let mean = mean / num_list.len() as f32;
    let median = num_list[middle];
    let mode = mode.0;
    println!("mean {:?}, median {:?}, mode {:?}", mean, median, mode);
    (mean, median, mode)
}
