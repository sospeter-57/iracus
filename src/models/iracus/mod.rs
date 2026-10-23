use crate::models::bar::Bar;
use crate::models::compartment::Compartment;
use crate::models::division::Division;
use crate::models::door::Door;
use crate::models::foo::Foo;
use crate::models::quix::Quix;
use crate::models::zar::Zar;
use crate::utils::InputSecret;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Iracus {
    pub door: Door,
    pub foo: Foo,
    pub bar: Bar,
    pub zar: Zar,
    pub quix: Quix,
}

impl Iracus {
    // create a new instance of a vault
    pub fn new() -> Self {
        Iracus {
            door: Door::new(),
            foo: Foo::new(),
            bar: Bar::new(),
            zar: Zar::new(),
            quix: Quix::new(),
        }
    }

    // initialize the vault
    // a pass phrase is needed
    pub fn open(&self, _pass_phrase: String) {
        if _pass_phrase != "password" {
            panic!("wrong pass phrase used: vault failed to open")
        } else {
            println!("pass phrase accepted")
        }
    }

    pub fn save_secret(&mut self, input_secret: &InputSecret) -> bool {
        if let Some(division) = &input_secret.division {
            let compartment = Compartment {
                label: input_secret.label.clone(),
                data: input_secret.secret.clone().into_bytes(),
            };
            match division {
                Division::Foo => {
                    // self.foo.add_secret(input_secret.label, input_secret.secret);
                    self.foo.compartments.push(compartment);
                    true
                }
                Division::Bar => {
                    self.bar.compartments.push(compartment);
                    true
                }
                Division::Zar => {
                    self.zar.compartments.push(compartment);
                    true
                }
                Division::Quix => {
                    self.quix.compartments.push(compartment);
                    true
                }
            }
        } else {
            false
        }
    }

    pub fn retrieve_secret(&self, label: String, division: Division) -> Vec<u8> {
        match division {
            Division::Bar => self.bar.find_secret(label).unwrap(), // handle this well later, it's critical in some way
            Division::Foo => self.foo.find_secret(label).unwrap(), // this too
            Division::Quix => self.quix.find_secret(label).unwrap(), // this too
            Division::Zar => self.zar.find_secret(label).unwrap(), // this too
        }
    }

    pub fn info(&self) -> String {
        let door_info = self.door.info();
        let division_info = format!(
            "{}\n{}\n{}\n{}",
            self.foo.info(),
            self.bar.info(),
            self.zar.info(),
            self.quix.info()
        );
        let bocal_info = self.bocal_info();
        let info = format!("{}\n\n{}\n\n{}", door_info, division_info, bocal_info);
        println!("{}", info);
        "".to_owned()
    }

    fn bocal_info(&self) -> &str {
        "\n|||* v *  [bocal info] * v *|||"
    }
}

// impl std::fmt::Display for Bocal {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Vault Bocal\n\t[Foo Divisions]\n{}[Bar Divisions]{}[Zar Divisions]{}[Quix Divisions]{}", self.foo, self.bar, self.zar, self.quix)
//     }
// }
