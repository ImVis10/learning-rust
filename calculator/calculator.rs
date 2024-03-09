use std::io;

fn main() {
    println!("enter 1st number:");

    // rust takes input as a string 
    let mut input1 = String::new();

    // read the line into the string
    io::stdin()
        .read_line(&mut input1)
        .expect("failed to read line");

    // parse the string as an integer
    let num1: i32 = match input1.trim().parse() {
        Ok(num1) => num1,
        Err(_) => {
            println!("PLEASE ENTER A VALID INTEGER");
            return;
        }
    };

    println!("enter 2nd number:");

    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input2)
        .expect("failed to read line");

    let num2: i32 = match input2.trim().parse() {
        Ok(num2) => num2,
        Err(_) => {
            println!("PLEASE ENTER A VALID INTEGER");
            return;
        }
    };

    println!("enter operator(+, -, *, /):");

    let mut input3 = String::new();

    io::stdin()
        .read_line(&mut input3)
        .expect("failed to read line"); 

    // trim the string and take the first character
    let op = input3.trim().chars().next().expect("no operator entered");

    let result = match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0 {
                println!("cannot divide by zero");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("unsupported operator");
            return;
        }
    };

    println!("Result: {}", result);
}