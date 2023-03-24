// use gstd::ActorId;

// #[derive(Debug, PartialEq, Encode, Decode, TypeInfo)]
// enum EscrowState {
//     AwaitingPayment,
//     AwaitingDelivery,
//     Closed,
// }

// #[derive(Debug, Encode, Decode, TypeInfo)]
// pub struct Escrow {
//     seller: ActorId,
//     buyer: ActorId,
//     price: u128,
//     state: EscrowState,
// }

// impl Escrow {
//     pub fn new(seller: ActorId, buyer: ActorId, price: u128) -> Self {
//         Escrow {
//             seller,
//             buyer,
//             price,
//             state: EscrowState::AwaitingPayment,
//         }
//     }

//     pub fn deposit_funds(&mut self, buyer: ActorId, funds: u128) {
//         assert_eq!(
//             self.state,
//             EscrowState::AwaitingPayment,
//             "State must be `AwaitingPayment`"
//         );
//         assert_eq!(self.buyer, buyer, "The message sender must be a buyer");
//         assert_eq!(
//             self.price, funds,
//             "The attached funds must be equal to the price set"
//         );
//         self.state = EscrowState::AwaitingDelivery
//     }

//     pub fn confirm_delivery(&mut self) {
//         self.state = EscrowState::Closed
//     }
// }
