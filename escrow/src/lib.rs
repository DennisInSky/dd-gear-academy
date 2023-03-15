#![no_std]

use escrow_io::*;
use gstd::prelude::*;

mod contract;

static mut ESCROW: Option<Escrow> = None;

#[no_mangle]
extern "C" fn init() {
    let init_msg: InitEscrow = gstd::msg::load().expect("Unable to decode init message");
    let escrow = Escrow::new(init_msg.seller, init_msg.buyer, init_msg.price);
    unsafe { ESCROW = Some(escrow) }
}

#[no_mangle]
extern "C" fn handle() {
    let action_msg: EscrowAction = gstd::msg::load().expect("Unable to decode action message");
    let escrow = unsafe { ESCROW.as_mut().expect("The contract is not initialized") };
    match action_msg {
        EscrowAction::DepositFunds => {
            escrow.deposit_funds(gstd::msg::source(), gstd::msg::value());
            gstd::msg::reply(EscrowEvents::FundsDeposited, 0)
                .expect("Unable to send the FundsDeposited reply");
        }
        EscrowAction::ConfirmDelivery => {
            escrow.confirm_delivery(gstd::msg::source());
            gstd::msg::send(escrow.seller(), "", escrow.price())
                .expect("Unable to transfer funds to seller");
            gstd::msg::reply(EscrowEvents::DeliveryConfirmed, 0)
                .expect("Unable to send the DeliveryConfirmed reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let escrow = unsafe { ESCROW.as_ref().expect("The contract is not initialized") };
    gstd::msg::reply(escrow, 0).expect("Unable to reply with state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash = include!("../.metahash");
    gstd::msg::reply(metahash, 0).expect("Unable to reply with metahash");
}
