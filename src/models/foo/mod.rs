use crate::models::compartment::Compartment;
// use std::array::from_fn;
use crate::models::Status;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Foo {
    // pub compartments: [Compartment; 280],
    pub status: Status,
    pub compartments: Vec<Compartment>,
}

impl Foo {
    pub fn new() -> Self {
        let cmps = Vec::with_capacity(280);
        let status = Status::Open;
        // let cmps: [Compartment; 280] = from_fn(|_| Compartment::new(""));
        Self {
            status,
            compartments: cmps,
        }
    }

    // pub fn add_secret(label: String, secret: Vec<u8>, bocal: &mut Bocal) {
    //     let compartment = Compartment {
    //         label, data: secret
    //     };
    // }
    //
    pub fn find_secret(&self, label: String) -> Option<Vec<u8>> {
        for cmpt in &self.compartments {
            if label == cmpt.label {
                return Some(cmpt.data.clone());
            }
        }
        None
    }

    pub fn info(&self) -> String {
        let compartments = self
            .compartments
            .iter()
            .map(|cmpt| cmpt.disp())
            .collect::<Vec<_>>()
            .join("\n");

        format!("\n----- > foo division info < ----\n{}", compartments)
    }
}
