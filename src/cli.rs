use std::fs;

pub fn start_cli(){
    let contents = fs::read_to_string("logo.txt")
        .expect("Failed to read logo");
    println!("{}", contents);
    println!("======================");
    println!("Welcome to baby-scope");
}
