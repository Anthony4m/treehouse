#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {
    let mut visitors_list = vec![
        Visitor::new("kwame", VisitorAction::Accept,45),
        Visitor::new("kojo", VisitorAction::AcceptWithNote {
            note: String::from("Welcome to the vip section")},20),
        Visitor::new("kwesi",  VisitorAction::Refuse, 30),
    ];
    loop {
        println!("Hello, what is your name?");
        let your_name = whats_your_name();
        let known_visitor = visitors_list
            .iter()
            .find(|visitor| visitor.name == your_name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if your_name.is_empty() {
                    println!("The final list of visitors");
                    println!("{visitors_list:#?}", );
                    break;

                }
                println!("You are not on the visitor's list");
                let probator_age = probation_user_age();
                visitors_list.push(Visitor::new(&your_name, VisitorAction::Probation, probator_age));
                println!("{visitors_list:#?}", );

            }
        }

    }
}

fn whats_your_name() -> String {
    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username.trim().to_lowercase()
}
fn probation_user_age() -> u8 {
    println!("What is your age?");
    let mut your_age =  String::new();
    stdin()
        .read_line(&mut your_age)
        .expect("Failed to read line");
        your_age.trim().parse::<u8>().unwrap()
}
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: u8,
}
impl Visitor {
    fn new(name: &str,action:VisitorAction,age:u8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to treehouse, {0}", self.name),
            VisitorAction::AcceptWithNote {note} => {
                println!("Welcome to treehouse, {0}", self.name);
                println!("{note}");
                if self.age <= 21 {
                    println!("You cant have alcohol");
                }
            }
            VisitorAction::Probation => {
                println!("Welcome to treehouse, {0}, Note you are on probation", self.name);
            },
            VisitorAction::Refuse => println!("You are not welcome, {0}", self.name),
        }
    }
}
#[derive(PartialEq)]
#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
