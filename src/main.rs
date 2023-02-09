#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {
    let visitors_list = [
        Visitor::new("kwame", "Hello kwame , how are you?"),
        Visitor::new("kojo", "Hello kojo , what's up?"),
        Visitor::new("kwesi", "Hi kwesi , welcome back"),
    ];
    println!("Hello, what is your name?");
    let your_name = whats_your_name();
    let known_visitor = visitors_list.iter().find(|visitor| visitor.name == your_name);
    match known_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor's list"),
    }
}

fn whats_your_name() -> String {
    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username.trim().to_lowercase()
}

struct Visitor {
    name: String,
    greeting: String,
}
impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}
