use std::io;
use std::thread;
use std::time::Duration;

pub fn intro() -> String {
    println!("\n");
    println!("\n");
    println!("\n");
    println!("\n");
    println!("You have activated... the");
    println!("\n");
    println!("---------------------------------");
    println!("     Old Person Generator!");
    println!("---------------------------------");
    let name1 = prompt("Player One, please enter your name:", Some("Very cool, "));
    thread::sleep(Duration::from_millis(1000));
    println!("\r");
    return String::from(name1.trim())
}

fn prompt(prompt: &str, response: Option<&str>) -> String {
    println!("\r");
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read");

    println!("\r");
    match response {
        Some(res) => print!("{}{}!", res, input.trim()),
        None => println!(" ")
    }

    return String::from(input.trim())
}

pub fn input_gender() -> String {

    let input = prompt("Please input a gender (male or female):", None);

    match input.as_str() { // converts to &str for match
        "male" => input,
        "female" => input,
        _ => prompt("Please input 'male' or 'female':", None)
    }
}