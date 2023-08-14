use crate::lifecycle::UPGRADE_BUFFER_SIZE;
use crate::memory::get_upgrades_memory;
use crate::take_state;
use ic_cdk_macros::pre_upgrade;
use ic_stable_structures::writer::{BufferedWriter, Writer};

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("Pre-upgrade starting");

    let state = take_state();

    let stable_state = state.data;

    let mut memory = get_upgrades_memory();

    let writer = BufferedWriter::new(UPGRADE_BUFFER_SIZE, Writer::new(&mut memory, 0));

    serializer::serialize(stable_state, writer).unwrap();
}
