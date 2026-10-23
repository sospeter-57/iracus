use std::io::{Write, stdin, stdout};

use crate::models::division::Division;

pub struct InputSecret {
    pub label: String,
    pub secret: String,
    pub division: Option<Division>,
}

impl InputSecret {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            secret: String::new(),
            division: None,
        }
    }
    pub fn input(&mut self) {
        let secret = input_something("Enter Secret: : ");
        let label = input_something("Enter secret label: ");
        let division = input_something("Enter secret division [foo/bar/zar/quix]: ");

        if validate_division(&division) {
            self.label = label;
            self.secret = secret;
            self.division = {
                match division.as_str() {
                    "foo" => Some(Division::Foo),
                    "bar" => Some(Division::Bar),
                    "zar" => Some(Division::Zar),
                    "quix" => Some(Division::Quix),
                    _ => None,
                }
            }
        } else {
            panic!("invalid division selected. not in [foo/bar/zar/quix]")
        }
    }
}

pub fn input_something(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn validate_division(division: &str) -> bool {
    match division {
        "foo" => true,
        "bar" => true,
        "zar" => true,
        "quix" => true,
        _ => false,
    }
}
