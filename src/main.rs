//use std;
use std::io;

fn main() {
    let mut input = String::new();
    //print!("Say something: ");
    io::stdin().read_line(&mut input);
    println!("You said \"{}\"", input.trim());
}
