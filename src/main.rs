use std::io;

fn main() {
    println!("Enter the number in the fibonacci sequence to calculate:");

    let mut n = String::new(); 
    io::stdin().read_line(&mut n).expect("Failed to readline");
    let n: u32 = n.trim().parse().expect("input a number!");

    let result = fibonacci(n);
    println!("\nNumber {n} in the fibonacci sequence is {result}");
}

fn fibonacci(n : u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}
