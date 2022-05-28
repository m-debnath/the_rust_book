use std::io;

fn main() {
    let mut number: String = String::new();
    
    println!("Enter value of n to generate the nth fibonacci number.");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");

    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a positive number!");
            return
        }
    };

    if number == 0 {
        println!("Please enter a positive number!");
    }

    for n in 1..number+1 {
        println!("The {} fibonacci number is: {}", numeric_place(n), fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn numeric_place(n: u32) -> String {
    if n == 1 {
        "1st".to_string()
    } else if n == 2 {
        "2nd".to_string()
    } else if n == 3 {
        "3rd".to_string()
    } else {
        n.to_string() + "th"
    }
}
