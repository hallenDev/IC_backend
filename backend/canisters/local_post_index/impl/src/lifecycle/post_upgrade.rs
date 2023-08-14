use crate::lifecycle::{init_env, init_state, UPGRADE_BUFFER_SIZE};
use crate::memory::get_upgrades_memory;
use crate::Data;
use ic_cdk_macros::post_upgrade;
use ic_stable_structures::reader::{BufferedReader, Reader};

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Post-upgrade starting");
    let env = init_env();

    let memory = get_upgrades_memory();
    let reader = BufferedReader::new(UPGRADE_BUFFER_SIZE, Reader::new(&memory, 0));

    let data: Data = serializer::deserialize(reader).unwrap();

    init_state(env, data);
}
