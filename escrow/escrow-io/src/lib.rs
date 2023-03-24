#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    DepositFunds,
    ConfirmDelivery,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum EscrowEvents {
    FundsDeposited,
    DeliveryConfirmed,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode, TypeInfo)]
pub enum EscrowState {
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct Escrow {
    seller: ActorId,
    buyer: ActorId,
    price: u128,
    state: EscrowState,
}

impl Escrow {
    pub fn new(seller: ActorId, buyer: ActorId, price: u128) -> Self {
        Escrow {
            seller,
            buyer,
            price,
            state: EscrowState::AwaitingPayment,
        }
    }

    pub fn deposit_funds(&mut self, buyer: ActorId, funds: u128) {
        assert_eq!(
            self.state,
            EscrowState::AwaitingPayment,
            "State must be `AwaitingPayment`"
        );
        assert_eq!(self.buyer, buyer, "The message sender must be a buyer");
        assert_eq!(
            self.price, funds,
            "The attached funds must be equal to the price set"
        );
        self.state = EscrowState::AwaitingDelivery
    }

    pub fn confirm_delivery(&mut self, buyer: ActorId) {
        assert_eq!(
            self.state,
            EscrowState::AwaitingDelivery,
            "State must be `AwaitingDelivery`"
        );
        assert_eq!(self.buyer, buyer, "The message sender must be a buyer");
        self.state = EscrowState::Closed
    }

    pub fn seller(&self) -> ActorId {
        self.seller
    }

    pub fn buyer(&self) -> ActorId {
        self.buyer
    }

    pub fn state<'a>(&'a self) -> &'a EscrowState {
        &self.state
    }

    pub fn price(&self) -> u128 {
        self.price
    }
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<InitEscrow, ()>;
    type Handle = InOut<EscrowAction, EscrowEvents>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = Escrow;
}
