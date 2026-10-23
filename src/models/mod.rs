#![allow(dead_code)]
mod bar;
pub mod iracus;
mod compartment;
pub mod division;
mod door;
mod foo;
mod quix;
mod zar;

#[derive(Debug)]
pub enum Status {
    Locked,
    Open,
}
