// use std::env;

mod models;
use models::{iracus::Iracus, division::Division};
mod utils;
use std::io::{Write, stdin, stdout};
use utils::*;

fn main() {
    let mut vault = Iracus::new();

    //     let vars: Vec<String> = env::args().collect();
    //
    //     // // i've discovered a problem the or (||) operator can't solve, both operands being true
    //     // if (vars.len() == 2 && vars[2] != String::from("info")) || vars.len() != 3{
    //     //     panic!("either urgument or parameter not supplied")
    //     // }
    //
    //     if vars.len() == 2 && vars[1] != String::from("info") {
    //         panic!("that's definitely not the info command buddy")
    //     }
    //
    //     // if vars.len() != 3 {
    //     //     panic!("either urgument or paramter not supplied")
    //     // }
    //
    //     let command = &vars[1];
    //
    //     if command == "add" {
    //         let mut input_secret = InputSecret::new();
    //         input_secret.input();
    //         if vault.save_secret(&input_secret) {
    //             println!("{:?} saved successfully", &input_secret.label)
    //         } else {
    //             println!("failed to input secret to vault")
    //         }
    //     } else if command == "retrieve" {
    //         let search_label = &vars[2];
    //         let search_division = &vars[3];
    //         let search_division = match search_division.as_str() {
    //             "foo" => Some(Division::Foo),
    //             "bar" => Some(Division::Bar),
    //             "zar" => Some(Division::Zar),
    //             "quix" => Some(Division::Quix),
    //             _ => None
    //         };
    //         let search_division = search_division.unwrap();
    //         let secret = vault.retrieve_secret(search_label.to_owned(), search_division);
    //         println!("here's your long lived secret: {:?}", secret);
    //     } else if command == "info" {
    //         let info = vault.info();
    //         println!("{}", info)
    //     } else {
    //         panic!("unknown bocal command used")
    //     }

    let mut movex: String = String::new();

    loop {
        println!("  1. add secret");
        println!("  2. retrieve secret");
        println!("  3. view vault info");
        println!(" 0. quit");
        print!("Enter move: ");

        stdout().flush().unwrap();
        stdin().read_line(&mut movex).unwrap();
        let movex = movex.trim();

        match movex {
            "1" => {
                let mut input_secret = InputSecret::new();
                input_secret.input();
                if vault.save_secret(&input_secret) {
                    println!("{:?} saved successfully", &input_secret.label)
                } else {
                    println!("failed to input secret to vault")
                }
            }
            "2" => {
                let search_label = input_something("Enter secret Label: ");
                let search_division = input_something("Enter secret division: ");
                let search_division = match search_division.as_str() {
                    "foo" => Some(Division::Foo),
                    "bar" => Some(Division::Bar),
                    "zar" => Some(Division::Zar),
                    "quix" => Some(Division::Quix),
                    _ => None,
                };
                let search_division: Division = search_division.expect("that's not a valid division we have")
                    ;
                let secret = vault.retrieve_secret(search_label.to_owned(), search_division);
                println!("here's your long lived secret: {:?}", secret);
            }
            "3" => {
                let info = vault.info();
                println!("{}", info)
            }
            "0" => {
                break;
            }
            _ => panic!("unknown iracus command used"),
        }
    }
    // vault.open("password".to_owned())
    // println!("{:#?}", vault);
}
