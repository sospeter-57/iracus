use crate::models::{Status, compartment::Compartment};
// use std::array::from_fn;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Zar {
    // pub compartments: [Compartment; 50],
    pub status: Status,
    pub compartments: Vec<Compartment>,
}

impl Zar {
    pub fn new() -> Self {
        let status = Status::Open;
        let cmps = Vec::with_capacity(50);
        // let cmps = from_fn(|_| Compartment::new(""));
        Self {
            status,
            compartments: cmps,
        }
    }

    pub fn info(&self) -> String {
        let compartments = self
            .compartments
            .iter()
            .map(|cmpt| cmpt.disp())
            .collect::<Vec<_>>()
            .join("\n");

        format!("\n| = _ = zar division info = _ =\n{}", compartments)
    }
}
