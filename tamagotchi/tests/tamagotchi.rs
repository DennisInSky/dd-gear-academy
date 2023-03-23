use gstd::prelude::*;
use gtest::{Program, System};
use tamagotchi_io::Tamagotchi;

const SENDER_ID: u64 = 100;
const TAMAGOTCHI_NAME: &str = "Dunya";

#[test]
fn tamagotchi_initializes() {
    let system = init_system();
    let program = Program::current(&system);

    let timestamp_before = system.block_timestamp();

    // The below code should be used instead by IDEA sends incorrectly encoded init message
    // so the contract has to treat it as bytes rather than a string
    //program.send(SENDER_ID, String::from(TAMAGOTCHI_NAME));
    let result = program.send_bytes(SENDER_ID, String::from(TAMAGOTCHI_NAME));
    assert!(result.log().is_empty());

    let tamagotchi: Tamagotchi = program
        .read_state()
        .expect("Unable to read tamagotchi state");

    let timestamp_after = system.block_timestamp();

    assert_eq!(TAMAGOTCHI_NAME, tamagotchi.name);
    assert!(
        timestamp_before <= tamagotchi.date_of_birth && tamagotchi.date_of_birth <= timestamp_after
    );
    assert!(9500 < tamagotchi.fed);
    assert!(9500 < tamagotchi.entertained);
    assert!(9500 < tamagotchi.rested);
}

#[test]
fn tamagotchi_replies_with_name() {
    let system = init_system();
    let program = Program::current(&system);
    // The below code should be used instead by IDEA sends incorrectly encoded init message
    // so the contract has to treat it as bytes rather than a string
    //program.send(SENDER_ID, String::from(TAMAGOTCHI_NAME));
    program.send_bytes(SENDER_ID, String::from(TAMAGOTCHI_NAME));

    let result = program.send(SENDER_ID, tamagotchi_io::TmgAction::Name);

    let log_entry = result
        .log()
        .iter()
        .find(|log_entry| log_entry.destination() == SENDER_ID.into())
        .expect("Unable to get tamagotchi reply");

    let tamagotchi_reply = tamagotchi_io::TmgReply::decode(&mut log_entry.payload())
        .expect("Unable to decode tamagotchi reply");
    if let tamagotchi_io::TmgReply::Name(tamagotchi_name) = tamagotchi_reply {
        assert_eq!(TAMAGOTCHI_NAME, tamagotchi_name.as_str());
    } else {
        unreachable!("Unexpected tamagotchi reply {:?}", tamagotchi_reply);
    }
}

#[test]
fn tamagotchi_replies_with_age() {
    let system = init_system();
    let program = Program::current(&system);
    let timestamp_before = system.block_timestamp();
    program.send(SENDER_ID, String::from(TAMAGOTCHI_NAME));

    let result = program.send(SENDER_ID, tamagotchi_io::TmgAction::Age);

    let timestamp_after = system.block_timestamp();

    let log_entry = result
        .log()
        .iter()
        .find(|log_entry| log_entry.destination() == SENDER_ID.into())
        .expect("Unable to get tamagotchi reply");

    let tamagotchi_reply = tamagotchi_io::TmgReply::decode(&mut log_entry.payload())
        .expect("Unable to decode tamagotchi reply");
    if let tamagotchi_io::TmgReply::Age(tamagotchi_age) = tamagotchi_reply {
        assert!(timestamp_before + tamagotchi_age <= timestamp_after);
    } else {
        unreachable!("Unexpected tamagotchi reply {:?}", tamagotchi_reply);
    }
}

fn init_system() -> System {
    let system = System::new();
    system.init_logger();
    system
}
