use std::io;

fn main() {
    let mut user_input = String::new();
    let choice: u32;

    loop {
        println!("Please input the (nth) fibonacci number you want:");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        choice = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    let fibonacci_number = generate_nth_fibonacci(choice);

    println!("The {choice} fibonacci number is {fibonacci_number}");
}

fn generate_nth_fibonacci(n: u32) -> u32 {
    let mut x1: u32 = 0;
    let mut x2: u32 = 1;

    for _ in 1..n {
        let temp = x2;
        x2 = x1 + x2;
        x1 = temp;
    }

    return x1;
}
