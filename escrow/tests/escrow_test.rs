use escrow_io::{EscrowAction, EscrowEvents, InitEscrow};
use gtest::{Log, Program, System};
const BUYER: u64 = 100;
const SELLER: u64 = 101;
const PRICE: u128 = 100_000;
const ESCROW_ID: u64 = 1;

#[test]
fn deposit_funds() {
    let sys = System::new();
    init_escrow(&sys);
    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, PRICE);

    let res = escrow.send_with_value(BUYER, EscrowAction::DepositFunds, PRICE);
    let log = Log::builder()
        .dest(BUYER)
        .payload(EscrowEvents::FundsDeposited);
    assert!(res.contains(&log));

    let escrow_balance = sys.balance_of(ESCROW_ID);
    assert_eq!(escrow_balance, PRICE);
}

#[test]
fn confirm_delivery() {
    let sys = System::new();
    init_escrow(&sys);
    let escrow = sys.get_program(ESCROW_ID);
    sys.mint_to(BUYER, PRICE);
    escrow.send_with_value(BUYER, EscrowAction::DepositFunds, PRICE);

    let result = escrow.send(BUYER, EscrowAction::ConfirmDelivery);
    let log = Log::builder()
        .dest(BUYER)
        .payload(EscrowEvents::DeliveryConfirmed);
    assert!(result.contains(&log));

    let transfer_log = Log::builder().source(ESCROW_ID).dest(SELLER);
    sys.get_mailbox(SELLER).claim_value(transfer_log);
    assert_eq!(sys.balance_of(SELLER), PRICE);
    assert_eq!(sys.balance_of(ESCROW_ID), 0);
}

#[test]
fn deposit_failures() {
    let sys = System::new();
    init_escrow(&sys);

    let escrow = sys.get_program(ESCROW_ID);

    sys.mint_to(BUYER, 2 * PRICE);
    // must fail since BUYER attaches not enough value
    let res = escrow.send_with_value(BUYER, EscrowAction::DepositFunds, 2 * PRICE - 500);
    assert!(res.main_failed());

    // must fail since the message sender is not BUYER
    let res = escrow.send(SELLER, EscrowAction::DepositFunds);
    assert!(res.main_failed());

    // successful deposit
    let res = escrow.send_with_value(BUYER, EscrowAction::DepositFunds, PRICE);
    assert!(!res.main_failed());

    // must fail since the state must be `AwaitingPayment`
    let res = escrow.send_with_value(BUYER, EscrowAction::DepositFunds, PRICE);
    assert!(res.main_failed());
}

#[test]
fn confirm_delivery_failures() {
    let sys = System::new();
    init_escrow(&sys);
    let escrow = sys.get_program(ESCROW_ID);
    sys.mint_to(BUYER, 2 * PRICE);

    // must fail since BUYER has not deposited funds yet
    let res = escrow.send(BUYER, EscrowAction::ConfirmDelivery);
    assert!(res.main_failed());

    // must fail since the message sender is not BUYER
    escrow.send_with_value(BUYER, EscrowAction::DepositFunds, PRICE);
    let res = escrow.send(SELLER, EscrowAction::DepositFunds);
    assert!(res.main_failed());
}

fn init_escrow(sys: &System) {
    sys.init_logger();
    let escrow = Program::current(&sys);
    let res = escrow.send(
        SELLER,
        InitEscrow {
            seller: SELLER.into(),
            buyer: BUYER.into(),
            price: PRICE,
        },
    );
    assert!(res.log().is_empty());
}
