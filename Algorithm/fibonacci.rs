use std::io;

fn run_algo() {
    println!("Please enter Fibonacci index: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = input.trim().parse().expect("Please enter a number");

    let mut current_val = 1;
    let mut previous_val = 0;

    for _ in 0..index {
        current_val += previous_val;
        if previous_val != 0 {
            previous_val = current_val - previous_val;
        }
        print!("-> {} ", current_val);
    }

    println!();
}

fn main() {
    run_algo();
}
