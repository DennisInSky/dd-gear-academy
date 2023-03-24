use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Play,
    Sleep,
}
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
}
