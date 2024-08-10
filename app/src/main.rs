fn main() {
    start();
}

fn start() {
    println!("Welcome on your calculator");
    println!("what operator do you want to enter + - * /");
    let mut input : String = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{}",input)
}