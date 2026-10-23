use crate::models::compartment::Compartment;
// use std::array::from_fn;
use crate::models::Status;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Bar {
    pub status: Status,
    pub compartments: Vec<Compartment>,
}

impl Bar {
    pub fn new() -> Self {
        let cmps: Vec<Compartment> = Vec::with_capacity(400);
        let status = Status::Open;
        // let cmps: [Compartment; 400] = from_fn(|_| Compartment::new(""));
        Self {
            status,
            compartments: cmps,
        }
    }

    pub fn find_secret(&self, label: String) -> Option<Vec<u8>> {
        for cmpt in &self.compartments {
            if cmpt.label == label {
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

        format!("\n||<><> bar division info <><> ||\n{}", compartments)
    }
}
