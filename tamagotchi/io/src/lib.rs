#![no_std]

pub use contract::*;
use gmeta::{InOut, Metadata};
use gstd::prelude::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<String, ()>;
    type Handle = InOut<TmgAction, TmgReply>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = Tamagotchi;
}

mod contract;
