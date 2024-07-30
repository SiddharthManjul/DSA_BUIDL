use std::io;

fn pattern_seven(n: i32) {
    for i in 0..n {
        for _j in 0..n-i-1 {
            print!(" ");
        }
        for _j in 0..2*i+1 {
            print!("*");
        }
        for _j in 0..n-i-1 {
            print!(" ");
        }
        println!();
    }
}

pub fn executable_seven() {
    println!("Numeric Input: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input!");
    let trimmed = n.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => pattern_seven(n),
        Err(..) => println!("Not an Integer"),
    }
}