use std::collections::HashMap;
use std::io;

fn main() {
    let c = ch3::f_to_c(212.);
    let f = ch3::c_to_f(c);
    println!("f to c {}, {} ", c, f);

    let fib = ch3::fib(8);
    println!("fib {}", fib);

    ch3::chris();

    let mut num_list = vec![3, 8, 5, 7, 2, 35, 2, 6, 2, 8, 9];

    ch8::stats(&mut num_list);

    let mut phrase = "hello you apple".to_string();
    let pl = ch8::pig_latin(&mut phrase);
    println!("{}", pl);

    let mut departments = HashMap::new();

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        if command.len() > 1 {
            ch8::parse_command(&command, &mut departments);
        }
    }
}
