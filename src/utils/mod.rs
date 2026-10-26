use std::io::{Write, stdin, stdout};
use std::path::Path;
use crate::models::{division::Division};

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
    pub fn input(&mut self, update: Option<&str>) {
        let secret: String = String::new();
        let labl: String = String::new();
        
        if update.is_some() {
             let secret = input_something("Enter New Secret Here: ");
              let labl = input_something("Enter new label: here: ");
        } else {
            let secret = input_something("Enter Secret Here: ");
             let labl = input_something("Enter secret label: ");
        }
        let division = input_something("Enter secret division [foo/bar/zar/quix]: ");

        if validate_division(&division) {
            self.label = labl;
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

use uuid::Uuid;

pub fn get_uid() -> String {
    Uuid::new_v4().to_string()
}

pub fn get_info() -> String {
    String::from("default infomation")
}

pub fn get_default_db_path<'a>() -> &'a Path {
    // let args: Vec<String> = std::env::args().collect();
    // let executable = args[0].clone();

    const PATH: &str = "iracus.db";
    use std::path::Path;

    let path = Path::new(PATH);
    // let project_root = executable.parent().and_then(|p| p.parent()).unwrap();
    // project_root
    path
}
