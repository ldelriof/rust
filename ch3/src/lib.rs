pub fn f_to_c(f: f32) -> f32 {
    (f - 32.) * 5./9.
}

pub fn c_to_f(f: f32) -> f32 {
    f  * 9. / 5. + 32.
}

pub fn fib(i: i32) -> i32 {
    if i > 1 {
        fib(i - 1) + fib(i - 2)
    } else {
        i
    }
}

pub fn chris() {

    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let presents = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping",
    ];

    for day in days.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", day.1);
        let days_past = &presents[..(day.0+1)];
        for present in days_past.iter().rev().enumerate() {
            if present.0 == days_past.len() - 1 && days_past.len() > 1 {
                println!("And {}", present.1)
            } else {
                println!("{}", present.1)
            };
        }

    }
    

}

