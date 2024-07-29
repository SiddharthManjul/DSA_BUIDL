use std::io;

fn pattern_three(n: i32) {
    for i in 1..=n {
        for j in 1..=i {
            print!("{}", j);
        }
        println!();
    }
}

pub fn executable_three() {
    println!("Numeric Input: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input!");
    let trimmed = n.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => pattern_three(n),
        Err(..) => println!("Not an Integer"),
    }
}