use std::io;

fn main() {
    let num1 = read_number("enter 1st number:");
    let num2 = read_number("enter 2nd number:");
    let op = read_operator("enter operator:");

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
        _ => { // defensive programming
            println!("unsupported operator");
            return;
        }
    };

    println!("Result: {}", result);
}

fn read_number(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);

        // rust takes input as a string 
        let mut input = String::new();

        // read the line into the string
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        // parse the string as an integer
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("PLEASE ENTER A VALID INTEGER");
                continue;
            }
        };
    }
}

fn read_operator(prompt: &str) -> char {
    loop {
        println!("{}", prompt);

        let mut op = String::new();

        io::stdin()
            .read_line(&mut op)
            .expect("failed to read line"); 

        if let Some(first_char) = op.trim().chars().next() {
            match first_char {
                '+' | '-' | '*' | '/' => return first_char,
                _ => println!("unsupported operator. please enter one of +, -, *, /."),
            }
        } else {
            println!("no operator entered. please enter one of +, -, *, /");
        }
    }
}