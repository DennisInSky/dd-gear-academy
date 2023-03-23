#![no_std]

use gstd::{msg, prelude::*, ActorId};

static mut TAMAGOTCHI: Option<tamagotchi::Tamagotchi> = None;
static mut FTOKEN_CONTRACT_ID: Option<ActorId> = None;
static mut APPROVE_SPENDING_TX_MANAGER: Option<tx_manager::TxManager<tamagotchi_io::TmgAction>> =
    None;
static mut PURCHASE_ATTRIBUTE_TX_MANAGER: Option<
    tx_manager::TxManager<tamagotchi_io::TmgAction, tamagotchi_io::AttributeId>,
> = None;

#[no_mangle]
extern "C" fn init() {
    // This should be used instead the below 2 strings, but IDEA sends incorrectly encoded
    // init message, so the contract just crashes on its decoding
    //let name: String = msg::load().expect("Unable to read init message");
    let name_vec = msg::load_bytes().expect("Unable to read init message");
    let name = String::from_utf8(name_vec).expect("Unable to convert init message");
    unsafe {
        TAMAGOTCHI = Some(tamagotchi::Tamagotchi::new(msg::source(), name));
        APPROVE_SPENDING_TX_MANAGER = Some(tx_manager::TxManager::new(42));
        PURCHASE_ATTRIBUTE_TX_MANAGER = Some(tx_manager::TxManager::new(45));
    }
}

#[gstd::async_main]
async fn main() {
    let actor_id = msg::source();
    let action: tamagotchi_io::TmgAction = msg::load().expect("Unable to read action message");
    let tamagotchi = unsafe { TAMAGOTCHI.as_mut().expect("Tamagotchi is not initialized") };
    ensure_action_is_authorized(actor_id, &action, tamagotchi);
    match action {
        tamagotchi_io::TmgAction::Name => {
            msg::reply(tamagotchi_io::TmgReply::Name(tamagotchi.name().into()), 0)
                .expect("Unable to reply with tamagotchi name");
        }
        tamagotchi_io::TmgAction::Age => {
            msg::reply(tamagotchi_io::TmgReply::Age(tamagotchi.age()), 0)
                .expect("Unable to reply with tamagotchi age");
        }
        tamagotchi_io::TmgAction::Feed => {
            tamagotchi.feed();
            msg::reply(tamagotchi_io::TmgReply::Fed, 0)
                .expect("Unable to reply with tamagotchi fed event");
        }
        tamagotchi_io::TmgAction::Play => {
            tamagotchi.play();
            msg::reply(tamagotchi_io::TmgReply::Entertained, 0)
                .expect("Unable to reply with tamagotchi entertained event");
        }
        tamagotchi_io::TmgAction::Sleep => {
            tamagotchi.sleep();
            msg::reply(tamagotchi_io::TmgReply::Slept, 0)
                .expect("Unable to reply with tamagotchi slept event");
        }
        tamagotchi_io::TmgAction::TransferOwnership(new_owner_id) => {
            tamagotchi.transfer_to(new_owner_id);
            msg::reply(
                tamagotchi_io::TmgReply::OwnershipTransferred(new_owner_id),
                0,
            )
            .expect("Unable to reply with tamagotchi transferred event");
        }
        tamagotchi_io::TmgAction::GrantOwnershipTransfer(transferor_id) => {
            tamagotchi.grant_transfer_permission(transferor_id);
            msg::reply(
                tamagotchi_io::TmgReply::OwnershipTransferGranted(transferor_id),
                0,
            )
            .expect("Unable to reply with tamagotchi transfer permission granted event");
        }
        tamagotchi_io::TmgAction::RevokeOwnershipTransfer => {
            tamagotchi.revoke_transfer_permission();
            msg::reply(tamagotchi_io::TmgReply::OwnershipTransferRevoked, 0)
                .expect("Unable to reply with tamagotchi transfer permission revoked event");
        }
        tamagotchi_io::TmgAction::SetFTokenContract(ftoken_contract_id) => {
            unsafe {
                FTOKEN_CONTRACT_ID = Some(ftoken_contract_id);
            }
            msg::reply(tamagotchi_io::TmgReply::FTokenContractSet, 0)
                .expect("Unable to reply with ftoken contract set");
        }
        tamagotchi_io::TmgAction::ApproveSpending {
            spendor_id: store_contract_id,
            amount,
        } => {
            let ftoken_contract_id = unsafe {
                FTOKEN_CONTRACT_ID
                    .as_ref()
                    .expect("FToken contract is not initialized")
            };
            let tx_manager = unsafe {
                APPROVE_SPENDING_TX_MANAGER
                    .as_mut()
                    .expect("Approve spending tx manager is not initialized")
            };

            let open_tx_result = tx_manager.open_tx(*ftoken_contract_id, action.clone(), (), false); // This is persisted in the first UoW.
            if let Err(open_tx_error) = open_tx_result {
                msg::reply(
                    tamagotchi_io::TmgReply::ApproveSpendingError(open_tx_error.into()),
                    0,
                )
                .expect("Unable to reply with ApproveSpendingError");
                return;
            }

            let call_result = msg::send_for_reply(
                *ftoken_contract_id,
                ft_main_io::FTokenAction::Message {
                    transaction_id: open_tx_result.unwrap(),
                    payload: ft_logic_io::Action::Approve {
                        approved_account: store_contract_id,
                        amount,
                    }
                    .encode(), // <- The interface lacks of being strongly typed - not good.
                },
                0,
            )
            .expect("Unable to send ApproveSpending")
            .await; // <-- The `send_for_reply` call splits the whole method into 2 UoWs which is not good/obvious to my POV.

            tx_manager
                .close_tx(*ftoken_contract_id, action, open_tx_result.unwrap()) // This is persisted in the second UoW.
                .unwrap_or_default(); // We swallow out of order or transaction closing.
            if let Err(contract_error) = call_result {
                msg::reply(
                    tamagotchi_io::TmgReply::ApproveSpendingError(format!("{:?}", contract_error)),
                    0,
                )
                .expect("Unable to reply with ApproveSpendingError");
                return;
            }

            msg::reply(
                tamagotchi_io::TmgReply::SpendingApproved {
                    spendor_id: store_contract_id,
                    amount,
                },
                0,
            )
            .expect("Unable to reply with SpendingApproved");
        }
        tamagotchi_io::TmgAction::PurchaseAttribute {
            store_id: store_contract_id,
            attribute_id,
        } => {
            let tx_manager = unsafe {
                PURCHASE_ATTRIBUTE_TX_MANAGER
                    .as_mut()
                    .expect("Purchase attribute tx manager is not initialized")
            };

            let open_tx_result =
                tx_manager.open_tx(store_contract_id, action.clone(), attribute_id, true);
            if open_tx_result.is_err() {
                msg::reply(
                    tamagotchi_io::TmgReply::PurchaseAttributeInProgressError(
                        *tx_manager.pending_tx_context(),
                    ),
                    0,
                )
                .expect("Unable to reply with PurchaseAttributeInProgressError");
                return;
            }

            let call_result = msg::send_for_reply(
                store_contract_id,
                store_io::StoreAction::BuyAttribute { attribute_id },
                0,
            )
            .expect("Unable to send PurchaseAttribute")
            .await;

            tx_manager
                .close_tx(store_contract_id, action, open_tx_result.unwrap())
                .unwrap_or_default();
            if let Err(contract_error) = call_result {
                msg::reply(
                    tamagotchi_io::TmgReply::PurchaseAttributeError(format!(
                        "{:?}",
                        contract_error
                    )),
                    0,
                )
                .expect("Unable to reply with PurchaseError");
                return;
            }

            msg::reply(tamagotchi_io::TmgReply::AttributePurchased(attribute_id), 0)
                .expect("Unable to reply with AttributePurchased");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
    msg::reply::<tamagotchi_io::Tamagotchi>(tamagotchi.into(), 0)
        .expect("Unable to reply with state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to reply with metahash");
}

fn ensure_action_is_authorized(
    actor_id: ActorId,
    action: &tamagotchi_io::TmgAction,
    tamagotchi: &tamagotchi::Tamagotchi,
) {
    match action {
        tamagotchi_io::TmgAction::Name | tamagotchi_io::TmgAction::Age => {}
        tamagotchi_io::TmgAction::Feed
        | tamagotchi_io::TmgAction::Play
        | tamagotchi_io::TmgAction::Sleep
        | tamagotchi_io::TmgAction::GrantOwnershipTransfer(_)
        | tamagotchi_io::TmgAction::RevokeOwnershipTransfer
        | tamagotchi_io::TmgAction::ApproveSpending { .. }
        | tamagotchi_io::TmgAction::SetFTokenContract(_)
        | tamagotchi_io::TmgAction::PurchaseAttribute { .. } => {
            assert_eq!(actor_id, tamagotchi.owner_id());
        }
        tamagotchi_io::TmgAction::TransferOwnership(_) => {
            assert!(
                actor_id == tamagotchi.owner_id() || tamagotchi.transferor_id() == Some(actor_id)
            );
        }
    }
}

mod tamagotchi;
mod tx_manager;
