fn main() {
    let number: i32 = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 or 4")
    }

    let condition: bool = false;
    let x: i32 = if condition { 5 } else { 6 };

    println!("The value of x is: {}", x);
}
