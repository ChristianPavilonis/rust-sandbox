// https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html

// Variables and immutablilty.

fn main() {
    mutability();
    shadowing();
    functions();
    control_flow();
}
// 3.1
// this code works because x is mutable.
fn mutability() {
    println!("MUTABILITY");
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
}

// Constants
#[allow(dead_code)]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Shadowing
fn shadowing() {
    println!("SHADOWING");
    let x = 5;

    // this x shadows over the first.
    let x = x + 1;

    {
        // this x shadows over the second until it becomes out of scope.
        let x = x * 2;
        // prints 12
        println!("the value of x is {x} inside of these braces");
    }

    // prints 6
    println!("the value of x outside is {x}");
}

// 3.2 Data Types
// obsidian note

// 3.3 functions

fn functions() {
    println!("------------------\n\nFUNCTIONS");

    print_number(4);

    let returned_value = return_value();

    println!("the returned value is {returned_value}");
}

fn print_number(x: u8) {
    println!("the value is {x}");
}

fn return_value() -> u8 {
    // no semi colon on last statement returns the value.
    200
}

// 3.5
fn control_flow() {
    println!("------------------\n\nCONTROL FLOW");

    if_statements();
    loops();
}

fn if_statements() {
    let number = 3;

    // if statements have no ().
    if number < 5 {
        println!("number is < 5");
    } else {
        println!("number is >= 5")
    }

    // must be a bool this will not compile
    // if number { }

    {
        // else ifs
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    {
        // let if statements if thatemsnet will return a value after assesing the condition
        let foo = if true { 5 } else { 6 }; // must be the same type though.
        println!("if statement evaluated to {foo}");
    }
}

fn loops() {
    println!("loops");
    let message = {
        // can assign a value from loop.
        let mut x = 0;
        loop {
            if x > 4 {
                break "done";
            }
            print!("{x},");

            x = x + 1;
        }
    };

    println!("{message}");

    {
        // labeld loops
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    // breaks the parent loop
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    {
        // while
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

    {
        // for
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
