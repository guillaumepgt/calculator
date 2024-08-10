fn main() {
    start();
}

fn start() {
    println!("Welcome on your calculator");
    println!("what operator do you want to enter + - * /");
    let mut input : String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("enter the value in front of the operator");
    let mut input_value_1 : String = String::new();
    std::io::stdin().read_line(&mut input_value_1).expect("Failed to read line");
    println!("enter the value in back of the operator");
    let mut input_value_2 : String = String::new();
    std::io::stdin().read_line(&mut input_value_2).expect("Failed to read line");
}