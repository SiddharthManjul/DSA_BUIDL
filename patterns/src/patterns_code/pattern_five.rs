use std::io;

fn pattern_five(n: i32) {
    for i in (1..=n).rev() {
        for _j in 0..i {
            print!("* ");
        }
        println!();
    }
}

pub fn executable_five() {
    println!("Numeric Input: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input!");
    let trimmed = n.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => pattern_five(n),
        Err(..) => println!("Not an Integer"),
    }
}