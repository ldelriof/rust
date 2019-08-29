use ch3;

fn main() {
    let c = ch3::f_to_c(212.);
    let f = ch3::c_to_f(c);
    println!("f to c {}, {} ", c, f);

    let fib = ch3::fib(8);
    println!("fib {}", fib);

    ch3::chris();

}
