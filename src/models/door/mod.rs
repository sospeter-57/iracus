use crate::models::Status;
use crate::models::compartment::Compartment;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Door {
    status: Status,
    compartment: Compartment,
}

impl Door {
    pub fn new() -> Self {
        let cmps = Compartment::new();
        Door {
            status: Status::Open,
            compartment: cmps,
        }
    }

    pub fn info(&self) -> String {
        format!("\n\t[[[ door info ]]]\n{}", self.compartment.disp())
    }
}
