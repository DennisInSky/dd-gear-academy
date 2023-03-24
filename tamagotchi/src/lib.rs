#![no_std]

use gstd::{msg, prelude::*};

static mut TAMAGOTCHI: Option<tamagotchi::Tamagotchi> = None;

#[no_mangle]
extern "C" fn init() {
    // This should be used instead the below 2 strings, but IDEA sends incorrectly encoded
    // init message, so the contract just crashes on its decoding
    //let name: String = msg::load().expect("Unable to read init message");
    let name_vec = msg::load_bytes().expect("Unable to read init message");
    let name = String::from_utf8(name_vec).expect("Unable to convert init message");
    unsafe { TAMAGOTCHI = Some(tamagotchi::Tamagotchi::new(msg::source(), name)) }
}

#[no_mangle]
extern "C" fn handle() {
    let action: tamagotchi_io::TmgAction = msg::load().expect("Unable to read action message");
    let tamagotchi = unsafe { TAMAGOTCHI.as_mut().expect("Tamagotchi is not initialized") };
    match action {
        tamagotchi_io::TmgAction::Name => {
            msg::reply(tamagotchi_io::TmgEvent::Name(tamagotchi.name().into()), 0)
                .expect("Unable to reply with tamagotchi name");
        }
        tamagotchi_io::TmgAction::Age => {
            msg::reply(tamagotchi_io::TmgEvent::Age(tamagotchi.age()), 0)
                .expect("Unable to reply with tamagotchi age");
        }
        tamagotchi_io::TmgAction::Feed => {
            tamagotchi.feed();
            msg::reply(tamagotchi_io::TmgEvent::Fed, 0)
                .expect("Unable to reply with tamagochi fed event");
        }
        tamagotchi_io::TmgAction::Play => {
            tamagotchi.play();
            msg::reply(tamagotchi_io::TmgEvent::Entertained, 0)
                .expect("Unable to reply with tamagochi entertained event");
        }
        tamagotchi_io::TmgAction::Sleep => {
            tamagotchi.sleep();
            msg::reply(tamagotchi_io::TmgEvent::Slept, 0)
                .expect("Unable to reply with tamagochi slept event");
        }
        tamagotchi_io::TmgAction::Transfer(actor_id) => {
            tamagotchi.transfer(msg::source(), actor_id);
            msg::reply(tamagotchi_io::TmgEvent::Transfer(actor_id), 0)
                .expect("Unable to reply with tamagochi transferred event");
        }
        tamagotchi_io::TmgAction::Approve(transferor_actor_id) => {
            tamagotchi.grant_transfer_permission(msg::source(), transferor_actor_id);
            msg::reply(tamagotchi_io::TmgEvent::Approve(transferor_actor_id), 0)
                .expect("Unable to reply with tamagochi transfer permission granted event");
        }
        tamagotchi_io::TmgAction::RevokeApproval => {
            tamagotchi.revoke_transfer_permission(msg::source());
            msg::reply(tamagotchi_io::TmgEvent::RevokeApproval, 0)
                .expect("Unable to reply with tamagochi transfer permission revoked event");
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

mod tamagotchi;
