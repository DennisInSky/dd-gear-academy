use gstd::{prelude::*, ActorId};

#[derive(Debug, Encode, Decode, TypeInfo, PartialEq, Clone)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Play,
    Sleep,
    TransferOwnership(ActorId),
    GrantOwnershipTransfer(ActorId),
    RevokeOwnershipTransfer,
    ApproveSpending {
        spendor_id: ActorId,
        amount: u128,
    },
    SetFTokenContract(ActorId),
    PurchaseAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
}
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum TmgReply {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    OwnershipTransferred(ActorId),
    OwnershipTransferGranted(ActorId),
    OwnershipTransferRevoked,
    SpendingApproved { spendor_id: ActorId, amount: u128 },
    ApproveSpendingError(String),
    FTokenContractSet,
    AttributePurchased(AttributeId),
    PurchaseAttributeInProgressError(AttributeId),
    PurchaseAttributeError(String),
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
    pub allowed_account: Option<ActorId>,
}

pub type AttributeId = u32;
