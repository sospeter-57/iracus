use crate::models::compartment::Compartment;
// use std::array::from_fn;
use crate::models::Status;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Quix {
    // pub compartments: [Compartment; 10],
    pub status: Status,
    pub compartments: Vec<Compartment>,
}

impl Quix {
    pub fn new() -> Self {
        let cmps = Vec::with_capacity(10);
        let status = Status::Open;
        // let cmps: [Compartment; 10] = from_fn(|_| Compartment::new(""));
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

        format!("\n^^^^^^ quix division info ^^^^^\n{}", compartments)
    }
}
