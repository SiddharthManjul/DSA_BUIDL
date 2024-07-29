use std::io;

fn pattern_six(n: i32) {
    for i in (1..=n).rev() {
        for j in 1..=i {
            print!("{}", j);
        }
        println!();
    }
}

pub fn executable_six() {
    println!("Numeric Input: ");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input!");
    let trimmed = n.trim();
    match trimmed.parse::<i32>() {
        Ok(n) => pattern_six(n),
        Err(..) => println!("Not an Integer"),
    }
}