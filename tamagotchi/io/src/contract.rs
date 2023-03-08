use gstd::prelude::*;

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
}
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
}
