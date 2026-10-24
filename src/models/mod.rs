#![allow(dead_code)]
mod bar;
pub mod iracus;
pub mod compartment;
pub mod division;
mod door;
mod foo;
mod quix;
mod zar;
mod lock;
pub mod user;
pub mod insider;

#[derive(Debug)]
pub enum Status {
    Locked,
    Open,
}
