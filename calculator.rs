// fn main() {
//     let a = 10;
//     let b = 5;

//     println!("Addition: {} + {} = {}", a, b, add(a, b));
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn subtract(a: i32, b: i32) -> i32 {
//     a - b
// }

// fn multiply(a: i32, b: i32) -> i32 {
//     a * b
// }

// fn divide(a: i32, b: i32) -> i32 {
//     a / b
// }

use std::io;

fn main() {
    println!("=== Rust Calculator ===");

    loop {
        println!("\nSelect operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        if choice == 5 {
            println!("Goodbye!");
            break;
        }

        if choice < 1 || choice > 5 {
            println!("Invalid choice! Please select 1-5.");
            continue;
        }

        println!("Enter first number:");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read input");
        let a: i32 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        println!("Enter second number:");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read input");
        let b: i32 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        let result = match choice {
            1 => add(a, b),
            2 => subtract(a, b),
            3 => multiply(a, b),
            4 => {
                if b == 0 {
                    println!("Error: Cannot divide by zero!");
                    continue;
                } else {
                    divide(a, b)
                }
            }
            _ => continue,
        };

        println!("Result: {}", result);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}
