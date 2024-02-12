use std::io;

fn main() {
    let mut y: u32 = 0;
    let mut x: u32;
    let mut result: u32;

    let mut input = String::new();

    println!("Welcome to the calculator program!");
    println!("");

    loop {
        println!("Please select what you want me to do...");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Remainder");
        println!("6. Factorial");
        println!("7. Fibonacci");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to take input");

        let choice: u32 = match input.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                return;
            }
        };

        println!("Please enter the first number");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to take input");

        x = match input.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Invalid input! Please enter a valid number.");
                return;
            }
        };

        if choice < 6u32 {
            println!("Please enter the second number");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to take input");

            y = match input.trim().parse() {
                Ok(y) => y,
                Err(_) => {
                    println!("Invalid input! Please enter a valid number.");
                    return;
                }
            };
        }

        match choice {
            1 => {
                result = x.saturating_add(y);
                match result {
                    u32::MAX => overflow(),
                    _ => println!("Sum = {}", result),
                }
            }
            2 => {
                result = x.saturating_sub(y);
                match result {
                    u32::MAX => overflow(),
                    _ => println!("Difference = {}", result),
                }
            }
            3 => {
                result = x.saturating_mul(y);
                match result {
                    u32::MAX => overflow(),
                    _ => println!("Product = {}", result),
                }
            }
            4 => {
                match x.checked_div(y){
                    Some(result) => {
                        println!("Quotient = {}", result);
                    }

                    None => {
                        println!("Invalid division operands");
                    }
                }
            }
            5 => {
                match x.checked_rem(y){
                    Some(result) => {
                        println!("Remainder = {}", result);
                    }

                    None => {
                        println!("Invalid division operands");
                    }
                }
            }
            6 => {
                let fact: u128 = factorial(x);
                if fact != 0 {
                    println!("Factorial of the number = {}", fact);
                }
            }
            7 => {
                result = fibonacci(x);
                if result != 0 {
                    println!("Fibonacci of the first {} numbers = {}", x, result);
                }
            }
            _ => {}
        }
    }
}

fn fibonacci(x: u32) -> u32 {
    let mut fib: u32 = 1;
    let mut crnt: u32 = 1;
    let mut i: u32 = 1;

    while i < x {
        let temp = fib;
        fib = fib.saturating_add(crnt);
        crnt = temp;
        i += 1;
    }

    if fib == u32::MAX {
        overflow();
        return 0u32;
    } else {
        return fib;
    }
}

fn factorial(mut x: u32) -> u128 {
    let mut fact: u128 = 1;

    while x > 1 {
        fact = fact.saturating_mul(x as u128);
        x -= 1;
    }

    if fact == u128::MAX {
        overflow();
        return 0u128;
    } else {
        return fact;
    }
}

fn overflow() {
    println!("Please select a smaller number");
}
