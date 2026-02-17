use std::fmt::Display;
use modular_bitfield::prelude::*;

fn main() {
    println!("UnionResult Size {}", core::mem::size_of::<UnionResult>());
    println!("UnionResult Ok {}", UnionResult::ok(42));
    println!("UnionResult Err {}", UnionResult::err(42));
}
#[derive(Specifier)]
pub enum Status {
    Ok, Err,
}
#[bitfield]
pub struct UnionResult {
    value: B31,
    #[bits = 1]
    status: Status,
}
impl Display for UnionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status() {
            Status::Ok => write!(f, "Ok({})", self.value()),
            Status::Err => write!(f, "Err({})", self.value()),
        }
    }
}
impl UnionResult {
    pub fn ok(value: u32) -> Self {
        Self::new()
            .with_value(value)
            .with_status(Status::Ok)
    }
    pub fn err(value: u32) -> Self {
        Self::new()
            .with_value(value)
            .with_status(Status::Err)
    }
}
