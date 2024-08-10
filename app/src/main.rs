fn main() {
    start();
}

fn start() {
    println!("Welcome on your calculator");
    println!("What operator do you want to enter (+, -, *, /)?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    println!("Enter the value in front of the operator:");
    let mut input_value_1 = String::new();
    std::io::stdin().read_line(&mut input_value_1).expect("Failed to read line");
    let number1: f64 = match input_value_1.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    println!("Enter the value in back of the operator:");
    let mut input_value_2 = String::new();
    std::io::stdin().read_line(&mut input_value_2).expect("Failed to read line");
    let number2: f64 = match input_value_2.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    let result = match input {
        "+" => number1 + number2,
        "-" => number1 - number2,
        "*" => number1 * number2,
        "/" => {
            if number2 == 0.0 {
                println!("Cannot divide by zero.");
                return;
            } else {
                number1 / number2
            }
        },
        _ => {
            println!("Invalid operator.");
            return;
        }
    };

    println!("The result is: {}", result);
    start();
}
