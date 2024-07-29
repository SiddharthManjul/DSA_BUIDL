use std::io;

fn pattern_one(n: i32) {
    for _i in 0..n {
        for _j in 0..n {
            print!("* ");
        }
        println!();
    }
}

pub fn executable_one() {
    println!("Numeric Input: ");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read the input!");
    let trimmed = n.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => pattern_one(n),
        Err(..) => println!("Not an Integer")
    }
}