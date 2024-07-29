use std::io;

fn pattern_four(n: i32) {
    for i in 1..=n {
        for _j in 1..=i {
            print!("{}", i);
        }
        println!();
    }
}

pub fn executable_four() {
    println!("Numeric Input: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input!");
    let trimmed = n.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => pattern_four(n),
        Err(..) => println!("Not an Integer"),
    }
}