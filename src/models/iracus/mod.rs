// use crate::models::compartment::Compartment;
// use crate::models::door::Door;
// use crate::utils::InputSecret;
// use crate::models::division::Division;
use crate::models::{foo::Foo, bar::Bar, zar::Zar, quix::Quix};

// use std::sync::Mutex;
#[derive(Debug)]
#[allow(dead_code)]
pub struct Iracus {
    foo: Foo,
    bar: Bar,
    zar: Zar,
    quix: Quix,
}

impl Iracus {
    // create a new instance of a vault
    pub fn new() -> Self {
        Self {
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

    pub fn info(&self) -> String {
        // let door_info = self.door.info();
        let division_info = format!(
            "{}\n{}\n{}\n{}",
            self.foo.info(),
            self.bar.info(),
            self.zar.info(),
            self.quix.info()
        );
        let iracus_info = self.iracus_info();
        let info = format!("{}\n\n{}", /*door_info,*/ division_info, iracus_info);
        println!("{}", info);
        "".to_owned()
    }

    fn iracus_info(&self) -> &str {
        "\n|||* v *  [bocal info] * v *|||"
    }
}
// 
// impl std::fmt::Display for Iracus {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Vault Bocal\n\t[Foo Divisions]\n{}[Bar Divisions]{}[Zar Divisions]{}[Quix Divisions]{}", self.foo, self.bar, self.zar, self.quix)
//     }
// }
