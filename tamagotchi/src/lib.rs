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
    unsafe { TAMAGOTCHI = Some(tamagotchi::Tamagotchi::new(name)) }
}

#[no_mangle]
extern "C" fn handle() {
    let action: tamagotchi_io::TmgAction = msg::load().expect("Unable to read action message");
    let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
    match action {
        tamagotchi_io::TmgAction::Name => {
            msg::reply(tamagotchi_io::TmgEvent::Name(tamagotchi.name().into()), 0)
                .expect("Unable to reply with tamagotchi name");
        }
        tamagotchi_io::TmgAction::Age => {
            msg::reply(tamagotchi_io::TmgEvent::Age(tamagotchi.age()), 0)
                .expect("Unable to reply with tamagotchi age");
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
    let metahash = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to reply with metahash");
}

mod tamagotchi;
